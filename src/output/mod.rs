//! Output module for Architect Linter v4.0
//!
//! This module provides visual dashboard rendering and report generation.

pub mod dashboard;
pub mod rich;

pub use dashboard::print_dashboard;
pub use rich::RichOutput;
