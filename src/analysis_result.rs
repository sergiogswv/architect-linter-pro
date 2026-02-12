//! Aggregated analysis results for v4.0 scoring system
//!
//! This module provides the AnalysisResult struct that collects all analysis
//! data needed for scoring and reporting.

use crate::autofix::Violation;
use crate::circular::CircularDependency;
use crate::config::ArchPattern;
use crate::metrics::{ComplexityStats, HealthScore, LayerStats};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Category of a violation for severity classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationCategory {
    /// Critical violation that breaks architecture rules
    Blocked,
    /// Warning that should be reviewed
    Warning,
    /// Informational notice
    Info,
}

impl ViolationCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ViolationCategory::Blocked => "blocked",
            ViolationCategory::Warning => "warning",
            ViolationCategory::Info => "info",
        }
    }
}

/// A categorized violation with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorizedViolation {
    /// The original violation
    pub violation: Violation,
    /// Category of this violation
    pub category: ViolationCategory,
    /// Optional suggestion for fixing
    pub suggestion: Option<String>,
}

impl CategorizedViolation {
    pub fn new(violation: Violation, category: ViolationCategory) -> Self {
        Self {
            violation,
            category,
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}

/// Long function detected during complexity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongFunction {
    /// File containing the function
    pub file_path: PathBuf,
    /// Function/method name
    pub name: String,
    /// Line number where function starts
    pub line_start: usize,
    /// Number of lines
    pub lines: usize,
    /// Threshold that was exceeded
    pub threshold: usize,
}

/// Complete analysis result with all metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Project name (derived from directory or package.json)
    pub project_name: String,
    /// Architecture pattern detected or configured
    pub pattern: ArchPattern,
    /// Total files analyzed
    pub files_analyzed: usize,
    /// Timestamp of analysis
    pub timestamp: DateTime<Utc>,
    /// All categorized violations found
    pub violations: Vec<CategorizedViolation>,
    /// All circular dependencies found
    pub circular_dependencies: Vec<CircularDependency>,
    /// All long functions found
    pub long_functions: Vec<LongFunction>,
    /// Layer statistics for scoring
    pub layer_stats: LayerStats,
    /// Complexity statistics for scoring
    pub complexity_stats: ComplexityStats,
    /// Computed health score (set after calculation)
    pub health_score: Option<HealthScore>,
}

impl AnalysisResult {
    /// Create a new empty analysis result
    pub fn new(project_name: String, pattern: ArchPattern) -> Self {
        Self {
            project_name,
            pattern,
            files_analyzed: 0,
            timestamp: Utc::now(),
            violations: Vec::new(),
            circular_dependencies: Vec::new(),
            long_functions: Vec::new(),
            layer_stats: LayerStats::default(),
            complexity_stats: ComplexityStats::default(),
            health_score: None,
        }
    }

    /// Add a violation to the result
    pub fn add_violation(&mut self, violation: CategorizedViolation) {
        match violation.category {
            ViolationCategory::Blocked => self.layer_stats.blocked_violations += 1,
            ViolationCategory::Warning | ViolationCategory::Info => {}
        }
        self.violations.push(violation);
    }

    /// Add a circular dependency to the result
    pub fn add_circular_dependency(&mut self, cycle: CircularDependency) {
        self.circular_dependencies.push(cycle);
    }

    /// Add a long function to the result
    pub fn add_long_function(&mut self, func: LongFunction) {
        self.long_functions.push(func);
        self.complexity_stats.long_functions += 1;
    }

    /// Get blocked violations count
    pub fn blocked_count(&self) -> usize {
        self.violations
            .iter()
            .filter(|v| v.category == ViolationCategory::Blocked)
            .count()
    }

    /// Get warning violations count
    pub fn warning_count(&self) -> usize {
        self.violations
            .iter()
            .filter(|v| v.category == ViolationCategory::Warning)
            .count()
    }

    /// Check if there are any critical issues
    pub fn has_critical_issues(&self) -> bool {
        self.blocked_count() > 0 || !self.circular_dependencies.is_empty()
    }

    /// Get pattern as display string
    pub fn pattern_display(&self) -> &str {
        match self.pattern {
            ArchPattern::Hexagonal => "Hexagonal Architecture",
            ArchPattern::Clean => "Clean Architecture",
            ArchPattern::MVC => "MVC Pattern",
            ArchPattern::Ninguno => "No Pattern",
        }
    }

    /// Generate a summary for display
    pub fn summary(&self) -> AnalysisSummary {
        AnalysisSummary {
            total_files: self.files_analyzed,
            total_violations: self.violations.len(),
            blocked_violations: self.blocked_count(),
            warning_violations: self.warning_count(),
            circular_deps: self.circular_dependencies.len(),
            long_functions: self.long_functions.len(),
        }
    }
}

/// Summary of analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub total_files: usize,
    pub total_violations: usize,
    pub blocked_violations: usize,
    pub warning_violations: usize,
    pub circular_deps: usize,
    pub long_functions: usize,
}

impl AnalysisSummary {
    pub fn is_clean(&self) -> bool {
        self.blocked_violations == 0 && self.circular_deps == 0
    }
}
