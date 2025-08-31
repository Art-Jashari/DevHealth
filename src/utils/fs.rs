use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;

pub fn find_git_repositories(root: Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
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