//! Metrics and scoring data structures for Architecture Health Score
//!
//! This module defines all the types needed for the v4.0 scoring system.

use serde::{Deserialize, Serialize};

/// Health grade based on score ranges
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthGrade {
    A, // 90-100
    B, // 80-89
    C, // 70-79
    D, // 60-69
    F, // 0-59
}

impl HealthGrade {
    pub fn from_score(score: u8) -> Self {
        match score {
            90..=100 => HealthGrade::A,
            80..=89 => HealthGrade::B,
            70..=79 => HealthGrade::C,
            60..=69 => HealthGrade::D,
            _ => HealthGrade::F,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            HealthGrade::A => "A",
            HealthGrade::B => "B",
            HealthGrade::C => "C",
            HealthGrade::D => "D",
            HealthGrade::F => "F",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            HealthGrade::A => "🏆",
            HealthGrade::B => "✨",
            HealthGrade::C => "👍",
            HealthGrade::D => "⚠️",
            HealthGrade::F => "❌",
        }
    }
}

/// Individual score components
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScoreComponents {
    /// Layer isolation score (0-100)
    /// Formula: 100 - (blocked_violations / total_imports * 100)
    pub layer_isolation: u8,

    /// Circular dependencies score (0 or 100)
    /// 100 if no cycles, 0 if cycles exist
    pub circular_deps: u8,

    /// Code complexity score (0-100)
    /// Formula: 100 - (long_functions / total_functions * 100)
    pub complexity: u8,

    /// Violations score (0-100)
    /// Formula: 100 - ((blocked * 2 + warnings) / total_checks * 100)
    pub violations: u8,
}

impl ScoreComponents {
    /// Calculate weighted total score
    /// Weights: Layer Isolation (30%), Circular Deps (25%), Complexity (20%), Violations (25%)
    pub fn calculate_total(&self) -> u8 {
        let total = (self.layer_isolation as f64 * 0.30)
            + (self.circular_deps as f64 * 0.25)
            + (self.complexity as f64 * 0.20)
            + (self.violations as f64 * 0.25);
        total.round() as u8
    }
}

/// Complete health score with all components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScore {
    /// Total score (0-100)
    pub total: u8,

    /// Grade (A-F)
    pub grade: HealthGrade,

    /// Individual component scores
    pub components: ScoreComponents,

    /// Status indicators for each component
    pub layer_isolation_status: ComponentStatus,
    pub circular_deps_status: ComponentStatus,
    pub complexity_status: ComponentStatus,
    pub violations_status: ComponentStatus,
}

impl HealthScore {
    pub fn new(components: ScoreComponents) -> Self {
        let total = components.calculate_total();
        let grade = HealthGrade::from_score(total);

        Self {
            total,
            grade,
            components,
            layer_isolation_status: ComponentStatus::from_score(components.layer_isolation),
            circular_deps_status: ComponentStatus::from_score(components.circular_deps),
            complexity_status: ComponentStatus::from_score(components.complexity),
            violations_status: ComponentStatus::from_score(components.violations),
        }
    }

    /// Get the status indicator string for display
    #[allow(dead_code)] // Used by integration tests
    pub fn status_indicator(&self) -> &'static str {
        match self.grade {
            HealthGrade::A => "✅ Excellent",
            HealthGrade::B => "✓ Good",
            HealthGrade::C => "! Fair",
            HealthGrade::D => "⚠ Needs Work",
            HealthGrade::F => "✗ Critical",
        }
    }
}

/// Status of an individual component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentStatus {
    Ok,      // No issues
    Warning, // Minor issues
    Blocked, // Critical issues
}

impl ComponentStatus {
    pub fn from_score(score: u8) -> Self {
        match score {
            90..=100 => ComponentStatus::Ok,
            60..=89 => ComponentStatus::Warning,
            _ => ComponentStatus::Blocked,
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            ComponentStatus::Ok => "[OK]",
            ComponentStatus::Warning => "[!]",
            ComponentStatus::Blocked => "[X]",
        }
    }
}

/// Statistics for layer isolation scoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LayerStats {
    pub total_imports: usize,
    pub blocked_violations: usize,
}

/// Statistics for complexity scoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComplexityStats {
    pub total_functions: usize,
    pub long_functions: usize, // Functions exceeding max_lines
    pub max_lines_threshold: usize,
}

