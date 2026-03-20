use axum::{
    routing::post,
    Json, Router, extract::State,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::path::Path;
use crate::agent_config::AgentConfig;
use crate::agent_models::{OrchestratorCommand, CommandAck};
use crate::agent_reporter::report_event;
use std::collections::HashMap;

// Estado compartido para el servidor
pub struct ServerState {
    pub config: AgentConfig,
}

pub async fn start_server(config: AgentConfig) -> anyhow::Result<()> {
    let state = Arc::new(ServerState {
        config: config.clone(),
    });

    let app = Router::new()
        .route("/command", post(handle_command))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("🚀 Architect Agente escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    // 🧠 Enviar evento architect_ready al Cerebro DESPUÉS de vincular el puerto
    // pero antes de empezar a servir
    println!("📨 Enviando evento 'architect_ready' al Cerebro...");
    match report_event(
        &config,
        "architect_ready",
        "info",
        HashMap::from([
            ("version".to_string(), serde_json::json!("6.0.0")),
            ("port".to_string(), serde_json::json!(config.port)),
        ]).into_iter().collect(),
    ).await {
        Ok(_) => println!("✅ Evento architect_ready enviado exitosamente"),
        Err(e) => eprintln!("❌ Error enviando evento architect_ready: {}", e),
    }

    // Spawnear el servidor en una tarea separada
    let server_task = tokio::spawn(async move {
        axum::serve(listener, app).await
    });

    // Esperar un poco para asegurar que el evento se envió
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    server_task.await??;

    Ok(())
}

async fn handle_command(
    State(state): State<Arc<ServerState>>,
    Json(cmd): Json<OrchestratorCommand>,
) -> Json<CommandAck> {
    let config = &state.config;
    let target = cmd.target.unwrap_or_else(|| ".".to_string());

    println!("📨 Comando recibido: action={} target={}", cmd.action, target);

    match cmd.action.as_str() {
        // Comandos que ejecutan análisis real
        "lint" | "analyze" => {
            // Enviar evento de inicio
            let _ = report_event(
                config,
                "command_lint_started",
                "info",
                HashMap::from([
                    ("action".to_string(), serde_json::json!("lint")),
                    ("target".to_string(), serde_json::json!(&target)),
                ]).into_iter().collect(),
            ).await;

            // Ejecutar análisis
            match run_lint_analysis(&target) {
                Ok(result) => {
                    // Enviar evento de completado con resultados
                    let _ = report_event(
                        config,
                        "command_lint_completed",
                        "info",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("lint")),
                            ("target".to_string(), serde_json::json!(&target)),
                            ("findings_count".to_string(), serde_json::json!(result.findings_count)),
                            ("health_score".to_string(), serde_json::json!(result.health_score)),
                            ("findings".to_string(), serde_json::json!(result.findings)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "completed".to_string(),
                        result: Some(serde_json::json!({
                            "action": "lint",
                            "target": target,
                            "findings_count": result.findings_count,
                            "health_score": result.health_score,
                            "findings": result.findings,
                        })),
                        error: None,
                    })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let _ = report_event(
                        config,
                        "command_lint_error",
                        "error",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("lint")),
                            ("error".to_string(), serde_json::json!(&error_msg)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(error_msg),
                    })
                }
            }
        }

        // Análisis profundo de arquitectura
        "deep-analysis" => {
            let _ = report_event(
                config,
                "command_deep_analysis_started",
                "info",
                HashMap::from([
                    ("action".to_string(), serde_json::json!("deep-analysis")),
                    ("target".to_string(), serde_json::json!(&target)),
                ]).into_iter().collect(),
            ).await;

            match run_deep_analysis(&target) {
                Ok(result) => {
                    let _ = report_event(
                        config,
                        "command_deep_analysis_completed",
                        "info",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("deep-analysis")),
                            ("target".to_string(), serde_json::json!(&target)),
                            ("god_services".to_string(), serde_json::json!(result.god_services)),
                            ("circular_deps".to_string(), serde_json::json!(result.circular_deps)),
                            ("high_coupling".to_string(), serde_json::json!(result.high_coupling)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "completed".to_string(),
                        result: Some(serde_json::json!({
                            "action": "deep-analysis",
                            "target": target,
                            "god_services": result.god_services,
                            "circular_deps": result.circular_deps,
                            "high_coupling": result.high_coupling,
                        })),
                        error: None,
                    })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let _ = report_event(
                        config,
                        "command_deep_analysis_error",
                        "error",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("deep-analysis")),
                            ("error".to_string(), serde_json::json!(&error_msg)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(error_msg),
                    })
                }
            }
        }

        // Detectar dependencias circulares
        "check-circular" => {
            let _ = report_event(
                config,
                "command_check_circular_started",
                "info",
                HashMap::from([
                    ("action".to_string(), serde_json::json!("check-circular")),
                    ("target".to_string(), serde_json::json!(&target)),
                ]).into_iter().collect(),
            ).await;

            match run_circular_check(&target) {
                Ok(result) => {
                    let _ = report_event(
                        config,
                        "command_check_circular_completed",
                        "info",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("check-circular")),
                            ("target".to_string(), serde_json::json!(&target)),
                            ("cycles".to_string(), serde_json::json!(result.cycles)),
                            ("has_cycles".to_string(), serde_json::json!(result.has_cycles)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "completed".to_string(),
                        result: Some(serde_json::json!({
                            "action": "check-circular",
                            "target": target,
                            "cycles": result.cycles,
                            "has_cycles": result.has_cycles,
                        })),
                        error: None,
                    })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let _ = report_event(
                        config,
                        "command_check_circular_error",
                        "error",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("check-circular")),
                            ("error".to_string(), serde_json::json!(&error_msg)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(error_msg),
                    })
                }
            }
        }

        // Generar reporte completo
        "full-report" => {
            let _ = report_event(
                config,
                "command_full_report_started",
                "info",
                HashMap::from([
                    ("action".to_string(), serde_json::json!("full-report")),
                    ("target".to_string(), serde_json::json!(&target)),
                ]).into_iter().collect(),
            ).await;

            match run_full_report(&target) {
                Ok(result) => {
                    let _ = report_event(
                        config,
                        "command_full_report_completed",
                        "info",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("full-report")),
                            ("target".to_string(), serde_json::json!(&target)),
                            ("health_score".to_string(), serde_json::json!(result.health_score)),
                            ("metrics".to_string(), serde_json::json!(result.metrics)),
                            ("violations".to_string(), serde_json::json!(result.violations)),
                            ("suggestions".to_string(), serde_json::json!(result.suggestions)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "completed".to_string(),
                        result: Some(serde_json::json!({
                            "action": "full-report",
                            "target": target,
                            "health_score": result.health_score,
                            "metrics": result.metrics,
                            "violations": result.violations,
                            "suggestions": result.suggestions,
                        })),
                        error: None,
                    })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let _ = report_event(
                        config,
                        "command_full_report_error",
                        "error",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("full-report")),
                            ("error".to_string(), serde_json::json!(&error_msg)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(error_msg),
                    })
                }
            }
        }

        // Validar configuración
        "validate-config" => {
            let _ = report_event(
                config,
                "command_validate_config_started",
                "info",
                HashMap::from([
                    ("action".to_string(), serde_json::json!("validate-config")),
                ]).into_iter().collect(),
            ).await;

            match run_config_validation(&target) {
                Ok(result) => {
                    let _ = report_event(
                        config,
                        "command_validate_config_completed",
                        "info",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("validate-config")),
                            ("valid".to_string(), serde_json::json!(result.valid)),
                            ("message".to_string(), serde_json::json!(&result.message)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "completed".to_string(),
                        result: Some(serde_json::json!({
                            "action": "validate-config",
                            "valid": result.valid,
                            "message": result.message,
                        })),
                        error: None,
                    })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let _ = report_event(
                        config,
                        "command_validate_config_error",
                        "error",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("validate-config")),
                            ("error".to_string(), serde_json::json!(&error_msg)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(error_msg),
                    })
                }
            }
        }

        // Análisis de archivos stale
        "analyze-stale" => {
            let _ = report_event(
                config,
                "command_analyze_stale_started",
                "info",
                HashMap::from([
                    ("action".to_string(), serde_json::json!("analyze-stale")),
                    ("target".to_string(), serde_json::json!(&target)),
                ]).into_iter().collect(),
            ).await;

            match run_stale_analysis(&target) {
                Ok(result) => {
                    let _ = report_event(
                        config,
                        "command_analyze_stale_completed",
                        "info",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("analyze-stale")),
                            ("target".to_string(), serde_json::json!(&target)),
                            ("stale_files".to_string(), serde_json::json!(result.stale_files)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "completed".to_string(),
                        result: Some(serde_json::json!({
                            "action": "analyze-stale",
                            "target": target,
                            "stale_files": result.stale_files,
                        })),
                        error: None,
                    })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let _ = report_event(
                        config,
                        "command_analyze_stale_error",
                        "error",
                        HashMap::from([
                            ("action".to_string(), serde_json::json!("analyze-stale")),
                            ("error".to_string(), serde_json::json!(&error_msg)),
                        ]).into_iter().collect(),
                    ).await;

                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(error_msg),
                    })
                }
            }
        }

        // Status del agente
        "status" => {
            Json(CommandAck {
                request_id: cmd.request_id,
                status: "completed".to_string(),
                result: Some(serde_json::json!({
                    "agent": "architect",
                    "version": "6.0.0",
                    "ready": true,
                    "port": config.port,
                })),
                error: None,
            })
        }

        // Acción desconocida
        _ => {
            Json(CommandAck {
                request_id: cmd.request_id,
                status: "rejected".to_string(),
                result: None,
                error: Some(format!("Acción desconocida: {}", cmd.action)),
            })
        }
    }
}

// Estructuras de resultado para cada tipo de análisis
struct LintResult {
    findings_count: usize,
    health_score: String,
    findings: Vec<String>,
}

struct DeepAnalysisResult {
    god_services: Vec<String>,
    circular_deps: Vec<String>,
    high_coupling: Vec<String>,
}

struct CircularCheckResult {
    cycles: Vec<String>,
    has_cycles: bool,
}

struct FullReportResult {
    health_score: String,
    metrics: HashMap<String, serde_json::Value>,
    violations: Vec<String>,
    suggestions: Vec<String>,
}

struct ConfigValidationResult {
    valid: bool,
    message: String,
}

struct StaleAnalysisResult {
    stale_files: Vec<String>,
}

// Funciones de análisis reales usando los módulos existentes

fn run_lint_analysis(target: &str) -> Result<LintResult, Box<dyn std::error::Error + Send + Sync>> {
    let project_root = Path::new(target);

    // Intentar cargar configuración silenciosamente
    let config_path = project_root.join("architect.json");
    if !config_path.exists() {
        return Ok(LintResult {
            findings_count: 0,
            health_score: "N/A".to_string(),
            findings: vec!["⚠️ No se encontró architect.json en el proyecto".to_string()],
        });
    }

    let ctx = crate::config::load_config(project_root)
        .map_err(|e| format!("Error loading config: {}", e))?;

    // Recolectar archivos
    let files = crate::discovery::collect_files(project_root, &ctx.ignored_paths);

    if files.is_empty() {
        return Ok(LintResult {
            findings_count: 0,
            health_score: "100".to_string(),
            findings: vec!["✅ No se encontraron archivos para analizar".to_string()],
        });
    }

    // Analizar archivos
    let mut analysis_result = crate::analyzer::analyze_all_files(
        &files,
        project_root,
        ctx.pattern.clone(),
        &ctx,
        None, // sin cache
    )?;

    // Verificar dependencias circulares
    if let Ok(cycles) = crate::circular::analyze_circular_dependencies(&files, project_root) {
        for cycle in cycles {
            analysis_result.add_circular_dependency(cycle);
        }
    }

    // Calcular health score
    let health_score = crate::scoring::calculate(&analysis_result);

    // Extraer violaciones
    let findings: Vec<String> = analysis_result.violations.iter()
        .map(|v| format!("{}:{} - {}", v.violation.file_path.display(), v.violation.line_number, v.violation.rule.from))
        .collect();

    Ok(LintResult {
        findings_count: analysis_result.violations.len(),
        health_score: format!("{}", health_score.total),
        findings,
    })
}

fn run_deep_analysis(target: &str) -> Result<DeepAnalysisResult, Box<dyn std::error::Error + Send + Sync>> {
    let project_root = Path::new(target);

    // Intentar cargar configuración silenciosamente
    let config_path = project_root.join("architect.json");
    if !config_path.exists() {
        return Ok(DeepAnalysisResult {
            god_services: vec![],
            circular_deps: vec![],
            high_coupling: vec![],
        });
    }

    let ctx = crate::config::load_config(project_root)
        .map_err(|e| format!("Error loading config: {}", e))?;

    // Recolectar archivos
    let files = crate::discovery::collect_files(project_root, &ctx.ignored_paths);

    // Analizar para detectar god services (archivos con muchas responsabilidades)
    let mut god_services = Vec::new();
    let mut high_coupling = Vec::new();

    for file in &files {
        if let Ok(violations) = crate::analyzer::collect_violations_from_file(file, &ctx) {
            if violations.len() > 10 {
                god_services.push(file.display().to_string());
            }
            if violations.len() > 5 {
                high_coupling.push(file.display().to_string());
            }
        }
    }

    // Dependencias circulares
    let circular_deps = if let Ok(cycles) = crate::circular::analyze_circular_dependencies(&files, project_root) {
        cycles.iter().map(|c| c.cycle.join(" -> ")).collect()
    } else {
        Vec::new()
    };

    Ok(DeepAnalysisResult {
        god_services,
        circular_deps,
        high_coupling,
    })
}

fn run_circular_check(target: &str) -> Result<CircularCheckResult, Box<dyn std::error::Error + Send + Sync>> {
    let project_root = Path::new(target);

    // Intentar cargar configuración silenciosamente
    let config_path = project_root.join("architect.json");
    if !config_path.exists() {
        return Ok(CircularCheckResult {
            cycles: vec![],
            has_cycles: false,
        });
    }

    let ctx = crate::config::load_config(project_root)
        .map_err(|e| format!("Error loading config: {}", e))?;

    // Recolectar archivos
    let files = crate::discovery::collect_files(project_root, &ctx.ignored_paths);

    // Analizar dependencias circulares
    let cycles = if let Ok(detected) = crate::circular::analyze_circular_dependencies(&files, project_root) {
        detected.iter()
            .map(|cycle| cycle.cycle.join(" -> "))
            .collect()
    } else {
        Vec::new()
    };

    Ok(CircularCheckResult {
        has_cycles: !cycles.is_empty(),
        cycles,
    })
}

fn run_full_report(target: &str) -> Result<FullReportResult, Box<dyn std::error::Error + Send + Sync>> {
    let project_root = Path::new(target);

    // Intentar cargar configuración silenciosamente
    let config_path = project_root.join("architect.json");
    if !config_path.exists() {
        return Ok(FullReportResult {
            health_score: "N/A".to_string(),
            metrics: HashMap::new(),
            violations: vec!["⚠️ No se encontró architect.json en el proyecto".to_string()],
            suggestions: vec!["Ejecuta el wizard de Architect para generar la configuración".to_string()],
        });
    }

    let ctx = crate::config::load_config(project_root)
        .map_err(|e| format!("Error loading config: {}", e))?;

    // Recolectar archivos
    let files = crate::discovery::collect_files(project_root, &ctx.ignored_paths);

    if files.is_empty() {
        return Ok(FullReportResult {
            health_score: "100".to_string(),
            metrics: HashMap::new(),
            violations: vec!["No se encontraron archivos para analizar".to_string()],
            suggestions: vec![],
        });
    }

    // Analizar
    let analysis_result = crate::analyzer::analyze_all_files(
        &files,
        project_root,
        ctx.pattern.clone(),
        &ctx,
        None,
    )?;

    // Calcular health score
    let health_score = crate::scoring::calculate(&analysis_result);

    // Métricas
    let mut metrics = HashMap::new();
    metrics.insert("total_files".to_string(), serde_json::json!(files.len()));
    metrics.insert("total_violations".to_string(), serde_json::json!(analysis_result.violations.len()));
    metrics.insert("critical_issues".to_string(), serde_json::json!(analysis_result.blocked_count()));

    // Violaciones
    let violations: Vec<String> = analysis_result.violations.iter()
        .map(|v| format!("{}:{} - {} ({})", v.violation.file_path.display(), v.violation.line_number, v.violation.rule.from, v.category.as_str()))
        .collect();

    // Sugerencias
    let mut suggestions = Vec::new();
    if analysis_result.blocked_count() > 0 {
        suggestions.push("Priorizar la corrección de violaciones críticas".to_string());
    }
    if health_score.total < 70 {
        suggestions.push("Considerar refactorización de módulos con bajo score".to_string());
    }

    Ok(FullReportResult {
        health_score: format!("{}", health_score.total),
        metrics,
        violations,
        suggestions,
    })
}

fn run_config_validation(target: &str) -> Result<ConfigValidationResult, Box<dyn std::error::Error + Send + Sync>> {
    let project_root = Path::new(target);
    let config_path = project_root.join("architect.json");

    if !config_path.exists() {
        return Ok(ConfigValidationResult {
            valid: false,
            message: "❌ No se encontró architect.json".to_string(),
        });
    }

    match crate::config::load_config(project_root) {
        Ok(_) => Ok(ConfigValidationResult {
            valid: true,
            message: "✅ Configuración válida encontrada".to_string(),
        }),
        Err(e) => Ok(ConfigValidationResult {
            valid: false,
            message: format!("❌ Error en configuración: {}", e),
        }),
    }
}

fn run_stale_analysis(target: &str) -> Result<StaleAnalysisResult, Box<dyn std::error::Error + Send + Sync>> {
    let project_root = Path::new(target);

    // Intentar cargar configuración silenciosamente
    let config_path = project_root.join("architect.json");
    if !config_path.exists() {
        return Ok(StaleAnalysisResult {
            stale_files: vec![],
        });
    }

    let ctx = crate::config::load_config(project_root)
        .map_err(|e| format!("Error loading config: {}", e))?;

    // Recolectar archivos
    let files = crate::discovery::collect_files(project_root, &ctx.ignored_paths);

    // Analizar archivos stale (complejos pero sin cambios recientes)
    let mut stale_files = Vec::new();

    for file in &files {
        if let Ok(violations) = crate::analyzer::collect_violations_from_file(file, &ctx) {
            // Archivo con violaciones pero sin cambios recientes (simplificado)
            if violations.len() > 3 {
                stale_files.push(format!("{} ({} violaciones)", file.display(), violations.len()));
            }
        }
    }

    Ok(StaleAnalysisResult { stale_files })
}
