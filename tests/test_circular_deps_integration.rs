#[test]
fn test_circular_dependency_fixture_structure() {
    let fixture_path = "./tests/fixtures/circular-deps";
    assert!(std::path::Path::new(&format!("{}/architect.json", fixture_path)).exists());
    assert!(std::path::Path::new(&format!("{}/module-a.ts", fixture_path)).exists());
    assert!(std::path::Path::new(&format!("{}/module-b.ts", fixture_path)).exists());
}

#[test]
fn test_circular_dependency_imports_present() {
    let module_a = std::fs::read_to_string("./tests/fixtures/circular-deps/module-a.ts").unwrap();
    let module_b = std::fs::read_to_string("./tests/fixtures/circular-deps/module-b.ts").unwrap();

    assert!(module_a.contains("from './module-b'"));
    assert!(module_b.contains("from './module-a'"));
}
