//! HTML report generation for Architecture Linter
//!
//! This module provides functionality to generate interactive HTML reports
//! with violations table, architecture score display, and CSS styling.

use std::fs;

/// HTML report generator for displaying violations and architecture scores
pub struct HtmlReporter;

impl HtmlReporter {
    /// Generate an HTML report with violations and architecture score
    ///
    /// # Arguments
    ///
    /// * `violations` - A slice of tuples containing (file path, line number, message)
    /// * `score` - The architecture score (0.0-100.0)
    /// * `output_path` - The path where the HTML report should be written
    ///
    /// # Returns
    ///
    /// `Ok(())` if the report was successfully generated, or an `Err` if file I/O failed
    ///
    /// # Example
    ///
    /// ```no_run
    /// use architect_linter_pro::output::HtmlReporter;
    ///
    /// let violations = vec![
    ///     ("src/main.rs".to_string(), 42, "Layer violation".to_string()),
    /// ];
    /// let score = 75.5;
    /// HtmlReporter::generate_report(&violations, score, "report.html")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn generate_report(
        violations: &[(String, usize, String)],
        score: f64,
        output_path: &str,
    ) -> std::io::Result<()> {
        // Generate HTML table rows for each violation
        let violations_html = violations
            .iter()
            .map(|(file, line, message)| {
                format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                    file, line, message
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Generate the complete HTML document
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Architecture Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1 {{ color: #333; }}
        h2 {{ color: #555; margin-top: 30px; }}
        table {{ border-collapse: collapse; width: 100%; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #4CAF50; color: white; }}
        tr:nth-child(even) {{ background-color: #f2f2f2; }}
        .score {{ font-size: 24px; font-weight: bold; color: #4CAF50; }}
        .score-container {{ margin: 15px 0; padding: 10px; background-color: #f0f8f0; border-left: 4px solid #4CAF50; }}
    </style>
</head>
<body>
    <h1>Architecture Linter Report</h1>
    <div class="score-container">
        <p>Architecture Score: <span class="score">{:.2}/100</span></p>
    </div>
    <h2>Violations</h2>
    <table>
        <tr><th>File</th><th>Line</th><th>Message</th></tr>
        {}
    </table>
</body>
</html>"#,
            score, violations_html
        );

        // Write the HTML to the output file
        fs::write(output_path, html)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_html_reporter_instantiation() {
        let _reporter = HtmlReporter;
        // Just verify it can be instantiated
    }

    #[test]
    fn test_generate_minimal_report() {
        let temp_dir = TempDir::new().unwrap();
        let report_path = temp_dir.path().join("test_report.html");

        let result = HtmlReporter::generate_report(&[], 50.0, report_path.to_str().unwrap());

        assert!(result.is_ok());
        assert!(report_path.exists());

        let content = fs::read_to_string(&report_path).unwrap();
        assert!(content.contains("<!DOCTYPE html>"));
        assert!(content.contains("50.00"));
    }

    #[test]
    fn test_generate_report_with_violations() {
        let temp_dir = TempDir::new().unwrap();
        let report_path = temp_dir.path().join("test_report.html");

        let violations = vec![
            ("src/main.rs".to_string(), 10, "Test violation".to_string()),
            ("src/lib.rs".to_string(), 20, "Another violation".to_string()),
        ];

        let result =
            HtmlReporter::generate_report(&violations, 65.5, report_path.to_str().unwrap());

        assert!(result.is_ok());

        let content = fs::read_to_string(&report_path).unwrap();
        assert!(content.contains("src/main.rs"));
        assert!(content.contains("src/lib.rs"));
        assert!(content.contains("65.50"));
        assert!(content.contains("Test violation"));
        assert!(content.contains("Another violation"));
    }
}
