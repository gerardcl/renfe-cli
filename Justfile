set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# -----------------------------------------------------------------------------
# Setup
# -----------------------------------------------------------------------------

# Set up the complete local development environment.
dev-setup: setup-python setup-rust setup-hooks
    uvx maturin develop --uv
    @echo ""
    @echo "Development environment ready."
    @echo "Run 'just run --help' to verify the CLI."

# Install the required Python version and create the virtual environment.
setup-python:
    uv python install 3.10
    uv venv --python 3.10

# Fetch Rust dependencies and verify the configured toolchain.
setup-rust:
    rustup show
    cargo fetch

# Install the Git hooks.
setup-hooks:
    prek install

# -----------------------------------------------------------------------------
# Development
# -----------------------------------------------------------------------------

# Run the development CLI through the local virtual environment.
run *args:
    uv run renfe-cli {{args}}

# Open a Python shell in the development environment.
python:
    uv run python

# Build the Rust binary.
build:
    cargo build

# Build the optimized Rust binary.
build-release:
    cargo build --release

# Install the Rust CLI globally.
install:
    cargo install --path .

# Reinstall the Rust CLI globally.
reinstall:
    cargo install --path . --force

# -----------------------------------------------------------------------------
# Formatting & linting
# -----------------------------------------------------------------------------

# Format Rust code.
fmt:
    cargo fmt --all

# Check Rust formatting without modifying files.
fmt-check:
    cargo fmt --all -- --check

# Run Clippy with warnings treated as errors.
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Check the Rust project.
check:
    cargo check --all-targets --all-features

# Run all pre-commit checks.
lint:
    prek run -a

# -----------------------------------------------------------------------------
# Tests
# -----------------------------------------------------------------------------

# Run the Rust test suite.
test:
    cargo test --all-features

# Run Rust tests with output enabled.
test-verbose:
    cargo test --all-features -- --nocapture

# Run the complete local CI suite.
ci: fmt-check clippy check test lint

# -----------------------------------------------------------------------------
# Python / packaging
# -----------------------------------------------------------------------------

# Build and install the extension into the local uv environment.
develop:
    uvx maturin develop --uv

# Build a release wheel into dist/.
package:
    uvx maturin build --release --out dist

# Build a debug wheel into dist/.
package-debug:
    uvx maturin build --out dist

# Install the release build into the local uv environment.
install-python:
    uvx maturin develop --uv --release

# -----------------------------------------------------------------------------
# Release
# -----------------------------------------------------------------------------

# Run the checks required before building release artifacts.
release-check: fmt-check clippy check test
    @echo "Release checks passed."

# Build release binary and Python wheel.
release-build: release-check
    cargo build --release
    uvx maturin build --release --out dist

# -----------------------------------------------------------------------------
# Maintenance
# -----------------------------------------------------------------------------

# Update Rust dependencies.
update:
    cargo update

# Remove Rust build artifacts.
clean:
    cargo clean

# Remove Python distribution artifacts.
clean-dist:
    rm -rf dist

# Remove all generated build artifacts.
clean-all: clean clean-dist
    rm -rf .pytest_cache .mypy_cache

# -----------------------------------------------------------------------------
# Information
# -----------------------------------------------------------------------------

# Show the versions of the development tools.
info:
    rustc --version
    cargo --version
    uv --version
    uv run python --version
    uvx maturin --version
    prek --version
    just --version

# -----------------------------------------------------------------------------
# Short aliases
# -----------------------------------------------------------------------------

alias f := fmt
alias c := check
alias t := test
alias b := build
alias br := build-release
alias p := package
