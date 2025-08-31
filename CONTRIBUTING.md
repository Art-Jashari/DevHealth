# Contributing to DevHealth

Thank you for your interest in contributing to DevHealth! This guide will help you get started.

##  Quick Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/DevHealth.git
   cd DevHealth
   ```
3. **Set up the development environment**:
   ```bash
   make dev-setup  # Or manually install rustfmt and clippy
   ```

##  Development Workflow

### Before Starting
```bash
# Ensure everything builds and tests pass
make all
```

### Making Changes
1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Write tests first** (TDD approach):
   ```bash
   # Add tests to appropriate test modules
   cargo test your_new_test
   ```

3. **Implement your feature**

4. **Add documentation**:
   ```bash
   # Add rustdoc comments to public APIs
   cargo doc --open
   ```

5. **Run quality checks**:
   ```bash
   make ci  # Runs fmt, clippy, and tests
   ```

### Submitting Changes
1. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add amazing new feature"
   ```

2. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Open a Pull Request** on GitHub

##  Code Standards

### Rust Code Style
- **Formatting**: Use `cargo fmt` (enforced)
- **Linting**: Pass `cargo clippy` checks
- **Naming**: Follow Rust naming conventions
- **Documentation**: Document all public APIs

### Documentation Standards
- **All public functions** must have doc comments
- **Include examples** for complex functionality
- **Document error conditions** and edge cases
- **Module-level documentation** for overview

### Testing Standards
- **Unit tests** for individual functions
- **Integration tests** for CLI commands  
- **Test naming**: Use descriptive names (`finds_git_repositories` not `test_find_git`)
- **Test organization**: Group related tests in modules

### Commit Message Convention
We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

feat: add new feature
fix: bug fix
docs: documentation changes
test: adding tests
refactor: code refactoring
style: formatting changes
chore: maintenance tasks
```

##  Testing

### Running Tests
```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only  
cargo test --test integration_tests

# Specific test
cargo test test_name
```

### Adding Tests
- **Unit tests**: Add to `#[cfg(test)]` modules in source files
- **Integration tests**: Add to `tests/` directory
- **Documentation tests**: Add examples in doc comments

##  Documentation

### Generating Documentation
```bash
# Standard documentation
cargo doc --open

# Include private items (for development)
cargo doc --document-private-items --open
```

### Documentation Best Practices
1. **Start with a summary line**
2. **Provide detailed description**
3. **Include examples for public APIs**
4. **Document error conditions**
5. **Link to related functionality**

##  Features to Implement

### High Priority
- [ ] **Dependency scanning**: Support for multiple package managers
- [ ] **System monitoring**: CPU, memory, disk usage
- [ ] **Configuration files**: Support for `.devhealth.toml`

### Medium Priority  
- [ ] **Project analytics**: Code quality metrics
- [ ] **Output formats**: JSON, YAML output options
- [ ] **Watch mode**: Continuous monitoring

### Low Priority
- [ ] **Web dashboard**: Browser-based interface
- [ ] **Notifications**: Email/Slack integration
- [ ] **Plugins**: Custom scanner support

##  Issue Reporting

### Bug Reports
Include:
- Rust version (`rustc --version`)
- OS and version
- Steps to reproduce
- Expected vs actual behavior
- Error messages (if any)

### Feature Requests
Include:
- Use case description
- Proposed solution
- Alternative solutions considered
- Additional context

##  Questions?

- **Documentation**: Check `docs/` directory
- **Examples**: See `examples/` directory (when added)
- **Issues**: Open a GitHub issue
- **Discussions**: Use GitHub Discussions

##  Recognition

Contributors will be:
- Listed in `CONTRIBUTORS.md`
- Mentioned in release notes
- Credited in documentation

Thank you for contributing to DevHealth!
