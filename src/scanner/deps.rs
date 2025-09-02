//! Dependency health scanner
//!
//! This module provides functionality for analyzing and monitoring
//! project dependencies across various ecosystems including:
//!
//! - Rust (`Cargo.toml`)
//! - Node.js (`package.json`, `package-lock.json`)
//! - Python (`requirements.txt`, `Pipfile`, `pyproject.toml`)
//! - Go (`go.mod`)
//!
//! The scanner identifies dependency files, parses them, and provides
//! health information including outdated packages and potential security issues.

use crate::utils::display;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

/// Errors that can occur during dependency scanning
#[derive(Error, Debug)]
pub enum DependencyError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Invalid semver version: {0}")]
    SemverParse(#[from] semver::Error),
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
}

/// Represents a project dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Name of the dependency package
    pub name: String,
    /// Current version specified in the project
    pub version: String,
    /// Type of dependency (runtime, dev, build, etc.)
    pub dependency_type: DependencyType,
    /// Source ecosystem (Rust, Node.js, Python, etc.)
    pub ecosystem: Ecosystem,
    /// File where this dependency was found
    pub source_file: PathBuf,
}

/// Types of dependencies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DependencyType {
    /// Runtime dependency required for the application to work
    Runtime,
    /// Development dependency only needed during development
    Development,
    /// Build dependency required during compilation
    Build,
    /// Optional dependency that can be enabled with features
    Optional,
}

/// Supported dependency ecosystems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Ecosystem {
    /// Rust cargo ecosystem
    Rust,
    /// Node.js npm ecosystem
    NodeJs,
    /// Python pip ecosystem
    Python,
    /// Go modules ecosystem
    Go,
}

impl fmt::Display for Ecosystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ecosystem::Rust => write!(f, "Rust"),
            Ecosystem::NodeJs => write!(f, "Node.js"),
            Ecosystem::Python => write!(f, "Python"),
            Ecosystem::Go => write!(f, "Go"),
        }
    }
}

/// Result of dependency scanning for a project
#[derive(Debug, Clone)]
pub struct DependencyReport {
    /// Path to the project root
    pub project_path: PathBuf,
    /// All discovered dependencies
    pub dependencies: Vec<Dependency>,
    /// Detected ecosystems in this project
    pub ecosystems: Vec<Ecosystem>,
    /// Any errors encountered during scanning
    pub errors: Vec<String>,
}

/// Scans a directory for dependency files and analyzes them
///
/// Recursively searches through the given directory to find dependency
/// files for various ecosystems (Cargo.toml, package.json, requirements.txt, etc.)
/// and parses them to extract dependency information.
///
/// # Arguments
///
/// * `path` - The directory to scan for dependency files
///
/// # Returns
///
/// A `Result` containing a vector of `DependencyReport`s for each project
/// found in the directory tree.
///
/// # Examples
///
/// ```rust
/// use devhealth::scanner::deps;
/// use std::path::Path;
///
/// let reports = deps::scan_dependencies(Path::new(".")).unwrap();
/// deps::display_results(&reports);
/// ```
///
/// # Errors
///
/// Returns an error if the directory cannot be accessed or if there are
/// critical parsing errors in dependency files.
pub fn scan_dependencies(path: &Path) -> Result<Vec<DependencyReport>, DependencyError> {
    let mut reports = Vec::new();
    let mut visited_projects = std::collections::HashSet::new();

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path();

        if let Some(ecosystem) = detect_dependency_file(file_path) {
            // Get the project root (parent directory of the dependency file)
            if let Some(project_root) = file_path.parent() {
                let project_root = project_root.to_path_buf();

                // Avoid duplicate processing of the same project
                if visited_projects.contains(&project_root) {
                    continue;
                }
                visited_projects.insert(project_root.clone());

                match scan_project(&project_root, ecosystem.clone()) {
                    Ok(mut report) => {
                        // Check for additional ecosystems in the same project
                        for additional_ecosystem in detect_all_ecosystems(&project_root) {
                            if additional_ecosystem != ecosystem {
                                if let Ok(additional_deps) =
                                    parse_dependencies(&project_root, additional_ecosystem.clone())
                                {
                                    report.dependencies.extend(additional_deps);
                                    if !report.ecosystems.contains(&additional_ecosystem) {
                                        report.ecosystems.push(additional_ecosystem);
                                    }
                                }
                            }
                        }
                        reports.push(report);
                    }
                    Err(e) => {
                        reports.push(DependencyReport {
                            project_path: project_root,
                            dependencies: Vec::new(),
                            ecosystems: vec![ecosystem],
                            errors: vec![e.to_string()],
                        });
                    }
                }
            }
        }
    }

    Ok(reports)
}

/// Scans a single project directory for dependencies
fn scan_project(
    project_path: &Path,
    primary_ecosystem: Ecosystem,
) -> Result<DependencyReport, DependencyError> {
    let dependencies = parse_dependencies(project_path, primary_ecosystem)?;
    let ecosystems = detect_all_ecosystems(project_path);

    Ok(DependencyReport {
        project_path: project_path.to_path_buf(),
        dependencies,
        ecosystems,
        errors: Vec::new(),
    })
}

/// Detects if a file is a dependency file and returns the ecosystem
fn detect_dependency_file(path: &Path) -> Option<Ecosystem> {
    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
        match filename {
            "Cargo.toml" => Some(Ecosystem::Rust),
            "package.json" => Some(Ecosystem::NodeJs),
            "requirements.txt" | "Pipfile" | "pyproject.toml" => Some(Ecosystem::Python),
            "go.mod" => Some(Ecosystem::Go),
            _ => None,
        }
    } else {
        None
    }
}

/// Detects all ecosystems present in a project directory
fn detect_all_ecosystems(project_path: &Path) -> Vec<Ecosystem> {
    let mut ecosystems = Vec::new();

    let files_to_check = [
        ("Cargo.toml", Ecosystem::Rust),
        ("package.json", Ecosystem::NodeJs),
        ("requirements.txt", Ecosystem::Python),
        ("Pipfile", Ecosystem::Python),
        ("pyproject.toml", Ecosystem::Python),
        ("go.mod", Ecosystem::Go),
    ];

    for (filename, ecosystem) in &files_to_check {
        if project_path.join(filename).exists() && !ecosystems.contains(ecosystem) {
            ecosystems.push(ecosystem.clone());
        }
    }

    ecosystems
}

/// Parses dependencies from a project for a specific ecosystem
fn parse_dependencies(
    project_path: &Path,
    ecosystem: Ecosystem,
) -> Result<Vec<Dependency>, DependencyError> {
    match ecosystem {
        Ecosystem::Rust => parse_cargo_toml(project_path),
        Ecosystem::NodeJs => parse_package_json(project_path),
        Ecosystem::Python => parse_python_dependencies(project_path),
        Ecosystem::Go => parse_go_mod(project_path),
    }
}

/// Parses Rust dependencies from Cargo.toml
fn parse_cargo_toml(project_path: &Path) -> Result<Vec<Dependency>, DependencyError> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml_path)?;

    #[derive(Deserialize)]
    struct CargoToml {
        dependencies: Option<HashMap<String, toml::Value>>,
        #[serde(rename = "dev-dependencies")]
        dev_dependencies: Option<HashMap<String, toml::Value>>,
        #[serde(rename = "build-dependencies")]
        build_dependencies: Option<HashMap<String, toml::Value>>,
    }

    let cargo_toml: CargoToml = toml::from_str(&content)?;
    let mut dependencies = Vec::new();

    // Parse runtime dependencies
    if let Some(deps) = cargo_toml.dependencies {
        for (name, value) in deps {
            let dependency =
                parse_cargo_dependency(name, value, DependencyType::Runtime, &cargo_toml_path)?;
            dependencies.push(dependency);
        }
    }

    // Parse dev dependencies
    if let Some(deps) = cargo_toml.dev_dependencies {
        for (name, value) in deps {
            let dependency =
                parse_cargo_dependency(name, value, DependencyType::Development, &cargo_toml_path)?;
            dependencies.push(dependency);
        }
    }

    // Parse build dependencies
    if let Some(deps) = cargo_toml.build_dependencies {
        for (name, value) in deps {
            let dependency =
                parse_cargo_dependency(name, value, DependencyType::Build, &cargo_toml_path)?;
            dependencies.push(dependency);
        }
    }

    Ok(dependencies)
}

/// Parses a single Cargo dependency entry
fn parse_cargo_dependency(
    name: String,
    value: toml::Value,
    dep_type: DependencyType,
    source_file: &Path,
) -> Result<Dependency, DependencyError> {
    let version = match value {
        toml::Value::String(v) => v,
        toml::Value::Table(table) => table
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("*")
            .to_string(),
        _ => "*".to_string(),
    };

    Ok(Dependency {
        name,
        version,
        dependency_type: dep_type,
        ecosystem: Ecosystem::Rust,
        source_file: source_file.to_path_buf(),
    })
}

/// Parses Node.js dependencies from package.json
fn parse_package_json(project_path: &Path) -> Result<Vec<Dependency>, DependencyError> {
    let package_json_path = project_path.join("package.json");
    let content = fs::read_to_string(&package_json_path)?;

    #[derive(Deserialize)]
    struct PackageJson {
        dependencies: Option<HashMap<String, String>>,
        #[serde(rename = "devDependencies")]
        dev_dependencies: Option<HashMap<String, String>>,
        #[serde(rename = "peerDependencies")]
        peer_dependencies: Option<HashMap<String, String>>,
    }

    let package_json: PackageJson = serde_json::from_str(&content)?;
    let mut dependencies = Vec::new();

    // Parse runtime dependencies
    if let Some(deps) = package_json.dependencies {
        for (name, version) in deps {
            dependencies.push(Dependency {
                name,
                version,
                dependency_type: DependencyType::Runtime,
                ecosystem: Ecosystem::NodeJs,
                source_file: package_json_path.clone(),
            });
        }
    }

    // Parse dev dependencies
    if let Some(deps) = package_json.dev_dependencies {
        for (name, version) in deps {
            dependencies.push(Dependency {
                name,
                version,
                dependency_type: DependencyType::Development,
                ecosystem: Ecosystem::NodeJs,
                source_file: package_json_path.clone(),
            });
        }
    }

    // Parse peer dependencies
    if let Some(deps) = package_json.peer_dependencies {
        for (name, version) in deps {
            dependencies.push(Dependency {
                name,
                version,
                dependency_type: DependencyType::Optional,
                ecosystem: Ecosystem::NodeJs,
                source_file: package_json_path.clone(),
            });
        }
    }

    Ok(dependencies)
}

/// Parses Python dependencies from various files
fn parse_python_dependencies(project_path: &Path) -> Result<Vec<Dependency>, DependencyError> {
    let mut dependencies = Vec::new();

    // Try requirements.txt first
    let requirements_path = project_path.join("requirements.txt");
    if requirements_path.exists() {
        dependencies.extend(parse_requirements_txt(&requirements_path)?);
    }

    // Try pyproject.toml
    let pyproject_path = project_path.join("pyproject.toml");
    if pyproject_path.exists() {
        dependencies.extend(parse_pyproject_toml(&pyproject_path)?);
    }

    // Try Pipfile
    let pipfile_path = project_path.join("Pipfile");
    if pipfile_path.exists() {
        dependencies.extend(parse_pipfile(&pipfile_path)?);
    }

    Ok(dependencies)
}

/// Parses requirements.txt file
fn parse_requirements_txt(file_path: &Path) -> Result<Vec<Dependency>, DependencyError> {
    let content = fs::read_to_string(file_path)?;
    let mut dependencies = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse "package==version" or "package>=version" format
        let parts: Vec<&str> = line.split(&['=', '>', '<', '!', '~'][..]).collect();
        if let Some(name) = parts.first() {
            let version = if parts.len() > 1 {
                parts[1..].join("")
            } else {
                "*".to_string()
            };

            dependencies.push(Dependency {
                name: name.trim().to_string(),
                version,
                dependency_type: DependencyType::Runtime,
                ecosystem: Ecosystem::Python,
                source_file: file_path.to_path_buf(),
            });
        }
    }

    Ok(dependencies)
}

/// Parses pyproject.toml file
fn parse_pyproject_toml(file_path: &Path) -> Result<Vec<Dependency>, DependencyError> {
    let content = fs::read_to_string(file_path)?;

    #[derive(Deserialize)]
    struct PyProjectToml {
        project: Option<ProjectSection>,
    }

    #[derive(Deserialize)]
    struct ProjectSection {
        dependencies: Option<Vec<String>>,
        #[serde(rename = "optional-dependencies")]
        optional_dependencies: Option<HashMap<String, Vec<String>>>,
    }

    let pyproject: PyProjectToml = toml::from_str(&content)?;
    let mut dependencies = Vec::new();

    if let Some(project) = pyproject.project {
        // Parse main dependencies
        if let Some(deps) = project.dependencies {
            for dep_str in deps {
                if let Some(dependency) =
                    parse_python_dependency_string(&dep_str, DependencyType::Runtime, file_path)
                {
                    dependencies.push(dependency);
                }
            }
        }

        // Parse optional dependencies
        if let Some(optional_deps) = project.optional_dependencies {
            for (_group, deps) in optional_deps {
                for dep_str in deps {
                    if let Some(dependency) = parse_python_dependency_string(
                        &dep_str,
                        DependencyType::Optional,
                        file_path,
                    ) {
                        dependencies.push(dependency);
                    }
                }
            }
        }
    }

    Ok(dependencies)
}

/// Parses Pipfile
fn parse_pipfile(file_path: &Path) -> Result<Vec<Dependency>, DependencyError> {
    let content = fs::read_to_string(file_path)?;

    #[derive(Deserialize)]
    struct Pipfile {
        packages: Option<HashMap<String, toml::Value>>,
        #[serde(rename = "dev-packages")]
        dev_packages: Option<HashMap<String, toml::Value>>,
    }

    let pipfile: Pipfile = toml::from_str(&content)?;
    let mut dependencies = Vec::new();

    // Parse runtime dependencies
    if let Some(packages) = pipfile.packages {
        for (name, value) in packages {
            let version = extract_version_from_toml_value(value);
            dependencies.push(Dependency {
                name,
                version,
                dependency_type: DependencyType::Runtime,
                ecosystem: Ecosystem::Python,
                source_file: file_path.to_path_buf(),
            });
        }
    }

    // Parse dev dependencies
    if let Some(dev_packages) = pipfile.dev_packages {
        for (name, value) in dev_packages {
            let version = extract_version_from_toml_value(value);
            dependencies.push(Dependency {
                name,
                version,
                dependency_type: DependencyType::Development,
                ecosystem: Ecosystem::Python,
                source_file: file_path.to_path_buf(),
            });
        }
    }

    Ok(dependencies)
}

/// Parses Go dependencies from go.mod
fn parse_go_mod(project_path: &Path) -> Result<Vec<Dependency>, DependencyError> {
    let go_mod_path = project_path.join("go.mod");
    let content = fs::read_to_string(&go_mod_path)?;
    let mut dependencies = Vec::new();
    let mut in_require_block = false;

    for line in content.lines() {
        let line = line.trim();
        
        // Check if we're entering a require block
        if line.starts_with("require (") {
            in_require_block = true;
            continue;
        }
        
        // Check if we're exiting a require block
        if in_require_block && line == ")" {
            in_require_block = false;
            continue;
        }
        
        // Parse single-line require statements
        if line.starts_with("require ") && !line.ends_with("(") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[1].to_string();
                let version = parts[2].to_string();
                let dep_type = DependencyType::Runtime;

                dependencies.push(Dependency {
                    name,
                    version,
                    dependency_type: dep_type,
                    ecosystem: Ecosystem::Go,
                    source_file: go_mod_path.clone(),
                });
            }
        }
        
        // Parse dependencies inside require blocks
        if in_require_block && !line.is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let version = parts[1].to_string();
                
                // Determine dependency type based on comments
                let dep_type = if line.contains("// indirect") {
                    DependencyType::Development
                } else {
                    DependencyType::Runtime
                };

                dependencies.push(Dependency {
                    name,
                    version,
                    dependency_type: dep_type,
                    ecosystem: Ecosystem::Go,
                    source_file: go_mod_path.clone(),
                });
            }
        }
    }

    Ok(dependencies)
}

/// Helper function to parse Python dependency strings
fn parse_python_dependency_string(
    dep_str: &str,
    dep_type: DependencyType,
    source_file: &Path,
) -> Option<Dependency> {
    // Parse formats like "requests>=2.25.0" or "django==3.2"
    let parts: Vec<&str> = dep_str.split(&['=', '>', '<', '!', '~'][..]).collect();
    if let Some(name) = parts.first() {
        let version = if parts.len() > 1 {
            parts[1..].join("")
        } else {
            "*".to_string()
        };

        Some(Dependency {
            name: name.trim().to_string(),
            version,
            dependency_type: dep_type,
            ecosystem: Ecosystem::Python,
            source_file: source_file.to_path_buf(),
        })
    } else {
        None
    }
}

/// Helper function to extract version from TOML value
fn extract_version_from_toml_value(value: toml::Value) -> String {
    match value {
        toml::Value::String(v) => v,
        toml::Value::Table(table) => table
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("*")
            .to_string(),
        _ => "*".to_string(),
    }
}

/// Displays dependency scan results in a formatted output
///
/// Prints a comprehensive summary of all discovered dependencies organized
/// by project and ecosystem, with statistics and detailed listings.
///
/// # Arguments
///
/// * `reports` - Slice of `DependencyReport`s to display
///
/// # Examples
///
/// ```rust
/// use devhealth::scanner::deps;
/// use std::path::Path;
///
/// let reports = deps::scan_dependencies(Path::new(".")).unwrap();
/// deps::display_results(&reports);
/// ```
pub fn display_results(reports: &[DependencyReport]) {
    if reports.is_empty() {
        println!("{}", display::header("No dependency files found", "📦", colored::Color::Yellow));
        return;
    }

    let total_dependencies: usize = reports.iter().map(|r| r.dependencies.len()).sum();
    let total_projects = reports.len();
    let ecosystems: std::collections::HashSet<_> =
        reports.iter().flat_map(|r| &r.ecosystems).collect();

    // Calculate dependency health metrics
    let total_errors: usize = reports.iter().map(|r| r.errors.len()).sum();
    
    // Display main header
    println!("{}", display::header(
        &format!("Dependency Analysis ({} ecosystems)", ecosystems.len()), 
        "📦", 
        colored::Color::BrightMagenta
    ));

    // Display summary box
    let summary_items = vec![
        ("Total Projects", total_projects.to_string()),
        ("Total Dependencies", total_dependencies.to_string()),
        ("Ecosystems", ecosystems.len().to_string()),
        ("Errors", if total_errors > 0 { 
            format!("{} ❌", total_errors) 
        } else { 
            "0".to_string() 
        }),
    ];
    
    print!("{}", display::summary_box(&summary_items));

    // Display ecosystem breakdown
    if !ecosystems.is_empty() {
        println!("{}", display::section_divider("Ecosystem Breakdown"));
        
        for ecosystem in &ecosystems {
            let count: usize = reports
                .iter()
                .flat_map(|r| &r.dependencies)
                .filter(|d| d.ecosystem == **ecosystem)
                .count();
            
            let ecosystem_display = format!("{} {} {} dependencies", 
                display::ecosystem_icon(&ecosystem.to_string()),
                ecosystem.to_string().bright_cyan().bold(),
                count.to_string().bright_white().bold()
            );
            
            println!("  {}", ecosystem_display);
        }
    }

    // Display detailed project breakdown
    println!("{}", display::section_divider("Project Details"));
    
    for (project_index, report) in reports.iter().enumerate() {
        let is_last_project = project_index == reports.len() - 1;
        let project_name = report
            .project_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // Project header with dependency count
        let project_header = format!("{} {} {} dependencies", 
            "📂".to_string(),
            project_name.bright_white().bold(),
            format!("({} deps)", report.dependencies.len()).bright_black()
        );
        
        println!("{}", display::tree_item(&project_header, is_last_project, 0));

        // Group by ecosystem for cleaner display
        let mut ecosystem_deps: HashMap<Ecosystem, Vec<&Dependency>> = HashMap::new();
        for dep in &report.dependencies {
            ecosystem_deps
                .entry(dep.ecosystem.clone())
                .or_default()
                .push(dep);
        }

        // Display dependencies by ecosystem
        for (ecosystem_index, (ecosystem, deps)) in ecosystem_deps.iter().enumerate() {
            let is_last_ecosystem = ecosystem_index == ecosystem_deps.len() - 1 && report.errors.is_empty();
            
            let ecosystem_header = format!("{} {} {}", 
                display::ecosystem_icon(&ecosystem.to_string()),
                ecosystem.to_string().bright_cyan(),
                format!("({} deps)", deps.len()).bright_black()
            );
            
            println!("{}", display::tree_item(&ecosystem_header, is_last_ecosystem, 1));

            // Show top dependencies (with limit for readability)
            let deps_to_show = deps.iter().take(8);
            let remaining = if deps.len() > 8 { deps.len() - 8 } else { 0 };
            
            for (dep_index, dep) in deps_to_show.enumerate() {
                let is_last_dep = dep_index == 7.min(deps.len() - 1) && remaining == 0;
                
                // Create dependency badge
                let type_badge = match dep.dependency_type {
                    DependencyType::Runtime => display::badge("prod", display::BadgeType::Runtime),
                    DependencyType::Development => display::badge("dev", display::BadgeType::Dev),
                    DependencyType::Build => display::badge("build", display::BadgeType::Build),
                    DependencyType::Optional => display::badge("opt", display::BadgeType::Optional),
                };

                let dep_display = format!("{} {} {}", 
                    display::version_display(&dep.name, &dep.version, None),
                    type_badge,
                    {
                        let path = dep.source_file.to_string_lossy();
                        let path_str = if path.len() > 35 {
                            format!("...{}", &path[path.len()-32..])
                        } else {
                            path.to_string()
                        };
                        display::file_path(&path_str)
                    }
                );
                
                println!("{}", display::tree_item(&dep_display, is_last_dep, 2));
            }
            
            // Show "... and X more" if there are remaining dependencies
            if remaining > 0 {
                let more_display = format!("{} {} more dependencies", 
                    "...".bright_black(),
                    remaining.to_string().bright_black()
                );
                println!("{}", display::tree_item(&more_display, is_last_ecosystem, 2));
            }
        }

        // Display any errors
        if !report.errors.is_empty() {
            let error_header = format!("{} {} Errors", "⚠️".bright_red(), report.errors.len());
            println!("{}", display::tree_item(&error_header, true, 1));
            
            for (error_index, error) in report.errors.iter().enumerate() {
                let is_last_error = error_index == report.errors.len() - 1;
                let error_display = format!("{}", error.bright_red());
                println!("{}", display::tree_item(&error_display, is_last_error, 2));
            }
        }
        
        // Add spacing between projects
        if !is_last_project {
            println!();
        }
    }

    // Display helpful tips
    if total_dependencies > 0 {
        println!("\n{}", "💡 Tips:".bright_blue().bold());
        
        let tips = vec![
            ("Check for updates", "Run package manager update commands"),
            ("Security scan", "Use tools like cargo audit, npm audit, or safety"),
            ("Clean unused deps", "Remove dependencies you're not using"),
        ];
        
        for tip in tips {
            println!("  {} {}: {}", 
                "•".bright_black(),
                tip.0.bright_cyan(),
                tip.1.bright_white()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_cargo_toml(dir: &Path) -> PathBuf {
        let cargo_toml_path = dir.join("Cargo.toml");
        let content = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
serde = "1.0"
clap = { version = "4.0", features = ["derive"] }

[dev-dependencies]
tempfile = "3.0"

[build-dependencies]
cc = "1.0"
"#;
        fs::write(&cargo_toml_path, content).unwrap();
        cargo_toml_path
    }

    fn create_test_package_json(dir: &Path) -> PathBuf {
        let package_json_path = dir.join("package.json");
        let content = r#"
{
  "name": "test-project",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0",
    "lodash": "4.17.21"
  },
  "devDependencies": {
    "jest": "^29.0.0",
    "typescript": "~4.9.0"
  }
}
"#;
        fs::write(&package_json_path, content).unwrap();
        package_json_path
    }

    fn create_test_requirements_txt(dir: &Path) -> PathBuf {
        let requirements_path = dir.join("requirements.txt");
        let content = r#"
# Production dependencies
requests>=2.28.0
django==4.1.0
numpy~=1.24.0

# Comments should be ignored
flask>=2.0.0
"#;
        fs::write(&requirements_path, content).unwrap();
        requirements_path
    }

    mod ecosystem_detection {
        use super::*;

        #[test]
        fn detects_rust_ecosystem() {
            let temp_dir = TempDir::new().unwrap();
            create_test_cargo_toml(temp_dir.path());

            let ecosystems = detect_all_ecosystems(temp_dir.path());
            assert!(ecosystems.contains(&Ecosystem::Rust));
        }

        #[test]
        fn detects_nodejs_ecosystem() {
            let temp_dir = TempDir::new().unwrap();
            create_test_package_json(temp_dir.path());

            let ecosystems = detect_all_ecosystems(temp_dir.path());
            assert!(ecosystems.contains(&Ecosystem::NodeJs));
        }

        #[test]
        fn detects_python_ecosystem() {
            let temp_dir = TempDir::new().unwrap();
            create_test_requirements_txt(temp_dir.path());

            let ecosystems = detect_all_ecosystems(temp_dir.path());
            assert!(ecosystems.contains(&Ecosystem::Python));
        }

        #[test]
        fn detects_multiple_ecosystems() {
            let temp_dir = TempDir::new().unwrap();
            create_test_cargo_toml(temp_dir.path());
            create_test_package_json(temp_dir.path());

            let ecosystems = detect_all_ecosystems(temp_dir.path());
            assert!(ecosystems.contains(&Ecosystem::Rust));
            assert!(ecosystems.contains(&Ecosystem::NodeJs));
            assert_eq!(ecosystems.len(), 2);
        }
    }

    mod cargo_parsing {
        use super::*;

        #[test]
        fn parses_cargo_toml_dependencies() {
            let temp_dir = TempDir::new().unwrap();
            create_test_cargo_toml(temp_dir.path());

            let dependencies = parse_cargo_toml(temp_dir.path()).unwrap();

            assert_eq!(dependencies.len(), 4); // 2 deps + 1 dev + 1 build

            // Check runtime dependencies
            let serde_dep = dependencies.iter().find(|d| d.name == "serde").unwrap();
            assert_eq!(serde_dep.version, "1.0");
            assert_eq!(serde_dep.dependency_type, DependencyType::Runtime);
            assert_eq!(serde_dep.ecosystem, Ecosystem::Rust);

            // Check complex dependency with features
            let clap_dep = dependencies.iter().find(|d| d.name == "clap").unwrap();
            assert_eq!(clap_dep.version, "4.0");
            assert_eq!(clap_dep.dependency_type, DependencyType::Runtime);

            // Check dev dependency
            let tempfile_dep = dependencies.iter().find(|d| d.name == "tempfile").unwrap();
            assert_eq!(tempfile_dep.dependency_type, DependencyType::Development);

            // Check build dependency
            let cc_dep = dependencies.iter().find(|d| d.name == "cc").unwrap();
            assert_eq!(cc_dep.dependency_type, DependencyType::Build);
        }
    }

    mod package_json_parsing {
        use super::*;

        #[test]
        fn parses_package_json_dependencies() {
            let temp_dir = TempDir::new().unwrap();
            create_test_package_json(temp_dir.path());

            let dependencies = parse_package_json(temp_dir.path()).unwrap();

            assert_eq!(dependencies.len(), 4); // 2 deps + 2 devDeps

            // Check runtime dependency
            let express_dep = dependencies.iter().find(|d| d.name == "express").unwrap();
            assert_eq!(express_dep.version, "^4.18.0");
            assert_eq!(express_dep.dependency_type, DependencyType::Runtime);
            assert_eq!(express_dep.ecosystem, Ecosystem::NodeJs);

            // Check dev dependency
            let jest_dep = dependencies.iter().find(|d| d.name == "jest").unwrap();
            assert_eq!(jest_dep.dependency_type, DependencyType::Development);
        }
    }

    mod requirements_parsing {
        use super::*;

        #[test]
        fn parses_requirements_txt() {
            let temp_dir = TempDir::new().unwrap();
            create_test_requirements_txt(temp_dir.path());

            let requirements_path = temp_dir.path().join("requirements.txt");
            let dependencies = parse_requirements_txt(&requirements_path).unwrap();

            assert_eq!(dependencies.len(), 4); // requests, django, numpy, flask

            let requests_dep = dependencies.iter().find(|d| d.name == "requests").unwrap();
            assert_eq!(requests_dep.version, "2.28.0");
            assert_eq!(requests_dep.ecosystem, Ecosystem::Python);

            let django_dep = dependencies.iter().find(|d| d.name == "django").unwrap();
            assert_eq!(django_dep.version, "4.1.0");
        }

        #[test]
        fn ignores_comments_and_empty_lines() {
            let temp_dir = TempDir::new().unwrap();
            let requirements_path = temp_dir.path().join("requirements.txt");
            let content = r#"
# This is a comment
   
requests>=2.28.0
# Another comment

flask>=2.0.0
"#;
            fs::write(&requirements_path, content).unwrap();

            let dependencies = parse_requirements_txt(&requirements_path).unwrap();
            assert_eq!(dependencies.len(), 2); // Only requests and flask
        }
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn scans_directory_with_multiple_projects() {
            let temp_dir = TempDir::new().unwrap();

            // Create multiple projects with different ecosystems
            let rust_project = temp_dir.path().join("rust-project");
            fs::create_dir_all(&rust_project).unwrap();
            create_test_cargo_toml(&rust_project);

            let node_project = temp_dir.path().join("node-project");
            fs::create_dir_all(&node_project).unwrap();
            create_test_package_json(&node_project);

            let reports = scan_dependencies(temp_dir.path()).unwrap();

            assert_eq!(reports.len(), 2);

            // Verify we found both projects
            let ecosystems: Vec<_> = reports.iter().flat_map(|r| &r.ecosystems).collect();
            assert!(ecosystems.contains(&&Ecosystem::Rust));
            assert!(ecosystems.contains(&&Ecosystem::NodeJs));
        }

        #[test]
        fn handles_empty_directory() {
            let temp_dir = TempDir::new().unwrap();
            let reports = scan_dependencies(temp_dir.path()).unwrap();
            assert!(reports.is_empty());
        }

        #[test]
        fn handles_mixed_ecosystem_project() {
            let temp_dir = TempDir::new().unwrap();

            // Create a project with both Rust and Node.js dependencies
            create_test_cargo_toml(temp_dir.path());
            create_test_package_json(temp_dir.path());

            let reports = scan_dependencies(temp_dir.path()).unwrap();

            assert_eq!(reports.len(), 1); // One project with multiple ecosystems
            let report = &reports[0];
            assert_eq!(report.ecosystems.len(), 2);
            assert!(report.ecosystems.contains(&Ecosystem::Rust));
            assert!(report.ecosystems.contains(&Ecosystem::NodeJs));

            // Should have dependencies from both ecosystems
            let rust_deps = report
                .dependencies
                .iter()
                .filter(|d| d.ecosystem == Ecosystem::Rust)
                .count();
            let node_deps = report
                .dependencies
                .iter()
                .filter(|d| d.ecosystem == Ecosystem::NodeJs)
                .count();
            assert!(rust_deps > 0);
            assert!(node_deps > 0);
        }
    }

    mod display_tests {
        use super::*;

        #[test]
        fn displays_empty_results() {
            let reports = vec![];
            // Should not panic
            display_results(&reports);
        }

        #[test]
        fn displays_single_project_results() {
            let temp_dir = TempDir::new().unwrap();
            let dependencies = vec![Dependency {
                name: "serde".to_string(),
                version: "1.0".to_string(),
                dependency_type: DependencyType::Runtime,
                ecosystem: Ecosystem::Rust,
                source_file: temp_dir.path().join("Cargo.toml"),
            }];

            let report = DependencyReport {
                project_path: temp_dir.path().to_path_buf(),
                dependencies,
                ecosystems: vec![Ecosystem::Rust],
                errors: Vec::new(),
            };

            // Should not panic
            display_results(&[report]);
        }
    }
}
