//! DevHealth - Development Environment Health Monitor
//!
//! Main entry point for the DevHealth CLI application.
//! This binary provides command-line interface for monitoring development
//! environment health including git repositories, dependencies, and system resources.

use clap::Parser;
use devhealth::cli::Cli;
use devhealth::scanner;
use std::process;

/// Application entry point
///
/// Parses command line arguments and executes the appropriate command.
/// Handles errors gracefully and exits with appropriate status codes.
fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Executes the main application logic based on parsed CLI arguments
///
/// Handles the routing of commands to their appropriate scanner modules
/// and manages error propagation from scanner operations.
///
/// # Arguments
///
/// * `cli` - Parsed CLI arguments containing the command and options
///
/// # Returns
///
/// A `Result` indicating success or failure of the operation.
///
/// # Errors
///
/// Returns an error if any scanner operation fails or if invalid
/// arguments are provided.
fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        devhealth::cli::Commands::Check { path } => {
            println!("🔍 Running health check on: {}", path.display());

            // Run git scanner
            let git_results = scanner::git::scan_directory(&path)?;
            scanner::git::display_results(&git_results);

            Ok(())
        }
        devhealth::cli::Commands::Scan {
            path,
            git,
            deps,
            system,
        } => {
            println!("🚀 Starting comprehensive scan on: {}", path.display());

            if git {
                println!("\n📁 Scanning Git repositories...");
                let git_results = scanner::git::scan_directory(&path)?;
                scanner::git::display_results(&git_results);
            }

            if deps {
                println!("\n📦 Checking dependencies...");
                scanner::deps::scan_dependencies();
            }

            if system {
                println!("\n💻 Monitoring system resources...");
                scanner::system::monitor_system();
            }

            if !git && !deps && !system {
                println!("ℹ️  No scan options specified. Use --git, --deps, or --system flags to enable specific scans.");
            }

            Ok(())
        }
    }
}
