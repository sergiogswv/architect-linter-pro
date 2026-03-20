use crate::agent_config::AgentConfig;
use crate::agent_models::AgentEvent;
use std::collections::HashMap;
use reqwest::Client;

pub async fn report_event(
    config: &AgentConfig,
    event_type: &str,
    severity: &str,
    payload: HashMap<String, serde_json::Value>,
) -> anyhow::Result<()> {
    println!("🔍 [report_event] cerebro_url={}, report_enabled={}", config.cerebro_url, config.report_enabled);

    if !config.report_enabled {
        println!("⚠️ [report_event] Reporte deshabilitado, saltando evento");
        return Ok(());
    }

    let event = AgentEvent::new("architect", event_type, severity, payload);
    let url = format!("{}/api/events", config.cerebro_url);

    println!("📡 [report_event] Enviando evento a: {}", url);

    let client = Client::new();
    let res = client.post(&url)
        .json(&event)
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_else(|_| "sin cuerpo".to_string());
        eprintln!("⚠️ Error reportando al Cerebro ({}): {}", status, body);
    } else {
        println!("✅ [report_event] Evento enviado exitosamente");
    }

    Ok(())
}
