//! # DevHealth - Development Environment Health Monitor
//!
//! DevHealth is a comprehensive CLI tool for monitoring and maintaining the health
//! of software development environments. It provides insights into git repository
//! status, dependency health, and system resource usage.
//!
//! ## Features
//!
//! - **Git Repository Health**: Scan directories for git repositories and check their status
//! - **Dependency Analysis**: Monitor project dependencies (planned feature)
//! - **System Monitoring**: Track system resource usage (planned feature)
//! - **Project Analytics**: Analyze code quality metrics (planned feature)
//!
//! ## Usage
//!
//! ```bash
//! # Quick health check of current directory
//! devhealth check
//!
//! # Comprehensive scan with specific options
//! devhealth scan --git --deps --system
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use devhealth::{cli::Cli, scanner};
//! use clap::Parser;
//! use std::path::Path;
//!
//! // Parse CLI arguments
//! let cli = Cli::parse_from(["devhealth", "check"]);
//!
//! // Scan for git repositories
//! let path = Path::new(".");
//! let repos = scanner::git::scan_directory(&path).unwrap();
//! scanner::git::display_results(&repos);
//! ```

pub mod cli;
pub mod scanner;
pub mod utils;

pub use cli::Cli;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_can_be_instantiated_programmatically() {
        use clap::Parser;

        let cli = Cli::parse_from(["devhealth", "check"]);

        // Verify we can match on the command
        match cli.command {
            cli::Commands::Check { .. } => {
                // Test passes if we can match successfully
            }
            _ => panic!("Expected Check command when parsing 'devhealth check'"),
        }
    }

    #[test]
    fn all_scanner_modules_are_accessible() {
        // This test ensures all modules compile and are accessible
        // If any module had compilation errors, this test would fail

        // Test that we can access types from git scanner
        let _status = scanner::git::GitStatus::Clean;

        // Test that we can call functions from all scanner modules
        scanner::deps::scan_dependencies();
        scanner::system::monitor_system();
        scanner::analytics::analyze_projects();
    }

    #[test]
    fn public_api_exports_expected_items() {
        use clap::Parser;

        // Verify that the lib exports the expected public API
        let _cli_type = Cli::parse_from(["devhealth", "check"]);

        // These should be accessible without explicit module paths
        // since they're re-exported in lib.rs
    }
}
