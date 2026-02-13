//! Analyzer module for Architect Linter Pro
//!
//! This module contains all file analysis logic:
//! - SWC-based parser for TypeScript/JavaScript
//! - Pattern matching utilities
//! - Metrics collection (imports, functions)
//! - File collection with caching

mod collector;
mod metrics;
mod pattern_matcher;
mod swc_parser;

// Re-export public functions
pub use collector::analyze_all_files;
pub use swc_parser::{analyze_file, collect_violations_from_file};
