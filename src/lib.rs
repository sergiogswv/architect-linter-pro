// Library exports for testing and potential programmatic use
//
// This module exposes the internal modules publicly so they can be tested
// and potentially used as a library.

// Internal modules (same as main.rs)
mod ai;
pub mod analysis_result;
pub mod analyzer;
mod autofix;
pub mod circular;
mod cli;
pub mod config;
mod detector;
mod discovery;
mod git;
pub mod metrics;
mod output;
mod parsers;
mod report;
pub mod scoring;
mod ui;
mod watch;

// Re-export commonly used types for convenience
pub use analysis_result::AnalysisResult;
pub use circular::CircularDependency;
pub use config::ArchPattern;
pub use metrics::{ComplexityStats, HealthGrade, HealthScore, LayerStats, ScoreComponents};
