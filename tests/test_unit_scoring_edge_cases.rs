use architect_linter_pro::scoring::{calculate_health_score, Violation, ViolationType};

#[test]
fn test_score_with_zero_violations() {
    let violations = vec![];
    let score = calculate_health_score(&violations, 100.0);
    assert_eq!(score, 100.0, "Perfect project should score 100");
}

#[test]
fn test_score_with_critical_violations() {
    let mut violations = vec![Violation {
        severity: ViolationType::Critical,
        file_path: "src/bad.rs".to_string(),
        line: 1,
        message: "Critical violation".to_string(),
    }];

    // Add 10 critical violations
    for i in 1..=10 {
        violations.push(Violation {
            severity: ViolationType::Critical,
            file_path: format!("src/bad{}.rs", i),
            line: i,
            message: "Critical violation".to_string(),
        });
    }

    let score = calculate_health_score(&violations, 100.0);
    assert!(
        score < 50.0,
        "Score should be <50 with 10 critical violations"
    );
}

#[test]
fn test_score_boundary_values() {
    // Test with minimum project size
    let violations = vec![];
    let score_min = calculate_health_score(&violations, 1.0);
    assert_eq!(
        score_min, 100.0,
        "1 file project should score 100 with no violations"
    );

    // Test with maximum project size
    let score_max = calculate_health_score(&violations, 1000.0);
    assert_eq!(
        score_max, 100.0,
        "Large project should score 100 with no violations"
    );
}

#[test]
fn test_score_with_1000_files() {
    // Simulate 1000 files with violations
    let mut violations = Vec::new();
    for i in 0..1000 {
        violations.push(Violation {
            severity: ViolationType::Warning,
            file_path: format!("src/file{}.rs", i),
            line: 1,
            message: "Warning violation".to_string(),
        });
    }

    let score = calculate_health_score(&violations, 1000.0);
    assert!(score < 80.0, "1000 files with warnings should score <80");
}
