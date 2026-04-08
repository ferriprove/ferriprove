# Soundness Assumptions

This document lists all soundness assumptions for the Ferriprove kernel. Soundness is a critical property - a soundness bug means the kernel might accept an invalid proof, which is a P0 issue.

## Current Soundness Assumptions

### Core Type System (ferriprove-types)

#### [A-1] Expr Representation
- All `Expr` variants are well-formed
- No circular references in `Expr` trees
- `Level` values are finite and well-founded
- `Name` values are properly interned and unique

#### [A-2] Memory Safety
- No memory corruption in `Expr` manipulation
- Proper bounds checking on all array/vector access
- No data races in concurrent access patterns

### Export Parser (ferriprove-export)

#### [B-1] NDJSON Format Validation
- Input follows the lean4export specification exactly
- All required fields are present and valid
- No malformed UTF-8 sequences in string fields
- Forward references are resolved correctly

#### [B-2] Reconstruction Correctness
- Reconstructed `Expr` trees match the original Lean terms
- Level substitution preserves typing
- Name resolution is consistent across the entire export
- No information loss during reconstruction

### Kernel Type Checker (ferriprove-kernel)

#### [C-1] Environment Invariants
- All declarations in the environment are well-typed
- No duplicate names in the environment
- Inductive types are properly formed (positivity, strict positivity)
- Recursive definitions are well-founded

#### [C-2] Reduction Engine
- `whnf` is strongly normalizing on well-typed terms
- All reduction rules preserve typing
- No infinite loops in reduction
- Beta reduction is implemented correctly

#### [C-3] Type Inference
- `infer_type` returns the correct principal type
- Type checking is decidable for all well-formed terms
- No false positives in type checking
- Error messages are sound (only reported for actual errors)

#### [C-4] Definitional Equality
- `def_eq` is reflexive, symmetric, and transitive
- `def_eq` respects conversion rules
- No false positives in definitional equality
- Conversion is decidable

#### [C-5] Inductive Types
- Inductive type formation rules are enforced
- Constructor arguments are correctly typed
- Recursor principles are correctly generated
- No inconsistency in inductive definitions

### Elaborator (ferriprove-elab)

#### [D-1] Implicit Arguments
- Implicit argument inference is sound
- No incorrect implicit argument synthesis
- Type class resolution is terminating
- Instance search is sound

#### [D-2] Unification
- Higher-order unification is sound
- No incorrect unification solutions
- Unification is terminating on well-typed terms
- No occurs check violations

### Tactic Engine (ferriprove-tactic)

#### [E-1] Tactic Correctness
- All tactics preserve soundness
- No tactic can produce invalid proofs
- Tactic state transitions are well-defined
- No unsound tactic combinations

## Formal Verification Status

### Verified Components
- None yet (target for M1-I)

### Aeneas Translation Status
- Not started (planned for M1-I)

### Lean4Lean Integration
- Not started (planned for M1-I)

## Soundness Bug Process

### Classification
- **P0**: Potential false positive in kernel
- **P1**: Soundness issue in non-kernel components
- **P2**: Performance issues affecting soundness checks

### Response Time
- **P0**: Immediate response, fix within 24 hours
- **P1**: Response within 48 hours, fix within 1 week
- **P2**: Response within 1 week, fix in next release

### Verification Requirements
- All P0 fixes must include regression tests
- P0 fixes require formal verification when possible
- All soundness fixes must be reviewed by multiple maintainers

## Testing Strategy

### Property-Based Testing
```rust
// Example property test
#[quickcheck]
fn def_eq_reflexivity(expr: Expr) -> bool {
    def_eq(&expr, &expr)
}
```

### Regression Testing
- All historical soundness bugs have regression tests
- Arena test suite runs on every PR
- Mathlib compatibility tests on every release

### Formal Verification
- Kernel functions translated to Lean via Aeneas
- Soundness proofs checked against Lean4Lean
- All kernel properties formally verified

## External Dependencies

### Trusted Dependencies
- `std` - Rust standard library (audited)
- `indexmap` - Ordered hash map (audited, no unsafe)
- `hashbrown` - Hash table implementation (audited)

### Untrusted Dependencies
- All parsing dependencies (treated as untrusted)
- All LSP dependencies (treated as untrusted)
- All CLI dependencies (treated as untrusted)

## Historical Soundness Issues

This section will be populated as issues are discovered and fixed.

### None yet

## Soundness Checklist

Before any release, verify:

- [ ] All kernel changes have formal verification
- [ ] Arena test suite passes 100%
- [ ] No new soundness assumptions introduced
- [ ] All property tests pass
- [ ] Mathlib compatibility verified
- [ ] No `unsafe` code in trusted components
- [ ] All dependencies audited

## Reporting Soundness Issues

Soundness bugs should be reported via GitHub issues with the `soundness` label and:

- Clear description of the issue
- Minimal reproduction case
- Expected vs actual behavior
- Potential impact assessment

For P0 issues, also email: **soundness@ferriprove.org**

## Soundness Team

The Ferriprove soundness team can be contacted at:
- **soundness@ferriprove.org** - Soundness bugs and concerns
- **maintainers@ferriprove.org** - General soundness questions
