use architect_linter_pro::scoring::{apply_severity_multiplier, ViolationType, Violation};

#[test]
fn test_empty_project_bonus() {
    let violations = vec![];
    let base_score = 100.0;

    let score = apply_severity_multiplier(&violations, base_score, 0);
    // Empty project gets 10% bonus (110.0) but is capped at 100.0
    assert!((score - 100.0).abs() < 0.001, "Empty project bonus should be capped at 100.0, got {}", score);
}

#[test]
fn test_architecture_pattern_bonus() {
    let violations = vec![];
    let base_score = 90.0;

    // Architecture pattern bonus: 5% for large projects (>=100 files) with no violations
    let score = apply_severity_multiplier(&violations, base_score, 100);
    // Expected: 90.0 * 1.05 = 94.5
    assert!((score - 94.5).abs() < 0.001, "Architecture pattern bonus should be 94.5, got {}", score);
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
    // TODO: This test is a placeholder for future functionality.
    // The apply_severity_multiplier function doesn't currently accept historical trend data.
    // When historical trend factor is implemented, this test should:
    // - Accept historical trend data as a parameter
    // - Verify the score is adjusted based on improvement/degradation
    // - Test various trend scenarios (improving, stable, degrading)
    let score = apply_severity_multiplier(&violations, base_score, 50);

    // For now, just verify the function returns a valid score
    assert!(score >= 0.0 && score <= 100.0, "Score should be valid range");
}
