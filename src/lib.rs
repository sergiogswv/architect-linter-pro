// Library exports for testing and potential programmatic use
//
// This module exposes the internal modules publicly so they can be tested
// and potentially used as a library.

pub mod ai;
// Note: main module is only available in binary context
// Functions from main are re-exported individually for testing
pub mod analysis_result;
pub mod analyzer;
pub mod autofix;
pub mod cache;
pub mod circular;
pub mod cli;
pub mod config;
pub mod detector;
pub mod discovery;
pub mod git;
pub mod git_changes;
pub mod memory_cache;
pub mod metrics;
pub mod output;
pub mod parsers;
pub mod report;
pub mod scoring;
pub mod ui;
pub mod watch;

// Re-export commonly used types for convenience
pub use analysis_result::AnalysisResult;
pub use circular::CircularDependency;
pub use config::ArchPattern;
pub use metrics::{
    ComplexityStats, HealthGrade, HealthScore, LayerStats, PerformanceMetrics, ScoreComponents,
};
