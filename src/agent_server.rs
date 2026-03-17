use axum::{
    routing::post,
    Json, Router,
};
use crate::agent_config::AgentConfig;
use crate::agent_models::{OrchestratorCommand, CommandAck};
use std::net::SocketAddr;
use std::collections::HashMap;

pub async fn start_server(config: AgentConfig) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/command", post(handle_command));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("🚀 Architect Agente escuchando en http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_command(
    Json(cmd): Json<OrchestratorCommand>,
) -> Json<CommandAck> {
    println!("📨 Comando recibido: action={} target={:?}", cmd.action, cmd.target);

    match cmd.action.as_str() {
        "lint" | "analyze" => {
            Json(CommandAck {
                request_id: cmd.request_id,
                status: "accepted".to_string(),
                result: Some(serde_json::json!({
                    "action": cmd.action,
                    "target": cmd.target.unwrap_or_else(|| ".".to_string()),
                    "message": "Análisis solicitado"
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
