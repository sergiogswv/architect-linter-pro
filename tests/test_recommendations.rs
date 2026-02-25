use architect_linter_pro::ai::recommendations::ArchitectureRecommendations;

#[test]
fn test_high_violations_recommendation() {
    let recs = ArchitectureRecommendations::recommend_improvements(60, 100, 0, 30);
    assert!(recs.iter().any(|r| r.contains("High violation")));
}

#[test]
fn test_circular_deps_recommendation() {
    let recs = ArchitectureRecommendations::recommend_improvements(10, 100, 3, 30);
    assert!(recs.iter().any(|r| r.contains("circular")));
}

#[test]
fn test_action_plan_critical_score() {
    let actions = ArchitectureRecommendations::generate_action_plan(20.0);
    assert!(actions.len() >= 4);
    assert!(actions[0].contains("critical"));
}

#[test]
fn test_action_plan_good_score() {
    let actions = ArchitectureRecommendations::generate_action_plan(85.0);
    assert!(actions.iter().any(|a| a.contains("✅")));
}

#[test]
fn test_large_codebase_recommendation() {
    let recs = ArchitectureRecommendations::recommend_improvements(10, 600, 0, 30);
    assert!(recs.iter().any(|r| r.contains("Large codebase")));
}

#[test]
fn test_method_length_high() {
    let recs = ArchitectureRecommendations::recommend_improvements(5, 100, 0, 60);
    assert!(recs.iter().any(|r| r.contains("method length")));
}

#[test]
fn test_no_recommendations_when_healthy() {
    let recs = ArchitectureRecommendations::recommend_improvements(5, 100, 0, 30);
    assert!(recs.is_empty());
}

#[test]
fn test_moderate_violations() {
    let recs = ArchitectureRecommendations::recommend_improvements(30, 100, 0, 30);
    assert!(recs.iter().any(|r| r.contains("Moderate")));
}

#[test]
fn test_action_plan_moderate_score() {
    let actions = ArchitectureRecommendations::generate_action_plan(50.0);
    assert!(actions.len() >= 3);
    assert!(actions[0].contains("remaining"));
}
