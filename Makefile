# DevHealth Development Makefile
# 
# Common development tasks for the DevHealth project

.PHONY: help build test doc clean fmt clippy check install run-check run-scan all

help: ## Show this help message
	@echo "DevHealth Development Commands:"
	@echo "==============================="
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the project
	cargo build

test: ## Run all tests
	cargo test

doc: ## Generate and open documentation
	cargo doc --document-private-items --open

clean: ## Clean build artifacts
	cargo clean

fmt: ## Format code
	cargo fmt

clippy: ## Run clippy linting
	cargo clippy -- -D warnings

check: ## Quick compile check
	cargo check

install: ## Install the binary locally
	cargo install --path .

run-check: ## Run devhealth check command
	cargo run -- check

run-scan: ## Run devhealth scan with all flags
	cargo run -- scan --git --deps --system

all: fmt clippy test doc ## Run all quality checks and generate docs

# Development workflow targets
dev-setup: ## Set up development environment
	@echo "🦀 Setting up DevHealth development environment..."
	rustup component add rustfmt clippy
	@echo "✅ Development environment ready!"

ci: fmt clippy test ## Run CI pipeline locally
	@echo "🎉 All CI checks passed!"
