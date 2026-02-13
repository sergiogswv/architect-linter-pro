use miette::{IntoDiagnostic, Result};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::time::{Duration, Instant};

/// Commands available in interactive watch mode
#[derive(Debug, Clone)]
pub enum WatchCommand {
    Fix,            // 'f' - Run AI auto-fix
    ReportJson,     // 'r' - Generate JSON report
    ReportMarkdown, // 'm' - Generate Markdown report
    FullAnalysis,   // 'a' - Full analysis with dashboard
    Violations,     // 'v' - List all current violations
    Dashboard,      // 'd' - Show health score
    Clear,          // 'c' - Clear screen
    Help,           // 'h' - Show commands
    Quit,           // 'q' - Exit
}

/// Print the interactive watch mode help banner
pub fn print_watch_help() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Interactive Watch Mode Commands");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  f + Enter  →  Fix: auto-fix violations with AI");
    println!("  r + Enter  →  Report: generate JSON report");
    println!("  m + Enter  →  Markdown: generate Markdown report");
    println!("  a + Enter  →  Analyze: full analysis with dashboard");
    println!("  v + Enter  →  Violations: list all current violations");
    println!("  d + Enter  →  Dashboard: show health score");
    println!("  c + Enter  →  Clear: clear terminal screen");
    println!("  h + Enter  →  Help: show this menu");
    println!("  q + Enter  →  Quit: exit watch mode");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    let _ = std::io::stdout().flush();
}

/// Parse a single character into a WatchCommand
fn parse_command(input: &str) -> Option<WatchCommand> {
    match input.trim().to_lowercase().as_str() {
        "f" => Some(WatchCommand::Fix),
        "r" => Some(WatchCommand::ReportJson),
        "m" => Some(WatchCommand::ReportMarkdown),
        "a" => Some(WatchCommand::FullAnalysis),
        "v" => Some(WatchCommand::Violations),
        "d" => Some(WatchCommand::Dashboard),
        "c" => Some(WatchCommand::Clear),
        "h" => Some(WatchCommand::Help),
        "q" => Some(WatchCommand::Quit),
        _ => None,
    }
}

/// Internal event used to unify file watcher and stdin into one channel
enum InternalEvent {
    FileNotify(notify::Result<Event>),
    UserInput(WatchCommand),
}

/// Spawn a thread that reads lines from stdin and sends parsed commands.
/// Uses `read_line` per iteration instead of holding a persistent `stdin.lock()`
/// to avoid blocking stdin on Windows.
fn spawn_stdin_reader(tx: Sender<InternalEvent>) {
    std::thread::spawn(move || {
        loop {
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(0) => break, // EOF — stdin closed
                Ok(_) => {
                    if let Some(cmd) = parse_command(&input) {
                        if tx.send(InternalEvent::UserInput(cmd)).is_err() {
                            break; // Channel closed, main loop exited
                        }
                    } else if !input.trim().is_empty() {
                        eprintln!(
                            "Unknown command: '{}'. Press h + Enter for help.",
                            input.trim()
                        );
                    }
                }
                Err(_) => break,
            }
        }
    });
}

/// Inicia el modo watch interactivo para observación continua
pub fn start_watch_mode<F, G>(
    project_root: &Path,
    ignored_paths: Vec<String>,
    mut on_change: F,
    mut on_command: G,
) -> Result<()>
where
    F: FnMut(&[PathBuf]) -> Result<()>,
    G: FnMut(WatchCommand) -> Result<bool>, // returns false to quit
{
    println!("👁️  Modo Watch activado");
    println!("📂 Observando: {}", project_root.display());

    if !ignored_paths.is_empty() {
        println!("🚫 Ignorando: {}", ignored_paths.join(", "));
    }

    println!("⏱️  Debounce: 300ms\n");

    print_watch_help();

    // Unified channel for both file events and user commands
    let (unified_tx, unified_rx) = channel::<InternalEvent>();

    // Setup file watcher sending to the unified channel
    let file_tx = unified_tx.clone();
    let _watcher = {
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = file_tx.send(InternalEvent::FileNotify(res));
            },
            Config::default(),
        )
        .into_diagnostic()?;

        watcher
            .watch(project_root, RecursiveMode::Recursive)
            .into_diagnostic()?;

        watcher // keep alive by binding to _watcher
    };

    // Spawn stdin reader thread
    spawn_stdin_reader(unified_tx);

    // Build file-filtering helpers (reuse FileWatcher logic inline)
    let project_root_buf = project_root.to_path_buf();
    let ignored = ignored_paths.clone();
    let debounce_duration = Duration::from_millis(300);

    let is_relevant_file = |path: &Path| -> bool {
        if let Some(ext) = path.extension() {
            matches!(
                ext.to_str(),
                Some("ts") | Some("tsx") | Some("js") | Some("jsx")
            )
        } else {
            false
        }
    };

    let should_ignore_path = |path: &Path| -> bool {
        let relative_path = path
            .strip_prefix(&project_root_buf)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        for pattern in &ignored {
            let normalized_pattern = pattern.replace('\\', "/");
            if relative_path.contains(normalized_pattern.trim_end_matches('/'))
                || relative_path.starts_with(&normalized_pattern)
                || relative_path
                    .starts_with(&format!("{}/", normalized_pattern.trim_end_matches('/')))
            {
                return true;
            }
        }
        false
    };

    let is_relevant_event = |event: &Event| -> bool {
        matches!(
            event.kind,
            EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
        )
    };

    print!("👁️  Esperando cambios o comandos... (h + Enter = ayuda)\n> ");
    let _ = std::io::stdout().flush();

    // Main loop with debouncing for file events
    let mut changed_files: Vec<PathBuf> = Vec::new();
    let mut last_event_time = Instant::now();

    loop {
        match unified_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(InternalEvent::FileNotify(Ok(event))) => {
                if is_relevant_event(&event) {
                    for path in &event.paths {
                        if is_relevant_file(path) && !should_ignore_path(path) {
                            if !changed_files.contains(path) {
                                changed_files.push(path.clone());
                            }
                        }
                    }
                    last_event_time = Instant::now();
                }
            }
            Ok(InternalEvent::FileNotify(Err(e))) => {
                eprintln!("⚠️  Error en file watcher: {:?}", e);
            }
            Ok(InternalEvent::UserInput(cmd)) => {
                match on_command(cmd) {
                    Ok(true) => {
                        print!("\n👁️  Esperando cambios o comandos...\n> ");
                        let _ = std::io::stdout().flush();
                    }
                    Ok(false) => {
                        // Quit requested
                        println!("👋 Saliendo del modo watch...");
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("❌ Error ejecutando comando: {}", e);
                        print!("\n👁️  Esperando cambios o comandos...\n> ");
                        let _ = std::io::stdout().flush();
                    }
                }
            }
            Err(_) => {
                // Timeout — check if debounced file changes are ready
            }
        }

        // Process accumulated file changes after debounce period
        if !changed_files.is_empty() && last_event_time.elapsed() >= debounce_duration {
            println!(
                "\n🔄 Cambios detectados en {} archivo(s):",
                changed_files.len()
            );
            for file in &changed_files {
                println!("   📝 {}", file.display());
            }
            println!();

            if let Err(e) = on_change(&changed_files) {
                eprintln!("❌ Error durante re-análisis: {}", e);
            } else {
                println!("✅ Re-análisis completado");
            }

            changed_files.clear();
            print!("\n👁️  Esperando cambios o comandos...\n> ");
            let _ = std::io::stdout().flush();
        }
    }
}
