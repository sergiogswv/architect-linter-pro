use std::fs;
use tempfile::TempDir;

fn make_nestjs_project(dir: &TempDir) {
    let pkg = r#"{"dependencies": {"@nestjs/core": "^10.0.0"}}"#;
    fs::write(dir.path().join("package.json"), pkg).unwrap();
}

#[test]
fn test_init_fails_if_config_exists_without_force() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("architect.json"), "{}").unwrap();

    let result = architect_linter_pro::init::check_no_existing_config(dir.path(), false);
    assert!(result.is_err(), "Should fail if architect.json exists and force=false");
}

#[test]
fn test_init_succeeds_if_config_exists_with_force() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("architect.json"), "{}").unwrap();

    let result = architect_linter_pro::init::check_no_existing_config(dir.path(), true);
    assert!(result.is_ok(), "Should succeed with --force even if file exists");
}

#[test]
fn test_write_config_creates_valid_json() {
    let dir = TempDir::new().unwrap();
    use architect_linter_pro::config::Framework;

    let tmpl = architect_linter_pro::init::templates::get_template(&Framework::NestJS, "hexagonal").unwrap();
    architect_linter_pro::init::write_config(dir.path(), &tmpl).unwrap();

    let content = fs::read_to_string(dir.path().join("architect.json")).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(parsed["forbidden_imports"].is_array());
    assert!(!parsed["forbidden_imports"].as_array().unwrap().is_empty());
}

// Suppress unused warning for the helper
#[allow(dead_code)]
fn _use_make_nestjs_project(dir: &TempDir) {
    make_nestjs_project(dir);
}
