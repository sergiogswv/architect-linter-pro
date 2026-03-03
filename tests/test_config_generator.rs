//! Integration tests for ConfigGenerator
//!
//! Tests the smart configuration generation based on project structure
//! and detected frameworks.

use architect_linter_pro::config::ConfigGenerator;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_generator_creates_valid_config() {
    let temp_dir = TempDir::new().unwrap();

    // Create a NestJS project marker
    let package_json = r#"{"name": "test", "dependencies": {"@nestjs/core": "^10.0.0"}}"#;
    fs::write(temp_dir.path().join("package.json"), package_json).unwrap();

    let generator = ConfigGenerator::new();
    let config = generator.generate(temp_dir.path());

    assert!(config.is_ok(), "Generator should create a valid config");
    let config = config.unwrap();

    // Verify config has required fields
    assert!(config.max_lines_per_function > 0, "max_lines should be > 0");
    assert!(
        !config.forbidden_imports.is_empty(),
        "Config should have rules"
    );
}

#[test]
fn test_config_generator_respects_detected_frameworks() {
    let temp_dir = TempDir::new().unwrap();

    // Create a NestJS project marker
    let package_json = r#"{"name": "test", "dependencies": {"@nestjs/core": "^10.0.0"}}"#;
    fs::write(temp_dir.path().join("package.json"), package_json).unwrap();

    let generator = ConfigGenerator::new();
    let config = generator.generate(temp_dir.path()).unwrap();

    // NestJS max_lines should be 40
    assert_eq!(
        config.max_lines_per_function, 40,
        "NestJS should have max_lines of 40"
    );

    // Should have forbidden_imports generated from NestJS template
    assert!(
        !config.forbidden_imports.is_empty(),
        "NestJS config should have forbidden_imports"
    );
}

#[test]
fn test_config_generator_detects_django() {
    let temp_dir = TempDir::new().unwrap();

    // Create a Django project marker
    let requirements = "Django==4.2.0\npsycopg2==2.9.0";
    fs::write(temp_dir.path().join("requirements.txt"), requirements).unwrap();

    let generator = ConfigGenerator::new();
    let config = generator.generate(temp_dir.path()).unwrap();

    // Django max_lines should be 50
    assert_eq!(
        config.max_lines_per_function, 50,
        "Django should have max_lines of 50"
    );
}

#[test]
fn test_config_generator_detects_laravel() {
    let temp_dir = TempDir::new().unwrap();

    // Create a Laravel project marker
    let composer_json = r#"{"require": {"laravel/framework": "^11.0"}}"#;
    fs::write(temp_dir.path().join("composer.json"), composer_json).unwrap();

    let generator = ConfigGenerator::new();
    let config = generator.generate(temp_dir.path()).unwrap();

    // Laravel max_lines should be 50
    assert_eq!(
        config.max_lines_per_function, 50,
        "Laravel should have max_lines of 50"
    );
}

#[test]
fn test_config_generator_fails_without_frameworks() {
    let temp_dir = TempDir::new().unwrap();

    // Don't create any framework markers
    let generator = ConfigGenerator::new();
    let result = generator.generate(temp_dir.path());

    assert!(
        result.is_err(),
        "Generator should fail if no frameworks detected"
    );
}
