use indicatif::{ProgressBar, ProgressStyle};
use miette::{GraphicalReportHandler, IntoDiagnostic, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use swc_common::SourceMap;

mod ai;
mod analysis_result;
mod analyzer;
mod autofix;
mod cache;
mod circular;
mod cli;
mod config;
mod detector;
mod discovery;
mod git;
mod metrics;
mod output;
mod parsers;
mod report;
mod scoring;
mod ui;
mod watch;

fn main() -> Result<()> {
    // 1. Procesar argumentos de línea de comandos
    let cli_args = match cli::process_args() {
        Some(args) => args,
        None => return Ok(()), // Se procesó --help o --version
    };

    ui::print_banner();

    // 2. Obtener la ruta del proyecto
    let project_root = if let Some(ref path) = cli_args.project_path {
        PathBuf::from(path).canonicalize().into_diagnostic()?
    } else {
        ui::get_interactive_path()?
    };

    // 3. Cargar o crear configuración asistida por IA
    let ctx = Arc::new(config::setup_or_load_config(&project_root)?);

    let no_cache = cli_args.no_cache;

    // 4. Decidir entre modo normal, watch o fix
    if cli_args.fix_mode {
        run_fix_mode(&project_root, Arc::clone(&ctx))?;
    } else if cli_args.watch_mode {
        run_watch_mode(&project_root, Arc::clone(&ctx), no_cache)?;
    } else {
        run_normal_mode(&project_root, Arc::clone(&ctx), &cli_args)?;
    }

    Ok(())
}

/// Ejecuta el análisis en modo normal (una sola vez)
fn run_normal_mode(
    project_root: &PathBuf,
    ctx: Arc<config::LinterContext>,
    cli_args: &cli::CliArgs,
) -> Result<()> {
    // Recolectar archivos de todos los lenguajes soportados
    let mut files = discovery::collect_files(project_root, &ctx.ignored_paths);

    // Filter to staged files if --staged flag is set
    if cli_args.staged_mode {
        if !git::is_git_repo(project_root) {
            return Err(miette::miette!(
                "El flag --staged requiere un repositorio git."
            ));
        }
        files = git::filter_staged_files(&files, project_root)?;
        if files.is_empty() {
            println!("✅ No hay archivos staged para analizar.");
            return Ok(());
        }
        println!("🔍 Analizando {} archivos staged...", files.len());
    }

    // Mostrar información de directorios ignorados
    if !ctx.ignored_paths.is_empty() && !cli_args.staged_mode {
        println!("📂 Ignorando directorios: {}", ctx.ignored_paths.join(", "));
    }

    if files.is_empty() {
        println!("✅ No se encontraron archivos para analizar (TypeScript, JavaScript, Python, Go, PHP, Java).");
        return Ok(());
    }

    // Barra de progreso y Análisis Paralelo con Rayon
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .into_diagnostic()?,
    );
    pb.set_message("Analyzing...");

    let cm = Arc::new(SourceMap::default());

    // Load or create analysis cache
    let use_cache = !cli_args.no_cache;
    let config_hash = cache::hash_config(&ctx);
    let mut analysis_cache = if use_cache {
        cache::AnalysisCache::load(project_root, &config_hash)
            .unwrap_or_else(|| cache::AnalysisCache::new(config_hash.clone()))
    } else {
        cache::AnalysisCache::new(config_hash.clone())
    };

    // v4.0: Use aggregated analysis for scoring
    let mut analysis_result = analyzer::analyze_all_files(
        &files,
        project_root,
        ctx.pattern.clone(),
        &ctx,
        &cm,
        if use_cache { Some(&mut analysis_cache) } else { None },
    )?;

    // Save cache to disk
    if use_cache {
        if let Err(e) = analysis_cache.save(project_root) {
            eprintln!("⚠️  Could not save analysis cache: {}", e);
        }
    }

    // Análisis de Dependencias Cíclicas
    pb.set_message("Checking circular deps...");
    let cycles = circular::analyze_circular_dependencies(&files, project_root, &cm);

    match cycles {
        Ok(detected_cycles) => {
            for cycle in &detected_cycles {
                analysis_result.add_circular_dependency(cycle.clone());
            }
        }
        Err(e) => {
            eprintln!("⚠️  No se pudo analizar dependencias cíclicas: {}", e);
        }
    }

    pb.finish_and_clear();

    // Calculate health score
    let health_score = scoring::calculate(&analysis_result);
    analysis_result.health_score = Some(health_score.clone());

    // Handle report export if requested
    if let Some(format) = cli_args.report_format {
        let report_content = report::generate_report(&analysis_result, format);

        if let Some(output_path) = &cli_args.output_path {
            let path = std::path::Path::new(output_path);
            report::write_report(&report_content, path)?;
            println!("📄 Report saved to: {}", output_path);
        } else {
            report::write_stdout(&report_content)?;
        }

        // Exit with appropriate code
        if analysis_result.has_critical_issues() {
            std::process::exit(1);
        }
        return Ok(());
    }

    // Print dashboard
    output::print_dashboard(&analysis_result);

    // Print summary
    output::dashboard::print_summary(&analysis_result);

    // Print circular dependency details if any
    if !analysis_result.circular_dependencies.is_empty() {
        println!();
        circular::print_circular_dependency_report(&analysis_result.circular_dependencies);
    }

    // Exit with appropriate code
    if analysis_result.has_critical_issues() {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}

/// Run a full analysis and return the AnalysisResult (reused by watch commands)
fn run_full_analysis(
    project_root: &PathBuf,
    ctx: &config::LinterContext,
    analysis_cache: Option<&mut cache::AnalysisCache>,
) -> Result<analysis_result::AnalysisResult> {
    let files = discovery::collect_files(project_root, &ctx.ignored_paths);
    let cm = Arc::new(SourceMap::default());

    let mut analysis_result = analyzer::analyze_all_files(
        &files,
        project_root,
        ctx.pattern.clone(),
        ctx,
        &cm,
        analysis_cache,
    )?;

    // Circular dependencies
    match circular::analyze_circular_dependencies(&files, project_root, &cm) {
        Ok(detected_cycles) => {
            for cycle in &detected_cycles {
                analysis_result.add_circular_dependency(cycle.clone());
            }
        }
        Err(e) => {
            eprintln!("⚠️  No se pudo analizar dependencias cíclicas: {}", e);
        }
    }

    // Health score
    let health_score = scoring::calculate(&analysis_result);
    analysis_result.health_score = Some(health_score);

    Ok(analysis_result)
}

/// Run the AI auto-fix flow (reused by watch commands)
fn run_fix_flow(project_root: &PathBuf, ctx: &config::LinterContext) -> Result<()> {
    use dialoguer::Confirm;

    if ctx.ai_configs.is_empty() {
        println!("⚠️  No se encontró configuración de IA (.architect.ai.json).");
        println!("   El modo fix requiere configuración de IA para funcionar.");
        return Ok(());
    }

    let files = discovery::collect_files(project_root, &ctx.ignored_paths);
    if files.is_empty() {
        println!("✅ No se encontraron archivos para analizar.");
        return Ok(());
    }

    println!("📊 Analizando {} archivos...\n", files.len());

    let cm = Arc::new(SourceMap::default());
    let mut all_violations = Vec::new();

    for file_path in &files {
        match analyzer::collect_violations_from_file(&cm, file_path, ctx) {
            Ok(violations) => all_violations.extend(violations),
            Err(e) => eprintln!("⚠️  Error analizando {}: {}", file_path.display(), e),
        }
    }

    if all_violations.is_empty() {
        println!("✨ ¡No se encontraron violaciones! Tu código está perfecto.");
        return Ok(());
    }

    println!(
        "🔍 Encontradas {} violación(es) arquitectónicas\n",
        all_violations.len()
    );

    let mut fixed_count = 0;
    let mut skipped_count = 0;

    for (index, violation) in all_violations.iter().enumerate() {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Violación #{}/{}", index + 1, all_violations.len());
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📄 Archivo: {}", violation.file_path.display());
        println!("📍 Línea: {}", violation.line_number);
        println!(
            "🚫 Regla violada: '{}' no puede importar de '{}'",
            violation.rule.from, violation.rule.to
        );
        println!("💥 Import ofensivo: {}", violation.offensive_import);
        println!();

        println!("🤖 Consultando sugerencia de fix (usando sistema de fallback multimodelo)...");

        let runtime = tokio::runtime::Runtime::new().into_diagnostic()?;
        let suggestion = match runtime.block_on(autofix::suggest_fix(
            violation,
            project_root,
            &ctx.ai_configs,
        )) {
            Ok(s) => s,
            Err(_e) => {
                eprintln!(
                    "❌ No se pudo obtener ninguna sugerencia de los modelos configurados."
                );
                println!("⏭️  Saltando esta violación...\n");
                skipped_count += 1;
                continue;
            }
        };

        println!();
        println!(
            "💡 Sugerencia de la IA (confianza: {}):",
            suggestion.confidence
        );
        println!("{}", suggestion.explanation);
        println!();

        match &suggestion.fix_type {
            autofix::FixType::Refactor { old_code, new_code } => {
                println!("📝 Tipo: Refactorización de código");
                println!("Cambiar:");
                println!("  ❌ {}", old_code);
                println!("Por:");
                println!("  ✅ {}", new_code);
            }
            autofix::FixType::MoveFile { from, to } => {
                println!("📦 Tipo: Mover archivo");
                println!("  De: {}", from);
                println!("  A:  {}", to);
            }
            autofix::FixType::CreateInterface {
                interface_path,
                interface_code,
                updated_import,
            } => {
                println!("🎯 Tipo: Crear interfaz");
                println!("  Nueva interfaz: {}", interface_path);
                println!("  Código: {} líneas", interface_code.lines().count());
                println!("  Nuevo import: {}", updated_import);
            }
        }

        println!();

        let should_apply = Confirm::new()
            .with_prompt("¿Aplicar este fix?")
            .default(false)
            .interact()
            .into_diagnostic()?;

        if should_apply {
            match autofix::apply_fix(&suggestion, violation, project_root) {
                Ok(message) => {
                    println!("{}", message);
                    fixed_count += 1;
                }
                Err(e) => {
                    eprintln!("❌ Error aplicando fix: {}", e);
                    skipped_count += 1;
                }
            }
        } else {
            println!("⏭️  Fix omitido");
            skipped_count += 1;
        }

        println!();
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 RESUMEN");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Fixes aplicados: {}", fixed_count);
    println!("⏭️  Omitidos: {}", skipped_count);
    println!("📝 Total violaciones: {}", all_violations.len());
    println!();

    if fixed_count > 0 {
        println!("🎉 ¡Se aplicaron {} fix(es) exitosamente!", fixed_count);
        println!(
            "💡 Tip: Ejecuta el linter nuevamente para verificar que todo esté correcto."
        );
    }

    Ok(())
}

/// Ejecuta el análisis en modo watch (observación continua e interactiva)
fn run_watch_mode(project_root: &PathBuf, ctx: Arc<config::LinterContext>, no_cache: bool) -> Result<()> {
    println!("🚀 Iniciando modo watch...\n");

    // Análisis inicial completo
    let files = discovery::collect_files(project_root, &ctx.ignored_paths);

    if !ctx.ignored_paths.is_empty() {
        println!("📂 Ignorando directorios: {}", ctx.ignored_paths.join(", "));
    }

    if files.is_empty() {
        println!("✅ No se encontraron archivos para analizar (TypeScript, JavaScript, Python, Go, PHP, Java).");
        return Ok(());
    }

    println!("📊 Análisis inicial de {} archivos...", files.len());
    let cm = Arc::new(SourceMap::default());

    // Construir grafo de dependencias inicial
    let mut dep_analyzer = circular::CircularDependencyAnalyzer::new(project_root);
    dep_analyzer.build_graph(&files, &cm)?;

    // Análisis inicial de violaciones
    let mut error_count = 0;
    for file_path in &files {
        if let Err(e) = analyzer::analyze_file(&cm, file_path, &ctx) {
            error_count += 1;
            let mut out = String::new();
            let _ = GraphicalReportHandler::new().render_report(&mut out, e.as_ref());
            println!("\n📌 Violación en: {}", file_path.display());
            println!("{}", out);
        }
    }

    // Análisis de ciclos inicial
    let cycles = dep_analyzer.detect_cycles();
    if !cycles.is_empty() {
        circular::print_circular_dependency_report(&cycles);
        println!(
            "\n⚠️  Se encontraron {} dependencias cíclicas.",
            cycles.len()
        );
    }

    if error_count > 0 {
        println!(
            "\n❌ Se encontraron {} violaciones arquitectónicas.",
            error_count
        );
    } else {
        println!("\n✨ ¡Proyecto impecable! La arquitectura se respeta.");
    }

    println!();

    // Initialize analysis cache for watch session
    let config_hash = cache::hash_config(&ctx);
    let watch_cache: Arc<Mutex<Option<cache::AnalysisCache>>> = if no_cache {
        Arc::new(Mutex::new(None))
    } else {
        let c = cache::AnalysisCache::load(project_root, &config_hash)
            .unwrap_or_else(|| cache::AnalysisCache::new(config_hash));
        Arc::new(Mutex::new(Some(c)))
    };

    // Shared state for the watch loop
    let dep_analyzer = Arc::new(Mutex::new(dep_analyzer));
    let project_root_arc = Arc::new(project_root.clone());
    let ignored_paths = ctx.ignored_paths.clone();

    // Clone Arcs for the on_command closure
    let cmd_ctx = Arc::clone(&ctx);
    let cmd_project_root = Arc::clone(&project_root_arc);
    let cmd_cache = Arc::clone(&watch_cache);

    let change_cache = Arc::clone(&watch_cache);

    watch::start_watch_mode(
        project_root_arc.as_ref(),
        ignored_paths,
        // on_change: incremental analysis for file changes
        |changed_files| {
            let dep_analyzer = Arc::clone(&dep_analyzer);
            let ctx = Arc::clone(&ctx);
            let cm = Arc::clone(&cm);
            let project_root = Arc::clone(&project_root_arc);
            let change_cache = Arc::clone(&change_cache);

            // Invalidate changed files in cache
            {
                let mut cache_guard = change_cache.lock().unwrap();
                if let Some(ref mut c) = *cache_guard {
                    for file_path in changed_files {
                        let key = cache::AnalysisCache::normalize_path(file_path, &project_root);
                        c.remove(&key);
                    }
                }
            }

            let mut error_count = 0;
            for file_path in changed_files {
                if let Err(e) = analyzer::analyze_file(&cm, file_path, &ctx) {
                    error_count += 1;
                    let mut out = String::new();
                    let _ =
                        GraphicalReportHandler::new().render_report(&mut out, e.as_ref());
                    println!("\n📌 Violación en: {}", file_path.display());
                    println!("{}", out);
                }

                let mut dep_analyzer = dep_analyzer.lock().unwrap();
                if let Err(e) = dep_analyzer.update_file(file_path, &cm) {
                    eprintln!("⚠️  Error actualizando grafo: {}", e);
                    continue;
                }

                let normalized_path =
                    if let Ok(relative) = file_path.strip_prefix(project_root.as_ref()) {
                        relative
                            .to_string_lossy()
                            .replace('\\', "/")
                            .to_lowercase()
                    } else {
                        file_path
                            .to_string_lossy()
                            .replace('\\', "/")
                            .to_lowercase()
                    };

                let affected_nodes = dep_analyzer.get_affected_nodes(&normalized_path);

                if !affected_nodes.is_empty() {
                    let cycles = dep_analyzer.detect_cycles_in_subgraph(&affected_nodes);
                    if !cycles.is_empty() {
                        circular::print_circular_dependency_report(&cycles);
                        println!(
                            "\n⚠️  Se encontraron {} dependencias cíclicas.",
                            cycles.len()
                        );
                    }
                }
            }

            if error_count > 0 {
                println!(
                    "\n❌ Se encontraron {} violaciones arquitectónicas.",
                    error_count
                );
            } else {
                println!("\n✨ Todo correcto!");
            }

            Ok(())
        },
        // on_command: handle interactive commands
        |cmd| {
            let ctx = Arc::clone(&cmd_ctx);
            let project_root = cmd_project_root.as_ref();
            let cmd_cache = Arc::clone(&cmd_cache);

            // Helper: run full analysis with cache, then save
            let run_cached_analysis = |cache_arc: &Arc<Mutex<Option<cache::AnalysisCache>>>| -> Result<analysis_result::AnalysisResult> {
                let mut guard = cache_arc.lock().unwrap();
                let result = run_full_analysis(
                    project_root,
                    &ctx,
                    guard.as_mut(),
                )?;
                // Save cache after analysis
                if let Some(ref c) = *guard {
                    if let Err(e) = c.save(project_root) {
                        eprintln!("⚠️  Could not save analysis cache: {}", e);
                    }
                }
                Ok(result)
            };

            match cmd {
                watch::WatchCommand::Fix => {
                    println!("\n🔧 Ejecutando auto-fix con IA...\n");
                    run_fix_flow(project_root, &ctx)?;
                }
                watch::WatchCommand::ReportJson => {
                    println!("\n📄 Generando reporte JSON...\n");
                    let result = run_cached_analysis(&cmd_cache)?;
                    let content =
                        report::generate_report(&result, cli::ReportFormat::Json);
                    let path = project_root.join("report.json");
                    report::write_report(&content, &path)?;
                    println!("📄 Reporte guardado en: {}", path.display());
                }
                watch::WatchCommand::ReportMarkdown => {
                    println!("\n📄 Generando reporte Markdown...\n");
                    let result = run_cached_analysis(&cmd_cache)?;
                    let content =
                        report::generate_report(&result, cli::ReportFormat::Markdown);
                    let path = project_root.join("report.md");
                    report::write_report(&content, &path)?;
                    println!("📄 Reporte guardado en: {}", path.display());
                }
                watch::WatchCommand::FullAnalysis => {
                    println!("\n📊 Ejecutando análisis completo...\n");
                    let result = run_cached_analysis(&cmd_cache)?;
                    output::print_dashboard(&result);
                    output::dashboard::print_summary(&result);
                    if !result.circular_dependencies.is_empty() {
                        println!();
                        circular::print_circular_dependency_report(
                            &result.circular_dependencies,
                        );
                    }
                }
                watch::WatchCommand::Violations => {
                    println!("\n🔍 Escaneando todas las violaciones...\n");
                    let result = run_cached_analysis(&cmd_cache)?;

                    if result.violations.is_empty()
                        && result.circular_dependencies.is_empty()
                    {
                        println!(
                            "✨ ¡No se encontraron violaciones! La arquitectura se respeta."
                        );
                    } else {
                        // Violations
                        if !result.violations.is_empty() {
                            println!(
                                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
                            );
                            println!(
                                "  🚫 {} violación(es) arquitectónicas",
                                result.violations.len()
                            );
                            println!(
                                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
                            );
                            for (i, cv) in result.violations.iter().enumerate() {
                                let icon = match cv.category {
                                    analysis_result::ViolationCategory::Blocked => {
                                        "❌"
                                    }
                                    analysis_result::ViolationCategory::Warning => {
                                        "⚠️"
                                    }
                                    analysis_result::ViolationCategory::Info => "ℹ️",
                                };
                                println!(
                                    "\n  {} #{} [{}]",
                                    icon,
                                    i + 1,
                                    cv.category.as_str().to_uppercase()
                                );
                                println!(
                                    "     📄 {}:{}",
                                    cv.violation.file_path.display(),
                                    cv.violation.line_number
                                );
                                println!(
                                    "     🚫 '{}' no puede importar de '{}'",
                                    cv.violation.rule.from, cv.violation.rule.to
                                );
                                println!(
                                    "     💥 {}",
                                    cv.violation.offensive_import
                                );
                            }
                            println!();
                        }

                        // Circular dependencies
                        if !result.circular_dependencies.is_empty() {
                            circular::print_circular_dependency_report(
                                &result.circular_dependencies,
                            );
                        }

                        // Summary line
                        println!(
                            "📊 Total: {} violaciones, {} dependencias cíclicas",
                            result.violations.len(),
                            result.circular_dependencies.len()
                        );
                    }
                }
                watch::WatchCommand::Dashboard => {
                    println!("\n📊 Calculando health score...\n");
                    let result = run_cached_analysis(&cmd_cache)?;
                    output::print_dashboard(&result);
                }
                watch::WatchCommand::Clear => {
                    // ANSI escape to clear screen and move cursor to top
                    print!("\x1B[2J\x1B[1;1H");
                    println!("👁️  Modo Watch activo");
                }
                watch::WatchCommand::Help => {
                    println!();
                    watch::print_watch_help();
                }
                watch::WatchCommand::Quit => {
                    return Ok(false);
                }
            }

            Ok(true)
        },
    )?;

    Ok(())
}

/// Ejecuta el análisis en modo fix (auto-reparación con IA)
fn run_fix_mode(project_root: &PathBuf, ctx: Arc<config::LinterContext>) -> Result<()> {
    println!("🔧 Modo Fix: Auto-reparación con IA\n");

    if ctx.ai_configs.is_empty() {
        return Err(miette::miette!(
            "No se encontró configuración de IA (.architect.ai.json).\n\
             El modo --fix requiere configuración de IA para funcionar."
        ));
    }

    run_fix_flow(project_root, &ctx)
}
