/// Módulo CLI - Funciones relacionadas con la interfaz de línea de comandos
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Formato de reporte para exportación
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Json,
    Markdown,
    CodeClimate,
}

impl ReportFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(ReportFormat::Json),
            "markdown" | "md" => Some(ReportFormat::Markdown),
            "codeclimate" | "gitlab" => Some(ReportFormat::CodeClimate),
            _ => None,
        }
    }
}

/// Argumentos procesados de la línea de comandos
#[derive(Debug, Clone)]
pub struct CliArgs {
    /// Ruta del proyecto a analizar (None = modo interactivo)
    pub project_path: Option<String>,
    /// Activar modo watch
    pub watch_mode: bool,
    /// Activar modo fix (auto-reparación con IA)
    pub fix_mode: bool,
    /// Solo analizar archivos staged en git
    pub staged_mode: bool,
    /// Analizar solo archivos modificados (Git-based)
    pub incremental_mode: bool,
    /// Formato de reporte para exportar (json, markdown)
    pub report_format: Option<ReportFormat>,
    /// Ruta del archivo de salida para el reporte
    pub output_path: Option<String>,
    /// Disable analysis cache
    pub no_cache: bool,
    /// Ejecutar en segundo plano (modo daemon)
    pub daemon_mode: bool,
    /// Enable debug logging
    pub debug_mode: bool,
    /// Only validate configuration and exit
    pub check_mode: bool,
    /// Minimum severity to report (error, warning, info)
    pub min_severity: crate::config::Severity,
    /// Run init wizard to generate architect.json
    pub init_mode: bool,
    /// Overwrite existing architect.json (used with init)
    pub init_force: bool,
    /// Target directory for init (default: current dir)
    pub init_path: Option<String>,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            project_path: None,
            watch_mode: false,
            fix_mode: false,
            staged_mode: false,
            incremental_mode: false,
            report_format: None,
            output_path: None,
            no_cache: false,
            daemon_mode: false,
            debug_mode: false,
            check_mode: false,
            min_severity: crate::config::Severity::Info,
            init_mode: false,
            init_force: false,
            init_path: None,
        }
    }
}

/// Muestra la ayuda del CLI
pub fn print_help() {
    println!("architect-linter-pro {}", VERSION);
    println!();
    println!("Multi-language architecture linter with Architecture Health Score");
    println!("Supported languages: TypeScript, JavaScript, Python [beta], Go [beta], PHP [beta], Java [beta], C# [beta], Ruby [beta], Kotlin [beta], Rust [beta]");
    println!();
    println!("USAGE:");
    println!("  architect-linter-pro [OPTIONS] [PATH]");
    println!();
    println!("ARGUMENTS:");
    println!("  [PATH]    Project path to analyze (optional, interactive mode if omitted)");
    println!();
    println!("OPTIONS:");
    println!("  -h, --help           Show this help message");
    println!("  -v, --version        Show version information");
    println!("  -w, --watch          Watch mode: automatically re-analyze on file changes");
    println!("  -f, --fix            Fix mode: suggest and apply AI-powered automatic corrections");
    println!("  -s, --staged         Analyze only git staged files");
    println!("  -i, --incremental    Analyze only modified files (Git-based)");
    println!("  -r, --report <FMT>   Export report: json, markdown, codeclimate");
    println!("  -o, --output <PATH>  Output file for report");
    println!("  --no-cache           Disable analysis cache");
    println!("  --debug              Enable debug logging (verbose output)");
    println!("  --check              Validate configuration only and exit");
    println!("  --severity <LEVEL>   Minimum severity level: error, warning, info");
    println!("  init                 Generate architect.json wizard for your project");
    println!("    --force            Overwrite existing architect.json");
    println!("    --path <DIR>       Target directory (default: current directory)");
    println!();
    println!("EXAMPLES:");
    println!("  architect-linter-pro                         # Interactive mode");
    println!("  architect-linter-pro .                       # Analyze current directory");
    println!("  architect-linter-pro /path/to/project        # Analyze specific project");
    println!("  architect-linter-pro --watch .               # Watch mode for current directory");
    println!("  architect-linter-pro --fix .                 # Analyze and auto-fix with AI");
    println!("  architect-linter-pro --staged .              # Analyze staged files only");
    println!("  architect-linter-pro --incremental .        # Analyze modified files only");
    println!("  architect-linter-pro --report json .         # Export JSON report to stdout");
    println!(
        "  architect-linter-pro -r markdown -o report.md . # Export Markdown report to file"
    );
    println!("  architect-linter-pro init                  # Run wizard in current directory");
    println!("  architect-linter-pro init --force          # Overwrite existing config");
    println!("  architect-linter-pro init --path ./backend # Run wizard in subdirectory");
    println!();
    println!("INTERACTIVE WATCH MODE:");
    println!("  When running with --watch, type a command + Enter:");
    println!("    f  Fix: auto-fix violations with AI");
    println!("    r  Report: generate JSON report");
    println!("    m  Markdown: generate Markdown report");
    println!("    a  Analyze: full analysis with dashboard");
    println!("    v  Violations: list all current violations");
    println!("    d  Dashboard: show health score");
    println!("    c  Clear: clear terminal screen");
    println!("    h  Help: show available commands");
    println!("    q  Quit: exit watch mode");
    println!();
    println!("ARCHITECTURE HEALTH SCORE GRADES:");
    println!("  A (90-100)  - Excellent - Well-organized architecture");
    println!("  B (80-89)   - Good - Minor improvements needed");
    println!("  C (70-79)   - Fair - Some architectural concerns");
    println!("  D (60-69)   - Poor - Significant improvements needed");
    println!("  F (0-59)    - Critical - Serious architectural issues");
    println!();
    println!("DOCUMENTATION & SUPPORT:");
    println!("  https://github.com/sergiogswv/architect-linter-pro");
}

/// Muestra la versión del linter
pub fn print_version() {
    println!("architect-linter-pro {}", VERSION);
}

/// Procesa los argumentos de línea de comandos
/// Retorna None si se procesó un flag especial (--help, --version)
/// Retorna Some(CliArgs) si hay que continuar con el análisis
pub fn process_args() -> Option<CliArgs> {
    let args: Vec<String> = env::args().collect();

    let mut watch_mode = false;
    let mut fix_mode = false;
    let mut staged_mode = false;
    let mut incremental_mode = false;
    let mut no_cache = false;
    let mut daemon_mode = false;
    let mut debug_mode = false;
    let mut check_mode = false;
    let mut report_format: Option<ReportFormat> = None;
    let mut output_path: Option<String> = None;
    let mut min_severity = crate::config::Severity::Info;
    let mut project_path: Option<String> = None;
    let mut init_mode = false;
    let mut init_force = false;
    let mut init_path: Option<String> = None;

    // Procesar argumentos
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--version" | "-v" => {
                print_version();
                return None;
            }
            "--help" | "-h" => {
                print_help();
                return None;
            }
            "--watch" | "-w" => {
                watch_mode = true;
            }
            "--fix" | "-f" => {
                fix_mode = true;
            }
            "--staged" | "-s" => {
                staged_mode = true;
            }
            "--incremental" | "-i" => {
                incremental_mode = true;
            }
            "--no-cache" => {
                no_cache = true;
            }
            "--daemon" | "-d" => {
                daemon_mode = true;
            }
            "--debug" => {
                debug_mode = true;
            }
            "--check" => {
                check_mode = true;
            }
            "--report" | "-r" => {
                // Next argument should be the format
                if i + 1 < args.len() {
                    i += 1;
                    if let Some(fmt) = ReportFormat::from_str(&args[i]) {
                        report_format = Some(fmt);
                    } else {
                        eprintln!(
                            "Error: Formato de reporte inválido '{}'. Usa 'json', 'markdown' o 'codeclimate'.",
                            args[i]
                        );
                        return None;
                    }
                } else {
                    eprintln!("Error: --report requiere un formato (json o markdown)");
                    return None;
                }
            }
            "--output" | "-o" => {
                // Next argument should be the output path
                if i + 1 < args.len() {
                    i += 1;
                    output_path = Some(args[i].clone());
                } else {
                    eprintln!("Error: --output requiere una ruta de archivo");
                    return None;
                }
            }
            "--severity" => {
                if i + 1 < args.len() {
                    i += 1;
                    match args[i].to_lowercase().as_str() {
                        "error" => min_severity = crate::config::Severity::Error,
                        "warning" => min_severity = crate::config::Severity::Warning,
                        "info" => min_severity = crate::config::Severity::Info,
                        _ => {
                            eprintln!(
                                "Error: Severidad inválida '{}'. Usa 'error', 'warning' o 'info'.",
                                args[i]
                            );
                            return None;
                        }
                    }
                } else {
                    eprintln!("Error: --severity requiere un nivel (error, warning, info)");
                    return None;
                }
            }
            "init" => {
                init_mode = true;
            }
            "--force" => {
                init_force = true;
            }
            "--path" => {
                if i + 1 < args.len() {
                    i += 1;
                    init_path = Some(args[i].clone());
                } else {
                    eprintln!("Error: --path requiere una ruta de directorio");
                    return None;
                }
            }
            _ => {
                // Si no es un flag, asumimos que es la ruta del proyecto
                if !args[i].starts_with('-') {
                    project_path = Some(args[i].clone());
                }
            }
        }
        i += 1;
    }

    Some(CliArgs {
        project_path,
        watch_mode,
        fix_mode,
        staged_mode,
        incremental_mode,
        no_cache,
        daemon_mode,
        debug_mode,
        check_mode,
        report_format,
        output_path,
        min_severity,
        init_mode,
        init_force,
        init_path,
    })
}
