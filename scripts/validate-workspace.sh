#!/bin/bash

# Validate Cargo workspace structure
# This script ensures the workspace is properly configured

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

print_status "Validating Cargo workspace structure..."

# Check if workspace is defined
if ! grep -q "^\[workspace\]" Cargo.toml; then
    print_error "No [workspace] section found in root Cargo.toml"
    exit 1
fi

# Check workspace members from Cargo.toml
WORKSPACE_MEMBERS=$(grep -A 10 "^\[workspace\]" Cargo.toml | grep -E "^\s*\".*\"" | sed 's/.*"\(.*\)".*/\1/')

for member in $WORKSPACE_MEMBERS; do
    if [[ ! -d "$member" ]]; then
        print_error "Workspace member $member does not exist"
        exit 1
    fi

    if [[ ! -f "$member/Cargo.toml" ]]; then
        print_error "Workspace member $member has no Cargo.toml"
        exit 1
    fi

    print_status "✓ Workspace member $member exists"
done

# Check if all workspace members have proper version consistency
ROOT_VERSION=$(grep "^version = " Cargo.toml | sed 's/version = "//' | sed 's/"//')
print_status "Root version: $ROOT_VERSION"

for member in $WORKSPACE_MEMBERS; do
    if [[ -f "$member/Cargo.toml" ]]; then
        # Check if member has its own version or inherits from workspace
        if grep -q "^version = " "$member/Cargo.toml"; then
            MEMBER_VERSION=$(grep "^version = " "$member/Cargo.toml" | sed 's/version = "//' | sed 's/"//')
            if [[ "$MEMBER_VERSION" != "$ROOT_VERSION" ]]; then
                print_warning "Version mismatch: $member has $MEMBER_VERSION, root has $ROOT_VERSION"
            else
                print_status "✓ $member version matches root"
            fi
        else
            print_status "✓ $member inherits version from workspace"
        fi
    fi
done

print_status "Workspace validation completed successfully!"
exit 0
