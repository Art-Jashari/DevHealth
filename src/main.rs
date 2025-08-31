use clap::Parser;
use devhealth::cli::Cli;
use devhealth::scanner;
use std::process;

fn main() 
    let cli = Cli::parse();
    
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        devhealth::cli::Commands::Check { path } => {
            println!("🔍 Running health check on: {}", path.display());
            
            // Run git scanner
            let git_results = scanner::git::scan_directory(&path)?;
            scanner::git::display_results(&git_results);
            
            Ok(())
        }
        devhealth::cli::Commands::Scan { path, git, deps, system } => {
            println!("🚀 Starting comprehensive scan on: {}", path.display());
            
            if git {
                println!("\n📁 Scanning Git repositories...");
                let git_results = scanner::git::scan_directory(&path)?;
                scanner::git::display_results(&git_results);
            }
            
            if deps {
                println!("\n📦 Checking dependencies...");
                // TODO: Implement dependency scanning
                println!("  Dependency scanning coming soon!");
            }
            
            if system {
                println!("\n💻 Monitoring system resources...");
                // TODO: Implement system monitoring
                println!("  System monitoring coming soon!");
            }
            
            Ok(())
        }
    }

