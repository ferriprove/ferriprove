# Ferriprove — TODO

> Atomic task list. All tasks follow Conventional Commits labeling.
> See [ARCHITECTURE.md](./ARCHITECTURE.md) for design rationale behind each decision point.
> See [README.md](./README.md) for project overview and status.

---

## License Decision
> **No code is written until this section is complete.**

- [ ] **L-1** Evaluate license options against project goals
  - [ ] L-1.1 Apache-2.0: permissive, patent grant, compatible with Lean ecosystem (Apache-2.0)
  - [ ] L-1.2 MIT: permissive, minimal, widely compatible
  - [ ] L-1.3 MIT OR Apache-2.0 dual (Rust convention): maximum ecosystem compatibility
  - [ ] L-1.4 GPL-3.0: copyleft, consider if community protection is priority
  - [ ] L-1.5 Document decision rationale in `GOVERNANCE.md`
- [ ] **L-2** Confirm license compatibility with all dependencies
  - [ ] L-2.1 `nanoda_lib` — MIT — verify fork/derive compatibility
  - [ ] L-2.2 `lean4lean` — Apache-2.0 — verify reference compatibility
  - [ ] L-2.3 `lean-kernel-arena` — verify test corpus usage terms
  - [ ] L-2.4 Planned crates: `bumpalo`, `dashmap`, `tower-lsp`, `logos`, `chumsky`, `rayon`, `criterion`
- [ ] **L-3** Write `LICENSE` file
- [ ] **L-4** Write `NOTICE` file (attribution for referenced implementations)
- [ ] **L-5** Add SPDX identifiers to all initial source files

---

## Milestone 0 — Foundation
> Governance, architecture, tooling. No kernel code yet.
> **Gate:** Repository publicly structured, CI green, license chosen, `AUDIT.md` complete.
> **SemVer:** No release. Internal only.

### M0-A: Repository Setup

- [ ] **M0-A-1** Initialize Cargo workspace
  - [ ] M0-A-1.1 `cargo new --lib ferriprove-types`
  - [ ] M0-A-1.2 `cargo new --lib ferriprove-export`
  - [ ] M0-A-1.3 `cargo new --lib ferriprove-kernel`
  - [ ] M0-A-1.4 `cargo new --bin ferriprove-cli`
  - [ ] M0-A-1.5 Set up `[workspace]` in root `Cargo.toml`
  - [ ] M0-A-1.6 Add workspace members: `ferriprove-types`, `ferriprove-export`, `ferriprove-kernel`, `ferriprove-elab` (stub), `ferriprove-tactic` (stub), `ferriprove-lsp` (stub), `ferriprove-cli`
  - [ ] M0-A-1.7 Set `edition = "2024"` in all crate `Cargo.toml` files
  - [ ] M0-A-1.8 Write `rust-toolchain.toml` pinned to latest stable
  - [ ] M0-A-1.9 Write `.cargo/config.toml` (lint config, target defaults)
  - [ ] M0-A-1.10 Verify dependency graph is acyclic: types → export → kernel → elab → tactic → {lsp, cli}
- [ ] **M0-A-2** Write governance documents
  - [ ] M0-A-2.1 `CONTRIBUTING.md`: code style (rustfmt, clippy config), Conventional Commits, PR process, issue templates
  - [ ] M0-A-2.2 `SECURITY.md`: trust model, soundness bug policy (distinct from security bugs), disclosure process
  - [ ] M0-A-2.3 `SOUNDNESS.md`: list all soundness assumptions (empty at start, updated per kernel change)
  - [ ] M0-A-2.4 `GOVERNANCE.md`: license decision rationale, maintainer policy
  - [ ] M0-A-2.5 `AUDIT.md`: skeleton with sections for nanoda_lib gap analysis (filled in M0-C)
  - [ ] M0-A-2.6 `CHANGELOG.md`: skeleton with `git-cliff` config

### M0-B: CI/CD Pipeline

- [ ] **M0-B-1** Base CI pipeline (`.github/workflows/ci.yml`)
  - [ ] M0-B-1.1 Jobs: `cargo check`, `cargo test --workspace`, `cargo clippy -- -D warnings`, `cargo fmt --check`
  - [ ] M0-B-1.2 Toolchain matrix: `stable`, `beta`, `nightly`
  - [ ] M0-B-1.3 Platform matrix: `ubuntu-latest`, `macos-latest`, `windows-latest`
  - [ ] M0-B-1.4 Cache `~/.cargo/registry` and `target/` between runs
  - [ ] M0-B-1.5 Block PR merge if any matrix job fails
- [ ] **M0-B-2** Security pipeline (`.github/workflows/security.yml`)
  - [ ] M0-B-2.1 `cargo audit` on every PR and nightly cron
  - [ ] M0-B-2.2 `cargo deny check` for license compliance on all transitive deps
  - [ ] M0-B-2.3 Dependabot: weekly patch updates, monthly minor updates
  - [ ] M0-B-2.4 Pin all GitHub Actions to exact SHA (not tag)
  - [ ] M0-B-2.5 SARIF output from `cargo audit` uploaded to GitHub Security tab
- [ ] **M0-B-3** Release pipeline (`.github/workflows/release.yml`)
  - [ ] M0-B-3.1 Trigger on `v*.*.*` semver tags
  - [ ] M0-B-3.2 Build release binaries: `linux-x86_64`, `macos-aarch64`, `windows-x86_64`
  - [ ] M0-B-3.3 Attach binaries to GitHub Release
  - [ ] M0-B-3.4 `cargo publish --dry-run` for `ferriprove-kernel` in CI pre-release check
  - [ ] M0-B-3.5 Generate `CHANGELOG.md` entry from Conventional Commits via `git-cliff`
  - [ ] M0-B-3.6 Sign release binaries with `cosign`
  - [ ] M0-B-3.7 Generate and attach SBOM per release
- [ ] **M0-B-4** Code coverage
  - [ ] M0-B-4.1 Add `cargo-llvm-cov` to CI
  - [ ] M0-B-4.2 Upload report to Codecov
  - [ ] M0-B-4.3 Gate: minimum 80% line coverage on `ferriprove-kernel`
- [ ] **M0-B-5** Benchmark baseline infrastructure
  - [ ] M0-B-5.1 Add `criterion` as dev-dependency in `ferriprove-kernel`
  - [ ] M0-B-5.2 Create `benches/` skeleton with placeholder benchmarks
  - [ ] M0-B-5.3 Store baselines as JSON in `benches/baselines/`
  - [ ] M0-B-5.4 Add `benchmark.yml` workflow (manual trigger + on `main` push)
  - [ ] M0-B-5.5 Fail CI if benchmark regresses >10% vs stored baseline
- [ ] **M0-B-6** Lean proof CI (`.github/workflows/proofs.yml`)
  - [ ] M0-B-6.1 Install `elan` + Lean 4 in CI
  - [ ] M0-B-6.2 Run `lake build` on `proofs/` directory
  - [ ] M0-B-6.3 Fail if any Lean proof in `proofs/` fails to check

### M0-C: `nanoda_lib` Audit

- [ ] **M0-C-1** Clone and build
  - [ ] M0-C-1.1 `git clone https://github.com/ammkrn/nanoda_lib`
  - [ ] M0-C-1.2 `cargo build --release` on current stable Rust — record result
  - [ ] M0-C-1.3 Record exact commit hash in `AUDIT.md`
  - [ ] M0-C-1.4 Read `ammkrn.github.io/type_checking_in_lean4` — note design decisions
- [ ] **M0-C-2** Run against `lean-kernel-arena`
  - [ ] M0-C-2.1 Clone `leanprover/lean-kernel-arena`
  - [ ] M0-C-2.2 Configure `nanoda_lib` as checker in arena `checkers/nanoda_lib.yml`
  - [ ] M0-C-2.3 Run full arena test suite
  - [ ] M0-C-2.4 Record pass/fail per test case in `AUDIT.md`
  - [ ] M0-C-2.5 Record wall-clock timing per test vs reference C++ kernel
- [ ] **M0-C-3** Feature gap analysis
  - [ ] M0-C-3.1 Mutual inductive reduction — present or pending?
  - [ ] M0-C-3.2 `reduceBool` / `native_decide` — supported?
  - [ ] M0-C-3.3 `abbrev`/`opaque` transparency levels — correct behavior?
  - [ ] M0-C-3.4 `Proj` expression type — supported?
  - [ ] M0-C-3.5 String and Nat kernel extensions — enabled and correct?
  - [ ] M0-C-3.6 Record each gap with severity `blocking` / `non-blocking` in `AUDIT.md`
- [ ] **M0-C-4** Architectural compatibility analysis
  - [ ] M0-C-4.1 Map `nanoda_lib` internal module structure
  - [ ] M0-C-4.2 Term representation: locally nameless or de Bruijn? — compatible with elaborator design?
  - [ ] M0-C-4.3 Metavar context: present or absent? — can elaborator be layered on top?
  - [ ] M0-C-4.4 Memory model: arena, `Arc`, or clone-heavy?
  - [ ] M0-C-4.5 Write architecture compatibility verdict in `AUDIT.md`
- [ ] **M0-C-5** Fork/fresh decision
  - [ ] M0-C-5.1 Document decision in `AUDIT.md`: fork `nanoda_lib` or fresh crate
  - [ ] M0-C-5.2 If fork: add attribution in `NOTICE`, notify upstream author
  - [ ] M0-C-5.3 If fresh: extract arena test fixtures from `nanoda_lib` as regression baseline
  - [ ] M0-C-5.4 Record rationale in `ARCHITECTURE.md`

---

## Milestone 1 — Kernel Parity + Verification
> `ferriprove-kernel` passes full lean-kernel-arena. Soundness proved via Lean4Lean.
> **Gate:** All arena tests pass. Mathlib typecheck clean. Performance within 2× C++ kernel.
> **SemVer:** `v0.1.0` — `ferriprove-kernel` published to crates.io.

### M1-A: Term Representation (`ferriprove-types`)

- [ ] **M1-A-1** Define core `Expr` type
  - [ ] M1-A-1.1 `Expr` enum: `Var(usize)`, `Sort(Level)`, `Const(Name, Vec<Level>)`, `App`, `Lam`, `Pi`, `Let`, `Lit(Literal)`, `FVar(FVarId)`, `MVar(MVarId)`
  - [ ] M1-A-1.2 Document variable convention choice (locally nameless) in source
  - [ ] M1-A-1.3 `Level` enum: `Zero`, `Succ`, `Max`, `IMax`, `Param(Name)`, `MVar(LevelMVarId)`
  - [ ] M1-A-1.4 `Name` type: hierarchical dotted names, interned by content
  - [ ] M1-A-1.5 `BinderInfo` enum: `Default`, `Implicit`, `StrictImplicit`, `InstImplicit`
  - [ ] M1-A-1.6 `Literal` enum: `Nat(u64)`, `String(Arc<str>)`
- [ ] **M1-A-2** Implement term interning
  - [ ] M1-A-2.1 Hash-cons `Expr` by structural content → stable `ExprId`
  - [ ] M1-A-2.2 Arena storage: `bumpalo` bump allocator per elaboration session
  - [ ] M1-A-2.3 Intern table: `dashmap` for concurrent read access during parallel elab
  - [ ] M1-A-2.4 Pointer-equality fast path in `def_eq`: identical `ExprId` → equal
  - [ ] M1-A-2.5 Benchmark: intern lookup overhead vs naive `Arc::clone` baseline
- [ ] **M1-A-3** Implement `Expr` utility functions (unit-tested each)
  - [ ] M1-A-3.1 `subst(expr, replacement, depth)` — capture-avoiding substitution
  - [ ] M1-A-3.2 `lift_vars(expr, n)` — de Bruijn index lifting by n
  - [ ] M1-A-3.3 `has_fvar(expr, id) → bool` — occurrence check
  - [ ] M1-A-3.4 `abstract_fvars(expr, fvars) → Expr` — replace FVars with bound vars
  - [ ] M1-A-3.5 `instantiate(expr, args) → Expr` — substitute bound vars with args
  - [ ] M1-A-3.6 `expr_size(expr) → usize` — structural size (used in termination argument)

### M1-B: `lean4export` Parser (`ferriprove-export`)

- [ ] **M1-B-1** Implement NDJSON parser
  - [ ] M1-B-1.1 Parse all declaration kinds from export format
  - [ ] M1-B-1.2 Reconstruct `Expr` tree from flat export representation
  - [ ] M1-B-1.3 Handle forward references (declarations referencing later-defined names)
  - [ ] M1-B-1.4 Fuzz test parser against malformed/truncated inputs with `cargo-fuzz`
  - [ ] M1-B-1.5 Unit test: round-trip a small export file, verify env reconstructed correctly
- [ ] **M1-B-2** Register `ferriprove-export` in lean-kernel-arena
  - [ ] M1-B-2.1 Confirm arena input format is covered by parser
  - [ ] M1-B-2.2 Expose `parse_file(path) → Result<Vec<Declaration>, ParseError>` as public API

### M1-C: Environment (`ferriprove-kernel`)

- [ ] **M1-D-1** Define `Declaration` type
  - [ ] M1-D-1.1 Variants: `Axiom`, `Definition`, `Theorem`, `Opaque`, `Quot`, `Inductive`, `Constructor`, `Recursor`
  - [ ] M1-D-1.2 Transparency field: `reducible`, `instances`, `default`, `all`
  - [ ] M1-D-1.3 Universe parameter list per declaration
  - [ ] M1-D-1.4 `is_definition() → bool`, `is_theorem() → bool` helpers
- [ ] **M1-D-2** Implement `Environment`
  - [ ] M1-D-2.1 `HashMap<Name, Declaration>` with O(1) lookup
  - [ ] M1-D-2.2 `add_declaration(decl) → Result<(), EnvError>` with duplicate name check
  - [ ] M1-D-2.3 `get_declaration(name, transparency) → Option<&Declaration>`
  - [ ] M1-D-2.4 Extension protocol for inductive types (add constructors + recursor atomically)

### M1-D: Reduction Engine (Pure — Aeneas scope)

- [ ] **M1-D-1** Implement `whnf(env, expr) → Expr`
  - [ ] M1-D-1.1 Beta: `(λx. e) a → e[x := a]`
  - [ ] M1-D-1.2 Delta: unfold `Definition` by transparency
  - [ ] M1-D-1.3 Iota: recursor applied to constructor
  - [ ] M1-D-1.4 Zeta: `let x := v; b → b[x := v]`
  - [ ] M1-D-1.5 Eta: `λx. f x → f` when `x ∉ fv(f)`
  - [ ] M1-D-1.6 Nat/String literal arithmetic reduction
  - [ ] M1-D-1.7 `reduceBool` (flag-gated, excluded from Aeneas path)
  - [ ] M1-D-1.8 Property test: `whnf(whnf(e)) == whnf(e)` (idempotency)
- [ ] **M1-D-2** Implement universe level operations
  - [ ] M1-D-2.1 `level_normalize(l) → Level`
  - [ ] M1-D-2.2 `level_leq(l1, l2, params) → bool`
  - [ ] M1-D-2.3 `level_equiv(l1, l2, params) → bool`
- [ ] **M1-D-3** Implement inductive type reduction
  - [ ] M1-D-3.1 Simple inductive recursor application
  - [ ] M1-D-3.2 Mutual inductive recursor application
  - [ ] M1-D-3.3 Nested inductive types
  - [ ] M1-D-3.4 `Quot` reduction: `Quot.lift`, `Quot.ind`
  - [ ] M1-D-3.5 Regression test each case against Lean 4 output

### M1-E: Type Inference (Pure — Aeneas scope)

- [ ] **M1-E-1** Define `LocalCtx`
  - [ ] M1-E-1.1 `push_fvar(name, type, value?) → FVarId`
  - [ ] M1-E-1.2 `lookup_fvar(id) → Option<LocalDecl>`
  - [ ] M1-E-1.3 Snapshot/restore interface (for elaborator backtracking)
- [ ] **M1-E-2** Implement `infer_type(env, ctx, expr) → Result<Expr, TypeError>`
  - [ ] M1-E-2.1 `Var(i)` → look up in `ctx` by de Bruijn index
  - [ ] M1-E-2.2 `Sort(l)` → `Sort(Succ(l))`
  - [ ] M1-E-2.3 `Const(n, levels)` → instantiate declaration type with `levels`
  - [ ] M1-E-2.4 `App(f, a)` → infer `f`, check it is `Pi`, check `a` against domain
  - [ ] M1-E-2.5 `Lam(bi, t, b)` → check `t : Sort`, infer `b` in extended ctx
  - [ ] M1-E-2.6 `Pi(bi, t, b)` → infer sorts of `t` and `b`, compute result sort
  - [ ] M1-E-2.7 `Let(t, v, b)` → check `v : t`, infer `b` in extended ctx
  - [ ] M1-E-2.8 `Lit(Nat(_))` → `Nat`
  - [ ] M1-E-2.9 `FVar(id)` → look up in `ctx` by `FVarId`
- [ ] **M1-E-3** Implement `check_type(env, ctx, expr, ty) → Result<(), TypeError>`
  - [ ] M1-E-3.1 Infer actual type
  - [ ] M1-E-3.2 Call `def_eq` on actual vs expected
  - [ ] M1-E-3.3 Error messages: include expected type, actual type, expression

### M1-F: Definitional Equality (Pure — Aeneas scope)

- [ ] **M1-F-1** Implement `def_eq_pure(env, ctx, t1, t2) → bool`
  - [ ] M1-F-1.1 Pointer (ExprId) equality fast path
  - [ ] M1-F-1.2 Reduce both to whnf, then dispatch on head form
  - [ ] M1-F-1.3 `Sort ~ Sort` → `level_equiv`
  - [ ] M1-F-1.4 `Const ~ Const` → name equality + all levels equiv
  - [ ] M1-F-1.5 `App ~ App` → unify function, then argument
  - [ ] M1-F-1.6 `Lam ~ Lam` → domain equality, extend ctx, body equality
  - [ ] M1-F-1.7 `Pi ~ Pi` → same as Lam
  - [ ] M1-F-1.8 `Lam ~ non-Lam` → eta expand non-Lam side, retry
  - [ ] M1-F-1.9 Proof irrelevance: two terms of `Prop` type are always equal
  - [ ] M1-F-1.10 Structural equality fallback after all reductions
  - [ ] M1-F-1.11 Property tests: reflexivity, symmetry
- [ ] **M1-F-2** Implement `def_eq` cache wrapper (not in Aeneas path)
  - [ ] M1-F-2.1 `DefEqCache`: `HashMap<(ExprId, ExprId), bool>`
  - [ ] M1-F-2.2 Lookup before calling `def_eq_pure`, store result after
  - [ ] M1-F-2.3 `debug_assert` coherence invariant in test builds
  - [ ] M1-F-2.4 Benchmark: cached vs uncached on Mathlib algebra corpus

### M1-G: Inductive Type Checking

- [ ] **M1-G-1** Implement inductive declaration validator
  - [ ] M1-G-1.1 Positivity check: no negative occurrences of inductive in constructors
  - [ ] M1-G-1.2 Universe level consistency check across all constructors
  - [ ] M1-G-1.3 Constructor type well-formedness
  - [ ] M1-G-1.4 Recursor type generation from constructor list
- [ ] **M1-G-2** Implement `Quot` axioms
  - [ ] M1-G-2.1 Register `Quot`, `Quot.mk`, `Quot.lift`, `Quot.ind`, `Quot.sound`
  - [ ] M1-G-2.2 Type-check each axiom against kernel rules
  - [ ] M1-G-2.3 Implement `Quot` reduction rules in `whnf`

### M1-H: Arena Testing

- [ ] **M1-H-1** Register Ferriprove in `lean-kernel-arena`
  - [ ] M1-H-1.1 Write `checkers/ferriprove.yml`
  - [ ] M1-H-1.2 Implement arena NDJSON input reader in `ferriprove-cli`
  - [ ] M1-H-1.3 Implement arena output writer
  - [ ] M1-H-1.4 Run all tutorial tests — all must pass
  - [ ] M1-H-1.5 Run full arena suite — record pass rate
- [ ] **M1-H-2** Mathlib corpus test
  - [ ] M1-H-2.1 Export Mathlib via `lean4export` (latest Mathlib commit)
  - [ ] M1-H-2.2 Run `ferriprove` typechecker on full export
  - [ ] M1-H-2.3 Any check failure is a bug — file issue, block release
  - [ ] M1-H-2.4 Record timing: total and per-declaration percentiles
  - [ ] M1-H-2.5 Gate: zero failures, total time within 2× C++ kernel

### M1-I: Aeneas Verification Loop

- [ ] **M1-I-1** Prepare pure kernel for Aeneas translation
  - [ ] M1-I-1.1 Audit all pure kernel functions: no `unsafe`, no `Mutex`, no `Arc` mutation
  - [ ] M1-I-1.2 All pure functions use only owned or `&` types
  - [ ] M1-I-1.3 Run Aeneas on `ferriprove-kernel` — collect translation errors
  - [ ] M1-I-1.4 Fix each translation error (document each fix in `proofs/NOTES.md`)
- [ ] **M1-I-2** Generate Lean functional models via Aeneas
  - [ ] M1-I-2.1 Translate `whnf` → `proofs/KernelModel.lean`
  - [ ] M1-I-2.2 Translate `def_eq_pure` → `proofs/KernelModel.lean`
  - [ ] M1-I-2.3 Translate `infer_type` → `proofs/KernelModel.lean`
  - [ ] M1-I-2.4 Confirm generated Lean compiles against Lean4Lean metatheory imports
- [ ] **M1-I-3** Prove soundness theorems in Lean
  - [ ] M1-I-3.1 `whnf_sound` — whnf preserves typing
  - [ ] M1-I-3.2 `def_eq_sound` — `def_eq_pure = true → ⊢ t1 ≡ t2`
  - [ ] M1-I-3.3 `infer_type_sound` — `infer_type = Ok(ty) → ⊢ e : ty`
  - [ ] M1-I-3.4 `def_eq_terminates` — well-founded on (term_size, reduction_depth)
  - [ ] M1-I-3.5 `cache_coherent` — cache hit implies `def_eq_pure` agrees
  - [ ] M1-I-3.6 File any open conjectures (e.g. completeness) as tracked issues
- [ ] **M1-I-4** Integrate Lean proofs into CI (M0-B-6 prerequisite)
  - [ ] M1-I-4.1 Proofs build cleanly in CI on every PR touching kernel or proof files
  - [ ] M1-I-4.2 Any proof regression blocks merge

---

## Milestone 2 — Elaborator
> `ferriprove-elab` elaborates a representative corpus of Lean 4 surface syntax.
> **Gate:** Output terms match Lean 4 on selected 50-file Mathlib corpus (100% agreement).
> **SemVer:** `v0.2.0` — `ferriprove-elab` crate published.

### M2-A: Metavariable Context

- [ ] **M2-A-1** Define `MetavarContext`
  - [ ] M2-A-1.1 `MetavarDecl`: `id`, `name`, `local_ctx`, `ty`, `kind`
  - [ ] M2-A-1.2 `kind` variants: `Natural`, `Synth`, `Tmp`
  - [ ] M2-A-1.3 `assignments: HashMap<MVarId, Expr>`
  - [ ] M2-A-1.4 `assign(id, expr) → Result<(), OccursError>` with occurs check
  - [ ] M2-A-1.5 `instantiate_mvars(expr) → Expr` — substitute all solved metavars
  - [ ] M2-A-1.6 Snapshot/restore for speculative elaboration and backtracking
- [ ] **M2-A-2** Define `ElabM` monad
  - [ ] M2-A-2.1 `StateT<ElabState, Result<T, ElabError>>`
  - [ ] M2-A-2.2 `ElabState`: `env`, `mctx`, `local_ctx`, `options`
  - [ ] M2-A-2.3 `with_mctx_snapshot(f)` — run speculatively, rollback on failure
  - [ ] M2-A-2.4 `ElabError` variants: `TypeMismatch`, `UnsolvedMVars`, `UnknownConst`, `SynthFailed`, etc.

### M2-B: Unification

- [ ] **M2-B-1** Structural unification
  - [ ] M2-B-1.1 Reduce both sides to whnf before comparing
  - [ ] M2-B-1.2 `MVar(id) ~ t` — occurs check, assign
  - [ ] M2-B-1.3 `App ~ App` — unify function then argument
  - [ ] M2-B-1.4 `Pi ~ Pi` — unify domain, extend ctx, unify body
  - [ ] M2-B-1.5 `Lam ~ Lam` — same structure as Pi
  - [ ] M2-B-1.6 `Sort ~ Sort` — delegate to `level_equiv`
  - [ ] M2-B-1.7 `Const ~ Const` — name equality + level unification
  - [ ] M2-B-1.8 Eta expansion: `f ~ λx. ?` — eta-expand `f` and retry
  - [ ] M2-B-1.9 Return `UnificationResult::Failed` (not error) on structural mismatch
- [ ] **M2-B-2** Universe level unification
  - [ ] M2-B-2.1 Level metavar assignment
  - [ ] M2-B-2.2 `l1 ≤ l2` constraint accumulation
  - [ ] M2-B-2.3 Level constraint solving post-elaboration
- [ ] **M2-B-3** Pattern higher-order unification
  - [ ] M2-B-3.1 Pattern check: `?f x1...xn ~ t` where all `xi` are distinct FVars
  - [ ] M2-B-3.2 Pattern unification: produce `λx1...xn. t'` where `t'` abstracts `xi`
  - [ ] M2-B-3.3 Document clearly: full HO unification is undecidable, not implemented
  - [ ] M2-B-3.4 Non-pattern flex-flex: postpone as deferred constraint

### M2-C: Implicit Argument Synthesis

- [ ] **M2-C-1** Implicit argument insertion
  - [ ] M2-C-1.1 On `App(f, a)`: check leading implicit `Pi`s in `f`'s type, insert fresh MVars
  - [ ] M2-C-1.2 `{}` strict implicit: insert only at application sites, never from named arg
  - [ ] M2-C-1.3 `[]` instance implicit: insert `Synth` MVar, trigger typeclass inference
  - [ ] M2-C-1.4 Named argument `(name := val)` — match by parameter name
- [ ] **M2-C-2** Typeclass inference (tabled resolution)
  - [ ] M2-C-2.1 Instance discrimination tree: index by head symbol of instance type
  - [ ] M2-C-2.2 `synth(ty)` memoization: table maps `ty → Option<Expr>` to break cycles
  - [ ] M2-C-2.3 Default instance support: lower priority, tried after all others
  - [ ] M2-C-2.4 `outParam` handling: determine which parameters drive search
  - [ ] M2-C-2.5 Failure error: "failed to synthesize instance" with inferred type display
  - [ ] M2-C-2.6 Test: diamond `Monoid → Semigroup` resolves without duplication

### M2-D: Term Elaboration

- [ ] **M2-D-1** Implement `elab_term(syn, expected_ty) → ElabM<Expr>`
  - [ ] M2-D-1.1 `Var` → resolve in `local_ctx` first, then global `env`
  - [ ] M2-D-1.2 `App(f, a)` → elab `f`, insert implicits, elab `a` against Pi domain
  - [ ] M2-D-1.3 `Lam` → elab binder type, push FVar, elab body
  - [ ] M2-D-1.4 `Pi` → elab binder, elab body, compute Sort
  - [ ] M2-D-1.5 `Let` → elab type, check value, push FVar with value, elab body
  - [ ] M2-D-1.6 `Sort(level)` → elab level, return `Sort(Succ(l))`
  - [ ] M2-D-1.7 Nat literal `42` → `OfNat.ofNat 42` via typeclass
  - [ ] M2-D-1.8 `_` (hole) → fresh `Natural` MVar with current ctx and expected type
  - [ ] M2-D-1.9 Coercion insertion: when expected type mismatches, try `CoeHTCoe`
- [ ] **M2-D-2** Implement `elab_command(cmd) → ElabM<()>`
  - [ ] M2-D-2.1 `def` / `theorem` → elab type, elab body, `check_type`, add to env
  - [ ] M2-D-2.2 `axiom` → elab type, add `Axiom` declaration to env, record in `SOUNDNESS.md` task
  - [ ] M2-D-2.3 `inductive` → delegate to inductive elaborator (M2-E)
  - [ ] M2-D-2.4 `structure` / `class` → desugar to inductive + projection definitions
  - [ ] M2-D-2.5 `instance` → elab as `def` with instance attribute, register in typeclass index
  - [ ] M2-D-2.6 `#check` → elab term, emit inferred type as diagnostic

### M2-E: Inductive Elaboration

- [ ] **M2-E-1** Inductive type elaborator
  - [ ] M2-E-1.1 Parse constructor types with parameters
  - [ ] M2-E-1.2 Auto-bound implicit variables in constructor signatures
  - [ ] M2-E-1.3 Generate recursor type from constructor list
  - [ ] M2-E-1.4 Generate `cases_on`, `rec`, `brecOn` variants
  - [ ] M2-E-1.5 Mutual inductive: validate positivity across all types jointly
- [ ] **M2-E-2** Pattern matching compilation
  - [ ] M2-E-2.1 Compile `match` expressions to recursor applications
  - [ ] M2-E-2.2 Exhaustiveness check: all constructors covered
  - [ ] M2-E-2.3 Dependent pattern matching: synthesize `motive`
  - [ ] M2-E-2.4 Inaccessible patterns `.(t)` — skip unification

### M2-F: Parser

- [ ] **M2-F-1** Lean 4 surface syntax lexer
  - [ ] M2-F-1.1 Implement lexer with `logos` crate
  - [ ] M2-F-1.2 Unicode identifiers: `α`, `β`, `∀`, `→`, `λ`, `⟨`, `⟩`, etc.
  - [ ] M2-F-1.3 Keywords: `def`, `theorem`, `let`, `fun`, `match`, `by`, `where`, `inductive`, `structure`, `class`, `instance`, `axiom`, `#check`
  - [ ] M2-F-1.4 Span tracking on every token
- [ ] **M2-F-2** Lean 4 surface syntax parser
  - [ ] M2-F-2.1 Pratt parser for expressions with precedence table
  - [ ] M2-F-2.2 Mixfix operator parsing (user-defined via `notation`)
  - [ ] M2-F-2.3 `do`-notation: basic monadic sugar desugaring
  - [ ] M2-F-2.4 `by` tactic block: parse as `TacticBlock(Vec<Tactic>)`, elaborate lazily
  - [ ] M2-F-2.5 Error recovery: continue after syntax error with error node in AST
  - [ ] M2-F-2.6 Every AST node carries `Span(file_id, start, end)`
- [ ] **M2-F-3** Minimal macro system
  - [ ] M2-F-3.1 `notation` command: fixed-precedence notation registration
  - [ ] M2-F-3.2 `macro_rules` basic pattern-matching macros
  - [ ] M2-F-3.3 Document explicitly: full hygienic macro system (Ullrich thesis Ch. 3) deferred to R-2

### M2-G: Elaboration Testing

- [ ] **M2-G-1** Build ground truth corpus
  - [ ] M2-G-1.1 Select 50 Lean 4 files from Mathlib: algebra, logic, Nat, List, Finset
  - [ ] M2-G-1.2 Export elaborated terms via `lean4export` (ground truth)
  - [ ] M2-G-1.3 Run `ferriprove-elab` on same source files
  - [ ] M2-G-1.4 Compare output terms structurally (modulo alpha-equivalence)
  - [ ] M2-G-1.5 Gate: 100% term agreement on all 50 files
- [ ] **M2-G-2** Golden test suite
  - [ ] M2-G-2.1 Store expected output in `tests/elab/golden/`
  - [ ] M2-G-2.2 Diff on every PR — any deviation requires explicit golden update
  - [ ] M2-G-2.3 Run in CI on every PR

---

## Milestone 3 — Core Tactics
> `ferriprove-tactic` verifies a representative tactic proof corpus.
> **Gate:** 1000 Mathlib tactic proofs verified without `sorry`.
> **SemVer:** `v0.3.0` — `ferriprove-tactic` crate published.

### M3-A: Tactic Monad

- [ ] **M3-A-1** Define `TacticM`
  - [ ] M3-A-1.1 `StateT<TacticState, ElabM<T>>`
  - [ ] M3-A-1.2 `TacticState`: `goals: Vec<Goal>`, `mctx: MetavarContext`
  - [ ] M3-A-1.3 `Goal`: `id: GoalId`, `ctx: LocalCtx`, `target: Expr`, `mvar: MVarId`
  - [ ] M3-A-1.4 `main_goal() → Result<Goal, NoGoals>`
  - [ ] M3-A-1.5 `replace_goal(new_goal)`
  - [ ] M3-A-1.6 `close_goal(term)` — assign term to goal MVar, remove goal
  - [ ] M3-A-1.7 `push_goals(new_goals: Vec<Goal>)`
  - [ ] M3-A-1.8 `all_goals_closed() → bool`

### M3-B: Core Tactics

- [ ] **M3-B-1** `exact`
  - [ ] M3-B-1.1 Elaborate term against goal type
  - [ ] M3-B-1.2 `def_eq` check: elaborated type matches goal
  - [ ] M3-B-1.3 Assign to goal MVar, close goal
- [ ] **M3-B-2** `intro`
  - [ ] M3-B-2.1 whnf goal, check it is `Pi`
  - [ ] M3-B-2.2 Push FVar for bound variable into `LocalCtx`
  - [ ] M3-B-2.3 Replace goal target with Pi body (FVar substituted)
  - [ ] M3-B-2.4 `intro h1 h2 h3` multi-intro
  - [ ] M3-B-2.5 `intro ⟨a, b⟩` destructuring (delegates to `cases` internally)
- [ ] **M3-B-3** `apply`
  - [ ] M3-B-3.1 Elaborate lemma term
  - [ ] M3-B-3.2 Create MVars for all premises
  - [ ] M3-B-3.3 Unify lemma conclusion with goal type
  - [ ] M3-B-3.4 Push unsolved premise MVars as new goals
- [ ] **M3-B-4** `cases`
  - [ ] M3-B-4.1 Resolve `cases_on` recursor for target type
  - [ ] M3-B-4.2 One new goal per constructor
  - [ ] M3-B-4.3 `cases h` where `h : T` is a local hypothesis
  - [ ] M3-B-4.4 `rcases` patterns: `⟨a, b⟩`, `h | h`
- [ ] **M3-B-5** `induction`
  - [ ] M3-B-5.1 Resolve recursor for induction target
  - [ ] M3-B-5.2 Generate base case + inductive step goals
  - [ ] M3-B-5.3 Named case syntax: `induction n with | zero => ... | succ n ih => ...`
- [ ] **M3-B-6** `rfl`
  - [ ] M3-B-6.1 whnf both sides of equality goal
  - [ ] M3-B-6.2 `def_eq` check, apply `Eq.refl`
- [ ] **M3-B-7** `rw`
  - [ ] M3-B-7.1 `rw [h]` — find occurrence of `h.lhs` in goal, replace with `h.rhs`
  - [ ] M3-B-7.2 `rw [← h]` — reverse direction
  - [ ] M3-B-7.3 `rw [h] at hyp` — rewrite in hypothesis
- [ ] **M3-B-8** `have`
  - [ ] M3-B-8.1 Create new goal for have-body type
  - [ ] M3-B-8.2 After proof, add as FVar in main goal ctx
- [ ] **M3-B-9** `constructor`
  - [ ] M3-B-9.1 whnf goal type, resolve inductive
  - [ ] M3-B-9.2 Try each constructor, commit on first that unifies with goal
- [ ] **M3-B-10** `assumption`
  - [ ] M3-B-10.1 Iterate `LocalCtx`, `def_eq` each hypothesis type against goal
  - [ ] M3-B-10.2 Close goal with first match
- [ ] **M3-B-11** `omega`
  - [ ] M3-B-11.1 Collect linear arithmetic hypotheses from `LocalCtx`
  - [ ] M3-B-11.2 Normalize to inequality system
  - [ ] M3-B-11.3 Run Omega decision procedure
  - [ ] M3-B-11.4 Produce proof term from Omega certificate
- [ ] **M3-B-12** `decide`
  - [ ] M3-B-12.1 Synthesize `Decidable` instance for goal type
  - [ ] M3-B-12.2 Evaluate decision procedure
  - [ ] M3-B-12.3 Apply `isTrue` or fail with counterexample

### M3-C: Tactic Testing

- [ ] **M3-C-1** Unit tests per tactic
  - [ ] M3-C-1.1 Minimal closed proof for each tactic in isolation
  - [ ] M3-C-1.2 Error case: wrong type, unsolvable goal — verify error message
- [ ] **M3-C-2** Mathlib corpus
  - [ ] M3-C-2.1 Extract 1000 theorems from Mathlib using only M3 tactics (no `simp`)
  - [ ] M3-C-2.2 Run `ferriprove`, compare proof terms against Lean 4 output
  - [ ] M3-C-2.3 Gate: all 1000 accepted, zero `sorry`

---

## Milestone 4 — `simp`
> Ferriprove `simp` is correct and within 2× Lean 4 performance on Mathlib corpus.
> **Gate:** 100% output correctness. Median latency within 2× Lean 4 on 100K call corpus.
> **SemVer:** `v0.4.0`

### M4-A: Corpus Extraction

- [ ] **M4-A-1** Instrument Lean 4 `simp`
  - [ ] M4-A-1.1 Write custom `simp` trace elaborator in Lean 4
  - [ ] M4-A-1.2 Output format: NDJSON `{input, simp_set, output, time_ns}`
  - [ ] M4-A-1.3 Run on Mathlib — collect minimum 100K calls
  - [ ] M4-A-1.4 Store corpus compressed in `benches/simp_corpus/`
  - [ ] M4-A-1.5 Analyze: distribution of term sizes, simp set sizes, unique lemmas

### M4-B: Discrimination Tree

- [ ] **M4-B-1** Implement discrimination tree (E-matching)
  - [ ] M4-B-1.1 Node types: `Root`, `App`, `Const(name)`, `Var(i)`, `Star`
  - [ ] M4-B-1.2 `insert(lhs_pattern, lemma_id)`
  - [ ] M4-B-1.3 `get_candidates(expr) → Vec<LemmaId>`
  - [ ] M4-B-1.4 Full verification of candidates against term after retrieval
  - [ ] M4-B-1.5 Benchmark: lookup time on 10K-lemma set from Mathlib

### M4-C: Congruence Closure

- [ ] **M4-C-1** Implement Nieuwenhuis-Oliveras congruence closure
  - [ ] M4-C-1.1 Union-find with path compression and rank
  - [ ] M4-C-1.2 Explanation tracking per `merge` for proof extraction
  - [ ] M4-C-1.3 `merge(t1, t2, proof)` — assert `t1 = t2` with justification
  - [ ] M4-C-1.4 `are_equal(t1, t2) → bool`
  - [ ] M4-C-1.5 `explain(t1, t2) → Option<ProofTerm>`
  - [ ] M4-C-1.6 Congruence rule: `f = g → a = b → f a = g b`
  - [ ] M4-C-1.7 Test: diamond equality chains, transitivity chains

### M4-D: Simp Rewrite Loop

- [ ] **M4-D-1** Main rewrite loop
  - [ ] M4-D-1.1 Bottom-up traversal: simplify subterms before head
  - [ ] M4-D-1.2 Retrieve candidates from discrimination tree
  - [ ] M4-D-1.3 Unify lemma LHS with subterm
  - [ ] M4-D-1.4 Apply substitution, produce `Eq.mpr`-based proof term
  - [ ] M4-D-1.5 Fixpoint: repeat until no candidate applies
  - [ ] M4-D-1.6 `simp only [h1, h2]` — restricted lemma set
  - [ ] M4-D-1.7 `simp [*]` — include all local hypotheses
  - [ ] M4-D-1.8 `simp at h` — simplify a hypothesis
  - [ ] M4-D-1.9 `simp_all` — simplify all goals and hypotheses
- [ ] **M4-D-2** Simp lemma priority system
  - [ ] M4-D-2.1 `@[simp]` attribute with default priority
  - [ ] M4-D-2.2 `@[simp high]` / `@[simp low]` priority overrides
  - [ ] M4-D-2.3 Application order: local hypotheses > explicit list > tagged lemmas

### M4-E: Benchmarking

- [ ] **M4-E-1** Benchmark against extracted corpus
  - [ ] M4-E-1.1 Run Ferriprove `simp` on all 100K corpus calls
  - [ ] M4-E-1.2 Correctness: output term structurally matches Lean 4 on 100% of calls
  - [ ] M4-E-1.3 Performance: median call latency within 2× Lean 4
  - [ ] M4-E-1.4 Profile slowest 1% percentile — identify bottlenecks
  - [ ] M4-E-1.5 Optimize until gate is met
  - [ ] M4-E-1.6 Store final baseline in `benches/baselines/simp.json`

---

## Milestone 5 — LSP + Developer Experience
> Ferriprove is usable as a VS Code extension with live goal display.
> **Gate:** Goal state updates within 500ms on file save on a Mathlib file (8-core machine).
> **SemVer:** `v0.5.0` — first public usable release, VS Code extension published.

### M5-A: Language Server

- [ ] **M5-A-1** Implement LSP server (`tower-lsp`)
  - [ ] M5-A-1.1 `initialize` / `initialized` handshake
  - [ ] M5-A-1.2 `textDocument/didOpen` → parse + elaborate + diagnostics
  - [ ] M5-A-1.3 `textDocument/didChange` → incremental re-elaboration
  - [ ] M5-A-1.4 `textDocument/didSave` → full re-check
  - [ ] M5-A-1.5 `textDocument/publishDiagnostics` → type errors, unsolved goals
  - [ ] M5-A-1.6 `textDocument/hover` → inferred type at cursor position
  - [ ] M5-A-1.7 Custom notification `ferriprove/goalState` → current tactic goals at cursor
- [ ] **M5-A-2** Incremental elaboration
  - [ ] M5-A-2.1 Build declaration dependency graph on first load
  - [ ] M5-A-2.2 On edit: invalidate changed declaration + all transitive dependents
  - [ ] M5-A-2.3 Re-elaborate only invalidated set
  - [ ] M5-A-2.4 Cache valid elaborated terms across edits
  - [ ] M5-A-2.5 Benchmark: re-elaboration latency on a 1000-line Mathlib file
- [ ] **M5-A-3** Parallel elaboration
  - [ ] M5-A-3.1 Elaborate independent declarations in parallel via `rayon`
  - [ ] M5-A-3.2 Serialize environment writes (single writer, concurrent readers)
  - [ ] M5-A-3.3 Benchmark: parallel speedup on 8-core vs single-threaded

### M5-B: VS Code Extension

- [ ] **M5-B-1** Scaffold extension
  - [ ] M5-B-1.1 `npm init` in `editors/vscode/`
  - [ ] M5-B-1.2 Launch Ferriprove LSP server as child process on extension activate
  - [ ] M5-B-1.3 Wire `vscode-languageclient` to server
  - [ ] M5-B-1.4 Syntax highlighting: reuse Lean 4 TextMate grammar (`leanprover.lean4`)
  - [ ] M5-B-1.5 Infoview panel: webview displaying `ferriprove/goalState` notification
  - [ ] M5-B-1.6 Publish to VS Code marketplace (`vsce publish`)

### M5-C: Documentation

- [ ] **M5-C-1** User documentation
  - [ ] M5-C-1.1 Installation: `cargo install ferriprove`
  - [ ] M5-C-1.2 VS Code extension setup guide
  - [ ] M5-C-1.3 Supported Lean 4 feature list (explicit: what works, what doesn't)
  - [ ] M5-C-1.4 Known deviations from Lean 4 (by design or open bug)
- [ ] **M5-C-2** Developer documentation
  - [ ] M5-C-2.1 Kernel internals: how to add a reduction rule
  - [ ] M5-C-2.2 Elaborator internals: how to add an elaboration rule
  - [ ] M5-C-2.3 How to add a tactic (step-by-step with example)
  - [ ] M5-C-2.4 Aeneas proof loop: how to update proofs after kernel change
  - [ ] M5-C-2.5 Confirm `docs.rs/ferriprove-kernel` renders correctly

---

## Cross-Cutting: DevSecOps
> Applies continuously across all milestones.

- [ ] **SEC-1** Dependency hygiene
  - [ ] SEC-1.1 `cargo deny` policy: no GPL in transitive deps unless explicitly waived
  - [ ] SEC-1.2 `cargo audit` in CI: block merge on any active RUSTSEC advisory
  - [ ] SEC-1.3 Dependabot: weekly patch PRs, monthly minor PRs
  - [ ] SEC-1.4 Pin all GitHub Actions to exact commit SHA
- [ ] **SEC-2** Fuzzing
  - [ ] SEC-2.1 Fuzz `lean4export` parser with `cargo-fuzz` (libFuzzer)
  - [ ] SEC-2.2 Fuzz kernel with randomly generated `Expr` trees
  - [ ] SEC-2.3 Run 60 seconds per fuzz target in nightly CI
  - [ ] SEC-2.4 Any panic in `ferriprove-kernel` is a P0 bug — block all releases
- [ ] **SEC-3** Unsoundness tracking
  - [ ] SEC-3.1 `SOUNDNESS.md`: maintain explicit list of all soundness assumptions
  - [ ] SEC-3.2 Any new axiom in kernel requires entry in `SOUNDNESS.md` before merge
  - [ ] SEC-3.3 Issues tagged `soundness` triaged within 24h
- [ ] **SEC-4** Release integrity
  - [ ] SEC-4.1 Sign binaries with `cosign`
  - [ ] SEC-4.2 Publish SBOM (CycloneDX format) per release
  - [ ] SEC-4.3 `crates.io` publish uses CI token only — no personal token in repo

---

## SemVer Release Map

| Version | Milestone | Gate |
|---|---|---|
| `v0.1.0` | M1 | Kernel parity. Mathlib typechecks. Soundness proved in Lean. |
| `v0.2.0` | M2 | Elaborator. 50-file Mathlib corpus matches Lean 4 output. |
| `v0.3.0` | M3 | Core tactics. 1000 Mathlib proofs verified. |
| `v0.4.0` | M4 | `simp`. 100% correctness + within 2× Lean 4 timing. |
| `v0.5.0` | M5 | LSP + VS Code extension. Goal state within 500ms. |
| `v1.0.0` | — | Public API stable. Full Mathlib tactic coverage. |

---

## Open Research Items
> Not scheduled. Tracked as GitHub issues with label `research`.

- [ ] **R-1** Full completeness proof for `def_eq` — requires Lean type theory confluence proof, open in Lean4Lean
- [ ] **R-2** Full hygienic macro system — Ullrich PhD thesis Chapter 3, significant scope
- [ ] **R-3** `native_decide` / `reduceBool` full integration — requires Lean IR interpreter
- [ ] **R-4** NbE (Normalization by Evaluation) kernel — Carneiro flags as risky without prior verification; Ferriprove's proof loop is the prerequisite
- [ ] **R-5** Cross-compile Ferriprove to WASM for browser-based proof checking

---

*Last updated: 2026-04-08 | Stage: pre-M0*
