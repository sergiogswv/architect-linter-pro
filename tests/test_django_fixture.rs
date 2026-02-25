#[test]
fn test_django_fixture_project_structure() {
    let fixture_path = "./tests/fixtures/django-project";

    assert!(std::path::Path::new(&format!("{}/architect.json", fixture_path)).exists());
    assert!(std::path::Path::new(&format!("{}/myapp/models/user.py", fixture_path)).exists());
    assert!(std::path::Path::new(&format!("{}/myapp/services/user_service.py", fixture_path)).exists());
    assert!(std::path::Path::new(&format!("{}/myapp/views/user_view.py", fixture_path)).exists());
}

#[test]
fn test_django_fixture_valid_config() {
    let config_path = "./tests/fixtures/django-project/architect.json";
    let content = std::fs::read_to_string(config_path).expect("Failed to read architect.json");
    let _: serde_json::Value = serde_json::from_str(&content).expect("Invalid JSON");
}
