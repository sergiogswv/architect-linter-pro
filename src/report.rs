//! Report generation for Architect Linter v4.0
//!
//! This module provides JSON and Markdown report export functionality.

use crate::analysis_result::AnalysisResult;
use crate::cli::ReportFormat;
use miette::{IntoDiagnostic, Result};
use serde_json::json;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Generate a report in the specified format
pub fn generate_report(result: &AnalysisResult, format: ReportFormat) -> String {
    match format {
        ReportFormat::Json => to_json(result),
        ReportFormat::Markdown => to_markdown(result),
    }
}

/// Export analysis result to JSON format
pub fn to_json(result: &AnalysisResult) -> String {
    let health_score = result.health_score.as_ref();

    let report = json!({
        "version": "4.0.0",
        "timestamp": result.timestamp.to_rfc3339(),
        "project": {
            "name": result.project_name,
            "pattern": result.pattern_display(),
            "files_analyzed": result.files_analyzed,
        },
        "health_score": health_score.map(|s| json!({
            "total": s.total,
            "grade": s.grade.as_str(),
            "components": {
                "layer_isolation": s.components.layer_isolation,
                "circular_deps": s.components.circular_deps,
                "complexity": s.components.complexity,
                "violations": s.components.violations,
            }
        })),
        "summary": {
            "total_violations": result.violations.len(),
            "blocked_violations": result.blocked_count(),
            "warning_violations": result.warning_count(),
            "circular_dependencies": result.circular_dependencies.len(),
            "long_functions": result.long_functions.len(),
        },
        "violations": result.violations.iter().map(|cv| {
            json!({
                "file": cv.violation.file_path.to_string_lossy().to_string(),
                "line": cv.violation.line_number,
                "category": cv.category.as_str(),
                "rule": {
                    "from": cv.violation.rule.from,
                    "to": cv.violation.rule.to,
                },
                "import": cv.violation.offensive_import,
            })
        }).collect::<Vec<_>>(),
        "circular_dependencies": result.circular_dependencies.iter().map(|cd| {
            json!({
                "cycle": cd.cycle,
                "description": cd.description,
            })
        }).collect::<Vec<_>>(),
        "long_functions": result.long_functions.iter().map(|lf| {
            json!({
                "file": lf.file_path.to_string_lossy().to_string(),
                "name": lf.name,
                "line_start": lf.line_start,
                "lines": lf.lines,
                "threshold": lf.threshold,
            })
        }).collect::<Vec<_>>(),
    });

    serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
}

/// Export analysis result to Markdown format
pub fn to_markdown(result: &AnalysisResult) -> String {
    let mut md = String::new();

    // Header
    md.push_str("# Architect Linter Pro Report\n\n");
    md.push_str(&format!("> Generated: {}\n\n", result.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));

    // Project info
    md.push_str("## Project\n\n");
    md.push_str(&format!("- **Name:** {}\n", result.project_name));
    md.push_str(&format!("- **Pattern:** {}\n", result.pattern_display()));
    md.push_str(&format!("- **Files Analyzed:** {}\n\n", result.files_analyzed));

    // Health Score
    if let Some(ref score) = result.health_score {
        md.push_str("## Architecture Health Score\n\n");
        md.push_str(&format!("# {} Score: {}/100\n\n", score.grade.as_str(), score.total));

        md.push_str("| Component | Score | Status |\n");
        md.push_str("|-----------|-------|--------|\n");
        md.push_str(&format!(
            "| Layer Isolation | {}% | {} |\n",
            score.components.layer_isolation,
            score.layer_isolation_status.emoji()
        ));
        md.push_str(&format!(
            "| Circular Deps | {}% | {} |\n",
            score.components.circular_deps,
            score.circular_deps_status.emoji()
        ));
        md.push_str(&format!(
            "| Complexity | {}% | {} |\n",
            score.components.complexity,
            score.complexity_status.emoji()
        ));
        md.push_str(&format!(
            "| Violations | {}% | {} |\n\n",
            score.components.violations,
            score.violations_status.emoji()
        ));
    }

    // Summary
    md.push_str("## Summary\n\n");
    md.push_str(&format!("- **Total Violations:** {}\n", result.violations.len()));
    md.push_str(&format!("- **Blocked:** {}\n", result.blocked_count()));
    md.push_str(&format!("- **Warnings:** {}\n", result.warning_count()));
    md.push_str(&format!("- **Circular Dependencies:** {}\n", result.circular_dependencies.len()));
    md.push_str(&format!("- **Long Functions:** {}\n\n", result.long_functions.len()));

    // Violations
    if !result.violations.is_empty() {
        md.push_str("## Violations\n\n");
        for (i, cv) in result.violations.iter().enumerate() {
            md.push_str(&format!(
                "### {}. {} (Line {})\n\n",
                i + 1,
                cv.violation.file_path.display(),
                cv.violation.line_number
            ));
            md.push_str(&format!(
                "- **Category:** {}\n",
                cv.category.as_str()
            ));
            md.push_str(&format!(
                "- **Rule:** `{}` cannot import from `{}`\n",
                cv.violation.rule.from,
                cv.violation.rule.to
            ));
            md.push_str(&format!(
                "- **Import:** `{}`\n\n",
                cv.violation.offensive_import
            ));
        }
    }

    // Circular Dependencies
    if !result.circular_dependencies.is_empty() {
        md.push_str("## Circular Dependencies\n\n");
        for (i, cd) in result.circular_dependencies.iter().enumerate() {
            md.push_str(&format!("### Cycle #{}\n\n```\n", i + 1));
            for (j, node) in cd.cycle.iter().enumerate() {
                if j < cd.cycle.len() - 1 {
                    md.push_str(&format!("{} →\n", node));
                } else {
                    md.push_str(&format!("{} ↑\n", node));
                }
            }
            md.push_str("```\n\n");
        }
    }

    // Long Functions
    if !result.long_functions.is_empty() {
        md.push_str("## Long Functions\n\n");
        md.push_str("| File | Function | Lines | Threshold |\n");
        md.push_str("|------|----------|-------|----------|\n");
        for lf in &result.long_functions {
            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                lf.file_path.display(),
                lf.name,
                lf.lines,
                lf.threshold
            ));
        }
        md.push_str("\n");
    }

    md
}

/// Write report to file
pub fn write_report(content: &str, path: &Path) -> Result<()> {
    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).into_diagnostic()?;
    }

    fs::write(path, content).into_diagnostic()
}

/// Write report to stdout
pub fn write_stdout(content: &str) -> Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(content.as_bytes()).into_diagnostic()?;
    stdout.flush().into_diagnostic()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{HealthGrade, HealthScore, ScoreComponents};
    use crate::config::ArchPattern;

    #[test]
    fn test_json_report_structure() {
        let mut result = AnalysisResult::new("test-project".to_string(), ArchPattern::MVC);
        result.files_analyzed = 10;
        result.health_score = Some(HealthScore::new(ScoreComponents {
            layer_isolation: 100,
            circular_deps: 100,
            complexity: 80,
            violations: 90,
        }));

        let json = to_json(&result);
        assert!(json.contains("test-project"));
        assert!(json.contains("MVC"));
    }

    #[test]
    fn test_markdown_report_structure() {
        let mut result = AnalysisResult::new("test-project".to_string(), ArchPattern::Hexagonal);
        result.files_analyzed = 5;

        let md = to_markdown(&result);
        assert!(md.contains("# Architect Linter Pro Report"));
        assert!(md.contains("test-project"));
        assert!(md.contains("Hexagonal"));
    }
}
