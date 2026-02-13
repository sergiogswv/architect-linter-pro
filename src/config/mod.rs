//! Configuración del Architect Linter Pro
//!
//! Este módulo contiene toda la lógica relacionada con:
//! - Tipos de configuración (Framework, ArchPattern, etc.)
//! - Carga y validación de architect.json
//! - Wizard de configuración con IA
//! - Gestión de ignored_paths
//! - Setup de hooks de git (husky)

mod husky;
mod ignored_paths;
mod loader;
mod types;
mod wizard;

// Re-export tipos públicos
pub use types::{ArchError, ArchPattern, AIConfig, AIProvider, ForbiddenRule, Framework, LinterContext};

// Re-export funciones de loader
pub use loader::load_config;

// Re-export funciones de wizard
pub use wizard::setup_or_load_config;

// Re-export funciones de ignored_paths
pub use ignored_paths::default_ignored_paths;
