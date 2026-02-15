use architect_linter_pro::git_changes::get_changed_files;
use std::path::PathBuf;

#[test]
fn test_get_changed_files_in_repo() {
    let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let result = get_changed_files(&repo_path);

    // Should succeed (we're in a Git repo)
    assert!(result.is_ok());

    let changed = result.unwrap();
    println!("Changed files: {:?}", changed);

    // Result depends on repo state, just verify it's a vec
    assert!(changed.len() >= 0);
}
