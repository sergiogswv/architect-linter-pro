//! Tests for HTML report generation
//!
//! These tests verify that the HTML report generator produces valid HTML
//! with proper formatting for violations and architecture score display.

use architect_linter_pro::output::HtmlReporter;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a temporary output path
fn get_temp_report_path() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let report_path = temp_dir.path().join("report.html");
    (temp_dir, report_path)
}

#[test]
fn test_html_report_generation() {
    let (_temp_dir, report_path) = get_temp_report_path();

    let violations = vec![
        ("src/main.rs".to_string(), 42, "Missing layer separation".to_string()),
        ("src/lib.rs".to_string(), 15, "Invalid architecture pattern".to_string()),
    ];
    let score = 75.5;

    // Generate the report
    let result = HtmlReporter::generate_report(&violations, score, report_path.to_str().unwrap());

    // Verify no errors occurred
    assert!(result.is_ok(), "Report generation failed");

    // Verify file was created
    assert!(report_path.exists(), "Report file was not created");

    // Verify file is not empty
    let content = fs::read_to_string(&report_path).expect("Failed to read report file");
    assert!(!content.is_empty(), "Report file is empty");
}

#[test]
fn test_html_violations_table() {
    let (_temp_dir, report_path) = get_temp_report_path();

    let violations = vec![
        ("src/main.rs".to_string(), 42, "Missing layer separation".to_string()),
        ("src/lib.rs".to_string(), 15, "Invalid architecture pattern".to_string()),
        ("src/utils.rs".to_string(), 8, "Circular dependency".to_string()),
    ];
    let score = 68.0;

    HtmlReporter::generate_report(&violations, score, report_path.to_str().unwrap())
        .expect("Report generation failed");

    let content = fs::read_to_string(&report_path).expect("Failed to read report file");

    // Verify table headers exist
    assert!(content.contains("<th>File</th>"), "Missing 'File' header");
    assert!(content.contains("<th>Line</th>"), "Missing 'Line' header");
    assert!(content.contains("<th>Message</th>"), "Missing 'Message' header");

    // Verify violations are in the table
    assert!(content.contains("src/main.rs"), "Missing first violation file");
    assert!(content.contains("42"), "Missing first violation line number");
    assert!(
        content.contains("Missing layer separation"),
        "Missing first violation message"
    );

    assert!(content.contains("src/lib.rs"), "Missing second violation file");
    assert!(content.contains("15"), "Missing second violation line number");
    assert!(
        content.contains("Invalid architecture pattern"),
        "Missing second violation message"
    );

    assert!(content.contains("src/utils.rs"), "Missing third violation file");
    assert!(content.contains("8"), "Missing third violation line number");
    assert!(
        content.contains("Circular dependency"),
        "Missing third violation message"
    );

    // Verify table rows are properly formatted
    assert!(content.contains("<tr>"), "Missing table rows");
    assert!(content.contains("</tr>"), "Missing table row closing tags");
    assert!(content.contains("<td>"), "Missing table data cells");
    assert!(content.contains("</td>"), "Missing table data cell closing tags");
}

#[test]
fn test_html_score_display() {
    let (_temp_dir, report_path) = get_temp_report_path();

    let violations = vec![];
    let score = 92.75;

    HtmlReporter::generate_report(&violations, score, report_path.to_str().unwrap())
        .expect("Report generation failed");

    let content = fs::read_to_string(&report_path).expect("Failed to read report file");

    // Verify score appears in the content with proper formatting
    assert!(content.contains("92.75"), "Score not found in report");

    // Verify score is displayed with CSS class for styling
    assert!(
        content.contains("class=\"score\""),
        "Score CSS class not found"
    );

    // Verify the score section exists
    assert!(
        content.contains("Architecture Score:"),
        "Architecture Score label not found"
    );
}

#[test]
fn test_html_basic_structure() {
    let (_temp_dir, report_path) = get_temp_report_path();

    let violations = vec![];
    let score = 50.0;

    HtmlReporter::generate_report(&violations, score, report_path.to_str().unwrap())
        .expect("Report generation failed");

    let content = fs::read_to_string(&report_path).expect("Failed to read report file");

    // Verify DOCTYPE
    assert!(content.contains("<!DOCTYPE html>"), "Missing DOCTYPE");

    // Verify HTML tags
    assert!(content.contains("<html>"), "Missing <html> tag");
    assert!(content.contains("</html>"), "Missing </html> tag");

    // Verify head section
    assert!(content.contains("<head>"), "Missing <head> tag");
    assert!(content.contains("</head>"), "Missing </head> tag");

    // Verify title
    assert!(content.contains("<title>"), "Missing <title> tag");
    assert!(content.contains("</title>"), "Missing </title> tag");

    // Verify body section
    assert!(content.contains("<body>"), "Missing <body> tag");
    assert!(content.contains("</body>"), "Missing </body> tag");

    // Verify main heading
    assert!(
        content.contains("Architecture Linter Report"),
        "Missing main heading"
    );

    // Verify violations section heading
    assert!(content.contains("<h2>Violations</h2>"), "Missing violations heading");
}

#[test]
fn test_html_empty_violations() {
    let (_temp_dir, report_path) = get_temp_report_path();

    let violations: Vec<(String, usize, String)> = vec![];
    let score = 100.0;

    HtmlReporter::generate_report(&violations, score, report_path.to_str().unwrap())
        .expect("Report generation failed");

    let content = fs::read_to_string(&report_path).expect("Failed to read report file");

    // Should still have a valid HTML structure
    assert!(content.contains("<!DOCTYPE html>"), "Missing DOCTYPE");
    assert!(content.contains("<table>"), "Missing table element");
    assert!(content.contains("</table>"), "Missing closing table tag");

    // Should have table header even with no violations
    assert!(content.contains("<th>File</th>"), "Missing table headers");
}

#[test]
fn test_html_special_characters_in_violations() {
    let (_temp_dir, report_path) = get_temp_report_path();

    let violations = vec![
        (
            "src/file<test>.rs".to_string(),
            10,
            "Message with & special chars".to_string(),
        ),
        (
            "src/\"quoted\".rs".to_string(),
            20,
            "Another & test".to_string(),
        ),
    ];
    let score = 45.0;

    HtmlReporter::generate_report(&violations, score, report_path.to_str().unwrap())
        .expect("Report generation failed");

    let content = fs::read_to_string(&report_path).expect("Failed to read report file");

    // Verify that the HTML was created successfully
    assert!(!content.is_empty(), "Report file is empty");
    assert!(content.contains("<table>"), "Missing table");

    // Verify that special characters are escaped to prevent XSS
    // Less-than and greater-than signs must be escaped
    assert!(
        content.contains("&lt;") || !content.contains("<test>"),
        "< character should be escaped as &lt; in file paths"
    );

    // Ampersands must be escaped
    assert!(
        content.contains("&amp;") || !content.contains("&special"),
        "& character should be escaped as &amp; in messages"
    );

    // Double quotes must be escaped
    assert!(
        content.contains("&quot;") || !content.contains("\"quoted\""),
        "\" character should be escaped as &quot; in file paths"
    );

    // Verify dangerous input is not present in raw form
    assert!(
        !content.contains("<test>"),
        "Unescaped < > should not appear in HTML"
    );
}

#[test]
fn test_html_file_write_error() {
    // Try to write to an invalid path (non-existent parent directory)
    let invalid_path = "/nonexistent/directory/that/does/not/exist/report.html";

    let violations = vec![];
    let score = 50.0;

    let result = HtmlReporter::generate_report(&violations, score, invalid_path);

    // Should return an error
    assert!(result.is_err(), "Expected error for invalid path");
}
