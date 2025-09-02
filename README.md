# DevHealth

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive CLI tool for monitoring and maintaining the health of software development environments.

## Features

### Currently Implemented
- **Git Repository Health**: Scan directories for git repositories and analyze their status
  - Detect uncommitted changes
  - Track unpushed commits
  - Monitor branch information
  - Recursive directory scanning
  - **Enhanced colorized display** with health percentages and progress bars
- **Dependency Analysis**: Monitor project dependencies across multiple ecosystems
  - Rust (Cargo.toml) dependency parsing with proper direct/indirect detection
  - Node.js (package.json) dependency parsing with dev/peer dependencies
  - Python (requirements.txt, pyproject.toml, Pipfile) dependency parsing
  - Go (go.mod) dependency parsing with require block support
  - Multi-ecosystem project support
  - **Professional tree-structured output** with ecosystem breakdown
  - **Color-coded dependency badges** and type indicators

### Planned Features
- **System Monitoring**: Track system resource usage and performance
- **Project Analytics**: Analyze code quality metrics and technical debt
- **Security Scanning**: Vulnerability detection and license compliance
- **Watch Mode**: Continuous monitoring of development environment

## Installation

### From Source
```bash
git clone https://github.com/Art-Jashari/DevHealth.git
cd DevHealth
cargo build --release
```

The binary will be available at `target/release/devhealth`.

### Using Cargo
```bash
cargo install --path .
```

## Usage

### Quick Health Check
Perform a fast assessment of the current directory:

```bash
devhealth check
```

Check a specific directory:
```bash
devhealth check --path /path/to/project
```

### Comprehensive Scan
Perform detailed analysis with specific scanners:

```bash
# Scan git repositories only
devhealth scan --git

# Scan dependencies only
devhealth scan --deps

# Monitor system resources (planned)
devhealth scan --system

# Run all scanners
devhealth scan --git --deps --system

# Scan specific directory
devhealth scan --git --path /path/to/projects
```

## Example Output

DevHealth now features **colorized, structured output** for enhanced readability:

### Git Repository Health
```
🚀 Starting comprehensive scan on: .

� Scanning Git repositories...
🔴 Git Repository Health (33%)
┌─ Summary ─────────────────────────────────────────┐
│ Total Repositories   │ 3                          │
│ Clean                │ 1 [███░░░░░░░] 1/3         │
│ Dirty                │ 2 ⚠️                       │
│ Errors               │ 0                          │
└───────────────────────────────────────────────────┘

──────────────────────────────────────────────────
▶ Repository Details
──────────────────────────────────────────────────
├─ ✅ Clean my-project on main  ./my-project
├─ ⚠ Dirty web-app on develop  ./web-app
└─ ⚠ Dirty utils-lib on main  ./utils-lib

💡 Tip:
  • Use git add . && git commit or git stash to clean dirty repositories
```

### Dependency Analysis
```
📦 Dependency Analysis (3 ecosystems)
┌─ Summary ─────────────────────────────────────────┐
│ Total Projects       │ 2                          │
│ Total Dependencies   │ 24                         │
│ Ecosystems           │ 3                          │
│ Errors               │ 0                          │
└───────────────────────────────────────────────────┘

──────────────────────────────────────────────────
▶ Ecosystem Breakdown
──────────────────────────────────────────────────
  🦀 Rust 8 dependencies
  📦 Node.js 11 dependencies
  🐍 Python 5 dependencies

──────────────────────────────────────────────────
▶ Project Details
──────────────────────────────────────────────────
└─ 📂 my-project (24 deps) dependencies
  ├─ 🦀 Rust (8 deps)
    ├─ serde 1.0  prod  ./Cargo.toml
    ├─ clap 4.0  prod  ./Cargo.toml
    ├─ tokio 1.0  prod  ./Cargo.toml
    ├─ colored 2.0  prod  ./Cargo.toml
    ├─ thiserror 1.0  prod  ./Cargo.toml
    └─ ... 3 more dependencies
  ├─ 📦 Node.js (11 deps)
    ├─ express ^4.18.2  prod  ./package.json
    ├─ react ^18.2.0  prod  ./package.json
    ├─ typescript ^5.0.0  dev  ./package.json
    └─ ... 8 more dependencies
  └─ 🐍 Python (5 deps)
    ├─ requests 2.31.0  prod  ./requirements.txt
    ├─ flask 2.3.0  prod  ./requirements.txt
    └─ ... 3 more dependencies

💡 Tips:
  • Check for updates: Run package manager update commands
  • Security scan: Use tools like cargo audit, npm audit, or safety
  • Clean unused deps: Remove dependencies you're not using
```

### Key Visual Features
- 🎨 **Color-coded output** for better readability
- 📊 **Progress bars** and health percentages  
- 🌳 **Tree-structured** project breakdowns
- 🏷️ **Dependency type badges** (prod/dev/build/optional)
- 🔗 **Ecosystem icons** (🦀 Rust, 📦 Node.js, 🐍 Python, 🐹 Go)
- 💡 **Helpful tips** for next steps

## Development

### Prerequisites
- Rust 1.70+ (2021 edition)
- Git (for git repository analysis)

### Building
```bash
cargo build
```

### Running Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test git::tests
```

### Documentation
Generate and view documentation:
```bash
# Generate docs
cargo doc --open

# Generate docs with private items
cargo doc --document-private-items --open
```

## Testing

The project includes comprehensive test coverage:

- **Unit Tests**: Located in each module (`#[cfg(test)]` blocks)
- **Integration Tests**: Located in `tests/` directory
- **Test Coverage**: 29+ tests covering all major functionality

Run tests with:
```bash
cargo test
```

## API Documentation

### Core Modules

- **`cli`**: Command-line interface definition and parsing
- **`scanner`**: Analysis modules for different environment aspects
  - `git`: Git repository health analysis
  - `deps`: Dependency scanning across multiple ecosystems
  - `system`: System resource monitoring (planned)
  - `analytics`: Project analytics (planned)
- **`utils`**: Utility functions and helpers
  - `fs`: File system operations
  - `display`: Terminal output formatting and colorization utilities

### Key Functions

```rust
use devhealth::scanner::{git, deps};
use std::path::Path;

// Scan for git repositories
let repos = git::scan_directory(Path::new("."))?;
git::display_results(&repos);

// Scan for dependencies
let dep_reports = deps::scan_dependencies(Path::new("."))?;
deps::display_results(&dep_reports);
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style
- Follow Rust formatting guidelines (`cargo fmt`)
- Ensure all tests pass (`cargo test`)
- Add documentation for public APIs
- Include tests for new functionality

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Architecture

DevHealth follows a modular architecture:

```
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library root with public API
├── cli.rs           # Command-line interface definition
├── scanner/         # Analysis modules
│   ├── git.rs       # Git repository analysis
│   ├── deps.rs      # Dependency scanning
│   ├── system.rs    # System monitoring (planned)
│   └── analytics.rs # Project analytics (planned)
└── utils/           # Utility functions
    ├── fs.rs        # File system operations
    └── display.rs   # Terminal output formatting and colors
```

## Roadmap

- [x] **v0.1.0**: Git repository health analysis
- [x] **v0.2.0**: Dependency health scanning with enhanced colorized display
- [ ] **v0.3.0**: System resource monitoring
- [ ] **v0.4.0**: Project analytics and metrics
- [ ] **v0.5.0**: Configuration file support
- [ ] **v0.6.0**: Security vulnerability scanning
- [ ] **v1.0.0**: Stable release with full feature set

## FAQ

**Q: Why another development environment tool?**
A: DevHealth aims to provide a unified view of your development environment health, combining git status, dependency health, and system monitoring in one tool.

**Q: What operating systems are supported?**
A: Currently tested on Linux. Windows and macOS support planned.

**Q: Can I extend DevHealth with custom scanners?**
A: Yes! The modular architecture makes it easy to add new scanner modules.

---