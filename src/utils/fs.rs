//! File system utilities for DevHealth
//!
//! This module provides file system operations specifically tailored for
//! development environment analysis, including git repository discovery
//! and directory traversal functionality.

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Finds all git repositories within a directory tree
///
/// Recursively searches through the given directory and its subdirectories
/// to locate all git repositories (directories containing a `.git` folder).
///
/// # Arguments
///
/// * `root` - The root directory to start searching from
///
/// # Returns
///
/// A `Result` containing a vector of `PathBuf`s pointing to git repository
/// root directories, or an error if the directory cannot be accessed.
///
/// # Examples
///
/// ```rust
/// use devhealth::utils::fs;
/// use std::path::Path;
///
/// let repos = fs::find_git_repositories(Path::new(".")).unwrap();
/// for repo in repos {
///     println!("Found git repository: {}", repo.display());
/// }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The root directory cannot be accessed
/// - Permission is denied for subdirectories
/// - File system errors occur during traversal
pub fn find_git_repositories(root: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut git_repos = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Check if this directory contains a .git folder
        if path.file_name().and_then(|name| name.to_str()) == Some(".git") {
            if let Some(parent) = path.parent() {
                git_repos.push(parent.to_path_buf());
            }
        }
    }

    Ok(git_repos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Test helper to create a temporary directory with a .git folder
    fn create_git_repo_in(parent: &std::path::Path, name: &str) -> std::path::PathBuf {
        let repo_path = parent.join(name);
        fs::create_dir_all(&repo_path).unwrap();
        fs::create_dir(repo_path.join(".git")).unwrap();
        repo_path
    }

    #[test]
    fn finds_no_repositories_in_empty_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repos = find_git_repositories(temp_dir.path()).expect("Function should succeed");
        assert!(
            repos.is_empty(),
            "Should find no repositories in empty directory"
        );
    }

    #[test]
    fn finds_single_git_repository() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).expect("Failed to create .git directory");

        let repos = find_git_repositories(temp_dir.path()).expect("Function should succeed");

        assert_eq!(repos.len(), 1, "Should find exactly one repository");
        assert_eq!(
            repos[0],
            temp_dir.path(),
            "Found repository should be the temp directory"
        );
    }

    #[test]
    fn finds_multiple_nested_repositories() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        // Create a more complex directory structure
        let project1 = create_git_repo_in(temp_dir.path(), "project1");
        let project2 = create_git_repo_in(temp_dir.path(), "nested/project2");
        let project3 = create_git_repo_in(temp_dir.path(), "deep/nested/project3");

        let repos = find_git_repositories(temp_dir.path()).expect("Function should succeed");

        assert_eq!(repos.len(), 3, "Should find exactly three repositories");

        // Convert to a set for order-independent comparison
        let found_repos: std::collections::HashSet<_> = repos.into_iter().collect();
        let expected_repos: std::collections::HashSet<_> =
            [project1, project2, project3].into_iter().collect();

        assert_eq!(
            found_repos, expected_repos,
            "Should find all created repositories"
        );
    }

    #[test]
    fn ignores_directories_that_are_not_git_repositories() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        // Create regular directories
        fs::create_dir_all(temp_dir.path().join("not-a-repo")).unwrap();
        fs::create_dir_all(temp_dir.path().join("also-not-a-repo/subdir")).unwrap();

        // Create one actual git repo
        create_git_repo_in(temp_dir.path(), "actual-repo");

        let repos = find_git_repositories(temp_dir.path()).expect("Function should succeed");

        assert_eq!(repos.len(), 1, "Should only find the actual git repository");
        assert_eq!(
            repos[0].file_name().and_then(|n| n.to_str()),
            Some("actual-repo"),
            "Should find the correct repository"
        );
    }

    #[test]
    fn handles_symlinks_correctly() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let git_repo = create_git_repo_in(temp_dir.path(), "real-repo");

        // Create a symlink to the git repo (if supported by the platform)
        #[cfg(unix)]
        {
            let symlink_path = temp_dir.path().join("symlink-repo");
            if std::os::unix::fs::symlink(&git_repo, &symlink_path).is_ok() {
                let repos =
                    find_git_repositories(temp_dir.path()).expect("Function should succeed");

                // Should find the real repo but not follow the symlink
                // (because we set follow_links(false))
                assert_eq!(
                    repos.len(),
                    1,
                    "Should find only the real repository, not the symlink"
                );
                assert_eq!(repos[0], git_repo, "Should find the original repository");
            }
        }
    }
}
