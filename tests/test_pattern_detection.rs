use architect_linter_pro::ai::pattern_detection::{ArchitecturePattern, PatternDetector};

#[test]
fn test_detects_hexagonal_pattern() {
    let files = vec!["src/domain/user.rs", "src/application/service.rs", "src/infrastructure/db.rs"];
    let pattern = PatternDetector::detect_pattern(&files);
    assert_eq!(pattern, ArchitecturePattern::Hexagonal);
}

#[test]
fn test_detects_mvc_pattern() {
    let files = vec!["src/controllers/user.rs", "src/services/user.rs", "src/models/user.rs"];
    let pattern = PatternDetector::detect_pattern(&files);
    assert_eq!(pattern, ArchitecturePattern::LayeredMvc);
}

#[test]
fn test_detects_monolithic_small_codebase() {
    let files = vec!["src/main.rs", "src/lib.rs"];
    let pattern = PatternDetector::detect_pattern(&files);
    assert_eq!(pattern, ArchitecturePattern::MonolithicMud);
}

#[test]
fn test_suggestion_for_pattern() {
    let suggestion = PatternDetector::suggest_pattern(&ArchitecturePattern::Hexagonal);
    assert!(suggestion.contains("Hexagonal"));
}

#[test]
fn test_suggestion_for_mvc_pattern() {
    let suggestion = PatternDetector::suggest_pattern(&ArchitecturePattern::LayeredMvc);
    assert!(suggestion.contains("MVC"));
}

#[test]
fn test_suggestion_for_clean_architecture() {
    let suggestion = PatternDetector::suggest_pattern(&ArchitecturePattern::CleanArchitecture);
    assert!(suggestion.contains("explicit layer names"));
}

#[test]
fn test_suggestion_for_monolithic_pattern() {
    let suggestion = PatternDetector::suggest_pattern(&ArchitecturePattern::MonolithicMud);
    assert!(suggestion.contains("small"));
}

#[test]
fn test_clean_architecture_default_for_large_projects() {
    let mut files = vec!["src/main.rs"];
    // Add 20+ files to trigger clean architecture detection
    for _ in 0..25 {
        files.push("src/module.rs");
    }
    let pattern = PatternDetector::detect_pattern(&files);
    assert_eq!(pattern, ArchitecturePattern::CleanArchitecture);
}

#[test]
fn test_pattern_enum_derives() {
    // Test that ArchitecturePattern properly derives the required traits
    let pattern1 = ArchitecturePattern::Hexagonal;
    let pattern2 = ArchitecturePattern::Hexagonal;
    let pattern3 = ArchitecturePattern::CleanArchitecture;

    // Test PartialEq (==)
    assert_eq!(pattern1, pattern2);
    assert_ne!(pattern1, pattern3);

    // Test Clone
    let pattern_cloned = pattern1.clone();
    assert_eq!(pattern_cloned, pattern1);

    // Test Copy (implicit in assertions above)
    let pattern_copy = pattern1;
    assert_eq!(pattern_copy, pattern1);
}
