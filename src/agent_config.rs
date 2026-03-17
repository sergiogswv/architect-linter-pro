use std::env;
use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct AgentConfig {
    pub cerebro_url: String,
    pub port: u16,
    pub report_enabled: bool,
}

impl AgentConfig {
    pub fn from_env() -> Self {
        // Cargar .env si existe
        let _ = dotenv();

        let cerebro_url = env::var("CEREBRO_URL")
            .unwrap_or_else(|_| "http://localhost:4000".to_string());
        
        let port = env::var("ARCHITECT_PORT")
            .unwrap_or_else(|_| "4002".to_string())
            .parse()
            .unwrap_or(4002);
            
        let report_enabled = env::var("CEREBRO_REPORT_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);

        Self {
            cerebro_url,
            port,
            report_enabled,
        }
    }
}
