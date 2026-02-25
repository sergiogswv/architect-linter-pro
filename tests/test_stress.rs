#[test]
fn test_large_project_fixture_created() {
    let fixture_path = "./tests/fixtures/large-project";
    assert!(std::path::Path::new(fixture_path).exists());
    assert!(std::path::Path::new(&format!("{}/architect.json", fixture_path)).exists());
}

#[test]
fn test_large_project_has_multiple_files() {
    let fixture_path = "./tests/fixtures/large-project/services";
    let entries: Vec<_> = std::fs::read_dir(fixture_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    // Should have 50+ files
    assert!(entries.len() >= 50, "Expected at least 50 service files, found {}", entries.len());
}
