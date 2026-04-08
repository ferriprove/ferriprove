#!/bin/bash

# Setup script for pre-commit hooks in Ferriprove
# This script installs pre-commit and sets up the hooks

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    print_error "This script must be run from the Ferriprove root directory (where Cargo.toml is located)"
    exit 1
fi

print_status "Setting up pre-commit hooks for Ferriprove..."

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    print_status "Installing pre-commit..."

    # Try to install pre-commit using pip
    if command -v pip3 &> /dev/null; then
        pip3 install pre-commit
    elif command -v pip &> /dev/null; then
        pip install pre-commit
    else
        print_error "Python/pip is required to install pre-commit. Please install Python and pip first."
        exit 1
    fi
else
    print_success "pre-commit is already installed"
fi

# Check if rustfmt is installed
if ! command -v rustfmt &> /dev/null; then
    print_status "Installing rustfmt..."
    rustup component add rustfmt
else
    print_success "rustfmt is already installed"
fi

# Check if clippy is installed
if ! command -v cargo-clippy &> /dev/null; then
    print_status "Installing clippy..."
    rustup component add clippy
else
    print_success "clippy is already installed"
fi

# Check if cargo-audit is installed
if ! command -v cargo-audit &> /dev/null; then
    print_status "Installing cargo-audit..."
    cargo install cargo-audit
else
    print_success "cargo-audit is already installed"
fi

# Check if cargo-deny is installed
if ! command -v cargo-deny &> /dev/null; then
    print_status "Installing cargo-deny..."
    cargo install cargo-deny
else
    print_success "cargo-deny is already installed"
fi

# Check if cargo-watch is installed (useful for development)
if ! command -v cargo-watch &> /dev/null; then
    print_status "Installing cargo-watch (optional, for development)..."
    cargo install cargo-watch
else
    print_success "cargo-watch is already installed"
fi

# Install pre-commit hooks
print_status "Installing pre-commit hooks..."
pre-commit install

# Run pre-commit on all files to ensure everything is clean
print_status "Running pre-commit on all files to ensure everything is clean..."
pre-commit run --all-files

print_success "Pre-commit setup completed successfully!"
echo ""
print_status "Available pre-commit commands:"
echo "  pre-commit run --all-files          # Run checks on all files"
echo "  pre-commit run --files <file>       # Run checks on specific files"
echo "  pre-commit install                  # Install hooks"
echo "  pre-commit uninstall                # Uninstall hooks"
echo "  pre-commit autoupdate               # Update hook versions"
echo ""
print_status "Development commands:"
echo "  cargo watch -x 'check'              # Watch for changes and run cargo check"
echo "  cargo watch -x 'test'               # Watch for changes and run tests"
echo "  cargo watch -x 'clippy'             # Watch for changes and run clippy"
echo ""
print_warning "Note: Some hooks (cargo-audit, cargo-deny) are skipped in CI by default"
print_warning "You can run them manually with: cargo audit and cargo deny check"
