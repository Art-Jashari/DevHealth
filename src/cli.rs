//! Command Line Interface module for DevHealth
//!
//! This module defines the CLI structure and commands using the `clap` crate.
//! It provides two main commands: `check` for quick health checks and `scan`
//! for comprehensive analysis with configurable options.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// DevHealth CLI application
///
/// A command-line tool for monitoring development environment health.
/// Supports quick health checks and comprehensive scans of development projects.
#[derive(Parser)]
#[command(name = "devhealth")]
#[command(about = "A CLI tool for monitoring your development environment health")]
#[command(version = "0.2.0")]
pub struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
///
/// DevHealth supports two main operation modes:
/// - `Check`: Quick health assessment of a directory
/// - `Scan`: Comprehensive analysis with configurable scanning options
#[derive(Subcommand)]
pub enum Commands {
    /// Quick health check of a directory
    ///
    /// Performs a fast assessment of the specified directory, checking for
    /// git repositories and their basic health status.
    Check {
        /// Path to scan (defaults to current directory)
        ///
        /// The directory path to analyze. If not specified, uses the current
        /// working directory.
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Comprehensive scan with specific options
    ///
    /// Performs detailed analysis of the development environment with
    /// configurable scanning modules. Use flags to enable specific scanners.
    Scan {
        /// Path to scan (defaults to current directory)
        ///
        /// The directory path to analyze. If not specified, uses the current
        /// working directory.
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Scan git repositories
        ///
        /// Enables git repository analysis including status checks,
        /// branch information, and commit tracking.
        #[arg(long)]
        git: bool,

        /// Check dependencies
        ///
        /// Enables dependency health analysis for various project types
        /// (Cargo.toml, package.json, requirements.txt, etc.).
        /// Analyzes dependency versions, types, and ecosystem health.
        #[arg(long)]
        deps: bool,

        /// Monitor system resources
        ///
        /// Enables system resource monitoring including CPU usage,
        /// memory consumption, and disk space analysis.
        /// Note: This feature is currently under development.
        #[arg(long)]
        system: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    mod check_command {
        use super::*;

        #[test]
        fn parses_with_default_path() {
            let cli = Cli::parse_from(["devhealth", "check"]);

            match cli.command {
                Commands::Check { path } => {
                    assert_eq!(
                        path.to_str().unwrap(),
                        ".",
                        "Default path should be current directory"
                    );
                }
                _ => panic!("Expected Check command"),
            }
        }

        #[test]
        fn parses_with_custom_path() {
            let test_path = "/custom/test/path";
            let cli = Cli::parse_from(["devhealth", "check", "--path", test_path]);

            match cli.command {
                Commands::Check { path } => {
                    assert_eq!(
                        path.to_str().unwrap(),
                        test_path,
                        "Should use provided path"
                    );
                }
                _ => panic!("Expected Check command"),
            }
        }

        #[test]
        fn supports_short_path_flag() {
            let test_path = "/short/flag/path";
            let cli = Cli::parse_from(["devhealth", "check", "-p", test_path]);

            match cli.command {
                Commands::Check { path } => {
                    assert_eq!(path.to_str().unwrap(), test_path, "Short flag should work");
                }
                _ => panic!("Expected Check command"),
            }
        }
    }

    mod scan_command {
        use super::*;

        #[test]
        fn parses_with_default_values() {
            let cli = Cli::parse_from(["devhealth", "scan"]);

            match cli.command {
                Commands::Scan {
                    path,
                    git,
                    deps,
                    system,
                } => {
                    assert_eq!(
                        path.to_str().unwrap(),
                        ".",
                        "Default path should be current directory"
                    );
                    assert!(!git, "Git flag should default to false");
                    assert!(!deps, "Deps flag should default to false");
                    assert!(!system, "System flag should default to false");
                }
                _ => panic!("Expected Scan command"),
            }
        }

        #[test]
        fn parses_all_flags_correctly() {
            let test_path = "/test/scan/path";
            let cli = Cli::parse_from([
                "devhealth",
                "scan",
                "--git",
                "--deps",
                "--system",
                "--path",
                test_path,
            ]);

            match cli.command {
                Commands::Scan {
                    path,
                    git,
                    deps,
                    system,
                } => {
                    assert_eq!(
                        path.to_str().unwrap(),
                        test_path,
                        "Should use provided path"
                    );
                    assert!(git, "Git flag should be true");
                    assert!(deps, "Deps flag should be true");
                    assert!(system, "System flag should be true");
                }
                _ => panic!("Expected Scan command"),
            }
        }

        #[test]
        fn parses_individual_flags() {
            // Test each flag individually
            let test_cases = [
                (vec!["devhealth", "scan", "--git"], (true, false, false)),
                (vec!["devhealth", "scan", "--deps"], (false, true, false)),
                (vec!["devhealth", "scan", "--system"], (false, false, true)),
            ];

            for (args, (expected_git, expected_deps, expected_system)) in test_cases {
                let args_clone = args.clone(); // Clone for error messages
                let cli = Cli::parse_from(args);

                match cli.command {
                    Commands::Scan {
                        git, deps, system, ..
                    } => {
                        assert_eq!(git, expected_git, "Git flag mismatch for {:?}", args_clone);
                        assert_eq!(
                            deps, expected_deps,
                            "Deps flag mismatch for {:?}",
                            args_clone
                        );
                        assert_eq!(
                            system, expected_system,
                            "System flag mismatch for {:?}",
                            args_clone
                        );
                    }
                    _ => panic!("Expected Scan command for args {:?}", args_clone),
                }
            }
        }
    }

    #[test]
    fn cli_has_correct_metadata() {
        // Test that the CLI struct has the expected metadata
        let cli = Cli::parse_from(["devhealth", "check"]);

        // These are compile-time checks that the attributes are correctly set
        // The actual metadata is tested through integration tests
        assert!(matches!(cli.command, Commands::Check { .. }));
    }
}
