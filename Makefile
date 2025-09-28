# PingTest Makefile
# Common development tasks

.PHONY: help build test run clean fmt clippy bench install uninstall

# Default target
help:
	@echo "PingTest - Available targets:"
	@echo "  build     - Build the project"
	@echo "  test      - Run tests"
	@echo "  run       - Run the application"
	@echo "  clean     - Clean build artifacts"
	@echo "  fmt       - Format code"
	@echo "  clippy    - Run clippy linter"
	@echo "  bench     - Run benchmarks"
	@echo "  install   - Install the binary"
	@echo "  uninstall - Uninstall the binary"
	@echo "  check     - Run all checks (fmt + clippy + test)"
	@echo "  release   - Build release version"

# Build the project
build:
	cargo build

# Build release version
release:
	cargo build --release

# Run tests
test:
	cargo test

# Run the application
run:
	cargo run

# Run with specific options
run-quick:
	cargo run -- --quick

run-theme:
	cargo run -- --theme dracula

run-debug:
	RUST_LOG=debug cargo run

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy -- -D warnings

# Run benchmarks
bench:
	cargo bench

# Install the binary
install:
	cargo install --path .

# Uninstall the binary
uninstall:
	cargo uninstall pingtest

# Run all checks
check: fmt clippy test
	@echo "All checks passed!"

# Run examples
example-basic:
	cargo run --example basic_usage

example-advanced:
	cargo run --example advanced_usage

example-theme:
	cargo run --example custom_theme

# Development helpers
watch:
	cargo watch -x run

watch-test:
	cargo watch -x test

# Documentation
docs:
	cargo doc --open

# Security audit
audit:
	cargo audit

# Update dependencies
update:
	cargo update

# Check for outdated dependencies
outdated:
	cargo outdated

# Run with different themes
theme-dracula:
	cargo run -- --theme dracula

theme-nord:
	cargo run -- --theme nord

theme-solarized:
	cargo run -- --theme solarized-dark

theme-monokai:
	cargo run -- --theme monokai

# Run speed tests with different parameters
test-5s:
	cargo run -- --duration 5

test-10s:
	cargo run -- --duration 10

test-30s:
	cargo run -- --duration 30

test-2conn:
	cargo run -- --connections 2

test-8conn:
	cargo run -- --connections 8

# Export results
export-json:
	cargo run -- --export results.json

export-csv:
	cargo run -- --export results.csv

# History commands
history:
	cargo run -- --history

stats:
	cargo run -- --stats 30

clear-history:
	cargo run -- --clear-history

# Ping analysis
ping-analysis:
	cargo run -- --ping-analysis

ping-jitter:
	cargo run -- --jitter-detection

# Server selection
server-test:
	cargo run -- --server 12345

# Quick tests
quick-download:
	cargo run -- --quick --no-upload

quick-upload:
	cargo run -- --quick --no-download

# Development setup
setup:
	rustup update
	cargo install cargo-watch cargo-audit cargo-fmt cargo-clippy
	cargo build

# CI/CD helpers
ci-test: fmt clippy test
	@echo "CI tests passed!"

# Package for distribution
package:
	cargo build --release
	tar -czf pingtest-$(shell cargo metadata --format-version 1 | jq -r '.packages[0].version')-$(shell uname -s)-$(shell uname -m).tar.gz target/release/pingtest

# Show project info
info:
	@echo "PingTest Project Information:"
	@echo "Version: $(shell cargo metadata --format-version 1 | jq -r '.packages[0].version')"
	@echo "Rust version: $(shell rustc --version)"
	@echo "Cargo version: $(shell cargo --version)"
	@echo "Target: $(shell rustc -vV | grep host | cut -d' ' -f2)"