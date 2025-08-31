use crate::utils::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GitRepo {
    pub path: PathBuf,
    pub status: GitStatus,
    pub branch: String,
    pub uncommitted_changes: bool,
    pub unpushed_commits: bool,
}

#[derive(Debug, Clone)]
pub enum GitStatus {
    Clean,
    Dirty,
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

pub fn scan_directory(path: &Path) -> Result<Vec<GitRepo>, Box<dyn std::error::Error>> {
    let git_repos = fs::find_git_repositories(path)?;
    let mut results = Vec::new();

    for repo_path in git_repos {
        println!("  Scanning: {}", repo_path.display());

        match analyze_git_repo(&repo_path) {
            Ok(repo) => results.push(repo),
            Err(r) =>  {
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

fn analyze_git_repo(repo_path: &Path) -> Result<GitRepo, Box<dyn std::error::Error>> {
    // Get current branch
    let branch_output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(repo_path)
        .output()?;
    
    let branch = String::from_utf8_lossy(&branch_output.stdout).trim().to_string();
    
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
        .arg(&format!("origin/{}..HEAD", branch))
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
pub fn display_results(repos: &[GitRepo]) {
    if repos.is_empty() {
        println!("  No git repositories found.");
        return;
    }
    
    println!("\n📊 Git Repository Summary:");
    println!("  Total repositories: {}", repos.len());

    let clean_count = repos.iter().filter(|r| matches!(r.status, GitStatus::Clean)).count();
    let dirty_count = repos.iter().filter(|r| matches!(r.status, GitStatus::Dirty)).count();
    let error_count = repos.iter().filter(|r| matches!(r.status, GitStatus::Error(_))).count();

    println!("  Clean: {}, Dirty: {}, Errors: {}", clean_count, dirty_count, error_count);

    println!("\n📁 Repository Details: ");
    for repo in repos {
        let path_str = repo.path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        let unpushed_indicator = if repo.unpushed_commits {" 🔄"} else {""};
        
        println!("  {} {} ({}){}",
            repo.status,
            path_str,
            repo.branch,
            unpushed_indicator);
    }   
}
