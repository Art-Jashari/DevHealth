# DevHealth Developer Guide

## Documentation Tools for Rust 🦀

Yes! Rust has excellent built-in documentation tools, similar to Doxygen for C/C++:

### Primary Tool: `rustdoc`
- **Built into Rust toolchain** - no additional installation needed
- **Generates HTML documentation** from doc comments
- **Supports Markdown** in documentation
- **Cross-links types and functions automatically**
- **Includes search functionality**
- **Can test code examples** in documentation

### Documentation Commands

```bash
# Generate and view documentation
cargo doc --open

# Include private items (for development)
cargo doc --document-private-items --open

# Test documentation examples
cargo test --doc

# Check documentation coverage (nightly only)
cargo +nightly rustdoc -- -Z unstable-options --show-coverage
```

## Project Documentation Structure

### 1. **Module Documentation** (`//!`)
```rust
//! Module-level documentation at the top of files
//! Describes the module's purpose and contents
```

### 2. **Item Documentation** (`///`)
```rust
/// Function/struct/enum documentation
/// Supports markdown formatting
/// 
/// # Arguments
/// # Returns  
/// # Examples
/// # Errors
```

### 3. **Documentation Sections**
- **Summary**: Brief one-line description
- **Description**: Detailed explanation
- **Arguments**: Parameter documentation
- **Returns**: Return value description
- **Examples**: Code usage examples
- **Errors**: Error conditions
- **Panics**: Panic conditions
- **Safety**: For unsafe code

## Development Workflow

### 1. **Setup Development Environment**
```bash
# Clone repository
git clone https://github.com/Art-Jashari/DevHealth.git
cd DevHealth

# Build project
cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

### 2. **Adding New Features**
1. Write tests first (TDD approach)
2. Implement functionality
3. Add comprehensive documentation
4. Update examples and README
5. Run full test suite

### 3. **Documentation Workflow**
1. Write doc comments as you code
2. Include examples in doc comments
3. Test examples with `cargo test --doc`
4. Generate docs with `cargo doc --open`
5. Review documentation in browser

## Quality Standards

### Code Quality
- ✅ All tests must pass
- ✅ No compiler warnings
- ✅ `cargo fmt` formatting
- ✅ `cargo clippy` linting

### Documentation Quality
- ✅ All public APIs documented
- ✅ Examples for complex functions
- ✅ Error conditions documented
- ✅ Module overviews provided

### Testing Standards
- ✅ Unit tests for all functions
- ✅ Integration tests for CLI commands
- ✅ Edge case testing
- ✅ Error condition testing

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite
4. Generate and review documentation
5. Tag release in git
6. Publish to crates.io (optional)

## Tools and Resources

### Development Tools
- **rustfmt**: Code formatting (`cargo fmt`)
- **clippy**: Linting (`cargo clippy`)
- **rustdoc**: Documentation generation (`cargo doc`)
- **cargo-outdated**: Check for outdated dependencies
- **cargo-audit**: Security vulnerability scanning

### IDE Integration
- **VS Code**: rust-analyzer extension
- **IntelliJ IDEA**: Rust plugin
- **Vim/Neovim**: Various Rust plugins

### Documentation Resources
- [The rustdoc book](https://doc.rust-lang.org/rustdoc/)
- [Documentation guidelines](https://rust-lang.github.io/api-guidelines/documentation.html)
- [RFC 1574 - API Documentation Conventions](https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html)
