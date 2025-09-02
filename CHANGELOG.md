# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Enhanced
- **Major Display Improvements** - Complete UI/UX overhaul
  - Added colorized terminal output with `colored` crate
  - Enhanced dependency scanner with structured display
    - Summary boxes with key metrics
    - Tree-structured project breakdown
    - Ecosystem breakdown with dependency counts
    - Color-coded dependency type badges (prod/dev/build/optional)
    - Ecosystem icons (🦀 Rust, 📦 Node.js, 🐍 Python, 🐹 Go)
    - Helpful tips section for dependency management
  - Enhanced git scanner with visual health indicators
    - Health percentage calculations and progress bars
    - Color-coded status indicators
    - Structured summary boxes
    - Professional tree-style repository details
  - Created comprehensive display utilities module (`src/utils/display.rs`)
    - Reusable formatting functions for headers, boxes, progress bars
    - Consistent styling across all scanners

### Fixed
- **Go Dependency Parsing** - Fixed critical parsing bug
  - Previously showed 0 dependencies for `go.mod` files
  - Now correctly parses both single-line and block-style `require` statements
  - Properly identifies direct vs indirect dependencies (prod vs dev)
- **File Path Display** - Improved readability
  - File paths now show complete filenames instead of truncated versions
  - Better path handling for long file paths

### Dependencies
- Added `colored` v2.0 for terminal output colorization

## [0.2.0] - 2025-08-31

### Added
- **Comprehensive dependency scanning** across multiple ecosystems
  - Rust (Cargo.toml) dependency parsing with support for dev/build dependencies  
  - Node.js (package.json) dependency parsing with dev and peer dependencies
  - Python (requirements.txt, pyproject.toml, Pipfile) dependency parsing
  - Go (go.mod) dependency parsing
  - Multi-ecosystem project detection and analysis
  - Dependency type classification (runtime, development, build, optional)
- **Advanced error handling** with structured error types using thiserror
- **Comprehensive test coverage** for dependency scanner (12+ new tests)
- **Integration tests** for mixed-ecosystem projects
- **Documentation** with examples and API reference

### Enhanced
- CLI now supports functional `--deps` flag for dependency analysis
- Updated all documentation to reflect implemented features
- Added serialization support for dependency data structures

### Dependencies
- Added `toml` v0.8 for TOML file parsing
- Added `serde_json` v1.0 for JSON parsing
- Added `reqwest` v0.11 for future online dependency checking
- Added `semver` v1.0 for semantic version handling
- Added `thiserror` v1.0 for structured error handling

## [0.1.0] - 2025-08-31

### Added
- Initial implementation of DevHealth CLI tool
- Git repository scanning and health analysis
- Support for checking repository status, branches, and changes
- Command-line interface with `check` and `scan` commands
- Modular architecture for future feature expansion
- Comprehensive test suite with 29+ unit and integration tests
- Extensive documentation using rustdoc
- Professional README with usage examples and architecture overview

### Features
- Recursive git repository discovery
- Repository status analysis (clean/dirty)
- Branch information display
- Uncommitted changes detection
- Unpushed commits tracking

### Fixed
- Type parameter issues in file system utilities
- Compilation errors related to Path handling
- Incomplete scan command implementation
- Removed unused imports and warnings

### Planned
- System resource monitoring  
- Project analytics and metrics
