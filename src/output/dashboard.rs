//! Visual dashboard for Architecture Health Score display
//!
//! This module provides rich terminal output with box drawing characters
//! and progress bars.

use crate::analysis_result::{AnalysisResult, CategorizedViolation};
use crate::metrics::{ComponentStatus, HealthGrade};
use crate::scoring::{get_grade_color, get_progress_bar, reset_color};

const BOX_TOP_LEFT: &str = "+";
const BOX_TOP_RIGHT: &str = "+";
const BOX_BOTTOM_LEFT: &str = "+";
const BOX_BOTTOM_RIGHT: &str = "+";
const BOX_HORIZONTAL: &str = "=";
const BOX_VERTICAL: &str = "|";
const BOX_T_LEFT: &str = "+";
const BOX_T_RIGHT: &str = "+";

const DASHBOARD_WIDTH: usize = 79;

/// Print the full analysis dashboard
pub fn print_dashboard(result: &AnalysisResult) {
    print_header(
        &result.project_name,
        result.pattern_display(),
        result.files_analyzed,
    );

    if let Some(ref score) = result.health_score {
        print_health_score(score.total, score.grade);
        print_components(
            score.components.layer_isolation,
            score.layer_isolation_status,
            "Layer isolation",
            &format_layer_details(result),
        );
        print_components(
            score.components.circular_deps,
            score.circular_deps_status,
            "No circular deps",
            &format_circular_details(result),
        );
        print_components(
            score.components.complexity,
            score.complexity_status,
            "Complexity",
            &format_complexity_details(result),
        );
        print_components(
            score.components.violations,
            score.violations_status,
            "Violations",
            &format_violations_details(result),
        );
    }

    print_violations_list(&result.violations);
}

/// Print the dashboard header with project info
fn print_header(project_name: &str, pattern: &str, files_analyzed: usize) {
    println!();
    print_horizontal_line(BOX_TOP_LEFT, BOX_TOP_RIGHT, BOX_HORIZONTAL);
    print_centered_line(&format!("ARCHITECT LINTER PRO v4.0"));
    print_horizontal_line(BOX_T_LEFT, BOX_T_RIGHT, BOX_HORIZONTAL);
    print_info_line("Project:", project_name);
    print_info_line("Pattern:", pattern);
    print_info_line("Files:", &format!("{} analyzed", files_analyzed));
    print_horizontal_line(BOX_T_LEFT, BOX_T_RIGHT, BOX_HORIZONTAL);
}

/// Print the health score section
fn print_health_score(score: u8, grade: HealthGrade) {
    let color = get_grade_color(grade);
    let reset = reset_color();

    println!(
        "{}{}{}  {}",
        BOX_VERTICAL,
        " ".repeat(DASHBOARD_WIDTH - 2),
        BOX_VERTICAL,
        ""
    );
    println!(
        "{}  ARCHITECTURE HEALTH: {}{}/100{}  {}  {}  {}{}",
        BOX_VERTICAL,
        color,
        score,
        reset,
        get_progress_bar(score, 14),
        grade.emoji(),
        grade.as_str(),
        " ".repeat(DASHBOARD_WIDTH - 50)
    );
    println!(
        "{}{}{}",
        BOX_VERTICAL,
        " ".repeat(DASHBOARD_WIDTH - 2),
        BOX_VERTICAL
    );
    print_horizontal_line(BOX_T_LEFT, BOX_T_RIGHT, BOX_HORIZONTAL);
}

/// Print a component status line
fn print_components(score: u8, status: ComponentStatus, name: &str, details: &str) {
    let status_emoji = status.emoji();
    let details_suffix = if details.is_empty() {
        String::new()
    } else {
        format!(" ({})", details)
    };

    let line = format!(
        "  +-- {} {}: {}%{}{}",
        status_emoji,
        name,
        score,
        details_suffix,
        " ".repeat(DASHBOARD_WIDTH.saturating_sub(20 + name.len() + details_suffix.len()))
    );

    // Truncate if too long
    if line.len() > DASHBOARD_WIDTH {
        println!(
            "{}{}{}",
            BOX_VERTICAL,
            &line[..DASHBOARD_WIDTH - 2],
            BOX_VERTICAL
        );
    } else {
        println!("{}{}{}", BOX_VERTICAL, line, BOX_VERTICAL);
    }
}

/// Format layer isolation details
fn format_layer_details(result: &AnalysisResult) -> String {
    let blocked = result.blocked_count();
    if blocked > 0 {
        format!("{} violations", blocked)
    } else {
        "100%".to_string()
    }
}

/// Format circular dependencies details
fn format_circular_details(result: &AnalysisResult) -> String {
    let cycles = result.circular_dependencies.len();
    if cycles > 0 {
        format!("{} cycles", cycles)
    } else {
        "Pass".to_string()
    }
}

/// Format complexity details
fn format_complexity_details(result: &AnalysisResult) -> String {
    let long = result.long_functions.len();
    if long > 0 {
        format!(
            "{} functions > {}",
            long, result.complexity_stats.max_lines_threshold
        )
    } else {
        "OK".to_string()
    }
}

/// Format violations details
fn format_violations_details(result: &AnalysisResult) -> String {
    let blocked = result.blocked_count();
    let warnings = result.warning_count();

    if blocked == 0 && warnings == 0 {
        "None".to_string()
    } else if warnings == 0 {
        format!("{} blocked", blocked)
    } else if blocked == 0 {
        format!("{} warnings", warnings)
    } else {
        format!("{} blocked, {} warnings", blocked, warnings)
    }
}

/// Print the violations list section
fn print_violations_list(violations: &[CategorizedViolation]) {
    if violations.is_empty() {
        return;
    }

    let blocked_count = violations
        .iter()
        .filter(|v| v.category == crate::analysis_result::ViolationCategory::Blocked)
        .count();
    let warning_count = violations
        .iter()
        .filter(|v| v.category == crate::analysis_result::ViolationCategory::Warning)
        .count();

    print_horizontal_line(BOX_T_LEFT, BOX_T_RIGHT, BOX_HORIZONTAL);
    let header_text_len = 30 + blocked_count.to_string().len() + warning_count.to_string().len();
    let header_padding = DASHBOARD_WIDTH.saturating_sub(header_text_len);
    println!(
        "{}  VIOLATIONS ({} blocked, {} warnings){}",
        BOX_VERTICAL,
        blocked_count,
        warning_count,
        " ".repeat(header_padding)
    );
    print_horizontal_line(BOX_T_LEFT, BOX_T_RIGHT, BOX_HORIZONTAL);

    for (i, categorized) in violations.iter().enumerate() {
        let v = &categorized.violation;
        let file_line_len =
            10 + v.file_path.to_string_lossy().len() + v.line_number.to_string().len();
        let file_padding = DASHBOARD_WIDTH.saturating_sub(file_line_len);
        println!(
            "{}  {}. {}:{}{}",
            BOX_VERTICAL,
            i + 1,
            v.file_path.display(),
            v.line_number,
            " ".repeat(file_padding)
        );
        let rule_line_len = 20 + v.rule.from.len() + v.rule.to.len();
        let rule_padding = DASHBOARD_WIDTH.saturating_sub(rule_line_len);
        println!(
            "{}     Rule: {} cannot import from {}{}",
            BOX_VERTICAL,
            v.rule.from,
            v.rule.to,
            " ".repeat(rule_padding)
        );

        if i < violations.len() - 1 {
            println!(
                "{}{}",
                BOX_VERTICAL,
                " ".repeat(DASHBOARD_WIDTH.saturating_sub(1))
            );
        }
    }

    print_horizontal_line(BOX_BOTTOM_LEFT, BOX_BOTTOM_RIGHT, BOX_HORIZONTAL);
}

/// Print a horizontal line
fn print_horizontal_line(left: &str, right: &str, fill: &str) {
    let middle = fill.repeat(DASHBOARD_WIDTH - 2);
    println!("{}{}{}", left, middle, right);
}

/// Print a centered line
fn print_centered_line(text: &str) {
    let padding = (DASHBOARD_WIDTH - 2 - text.len()) / 2;
    let extra = (DASHBOARD_WIDTH - 2 - text.len()) % 2;
    println!(
        "{}{}{}{}{}{}",
        BOX_VERTICAL,
        " ".repeat(padding),
        text,
        " ".repeat(padding + extra),
        "",
        BOX_VERTICAL
    );
}

/// Print an info line with label and value
fn print_info_line(label: &str, value: &str) {
    let content_len = label.len() + 1 + value.len();
    let padding = DASHBOARD_WIDTH - 2 - content_len;
    println!(
        "{}  {} {}{}{}",
        BOX_VERTICAL,
        label,
        value,
        " ".repeat(padding),
        BOX_VERTICAL
    );
}

/// Print a summary message
pub fn print_summary(result: &AnalysisResult) {
    println!();

    if result.has_critical_issues() {
        let score = result.health_score.as_ref().map(|s| s.total).unwrap_or(0);
        println!(
            "❌ Architecture Health: {}/100 - Critical issues found",
            score
        );
        println!();
        println!("💡 Fix the blocked violations and circular dependencies to improve your score.");
    } else {
        let score = result.health_score.as_ref().map(|s| s.total).unwrap_or(100);
        let grade = result
            .health_score
            .as_ref()
            .map(|s| s.grade)
            .unwrap_or(HealthGrade::A);

        if score >= 90 {
            println!(
                "✨ Architecture Health: {}/100 ({}) - Excellent!",
                score,
                grade.as_str()
            );
        } else if score >= 70 {
            println!(
                "✓ Architecture Health: {}/100 ({}) - Good",
                score,
                grade.as_str()
            );
        } else {
            println!(
                "⚠ Architecture Health: {}/100 ({}) - Needs improvement",
                score,
                grade.as_str()
            );
        }
    }
}
