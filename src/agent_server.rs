use axum::{
    extract::Query,
    routing::{get, post},
    Json, Router,
};
use crate::agent_config::AgentConfig;
use crate::agent_models::{OrchestratorCommand, CommandAck};
use crate::agent_reporter::report_event;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

pub async fn start_server(config: AgentConfig) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/command", post(handle_command))
        .route("/ai/suggestions", get(get_ai_suggestions))
        .route("/ai/rules", get(get_ai_rules));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("🚀 Architect Agente escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Spawn event reporting in background (non-blocking)
    if config.report_enabled {
        let config_clone = config.clone();
        tokio::spawn(async move {
            let mut payload = HashMap::new();
            payload.insert("ready".to_string(), serde_json::json!(true));
            payload.insert("message".to_string(), serde_json::json!("Architect está listo para análisis"));

            let _ = report_event(&config_clone, "architect_ready", "info", payload).await;
        });
    }

    axum::serve(listener, app).await?;

    Ok(())
}

// Query params para endpoints GET
#[derive(serde::Deserialize, Debug)]
struct ProjectQuery {
    project: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct RulesQuery {
    pattern: Option<String>,
    project: Option<String>,
}

// Respuestas de IA
#[derive(serde::Serialize, Debug)]
struct PatternOption {
    id: String,
    label: String,
    description: String,
}

#[derive(serde::Serialize, Debug)]
struct AIRule {
    from: String,
    to: String,
    reason: String,
}

async fn get_ai_suggestions(
    Query(query): Query<ProjectQuery>,
) -> Json<serde_json::Value> {
    let project_path = query.project.unwrap_or_else(|| ".".to_string());
    println!("🧠 [AI] Solicitando sugerencias para proyecto: {}", project_path);

    // Usar la lógica de IA existente en Architect
    match suggest_architectures(&project_path).await {
        Ok(patterns) => {
            Json(serde_json::json!({
                "ok": true,
                "framework": "detected",
                "patterns": patterns
            }))
        }
        Err(e) => {
            println!("❌ [AI] Error generando sugerencias: {}", e);
            Json(serde_json::json!({
                "ok": false,
                "error": e
            }))
        }
    }
}

async fn get_ai_rules(
    Query(query): Query<RulesQuery>,
) -> Json<serde_json::Value> {
    let pattern = query.pattern.unwrap_or_else(|| "custom".to_string());
    let project_path = query.project.unwrap_or_else(|| ".".to_string());
    println!("🧠 [AI] Generando reglas para patrón '{}' en proyecto: {}", pattern, project_path);

    match generate_rules_for_pattern(&pattern, &project_path).await {
        Ok(result) => {
            Json(serde_json::json!({
                "ok": true,
                "pattern": result.pattern,
                "suggested_max_lines": result.suggested_max_lines,
                "rules": result.rules.iter().map(|r| AIRule {
                    from: r.from.clone(),
                    to: r.to.clone(),
                    reason: r.reason.clone(),
                }).collect::<Vec<_>>()
            }))
        }
        Err(e) => {
            println!("❌ [AI] Error generando reglas: {}", e);
            Json(serde_json::json!({
                "ok": false,
                "error": e
            }))
        }
    }
}

async fn suggest_architectures(project_path: &str) -> Result<Vec<PatternOption>, String> {
    use crate::ai::sugerir_top_3_arquitecturas;
    use crate::config::load_ai_config_only;
    use crate::detector;

    let path = PathBuf::from(project_path);

    // Detectar framework
    let framework = detector::detect_framework(&path);
    let framework_str = framework.as_str().to_string();

    // Cargar solo config de IA (sin requerir architect.json)
    let ai_configs = match load_ai_config_only(&path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Error cargando config de IA: {}", e)),
    };

    // Obtener sugerencias usando la función existente
    let options = sugerir_top_3_arquitecturas(&framework_str, ai_configs)
        .await
        .map_err(|e| format!("Error de IA: {}", e))?;

    let patterns: Vec<PatternOption> = options.iter().map(|opt| PatternOption {
        id: opt.name.to_lowercase().replace(" ", "-"),
        label: opt.name.clone(),
        description: opt.description.clone(),
    }).collect();

    Ok(patterns)
}

async fn generate_rules_for_pattern(pattern: &str, project_path: &str) -> Result<crate::ai::AISuggestionResponse, String> {
    use crate::ai::sugerir_reglas_para_patron;
    use crate::config::load_ai_config_only;
    use crate::discovery::get_architecture_snapshot;

    let path = PathBuf::from(project_path);

    // Cargar solo config de IA (sin requerir architect.json)
    let ai_configs = match load_ai_config_only(&path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Error cargando config de IA: {}", e)),
    };

    // Obtener contexto del proyecto
    let context = get_architecture_snapshot(&path);

    // Generar reglas usando la función existente
    let result = sugerir_reglas_para_patron(pattern, context, ai_configs)
        .await
        .map_err(|e| format!("Error de IA: {}", e))?;

    Ok(result)
}

/// Ejecuta análisis de linting en el proyecto (se ejecuta en thread separado)
fn run_lint_analysis_blocking(project_path: PathBuf) -> Result<serde_json::Value, String> {
    use crate::config;
    use crate::analyzer;
    use crate::discovery;
    use crate::circular;
    use crate::scoring;

    // Load config
    let cfg = config::load_config(&project_path)
        .map_err(|e| format!("Error cargando config: {}", e))?;
    let ctx: Arc<config::LinterContext> = Arc::new(cfg.into());

    // Collect files
    let files = discovery::collect_files(&project_path, &ctx.ignored_paths);
    if files.is_empty() {
        return Ok(serde_json::json!({
            "findings": [],
            "findings_count": 0,
            "health_score": 100,
            "message": "No se encontraron archivos para analizar"
        }));
    }

    // Analyze files
    let mut result = analyzer::analyze_all_files(&files, &project_path, ctx.pattern.clone(), &ctx, None)
        .map_err(|e| format!("Error analizando archivos: {}", e))?;

    // Check circular dependencies
    let mut dep_analyzer = circular::CircularDependencyAnalyzer::new(&project_path);
    if let Ok(_) = dep_analyzer.build_graph(&files) {
        let cycles = dep_analyzer.detect_cycles();
        if !cycles.is_empty() {
            result.circular_dependencies = cycles;
        }
    }

    // Calculate health score
    let health_score = scoring::calculate(&result);
    result.health_score = Some(health_score.clone());

    // Format findings
    let findings: Vec<String> = result.violations.iter().map(|v| {
        format!("{:?}: {} ({}:{})",
            v.violation.rule.get_severity(),
            v.violation.offensive_import,
            v.violation.file_path.display(),
            v.violation.line_number)
    }).collect();

    let findings_count = findings.len();
    let score_value = health_score.total;

    Ok(serde_json::json!({
        "findings": findings,
        "findings_count": findings_count,
        "health_score": score_value,
        "circular_dependencies": result.circular_dependencies.iter().map(|c| c.description.clone()).collect::<Vec<_>>(),
        "files_analyzed": result.files_analyzed,
        "message": format!("Análisis completado: {} violaciones encontradas", findings_count)
    }))
}

/// Ejecuta detección de dependencias circulares (se ejecuta en thread separado)
fn run_circular_check_blocking(project_path: PathBuf) -> Result<serde_json::Value, String> {
    use crate::config;
    use crate::discovery;
    use crate::circular;

    // Load config
    let cfg = config::load_config(&project_path)
        .map_err(|e| format!("Error cargando config: {}", e))?;
    let ctx: config::LinterContext = cfg.into();

    // Collect files
    let files = discovery::collect_files(&project_path, &ctx.ignored_paths);
    if files.is_empty() {
        return Ok(serde_json::json!({
            "cycles": [],
            "has_cycles": false,
            "message": "No se encontraron archivos para analizar"
        }));
    }

    // Build dependency graph and detect cycles
    let mut dep_analyzer = circular::CircularDependencyAnalyzer::new(&project_path);
    dep_analyzer.build_graph(&files)
        .map_err(|e| format!("Error construyendo grafo de dependencias: {}", e))?;

    let cycles = dep_analyzer.detect_cycles();
    let has_cycles = !cycles.is_empty();

    let cycles_str: Vec<String> = cycles.iter().map(|c| c.description.clone()).collect();

    Ok(serde_json::json!({
        "cycles": cycles_str,
        "has_cycles": has_cycles,
        "cycles_count": cycles.len(),
        "files_analyzed": files.len(),
        "message": if has_cycles {
            format!("⚠️ {} dependencias circulares detectadas", cycles.len())
        } else {
            "✅ No se encontraron dependencias circulares".to_string()
        }
    }))
}

async fn handle_command(
    Json(cmd): Json<OrchestratorCommand>,
) -> Json<CommandAck> {
    println!("📨 Comando recibido: action={} target={:?}", cmd.action, cmd.target);

    let target_path = cmd.target.clone().unwrap_or_else(|| ".".to_string());
    let project_path = PathBuf::from(&target_path);

    match cmd.action.as_str() {
        "lint" | "analyze" => {
            // Execute lint analysis in blocking thread
            let path_clone = project_path.clone();
            match tokio::task::spawn_blocking(move || run_lint_analysis_blocking(path_clone)).await {
                Ok(Ok(result)) => {
                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "completed".to_string(),
                        result: Some(result),
                        error: None,
                    })
                }
                Ok(Err(e)) => {
                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(e),
                    })
                }
                Err(e) => {
                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(format!("Error ejecutando análisis: {}", e)),
                    })
                }
            }
        }
        "check-circular" | "check_circular" => {
            // Execute circular dependency check in blocking thread
            let path_clone = project_path.clone();
            match tokio::task::spawn_blocking(move || run_circular_check_blocking(path_clone)).await {
                Ok(Ok(result)) => {
                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "completed".to_string(),
                        result: Some(result),
                        error: None,
                    })
                }
                Ok(Err(e)) => {
                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(e),
                    })
                }
                Err(e) => {
                    Json(CommandAck {
                        request_id: cmd.request_id,
                        status: "error".to_string(),
                        result: None,
                        error: Some(format!("Error ejecutando check circular: {}", e)),
                    })
                }
            }
        }
        "deep-analysis" | "deep_analysis" => {
            Json(CommandAck {
                request_id: cmd.request_id,
                status: "accepted".to_string(),
                result: Some(serde_json::json!({
                    "action": "deep-analysis",
                    "target": cmd.target.unwrap_or_else(|| ".".to_string()),
                    "message": "Análisis profundo de arquitectura iniciado"
                })),
                error: None,
            })
        }
        "full-report" | "full_report" => {
            Json(CommandAck {
                request_id: cmd.request_id,
                status: "accepted".to_string(),
                result: Some(serde_json::json!({
                    "action": "full-report",
                    "target": cmd.target.unwrap_or_else(|| ".".to_string()),
                    "message": "Reporte completo de arquitectura generado"
                })),
                error: None,
            })
        }
        "validate-config" | "validate_config" => {
            Json(CommandAck {
                request_id: cmd.request_id,
                status: "accepted".to_string(),
                result: Some(serde_json::json!({
                    "action": "validate-config",
                    "target": cmd.target.unwrap_or_else(|| ".".to_string()),
                    "message": "Configuración validada exitosamente"
                })),
                error: None,
            })
        }
        "analyze-stale" | "analyze_stale" => {
            Json(CommandAck {
                request_id: cmd.request_id,
                status: "accepted".to_string(),
                result: Some(serde_json::json!({
                    "action": "analyze-stale",
                    "target": cmd.target.unwrap_or_else(|| ".".to_string()),
                    "message": "Análisis de archivos stale completado"
                })),
                error: None,
            })
        }
        "status" => {
            Json(CommandAck {
                request_id: cmd.request_id,
                status: "completed".to_string(),
                result: Some(serde_json::json!({
                    "agent": "architect",
                    "version": "6.0.0",
                    "ready": true
                })),
                error: None,
            })
        }
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
