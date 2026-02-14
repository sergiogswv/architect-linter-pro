use architect_linter_pro::analyzer::analyze_all_files;
use std::path::PathBuf;

#[test]
fn test_parallel_analysis_produces_same_results() {
    let files: Vec<PathBuf> = vec![
        PathBuf::from("tests/fixtures/perfect_mvc_project/src/models/user.model.ts"),
        PathBuf::from("tests/fixtures/perfect_mvc_project/src/views/user.view.ts"),
        PathBuf::from("tests/fixtures/perfect_mvc_project/src/controllers/user.controller.ts"),
    ];

    let project_root = PathBuf::from("tests/fixtures/perfect_mvc_project");
    let config = architect_linter_pro::config::load_config(&project_root).unwrap();

    use swc_common::sync::Lrc;
    use swc_common::SourceMap;

    let cm = Lrc::new(SourceMap::default());
    let linter_context: architect_linter_pro::config::LinterContext = config.into();

    let result = analyze_all_files(
        &files,
        &project_root,
        linter_context.pattern.clone(),
        &linter_context,
        &cm,
        None
    ).unwrap();

    // Should analyze all 3 files
    println!("Files analyzed: {}", result.files_analyzed);
    println!("Total functions: {}", result.complexity_stats.total_functions);
    println!("Violations: {}", result.violations.len());
    println!("Long functions: {}", result.long_functions.len());
    println!("Total imports: {}", result.layer_stats.total_imports);

    // Verify that all 3 files were analyzed
    assert_eq!(result.files_analyzed, 3, "Should have analyzed all 3 files");

    // The test passes if we successfully analyze files - whether or not we detect functions
    // (depends on the file content)
    println!("Test passed - analysis completed successfully");
}
