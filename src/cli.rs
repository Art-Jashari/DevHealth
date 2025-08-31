use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "devhealth")]
#[command(about = "A CLI tool for monitoring your development environment health")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Quick health check of a directory
    Check {
        /// Path to scan (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Comprehensive scan with specific options
    Scan {
        /// Path to scan (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        
        /// Scan git repositories
        #[arg(long)]
        git: bool,
        
        /// Check dependencies
        #[arg(long)]
        deps: bool,
        
        /// Monitor system resources
        #[arg(long)]
        system: bool,
    },
}