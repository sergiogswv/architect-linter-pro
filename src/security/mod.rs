#![allow(dead_code)]
pub mod cfg;
pub mod data_flow;
pub mod vulnerabilities;

pub use vulnerabilities::{Severity, VulnerabilityDetector, VulnerabilityType};

use crate::autofix::Violation;
use crate::config::LinterContext;
use miette::Result;
use std::path::Path;

/// Trait que deben implementar los parsers que soporten auditoría de seguridad
pub trait SecurityAuditor: Send + Sync {
    fn audit(
        &self,
        source_code: &str,
        file_path: &Path,
        context: &LinterContext,
    ) -> Result<Vec<Violation>>;
}

/// Analiza la seguridad de un archivo delegando al auditor correspondiente
pub fn audit_file(
    source_code: &str,
    file_path: &Path,
    context: &LinterContext,
) -> Result<Vec<Violation>> {
    // Intentamos obtener el parser multi-lenguaje que ahora también actuará como auditor
    if let Some(parser) = crate::parsers::get_parser_for_file(file_path) {
        // En el futuro, los parsers implementarán SecurityAuditor
        // Por ahora, simulamos la llamada o delegamos si el lenguaje está soportado en nuestro motor base
        return parser.audit_security(source_code, file_path, context);
    }

    Ok(Vec::new())
}
