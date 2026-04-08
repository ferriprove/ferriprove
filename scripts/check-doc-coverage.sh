#!/bin/bash

# Check documentation coverage for Rust code
# This script ensures adequate documentation coverage

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    print_error "This script must be run from the Ferriprove root directory"
    exit 1
fi

print_status "Checking documentation coverage..."

# Check if documentation builds without warnings
if ! cargo doc --no-deps --document-private-items --quiet 2>/dev/null; then
    print_error "Documentation build failed"
    cargo doc --no-deps --document-private-items
    exit 1
fi

# Check for undocumented public items in main crates
CRATES=("ferriprove-types" "ferriprove-export" "ferriprove-kernel")

for crate in "${CRATES[@]}"; do
    if [[ -d "$crate" ]]; then
        print_status "Checking documentation coverage for $crate..."
        
        # Count public items
        PUBLIC_ITEMS=$(cargo doc --no-deps --document-private-items --quiet --message-format=json 2>/dev/null | \
            jq -r 'select(.target.crate_name == "'$crate'") | select(.message.level == "warning") | .message.message' | \
            grep -c "missing documentation" || echo "0")
        
        if [[ "$PUBLIC_ITEMS" -gt 0 ]]; then
            print_warning "$crate has $PUBLIC_ITEMS undocumented public items"
        else
            print_status "$crate has good documentation coverage"
        fi
    fi
done

# Check for README files in each crate
for crate in "${CRATES[@]}"; do
    if [[ -d "$crate" ]]; then
        if [[ ! -f "$crate/README.md" ]]; then
            print_warning "$crate has no README.md file"
        fi
    fi
done

# Check if examples exist
if [[ ! -d "examples" ]]; then
    print_warning "No examples directory found"
fi

print_status "Documentation coverage check completed!"
