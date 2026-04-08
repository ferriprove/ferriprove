#!/bin/bash

# Run performance benchmarks on changes
# This script runs basic performance tests to catch regressions

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    print_error "This script must be run from the Ferriprove root directory"
    exit 1
fi

print_status "Running performance benchmarks..."

# Build in release mode for accurate benchmarks
print_status "Building in release mode..."
cargo build --release --workspace

# Test ferriprove-types performance
if [[ -d "ferriprove-types" ]]; then
    print_status "Running ferriprove-types benchmarks..."
    
    # Create a simple benchmark script
    cat > /tmp/benchmark_types.rs << 'EOF'
use std::time::Instant;
use ferriprove_types::*;
use ferriprove_types::interning::ExprInterner;

fn main() {
    let start = Instant::now();
    
    // Test expression creation performance
    let mut interner = ExprInterner::new();
    let nat = Name::from("Nat");
    
    for i in 0..1000 {
        let expr = Expr::app(
            Expr::const_(nat.clone()),
            Expr::lit(Literal::Nat(i))
        );
        let _interned = interner.intern(expr).unwrap();
    }
    
    let duration = start.elapsed();
    println!("Created and interned 1000 expressions in {:?}", duration);
    
    // Test substitution performance
    let start = Instant::now();
    let x = Expr::Var(0);
    let replacement = Expr::lit(Literal::Nat(42));
    let mut expr = x.clone();
    
    for _ in 0..100 {
        expr = utils::subst(&expr, &replacement, 0);
    }
    
    let duration = start.elapsed();
    println!("Performed 100 substitutions in {:?}", duration);
    
    println!("Performance benchmarks completed!");
}
EOF

    # Compile and run the benchmark
    rustc --edition 2021 -L target/release/deps /tmp/benchmark_types.rs -o /tmp/benchmark_types --extern ferriprove_types=target/release/libferriprove_types.rlib
    /tmp/benchmark_types
    rm -f /tmp/benchmark_types.rs /tmp/benchmark_types
fi

# Test compilation time
print_status "Testing compilation time..."
COMPILE_START=$(date +%s%N)
cargo check --workspace --all-targets
COMPILE_END=$(date +%s%N)
COMPILE_TIME=$(( (COMPILE_END - COMPILE_START) / 1000000 ))
print_status "Full workspace compilation took ${COMPILE_TIME}ms"

# Set performance thresholds (in milliseconds)
COMPILE_THRESHOLD=10000

if [[ $COMPILE_TIME -gt $COMPILE_THRESHOLD ]]; then
    print_warning "Compilation time (${COMPILE_TIME}ms) exceeds threshold (${COMPILE_THRESHOLD}ms)"
else
    print_success "Compilation time is within acceptable limits"
fi

# Check binary size
if [[ -f "target/release/ferriprove-cli" ]]; then
    BINARY_SIZE=$(stat -c%s "target/release/ferriprove-cli")
    SIZE_MB=$((BINARY_SIZE / 1024 / 1024))
    print_status "CLI binary size: ${SIZE_MB}MB"
    
    SIZE_THRESHOLD=50MB
    if [[ $BINARY_SIZE -gt $((50 * 1024 * 1024)) ]]; then
        print_warning "Binary size (${SIZE_MB}MB) exceeds threshold (${SIZE_THRESHOLD})"
    else
        print_success "Binary size is within acceptable limits"
    fi
fi

print_success "Performance benchmarks completed successfully!"
