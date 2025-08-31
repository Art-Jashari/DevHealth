# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive test suite with 29+ unit and integration tests
- Extensive documentation using rustdoc
- Professional README with usage examples and architecture overview

### Fixed
- Type parameter issues in file system utilities
- Compilation errors related to Path handling
- Incomplete scan command implementation
- Removed unused imports and warnings

## [0.1.0] - 2025-08-31

### Added
- Initial implementation of DevHealth CLI tool
- Git repository scanning and health analysis
- Support for checking repository status, branches, and changes
- Command-line interface with `check` and `scan` commands
- Modular architecture for future feature expansion

### Features
- Recursive git repository discovery
- Repository status analysis (clean/dirty)
- Branch information display
- Uncommitted changes detection
- Unpushed commits tracking

### Planned
- Dependency health scanning
- System resource monitoring  
- Project analytics and metrics
