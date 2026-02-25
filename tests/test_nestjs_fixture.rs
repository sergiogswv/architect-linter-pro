#[test]
fn test_nestjs_fixture_project_structure() {
    let fixture_path = "./tests/fixtures/nestjs-project";

    // Verify fixture files exist
    assert!(std::path::Path::new(&format!("{}/architect.json", fixture_path)).exists());
    assert!(std::path::Path::new(&format!("{}/src/domain/user.entity.ts", fixture_path)).exists());
    assert!(std::path::Path::new(&format!("{}/src/application/user.service.ts", fixture_path)).exists());
    assert!(std::path::Path::new(&format!("{}/src/infrastructure/database.repository.ts", fixture_path)).exists());
}

#[test]
fn test_nestjs_fixture_valid_json() {
    let fixture_path = "./tests/fixtures/nestjs-project/architect.json";
    let content = std::fs::read_to_string(fixture_path).expect("Failed to read architect.json");
    let _: serde_json::Value = serde_json::from_str(&content).expect("Invalid JSON in architect.json");
}
