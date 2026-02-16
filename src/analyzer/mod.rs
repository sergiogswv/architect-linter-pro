//! Analyzer module for Architect Linter Pro
//!
//! This module contains all file analysis logic:
//! - SWC-based parser for TypeScript/JavaScript
//! - Pattern matching utilities
//! - Metrics collection (imports, functions)
//! - File collection with caching

pub mod collector;
pub mod metrics;
mod pattern_matcher;
pub mod swc_parser;

// Re-export public functions
pub use collector::analyze_all_files;
pub use metrics::{count_functions, count_imports, find_long_functions};
pub use swc_parser::{analyze_file, collect_violations_from_file};

// Note: analyze_changed_files is defined in main.rs to avoid circular imports
// For tests, import directly from main
