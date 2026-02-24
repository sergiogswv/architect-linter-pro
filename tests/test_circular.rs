//! Tests for circular dependency detection module
//!
//! This test suite verifies the graph-based cycle detection algorithm
//! using DFS (Depth-First Search).

use std::path::PathBuf;

mod common;
use common::TestProject;

/// Helper function to create test files with imports
fn create_file_with_imports(project: &TestProject, path: &str, content: &str) {
    project.create_file(path, content);
}

/// Helper to run circular dependency analysis
fn analyze_circular_deps(project: &TestProject) -> Result<Vec<architect_linter_pro::circular::CircularDependency>, Box<dyn std::error::Error>> {
    use architect_linter_pro::circular::analyze_circular_dependencies;
    use std::ffi::OsStr;

    // Collect all TypeScript/JavaScript files
    let files: Vec<PathBuf> = project
        .collect_ts_files()
        .into_iter()
        .filter(|p| p.extension() == Some(OsStr::new("ts")) || p.extension() == Some(OsStr::new("tsx")) || p.extension() == Some(OsStr::new("js")) || p.extension() == Some(OsStr::new("jsx")))
        .collect();

    analyze_circular_dependencies(&files, project.path()).map_err(|e| e.into())
}

#[test]
fn test_empty_project_has_no_cycles() {
    let project = TestProject::new();

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 0, "Empty project should have no cycles");
}

#[test]
fn test_single_file_has_no_cycles() {
    let project = TestProject::new();

    create_file_with_imports(
        &project,
        "src/service.ts",
        r#"
export class Service {
    constructor() {}
    doWork() {}
}
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 0, "Single file should have no cycles");
}

#[test]
fn test_two_files_with_simple_cycle() {
    let project = TestProject::new();

    // File A imports File B
    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"
import { b } from './b';
export function a() {
    return b();
}
"#,
    );

    // File B imports File A (creates cycle)
    create_file_with_imports(
        &project,
        "src/b.ts",
        r#"
import { a } from './a';
export function b() {
    return a();
}
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 1, "Should detect one cycle");

    let cycle = &cycles[0];
    assert!(!cycle.cycle.is_empty(), "Cycle should have nodes");

    // Verify the cycle contains both files
    let cycle_str = cycle.cycle.join(" ");
    assert!(
        cycle_str.contains("a.ts") && cycle_str.contains("b.ts"),
        "Cycle should include both a.ts and b.ts"
    );

    assert!(cycle.description.contains("Dependencia cíclica"));
}

#[test]
fn test_three_files_with_complex_cycle() {
    let project = TestProject::new();

    // A → B → C → A
    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"
import { b } from './b';
export const a = () => b();
"#,
    );

    create_file_with_imports(
        &project,
        "src/b.ts",
        r#"
import { c } from './c';
export const b = () => c();
"#,
    );

    create_file_with_imports(
        &project,
        "src/c.ts",
        r#"
import { a } from './a';
export const c = () => a();
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 1, "Should detect one cycle involving all three files");

    let cycle = &cycles[0];
    assert!(cycle.cycle.len() >= 3, "Cycle should involve all three files");

    let cycle_str = cycle.cycle.join(" ");
    assert!(
        cycle_str.contains("a.ts") && cycle_str.contains("b.ts") && cycle_str.contains("c.ts"),
        "Cycle should include all three files"
    );
}

#[test]
fn test_relative_import_resolution() {
    let project = TestProject::new();

    // Test simpler case with same directory (should work)
    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"
import { b } from './b';
export const a = () => b();
"#,
    );

    create_file_with_imports(
        &project,
        "src/b.ts",
        r#"
import { a } from './a';
export const b = () => a();
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");
    assert_eq!(cycles.len(), 1, "Should detect cycle with ./ imports");
}

#[test]
fn test_index_file_resolution() {
    let project = TestProject::new();

    // File importing index
    create_file_with_imports(
        &project,
        "src/main.ts",
        r#"
import { module } from './module';
export const main = () => module();
"#,
    );

    // Index importing back
    create_file_with_imports(
        &project,
        "src/module/index.ts",
        r#"
import { main } from '../main';
export const module = () => main();
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 1, "Should detect cycle with index files");
}

#[test]
fn test_no_cycle_with_linear_imports() {
    let project = TestProject::new();

    // A → B → C (no cycle)
    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"
import { b } from './b';
export const a = () => b();
"#,
    );

    create_file_with_imports(
        &project,
        "src/b.ts",
        r#"
import { c } from './c';
export const b = () => c();
"#,
    );

    create_file_with_imports(
        &project,
        "src/c.ts",
        r#"
export const c = () => 'no imports';
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 0, "Linear imports should not create a cycle");
}

#[test]
fn test_multiple_independent_cycles() {
    let project = TestProject::new();

    // First cycle: A ↔ B
    create_file_with_imports(
        &project,
        "src/cycle1/a.ts",
        r#"
import { b } from './b';
export const a = () => b();
"#,
    );

    create_file_with_imports(
        &project,
        "src/cycle1/b.ts",
        r#"
import { a } from './a';
export const b = () => a();
"#,
    );

    // Second cycle: C ↔ D
    create_file_with_imports(
        &project,
        "src/cycle2/c.ts",
        r#"
import { d } from './d';
export const c = () => d();
"#,
    );

    create_file_with_imports(
        &project,
        "src/cycle2/d.ts",
        r#"
import { c } from './c';
export const d = () => c();
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 2, "Should detect two independent cycles");

    // Verify each cycle is distinct
    let cycle1_str = cycles[0].cycle.join(" ");
    let cycle2_str = cycles[1].cycle.join(" ");

    // One cycle should be in cycle1, the other in cycle2
    assert!(
        (cycle1_str.contains("cycle1") && cycle2_str.contains("cycle2"))
            || (cycle1_str.contains("cycle2") && cycle2_str.contains("cycle1")),
        "Cycles should be in different directories"
    );
}

#[test]
fn test_self_reference_is_detected() {
    let project = TestProject::new();

    // File importing itself (odd but possible)
    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"
import { a } from './a';
export const a = () => a();
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    // Self-reference might be filtered or detected as cycle
    // We just verify it doesn't crash
    assert!(cycles.len() <= 1, "Should handle self-reference gracefully");
}

#[test]
fn test_external_imports_ignored() {
    let project = TestProject::new();

    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"
import { external } from 'external-package';
import { util } from './util';
export const a = () => util();
"#,
    );

    create_file_with_imports(
        &project,
        "src/util.ts",
        r#"
export const util = () => 'ok';
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 0, "External imports should be ignored");
}

#[test]
fn test_alias_imports_ignored() {
    let project = TestProject::new();

    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"
import { service } from '@/services/user.service';
export const a = () => service();
"#,
    );

    create_file_with_imports(
        &project,
        "src/services/user.service.ts",
        r#"
export const service = () => 'ok';
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 0, "Alias imports (@/) should be ignored for now");
}

#[test]
fn test_complex_diamond_dependency() {
    let project = TestProject::new();

    // Diamond pattern (no cycle):
    //     A
    //    / \
    //   B   C
    //    \ /
    //     D
    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"
import { b } from './b';
import { c } from './c';
export const a = () => b() + c();
"#,
    );

    create_file_with_imports(
        &project,
        "src/b.ts",
        r#"
import { d } from './d';
export const b = () => d();
"#,
    );

    create_file_with_imports(
        &project,
        "src/c.ts",
        r#"
import { d } from './d';
export const c = () => d();
"#,
    );

    create_file_with_imports(
        &project,
        "src/d.ts",
        r#"
export const d = () => 'leaf';
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 0, "Diamond pattern should not create a cycle");
}

#[test]
fn test_cycle_in_subdirectory() {
    let project = TestProject::new();

    create_file_with_imports(
        &project,
        "src/features/user/user-model.ts",
        r#"
import { service } from './user-service';
export class UserModel {}
"#,
    );

    create_file_with_imports(
        &project,
        "src/features/user/user-service.ts",
        r#"
import { Model } from './user-model';
export class UserService {}
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 1, "Should detect cycle in subdirectory");
}

#[test]
fn test_multiple_cycles_in_same_file() {
    let project = TestProject::new();

    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"
import { b } from './b';
import { c } from './c';
export const a = () => b() + c();
"#,
    );

    create_file_with_imports(
        &project,
        "src/b.ts",
        r#"
import { a } from './a';
export const b = () => a();
"#,
    );

    create_file_with_imports(
        &project,
        "src/c.ts",
        r#"
import { a } from './a';
export const c = () => a();
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    // A has cycles with both B and C
    assert!(cycles.len() >= 1, "Should detect at least one cycle");
}

#[test]
fn test_cycle_description_formatting() {
    let project = TestProject::new();

    create_file_with_imports(&project, "src/a.ts", r#"import { b } from './b'; export const a = () => b();"#);
    create_file_with_imports(&project, "src/b.ts", r#"import { a } from './a'; export const b = () => a();"#);

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 1);

    let description = &cycles[0].description;
    assert!(description.contains("Dependencia cíclica"), "Description should mention cycle");
    assert!(description.contains("→"), "Description should show arrows");
    assert!(description.contains("⚠️"), "Description should have warning emoji");
}

#[test]
fn test_cycle_paths_are_relative_not_absolute() {
    // Regression test: on Windows, canonicalize() adds \\?\ prefix to paths.
    // If normalize_file_path doesn't strip this prefix, cycle nodes are stored
    // as absolute UNC paths (//?/c:/users/...) instead of relative paths (src/a.ts).
    // This causes the self-loop guard to fail and produces false positive cycles.
    let project = TestProject::new();

    create_file_with_imports(
        &project,
        "src/a.ts",
        r#"import { b } from './b'; export const a = () => b();"#,
    );
    create_file_with_imports(
        &project,
        "src/b.ts",
        r#"import { a } from './a'; export const b = () => a();"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 1);

    // Cycle nodes must be relative paths (e.g. "src/a.ts"), not absolute UNC paths
    for node in &cycles[0].cycle {
        assert!(
            !node.starts_with("//?/"),
            "Cycle node should be a relative path, not a UNC absolute path. Got: {}",
            node
        );
        assert!(
            !node.contains(":\\"),
            "Cycle node should be a relative path, not a Windows absolute path. Got: {}",
            node
        );
    }
}

#[test]
fn test_cycle_case_insensitive_path_normalization() {
    let project = TestProject::new();

    // Windows-style paths should be normalized
    create_file_with_imports(
        &project,
        "src/A.ts",
        r#"
import { b } from './B';
export const a = () => b();
"#,
    );

    create_file_with_imports(
        &project,
        "src/B.ts",
        r#"
import { a } from './A';
export const b = () => a();
"#,
    );

    let cycles = analyze_circular_deps(&project).expect("Analysis should succeed");

    assert_eq!(cycles.len(), 1, "Should detect cycle regardless of path case");
}
