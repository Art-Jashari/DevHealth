//! Git repository scanner and analyzer
//!
//! This module provides functionality for discovering and analyzing git repositories
//! within a directory tree. It can detect repository status, branch information,
//! uncommitted changes, and unpushed commits.

use crate::utils::fs;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents a git repository and its current state
///
/// Contains all relevant information about a discovered git repository,
/// including its location, status, branch, and change tracking.
#[derive(Debug, Clone)]
pub struct GitRepo {
    /// Absolute path to the repository root directory
    pub path: PathBuf,
    /// Current status of the repository (clean, dirty, or error)
    pub status: GitStatus,
    /// Name of the current branch
    pub branch: String,
    /// Whether there are uncommitted changes in the working directory
    pub uncommitted_changes: bool,
    /// Whether there are commits that haven't been pushed to the remote
    pub unpushed_commits: bool,
}

/// Represents the current status of a git repository
///
/// Indicates whether the repository is in a clean state, has uncommitted
/// changes, or encountered an error during analysis.
#[derive(Debug, Clone)]
pub enum GitStatus {
    /// Repository is clean with no uncommitted changes
    Clean,
    /// Repository has uncommitted changes in the working directory
    Dirty,
    /// An error occurred while analyzing the repository
    Error(String),
}

impl fmt::Display for GitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitStatus::Clean => write!(f, "✅ Clean"),
            GitStatus::Dirty => write!(f, "⚠️  Dirty"),
            GitStatus::Error(msg) => write!(f, "❌ Error: {}", msg),
        }
    }
}

/// Scans a directory tree for git repositories and analyzes their status
///
/// Recursively searches through the given directory to find all git repositories
/// and analyzes each one to determine its current state, including branch info,
/// uncommitted changes, and unpushed commits.
///
/// # Arguments
///
/// * `path` - The root directory to scan for git repositories
///
/// # Returns
///
/// A `Result` containing a vector of `GitRepo` structs representing all
/// discovered repositories and their analysis results.
///
/// # Examples
///
/// ```rust
/// use devhealth::scanner::git;
/// use std::path::Path;
///
/// let results = git::scan_directory(Path::new(".")).unwrap();
/// git::display_results(&results);
/// ```
///
/// # Errors
///
/// Returns an error if the directory cannot be accessed or traversed.
/// Individual git command failures are captured in the `GitStatus::Error` variant.
pub fn scan_directory(path: &Path) -> Result<Vec<GitRepo>, Box<dyn std::error::Error>> {
    let git_repos = fs::find_git_repositories(path)?;
    let mut results = Vec::new();

    for repo_path in git_repos {
        println!("  Scanning: {}", repo_path.display());

        match analyze_git_repo(&repo_path) {
            Ok(repo) => results.push(repo),
            Err(r) => {
                results.push(GitRepo {
                    path: repo_path,
                    status: GitStatus::Error(r.to_string()),
                    branch: "unknown".to_string(),
                    uncommitted_changes: false,
                    unpushed_commits: false,
                });
            }
        }
    }
    Ok(results)
}

/// Analyzes a single git repository to determine its current state
///
/// Executes git commands to gather information about the repository's
/// current branch, uncommitted changes, and unpushed commits.
///
/// # Arguments
///
/// * `repo_path` - Path to the git repository root directory
///
/// # Returns
///
/// A `Result` containing a `GitRepo` struct with the analysis results
/// or an error if the git commands fail.
///
/// # Errors
///
/// Returns an error if:
/// - Git is not installed or accessible
/// - The directory is not a valid git repository
/// - Git commands fail due to repository corruption or other issues
fn analyze_git_repo(repo_path: &Path) -> Result<GitRepo, Box<dyn std::error::Error>> {
    // Get current branch
    let branch_output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(repo_path)
        .output()?;

    let branch = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();

    // Check for uncommitted changes
    let status_output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(repo_path)
        .output()?;

    let uncommitted_changes = !status_output.stdout.is_empty();

    // Check for unpushed commits
    let unpushed_output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg(format!("origin/{}..HEAD", branch))
        .current_dir(repo_path)
        .output();

    let unpushed_commits = match unpushed_output {
        Ok(output) => !output.stdout.is_empty(),
        Err(_) => false, // Assume no unpushed commits if we can't check
    };

    let status = if uncommitted_changes {
        GitStatus::Dirty
    } else {
        GitStatus::Clean
    };

    Ok(GitRepo {
        path: repo_path.to_path_buf(),
        status,
        branch,
        uncommitted_changes,
        unpushed_commits,
    })
}

/// Displays the git repository scan results in a formatted output
///
/// Prints a comprehensive summary of all discovered git repositories,
/// including statistics and detailed information about each repository's status.
///
/// # Arguments
///
/// * `repos` - Slice of `GitRepo` structs to display
///
/// # Examples
///
/// ```rust
/// use devhealth::scanner::git;
/// use std::path::Path;
///
/// let repos = git::scan_directory(Path::new(".")).unwrap();
/// git::display_results(&repos);
/// ```
///
/// # Output Format
///
/// The function displays:
/// - Total number of repositories found
/// - Count of clean, dirty, and error repositories
/// - Detailed list with status, name, branch, and unpushed commit indicators
pub fn display_results(repos: &[GitRepo]) {
    if repos.is_empty() {
        println!("  No git repositories found.");
        return;
    }

    println!("\n📊 Git Repository Summary:");
    println!("  Total repositories: {}", repos.len());

    let clean_count = repos
        .iter()
        .filter(|r| matches!(r.status, GitStatus::Clean))
        .count();
    let dirty_count = repos
        .iter()
        .filter(|r| matches!(r.status, GitStatus::Dirty))
        .count();
    let error_count = repos
        .iter()
        .filter(|r| matches!(r.status, GitStatus::Error(_)))
        .count();

    println!(
        "  Clean: {}, Dirty: {}, Errors: {}",
        clean_count, dirty_count, error_count
    );

    println!("\n📁 Repository Details: ");
    for repo in repos {
        let path_str = repo
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        let unpushed_indicator = if repo.unpushed_commits { " 🔄" } else { "" };

        println!(
            "  {} {} ({}){}",
            repo.status, path_str, repo.branch, unpushed_indicator
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Create a test GitRepo with default values for easier testing
    fn create_test_repo(name: &str, status: GitStatus) -> GitRepo {
        GitRepo {
            path: PathBuf::from(format!("/test/{}", name)),
            status,
            branch: "main".to_string(),
            uncommitted_changes: false,
            unpushed_commits: false,
        }
    }

    mod git_status {
        use super::*;

        #[test]
        fn displays_clean_status_correctly() {
            assert_eq!(format!("{}", GitStatus::Clean), "✅ Clean");
        }

        #[test]
        fn displays_dirty_status_correctly() {
            assert_eq!(format!("{}", GitStatus::Dirty), "⚠️  Dirty");
        }

        #[test]
        fn displays_error_status_with_message() {
            let error_msg = "Repository not found";
            assert_eq!(
                format!("{}", GitStatus::Error(error_msg.to_string())),
                format!("❌ Error: {}", error_msg)
            );
        }
    }

    mod git_repo {
        use super::*;

        #[test]
        fn creates_repo_with_correct_properties() {
            let repo = GitRepo {
                path: PathBuf::from("/test/my-project"),
                status: GitStatus::Clean,
                branch: "develop".to_string(),
                uncommitted_changes: true,
                unpushed_commits: false,
            };

            assert_eq!(repo.path, PathBuf::from("/test/my-project"));
            assert_eq!(repo.branch, "develop");
            assert!(repo.uncommitted_changes);
            assert!(!repo.unpushed_commits);
            assert!(matches!(repo.status, GitStatus::Clean));
        }

        #[test]
        fn handles_different_status_types() {
            let clean_repo = create_test_repo("clean", GitStatus::Clean);
            let dirty_repo = create_test_repo("dirty", GitStatus::Dirty);
            let error_repo = create_test_repo("error", GitStatus::Error("test error".to_string()));

            assert!(matches!(clean_repo.status, GitStatus::Clean));
            assert!(matches!(dirty_repo.status, GitStatus::Dirty));
            assert!(matches!(error_repo.status, GitStatus::Error(_)));
        }
    }

    mod scan_directory {
        use super::*;

        #[test]
        fn returns_empty_vec_for_directory_without_git_repos() {
            let temp_dir = TempDir::new().expect("Failed to create temp directory");

            let result = scan_directory(temp_dir.path())
                .expect("scan_directory should succeed on empty directory");

            assert!(
                result.is_empty(),
                "Should return empty vector for directory with no git repos"
            );
        }

        #[test]
        fn finds_git_repository_in_directory() {
            let temp_dir = TempDir::new().expect("Failed to create temp directory");
            fs::create_dir(temp_dir.path().join(".git")).expect("Failed to create .git directory");

            let result = scan_directory(temp_dir.path()).expect("scan_directory should succeed");

            assert_eq!(result.len(), 1, "Should find exactly one git repository");
            assert_eq!(
                result[0].path,
                temp_dir.path(),
                "Should find the correct repository path"
            );
        }

        #[test]
        fn handles_inaccessible_git_repositories_gracefully() {
            let temp_dir = TempDir::new().expect("Failed to create temp directory");
            fs::create_dir(temp_dir.path().join(".git")).expect("Failed to create .git directory");

            // This test ensures we don't panic on git command failures
            let result = scan_directory(temp_dir.path());

            assert!(
                result.is_ok(),
                "Should handle git command failures gracefully"
            );
            if let Ok(repos) = result {
                assert_eq!(repos.len(), 1, "Should still find the repository");
                // The status might be Error due to git commands failing, which is expected
            }
        }
    }

    mod display_results {
        use super::*;

        #[test]
        fn handles_empty_repository_list() {
            let repos = vec![];
            // This should not panic
            display_results(&repos);
        }

        #[test]
        fn displays_multiple_repositories_correctly() {
            let repos = vec![
                GitRepo {
                    path: PathBuf::from("/test/clean-repo"),
                    status: GitStatus::Clean,
                    branch: "main".to_string(),
                    uncommitted_changes: false,
                    unpushed_commits: false,
                },
                GitRepo {
                    path: PathBuf::from("/test/dirty-repo"),
                    status: GitStatus::Dirty,
                    branch: "feature/new-feature".to_string(),
                    uncommitted_changes: true,
                    unpushed_commits: true,
                },
                GitRepo {
                    path: PathBuf::from("/test/error-repo"),
                    status: GitStatus::Error("Permission denied".to_string()),
                    branch: "unknown".to_string(),
                    uncommitted_changes: false,
                    unpushed_commits: false,
                },
            ];

            // This should not panic and should handle all status types
            display_results(&repos);
        }
    }
}
