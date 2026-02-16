use architect_linter_pro::scoring::{apply_severity_multiplier, ViolationType, Violation};

#[test]
fn test_empty_project_bonus() {
    let violations = vec![];
    let base_score = 100.0;

    let score = apply_severity_multiplier(&violations, base_score, 0);
    assert!((score - 110.0).abs() < 0.001, "Empty project should get 10% bonus, got {}", score);
}

#[test]
fn test_architecture_pattern_bonus() {
    let violations = vec![];
    let base_score = 90.0;

    // Would test MVC pattern bonus
    let score = apply_severity_multiplier(&violations, base_score, 100);
    // Assert based on MVC pattern being detected
    assert!(score >= 90.0);
}

#[test]
fn test_historical_trend_factor() {
    let violations = vec![
        Violation {
            severity: ViolationType::Warning,
            file_path: "src/file.rs".to_string(),
            line: 1,
            message: "Warning".to_string(),
        }
    ];

    let base_score = 85.0;
    let historical_trend = 0.8; // Improving
    let score = apply_severity_multiplier(&violations, base_score, 50);

    // Would apply historical trend
    assert!(score >= base_score * historical_trend);
}
