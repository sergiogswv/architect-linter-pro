//! Analyzer module for Architect Linter Pro
//!
//! This module contains all file analysis logic:
//! - Tree-sitter parser for TypeScript/JavaScript
//! - Pattern matching utilities
//! - Metrics collection (imports, functions)
//! - File collection with caching

pub mod collector;
pub mod metrics;
mod pattern_matcher;
pub mod swc_parser;

// Re-export public functions
pub use collector::analyze_all_files;
pub use swc_parser::collect_violations_from_file;

// Note: analyze_changed_files is defined in main.rs to avoid circular imports
// For tests, import directly from main
