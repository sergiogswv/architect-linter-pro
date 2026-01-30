use miette::Diagnostic;
use miette::SourceSpan;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Default)]
pub struct LinterConfig {
    #[serde(default = "default_max_lines")]
    pub max_lines_per_function: usize,
}

fn default_max_lines() -> usize {
    50
}

#[allow(dead_code)]
#[derive(Error, Debug, Diagnostic)]
#[error("Violación de Arquitectura")]
#[diagnostic(code(arch::violation), severity(error))]
pub struct ArchError {
    #[source_code]
    pub src: String,
    #[label("{message}")]
    pub span: SourceSpan,
    pub message: String,
}
