use git2::{Error, Repository};
use std::path::{Path, PathBuf};

/// Get list of files changed since last commit
pub fn get_changed_files(repo_path: &Path) -> Result<Vec<PathBuf>, Error> {
    let repo = Repository::discover(repo_path)?;

    // Get HEAD commit
    let head = repo
        .head()?
        .target()
        .ok_or_else(|| Error::from_str("No HEAD commit"))?;

    let head_commit = repo.find_commit(head)?;
    let head_tree = head_commit.tree()?;

    // Get parent commit (if exists)
    let changed_files = if head_commit.parent_count() > 0 {
        let parent = head_commit.parent(0)?;
        let parent_tree = parent.tree()?;

        // Diff parent vs HEAD
        let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&head_tree), None)?;

        collect_diff_files(&diff, &repo)
    } else {
        // First commit, all files are "changed"
        collect_all_files(&head_tree, &repo)
    };

    Ok(changed_files)
}

fn collect_diff_files(diff: &git2::Diff, repo: &Repository) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let workdir = repo.workdir().unwrap_or(Path::new("."));

    let _ = diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                if is_typescript_file(path) {
                    files.push(workdir.join(path));
                }
            }
            true
        },
        None,
        None,
        None,
    );

    files
}

fn collect_all_files(tree: &git2::Tree, repo: &Repository) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let workdir = repo.workdir().unwrap_or(Path::new("."));

    let _ = tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
        if let Some(name) = entry.name() {
            let path = Path::new(name);
            if is_typescript_file(path) {
                files.push(workdir.join(root).join(name));
            }
        }
        git2::TreeWalkResult::Ok
    });

    files
}

fn is_typescript_file(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s == "ts" || s == "tsx" || s == "js" || s == "jsx")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_git_detection_in_crate() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let result = get_changed_files(&repo_path);
        assert!(result.is_ok());
    }
}
