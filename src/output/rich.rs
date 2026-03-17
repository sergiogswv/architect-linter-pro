//! Rich terminal output with colors and formatting

#[allow(dead_code)]
pub struct RichOutput;

impl RichOutput {
    #[allow(dead_code)]
    pub fn print_violation(file: &str, line: usize, severity: &str, message: &str) {
        let severity_prefix = match severity {
            "error" => "❌ ERROR",
            "warning" => "⚠️  WARNING",
            _ => "ℹ️  INFO",
        };

        println!(
            "{} {}:{} - {}",
            severity_prefix,
            file,
            line,
            message
        );
    }

    #[allow(dead_code)]
    pub fn print_summary(violations: usize, score: f64) {
        println!("\n{}", "=".repeat(50));
        println!(
            "Architecture Score: {:.2}/100 | Violations: {}",
            score, violations
        );
        println!("{}\n", "=".repeat(50));
    }

    #[allow(dead_code)]
    pub fn print_success(message: &str) {
        println!("✅ {}", message);
    }

    #[allow(dead_code)]
    pub fn print_error(message: &str) {
        println!("❌ {}", message);
    }

    #[allow(dead_code)]
    pub fn print_header(title: &str) {
        println!("\n📊 {}\n", title);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rich_output_created() {
        let rich = RichOutput;
        // Just verify it can be instantiated
        drop(rich);
    }
}
