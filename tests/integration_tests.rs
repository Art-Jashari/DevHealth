use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to run the devhealth CLI with given arguments
fn run_devhealth(args: &[&str]) -> std::process::Output {
    let mut cmd_args = vec!["run", "--"];
    cmd_args.extend(args);

    Command::new("cargo")
        .args(cmd_args)
        .output()
        .expect("Failed to execute devhealth command")
}

/// Helper function to create a directory structure with git repositories
fn create_test_git_repos(temp_dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let repos = vec![
        temp_dir.join("project1"),
        temp_dir.join("nested").join("project2"),
        temp_dir.join("deep").join("nested").join("project3"),
    ];

    for repo in &repos {
        fs::create_dir_all(repo).expect("Failed to create repository directory");
        fs::create_dir(repo.join(".git")).expect("Failed to create .git directory");
    }

    repos
}

mod cli_interface {
    use super::*;

    #[test]
    fn shows_help_when_requested() {
        let output = run_devhealth(&["--help"]);

        assert!(output.status.success(), "Help command should succeed");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("DevHealth CLI application"));
        assert!(stdout.contains("command-line tool for monitoring development environment health"));
        assert!(
            stdout.contains("check"),
            "Help should mention 'check' command"
        );
        assert!(
            stdout.contains("scan"),
            "Help should mention 'scan' command"
        );
    }

    #[test]
    fn shows_version_when_requested() {
        let output = run_devhealth(&["--version"]);

        assert!(output.status.success(), "Version command should succeed");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("0.2.0"),
            "Should display correct version number"
        );
    }

    #[test]
    fn fails_gracefully_with_invalid_command() {
        let output = run_devhealth(&["invalid-command"]);

        assert!(!output.status.success(), "Invalid command should fail");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("error:") || stderr.contains("Error"),
            "Should show error message"
        );
    }
}

mod check_command {
    use super::*;

    #[test]
    fn handles_directory_with_no_git_repositories() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        let output = run_devhealth(&["check", "--path", temp_dir.path().to_str().unwrap()]);

        assert!(
            output.status.success(),
            "Check command should succeed even with no repos"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Running health check"),
            "Should indicate it's running a health check"
        );
        assert!(
            stdout.contains("No git repositories found"),
            "Should report no repositories found"
        );
    }

    #[test]
    fn finds_and_reports_git_repository() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        fs::create_dir(temp_dir.path().join(".git")).expect("Failed to create .git directory");

        let output = run_devhealth(&["check", "--path", temp_dir.path().to_str().unwrap()]);

        assert!(
            output.status.success(),
            "Check command should succeed with git repo"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Running health check"),
            "Should indicate health check is running"
        );
        assert!(
            stdout.contains("Git Repository Summary"),
            "Should show repository summary"
        );
        assert!(
            stdout.contains("Total repositories: 1"),
            "Should find exactly one repository"
        );
    }

    #[test]
    fn finds_multiple_nested_repositories() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        create_test_git_repos(temp_dir.path());

        let output = run_devhealth(&["check", "--path", temp_dir.path().to_str().unwrap()]);

        assert!(
            output.status.success(),
            "Check command should succeed with multiple repos"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Total repositories: 3"),
            "Should find all three repositories"
        );
    }

    #[test]
    fn uses_current_directory_as_default() {
        let output = run_devhealth(&["check"]);

        assert!(
            output.status.success(),
            "Check command should succeed with default path"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Running health check on: ."),
            "Should use current directory as default"
        );
    }
}

mod scan_command {
    use super::*;

    #[test]
    fn shows_information_message_when_no_flags_provided() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        let output = run_devhealth(&["scan", "--path", temp_dir.path().to_str().unwrap()]);

        assert!(output.status.success(), "Scan command should succeed");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Starting comprehensive scan"),
            "Should indicate scan is starting"
        );
        assert!(
            stdout.contains("No scan options specified"),
            "Should inform user about missing flags"
        );
        assert!(
            stdout.contains("--git, --deps, or --system"),
            "Should suggest available flags"
        );
    }

    #[test]
    fn runs_git_scan_when_git_flag_provided() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        create_test_git_repos(temp_dir.path());

        let output = run_devhealth(&["scan", "--git", "--path", temp_dir.path().to_str().unwrap()]);

        assert!(output.status.success(), "Scan with git flag should succeed");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Starting comprehensive scan"),
            "Should indicate scan is starting"
        );
        assert!(
            stdout.contains("Scanning Git repositories"),
            "Should indicate git scanning"
        );
        assert!(
            stdout.contains("Total repositories: 3"),
            "Should find all repositories"
        );
    }

    #[test]
    fn runs_dependency_scan_when_deps_flag_provided() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        // Create a test Cargo.toml file
        let cargo_toml_content = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
serde = "1.0"
clap = "4.0"
"#;
        fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml_content)
            .expect("Failed to create test Cargo.toml");

        let output = run_devhealth(&[
            "scan",
            "--deps",
            "--path",
            temp_dir.path().to_str().unwrap(),
        ]);

        assert!(
            output.status.success(),
            "Scan with deps flag should succeed"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Checking dependencies"),
            "Should indicate dependency checking"
        );
        assert!(
            stdout.contains("Dependency Summary"),
            "Should show dependency summary"
        );
        assert!(
            stdout.contains("Total dependencies:"),
            "Should show total dependency count"
        );
        assert!(stdout.contains("Rust:"), "Should detect Rust ecosystem");
    }

    #[test]
    fn detects_multiple_ecosystems_in_single_project() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        // Create both Cargo.toml and package.json
        let cargo_content = r#"
[package]
name = "multi-ecosystem"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#;
        let package_content = r#"
{
  "name": "multi-ecosystem",
  "dependencies": {
    "express": "^4.18.0"
  }
}
"#;

        fs::write(temp_dir.path().join("Cargo.toml"), cargo_content)
            .expect("Failed to create Cargo.toml");
        fs::write(temp_dir.path().join("package.json"), package_content)
            .expect("Failed to create package.json");

        let output = run_devhealth(&[
            "scan",
            "--deps",
            "--path",
            temp_dir.path().to_str().unwrap(),
        ]);

        assert!(
            output.status.success(),
            "Mixed ecosystem scan should succeed"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Rust:"), "Should detect Rust dependencies");
        assert!(
            stdout.contains("Node.js:"),
            "Should detect Node.js dependencies"
        );
    }

    #[test]
    fn runs_system_monitor_when_system_flag_provided() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        let output = run_devhealth(&[
            "scan",
            "--system",
            "--path",
            temp_dir.path().to_str().unwrap(),
        ]);

        assert!(
            output.status.success(),
            "Scan with system flag should succeed"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Monitoring system resources"),
            "Should indicate system monitoring"
        );
        assert!(
            stdout.contains("System monitoring not implemented yet"),
            "Should show placeholder message"
        );
    }

    #[test]
    fn runs_all_scans_when_all_flags_provided() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        create_test_git_repos(temp_dir.path());

        let output = run_devhealth(&[
            "scan",
            "--git",
            "--deps",
            "--system",
            "--path",
            temp_dir.path().to_str().unwrap(),
        ]);

        assert!(
            output.status.success(),
            "Scan with all flags should succeed"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Starting comprehensive scan"),
            "Should start comprehensive scan"
        );
        assert!(
            stdout.contains("Scanning Git repositories"),
            "Should run git scan"
        );
        assert!(
            stdout.contains("Checking dependencies"),
            "Should run dependency scan"
        );
        assert!(
            stdout.contains("Monitoring system resources"),
            "Should run system monitoring"
        );
        assert!(
            stdout.contains("Total repositories: 3"),
            "Should find git repositories"
        );
    }
}

mod error_handling {
    use super::*;

    #[test]
    fn handles_nonexistent_directory_gracefully() {
        let nonexistent_path = "/this/path/should/not/exist/anywhere";

        let output = run_devhealth(&["check", "--path", nonexistent_path]);

        // The behavior here depends on implementation - it might succeed with no repos
        // or fail gracefully. Both are acceptable as long as it doesn't panic.
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Either succeeds with appropriate message or fails with error
        let handles_gracefully = output.status.success()
            || stdout.contains("Error")
            || stderr.contains("Error")
            || stdout.contains("No git repositories found");

        assert!(
            handles_gracefully,
            "Should handle nonexistent directory gracefully"
        );
    }

    #[test]
    fn handles_permission_denied_gracefully() {
        // This test is platform-specific and might not work on all systems
        // so we make it conditional
        #[cfg(unix)]
        {
            // Try to scan a directory that typically has restricted permissions
            let restricted_path = "/root";

            let output = run_devhealth(&["check", "--path", restricted_path]);

            // Should either succeed with no repos or fail gracefully
            let _stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            // As long as it doesn't panic, we consider this a success
            assert!(
                output.status.success() || !stderr.is_empty(),
                "Should handle permission issues gracefully"
            );
        }
    }
}
