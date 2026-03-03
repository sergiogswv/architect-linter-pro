/// Real-world project validation test
///
/// This test validates that the tool can analyze real projects without panicking
/// or producing invalid results. It's marked as #[ignore] because it runs on the
/// actual project directory and can be slow.
///
/// To run: cargo test --test integration/real_world -- --ignored --nocapture

use std::path::Path;

#[test]
#[ignore]
fn test_on_current_project() {
    use architect_linter::detection::detect_all_frameworks;

    let result = detect_all_frameworks(Path::new("."));
    assert!(result.is_ok(), "Framework detection failed: {:?}", result.err());

    let frameworks = result.unwrap();
    println!("Detected frameworks: {:?}", frameworks);

    // Should detect at least the Rust framework (from Cargo.toml)
    assert!(
        !frameworks.is_empty(),
        "Should detect at least one framework in current project"
    );
}
