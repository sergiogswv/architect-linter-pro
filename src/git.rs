//! Git operations for --staged flag support
//!
//! This module provides functions to interact with git repositories
//! and get information about staged files.

use git2::{Repository, Status, StatusOptions, StatusShow};
use miette::Result;
use std::path::{Path, PathBuf};

/// Check if the given path is inside a git repository
pub fn is_git_repo(path: &Path) -> bool {
    Repository::discover(path).is_ok()
}

/// Get all staged files in the repository
/// Returns a list of absolute paths to staged files
pub fn get_staged_files(repo_path: &Path) -> Result<Vec<PathBuf>> {
    let repo = Repository::discover(repo_path)
        .map_err(|e| miette::miette!("No se encontró repositorio git: {}", e))?;

    let workdir = repo
        .workdir()
        .ok_or_else(|| miette::miette!("El repositorio no tiene directorio de trabajo"))?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(false)
        .include_ignored(false)
        .include_unmodified(false)
        .show(StatusShow::Index); // Only staged files

    let statuses = repo
        .statuses(Some(&mut opts))
        .map_err(|e| miette::miette!("Error al obtener estados de git: {}", e))?;

    let staged_files: Vec<PathBuf> = statuses
        .iter()
        .filter(|entry| {
            let status = entry.status();
            // Include files that are staged (added, modified, renamed, etc.)
            status.contains(Status::INDEX_NEW)
                || status.contains(Status::INDEX_MODIFIED)
                || status.contains(Status::INDEX_RENAMED)
                || status.contains(Status::INDEX_TYPECHANGE)
        })
        .filter_map(|entry| {
            let path = entry.path()?;
            let full_path = workdir.join(path);
            // Only include existing files
            if full_path.exists() {
                Some(full_path)
            } else {
                None
            }
        })
        .collect();

    Ok(staged_files)
}

/// Filter files to only include those that are staged
pub fn filter_staged_files(all_files: &[PathBuf], repo_path: &Path) -> Result<Vec<PathBuf>> {
    let staged = get_staged_files(repo_path)?;
    let staged_set: std::collections::HashSet<_> = staged.iter().collect();

    Ok(all_files
        .iter()
        .filter(|f| staged_set.contains(f))
        .cloned()
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_repo() {
        // This test depends on running from within a git repo
        let current_dir = std::env::current_dir().unwrap();
        // Just verify the function doesn't panic
        let _ = is_git_repo(&current_dir);
    }
}
