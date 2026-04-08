#!/bin/bash

# Development script for Ferriprove
# Provides common development commands and shortcuts

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

# Function to show usage
show_usage() {
    echo "Ferriprove Development Script"
    echo ""
    echo "Usage: $0 <command>"
    echo ""
    echo "Commands:"
    echo "  setup           - Setup development environment (runs setup-precommit.sh)"
    echo "  check           - Run cargo check on all packages"
    echo "  test            - Run cargo test on all packages"
    echo "  fmt             - Format code with rustfmt"
    echo "  clippy          - Run clippy lints"
    echo "  doc             - Generate documentation"
    echo "  audit           - Run cargo audit (security check)"
    echo "  deny            - Run cargo deny (license check)"
    echo "  precommit       - Run pre-commit hooks on all files"
    echo "  watch-check     - Watch for changes and run cargo check"
    echo "  watch-test      - Watch for changes and run cargo test"
    echo "  watch-clippy    - Watch for changes and run clippy"
    echo "  clean           - Clean build artifacts"
    echo "  update          - Update dependencies"
    echo "  publish-dry-run - Dry run publish for all packages"
    echo "  benchmark      - Run performance benchmarks"
    echo ""
    echo "Examples:"
    echo "  $0 setup        # Initial setup"
    echo "  $0 test         # Run all tests"
    echo "  $0 watch-test   # Watch and test on changes"
    echo "  $0 benchmark    # Run performance benchmarks"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    print_error "This script must be run from the Ferriprove root directory (where Cargo.toml is located)"
    exit 1
fi

# Parse command
case "${1:-}" in
    "setup")
        print_status "Running development environment setup..."
        ./scripts/setup-precommit.sh
        ;;
    "check")
        print_status "Running cargo check..."
        cargo check --workspace --all-targets --all-features
        ;;
    "test")
        print_status "Running cargo test..."
        cargo test --workspace --all-features
        ;;
    "fmt")
        print_status "Formatting code with rustfmt..."
        cargo fmt --all
        ;;
    "clippy")
        print_status "Running clippy lints..."
        cargo clippy --workspace --all-targets --all-features -- -D warnings
        ;;
    "doc")
        print_status "Generating documentation..."
        cargo doc --workspace --no-deps --document-private-items
        print_success "Documentation generated. Run 'cargo doc --open' to view."
        ;;
    "audit")
        print_status "Running cargo audit..."
        cargo audit
        ;;
    "deny")
        print_status "Running cargo deny..."
        cargo deny check
        ;;
    "precommit")
        print_status "Running pre-commit hooks..."
        pre-commit run --all-files
        ;;
    "watch-check")
        print_status "Watching for changes and running cargo check..."
        cargo watch -x 'check --workspace --all-targets --all-features'
        ;;
    "watch-test")
        print_status "Watching for changes and running cargo test..."
        cargo watch -x 'test --workspace --all-features'
        ;;
    "watch-clippy")
        print_status "Watching for changes and running clippy..."
        cargo watch -x 'clippy --workspace --all-targets --all-features -- -D warnings'
        ;;
    "clean")
        print_status "Cleaning build artifacts..."
        cargo clean
        ;;
    "update")
        print_status "Updating dependencies..."
        cargo update
        ;;
    "publish-dry-run")
        print_status "Running publish dry run for all packages..."
        for package in ferriprove-types ferriprove-export ferriprove-kernel ferriprove-cli; do
            print_status "Dry run publishing $package..."
            cargo publish --dry-run -p "$package"
        done
        ;;
    "benchmark")
        print_status "Running performance benchmarks..."
        ./scripts/run-benchmarks.sh
        ;;
    "help"|"-h"|"--help"|"")
        show_usage
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        show_usage
        exit 1
        ;;
esac

print_success "Command completed successfully!"
