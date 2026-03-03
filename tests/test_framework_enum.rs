use architect_linter_pro::config::Framework;

#[test]
fn test_framework_enum_has_no_deprecated() {
    // Verify Go, Java, Ruby, etc. don't exist
    let all_frameworks = vec![
        Framework::NestJS,
        Framework::Express,
        Framework::React,
        Framework::NextJS,
        Framework::Vue,
        Framework::Svelte,
        Framework::Remix,
        Framework::SolidJS,
        Framework::Django,
        Framework::Flask,
        Framework::FastAPI,
        Framework::Laravel,
        Framework::Symfony,
    ];

    // Should be exactly 13 frameworks (no deprecated)
    assert_eq!(all_frameworks.len(), 13);
}

#[test]
fn test_framework_serialization() {
    let framework = Framework::NestJS;
    let serialized = serde_json::to_string(&framework).unwrap();
    assert_eq!(serialized, "\"NestJS\"");

    let deserialized: Framework = serde_json::from_str("\"NestJS\"").unwrap();
    assert_eq!(deserialized, Framework::NestJS);
}

#[test]
fn test_framework_language_method() {
    // Test TypeScript frameworks
    assert_eq!(Framework::NestJS.language(), "typescript");
    assert_eq!(Framework::Express.language(), "typescript");
    assert_eq!(Framework::React.language(), "typescript");
    assert_eq!(Framework::NextJS.language(), "typescript");
    assert_eq!(Framework::Vue.language(), "typescript");
    assert_eq!(Framework::Svelte.language(), "typescript");
    assert_eq!(Framework::Remix.language(), "typescript");
    assert_eq!(Framework::SolidJS.language(), "typescript");

    // Test Python frameworks
    assert_eq!(Framework::Django.language(), "python");
    assert_eq!(Framework::Flask.language(), "python");
    assert_eq!(Framework::FastAPI.language(), "python");

    // Test PHP frameworks
    assert_eq!(Framework::Laravel.language(), "php");
    assert_eq!(Framework::Symfony.language(), "php");

    // Test Unknown
    assert_eq!(Framework::Unknown.language(), "unknown");
}
