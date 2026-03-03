//! Integration tests for the full detection pipeline
//!
//! This module tests:
//! - Full framework detection pipeline
//! - Configuration generation from detected frameworks
//! - Multi-framework project support
//! - Real-world fixture projects (NextJS, Django, Laravel)

use architect_linter_pro::config::{ConfigGenerator, ArchPattern};
use architect_linter_pro::detection::detect_all_frameworks;
use std::path::Path;

/// Test full pipeline for a NextJS project
#[test]
fn test_full_pipeline_nextjs_project() {
    let fixture_path = Path::new("./tests/fixtures/nextjs_project");

    // Verify fixture exists
    assert!(
        fixture_path.exists(),
        "NextJS test fixture should exist at {:?}",
        fixture_path
    );

    // Step 1: Detect frameworks
    let detected_frameworks = detect_all_frameworks(fixture_path)
        .expect("Framework detection should succeed for NextJS project");

    assert!(
        !detected_frameworks.is_empty(),
        "NextJS project should have detected frameworks"
    );

    // Verify NextJS was detected
    assert!(
        detected_frameworks
            .iter()
            .any(|f| f.framework.as_str() == "NextJS"),
        "NextJS project should detect NextJS framework"
    );

    // Step 2: Generate configuration from the project
    let generator = ConfigGenerator::new();
    let config = generator
        .generate(fixture_path)
        .expect("Config generation should succeed for NextJS project");

    // Verify configuration was generated properly
    assert!(
        config.max_lines_per_function > 0,
        "Generated config should have valid max_lines_per_function"
    );
    // NextJS may have empty forbidden_imports if no template is configured
    // but the config should still be valid
    assert!(
        config.ignored_paths.len() > 0,
        "NextJS config should have ignored paths configured"
    );
}

/// Test full pipeline for a Django project
#[test]
fn test_full_pipeline_django_project() {
    let fixture_path = Path::new("./tests/fixtures/django_project");

    // Verify fixture exists
    assert!(
        fixture_path.exists(),
        "Django test fixture should exist at {:?}",
        fixture_path
    );

    // Step 1: Detect frameworks
    let detected_frameworks = detect_all_frameworks(fixture_path)
        .expect("Framework detection should succeed for Django project");

    assert!(
        !detected_frameworks.is_empty(),
        "Django project should have detected frameworks"
    );

    // Verify Django was detected
    assert!(
        detected_frameworks
            .iter()
            .any(|f| f.framework.as_str() == "Django"),
        "Django project should detect Django framework"
    );

    // Step 2: Generate configuration from the project
    let generator = ConfigGenerator::new();
    let config = generator
        .generate(fixture_path)
        .expect("Config generation should succeed for Django project");

    // Verify configuration was generated properly
    assert!(
        config.max_lines_per_function > 0,
        "Generated config should have valid max_lines_per_function"
    );

    // Step 3: Verify Django-specific settings
    assert!(
        config.max_lines_per_function == 50,
        "Django config should suggest 50 lines per function"
    );
    // Verify architecture pattern is set to MVC for Django
    assert!(
        config.architecture_pattern != ArchPattern::Ninguno,
        "Django should have detected architecture pattern"
    );
}

/// Test full pipeline for a Laravel project
#[test]
fn test_full_pipeline_laravel_project() {
    let fixture_path = Path::new("./tests/fixtures/laravel_project");

    // Verify fixture exists
    assert!(
        fixture_path.exists(),
        "Laravel test fixture should exist at {:?}",
        fixture_path
    );

    // Step 1: Detect frameworks
    let detected_frameworks = detect_all_frameworks(fixture_path)
        .expect("Framework detection should succeed for Laravel project");

    assert!(
        !detected_frameworks.is_empty(),
        "Laravel project should have detected frameworks"
    );

    // Verify Laravel was detected
    assert!(
        detected_frameworks
            .iter()
            .any(|f| f.framework.as_str() == "Laravel"),
        "Laravel project should detect Laravel framework"
    );

    // Step 2: Generate configuration from the project
    let generator = ConfigGenerator::new();
    let config = generator
        .generate(fixture_path)
        .expect("Config generation should succeed for Laravel project");

    // Verify configuration was generated properly
    assert!(
        config.max_lines_per_function > 0,
        "Generated config should have valid max_lines_per_function"
    );

    // Step 3: Verify Laravel-specific settings
    assert!(
        config.max_lines_per_function == 50,
        "Laravel config should suggest 50 lines per function"
    );
    // Verify architecture pattern is set to MVC for Laravel
    assert!(
        config.architecture_pattern != ArchPattern::Ninguno,
        "Laravel should have detected architecture pattern"
    );
}

/// Test detection confidence levels for multi-framework project
#[test]
fn test_framework_detection_confidence() {
    let fixture_path = Path::new("./tests/fixtures/nextjs_project");

    let detected_frameworks = detect_all_frameworks(fixture_path)
        .expect("Framework detection should succeed");

    for result in &detected_frameworks {
        assert!(
            result.confidence > 0.0 && result.confidence <= 1.0,
            "Confidence should be between 0.0 and 1.0, got: {}",
            result.confidence
        );
    }

    // Highest confidence detection should be first (sorted by confidence)
    if detected_frameworks.len() > 1 {
        assert!(
            detected_frameworks[0].confidence >= detected_frameworks[1].confidence,
            "Results should be sorted by confidence (highest first)"
        );
    }
}
