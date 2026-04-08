# Ferriprove — Architecture

---

## Layer Overview

```
┌──────────────────────────────────────────────────────┐
│                  ferriprove-lsp                       │  LSP server, VS Code infoview
├──────────────────────────────────────────────────────┤
│                 ferriprove-tactic                     │  TacticM monad, intro/apply/simp/...
├──────────────────────────────────────────────────────┤
│                  ferriprove-elab                      │  Elaborator, unification, typeclass
├──────────────────────────────────────────────────────┤
│                 ferriprove-kernel                     │  Trusted kernel: def_eq, infer, whnf
├──────────────────────────────────────────────────────┤
│                 ferriprove-export                     │  lean4export NDJSON parser
├──────────────────────────────────────────────────────┤
│                  ferriprove-types                     │  Expr, Level, Name, BinderInfo
└──────────────────────────────────────────────────────┘
```

**Dependency graph (acyclic, enforced by Cargo):**

```
ferriprove-types
      ↓
ferriprove-export
      ↓
ferriprove-kernel
      ↓
ferriprove-elab
      ↓
ferriprove-tactic
    ↓       ↓
ferriprove-lsp  ferriprove-cli
```

**Trust boundary:** Only `ferriprove-kernel` is trusted. Everything above it can be buggy without compromising soundness — a faulty elaborator can only fail to produce a kernel-checkable term, not produce a false one. Soundness is a property of the kernel alone.

---

## Trust Model

| Layer | Trusted? | Consequence of bug |
|---|---|---|
| `ferriprove-types` | ✅ Yes | Type corruption — P0, kernel depends on it |
| `ferriprove-export` | ❌ No | Parse failure, not false proof |
| `ferriprove-kernel` | ✅ Yes | Soundness hole — P0 |
| `ferriprove-elab` | ❌ No | Elaboration failure, not false proof |
| `ferriprove-tactic` | ❌ No | Tactic failure, not false proof |
| `ferriprove-lsp` | ❌ No | UI error, not false proof |
| Aeneas-generated Lean model | ✅ Yes (proof artifact) | Separate from runtime trust |

The kernel's trust is independently verified: Ferriprove's pure kernel functions are translated to Lean via Aeneas, and soundness is proved against the Lean4Lean metatheory. This creates a bootstrap verification loop where Lean certifies the Rust kernel that serves as an independent checker for Lean.

---

## Term Representation

**Choice: Locally nameless** (FVar for free variables, de Bruijn indices for bound variables).

Rationale: Lean 4's own kernel uses locally nameless. `nanoda_lib` uses it. Matching this convention simplifies reading `lean4export` format and cross-referencing against Lean 4 source.

```rust
pub enum Expr {
    Var(usize),                             // de Bruijn index (bound)
    Sort(Level),                            // Prop | Type u
    Const(Name, Vec<Level>),                // global declaration
    App(Arc<Expr>, Arc<Expr>),              // f a
    Lam(BinderInfo, Arc<Expr>, Arc<Expr>),  // fun (x : α) => body
    Pi(BinderInfo, Arc<Expr>, Arc<Expr>),   // (x : α) → β
    Let(Arc<Expr>, Arc<Expr>, Arc<Expr>),   // let x : α := v; body
    Lit(Literal),                           // nat / string literals
    FVar(FVarId),                           // free variable (local ctx)
    MVar(MVarId),                           // metavariable (elaborator only)
}
```

Bound variable alternative (de Bruijn indices only) was considered and rejected: pure de Bruijn requires re-indexing on every substitution into an open term, which is expensive in the elaborator's metavar context. Locally nameless avoids this at the cost of an `abstract_fvars` step at lambda introduction.

---

## Memory Model

### Term Interning (kernel)

Terms in the kernel are hash-consed by structural content. Every unique term has a single allocation and a stable `ExprId`. This enables:

- **O(1) pointer-equality fast path** in `def_eq`: if two `ExprId`s are identical, the terms are definitionally equal without reduction
- **`def_eq` cache keyed by `(ExprId, ExprId)`**: avoids re-checking structurally identical subterms

Implementation: `bumpalo` arena for allocations, `dashmap` for the intern table (concurrent reads during parallel elaboration).

### Cache Architecture (kernel)

The `def_eq` cache is architecturally separated from the pure kernel:

```
def_eq_pure(env, ctx, t1, t2) → bool   ← pure, Aeneas-translatable, proved correct
        ↑
def_eq(env, ctx, t1, t2) → bool        ← wraps pure with cache lookup/store
```

**Cache coherence invariant:** `cache[(t1, t2)] = true → def_eq_pure(t1, t2) = true`

This invariant is stated and proved separately in Lean (not as part of the soundness proof for `def_eq_pure`). The two-layer design is the standard refinement pattern — pure function proved correct, imperative optimization argued sound separately. Lean4Lean uses an identical approach for its own monadic kernel.

### Elaborator State

The elaborator requires mutable metavar context alongside read-only environment access. This does not go through Aeneas — the elaborator is not in the trusted base.

```rust
struct ElabState {
    env: Arc<Environment>,       // read-only, shared
    mctx: MetavarContext,        // mutable, owned per elaboration session
    local_ctx: LocalCtx,         // mutable, stack-structured
    options: ElabOptions,
}
```

---

## Aeneas Verification Strategy

The kernel's pure functions are verified using [Aeneas](https://github.com/AeneasVerif/aeneas): a tool that translates Rust programs to Lean 4 purely functional models, which can then be proved correct.

### What goes through Aeneas

Only pure kernel functions — those with no `Mutex`, `Arc` mutation, or `unsafe`:

- `whnf(env, expr) → Expr`
- `def_eq_pure(env, ctx, t1, t2) → bool`
- `infer_type(env, ctx, expr) → Result<Expr, TypeError>`
- `level_equiv(l1, l2, params) → bool`

### What does not go through Aeneas

- `def_eq` (the cached wrapper) — argued correct by cache coherence proof separately
- The elaborator (`ferriprove-elab`) — not in trust base
- The tactic engine — not in trust base

### Proof targets

```lean
-- Whnf preserves typing
theorem whnf_sound : ∀ env ctx e ty,
    infer_type env ctx e = Ok ty →
    infer_type env ctx (whnf env e) = Ok ty

-- Definitional equality soundness
theorem def_eq_sound : ∀ env ctx t1 t2,
    def_eq_pure env ctx t1 t2 = true → env ⊢ t1 ≡ t2

-- Type inference soundness
theorem infer_type_sound : ∀ env ctx e ty,
    infer_type env ctx e = Ok ty → env ⊢ e : ty

-- Termination (well-founded on term size × reduction depth)
theorem def_eq_terminates : ∀ env ctx t1 t2,
    ∃ r, def_eq_pure env ctx t1 t2 = r

-- Cache coherence (separate from above)
theorem cache_coherent : ∀ cache t1 t2,
    cache.lookup t1 t2 = some true →
    def_eq_pure env ctx t1 t2 = true
```

Completeness (`def_eq_pure = true ← ⊢ t1 ≡ t2`) is deferred — this requires a confluence proof for Lean's reduction, which is an open conjecture even in Lean4Lean.

---

## Reduction Strategy

`whnf` (weak head normal form) applies reductions in this order:

1. **Beta** — `(λx. e) a → e[x := a]`
2. **Delta** — unfold `Definition` by transparency level
3. **Iota** — recursor applied to constructor
4. **Zeta** — `let x := v; b → b[x := v]`
5. **Eta** — `λx. f x → f` (when `x ∉ fv(f)`)
6. **Nat/String extensions** — literal arithmetic reduction
7. **`reduceBool`** — native decide (flag-gated, not in Aeneas-verified path)

Full reduction (`full_reduce`) is only used in `simp` and `decide`. The kernel uses only `whnf`.

---

## Elaborator Design

The elaborator converts surface syntax to kernel terms, filling in implicit arguments and resolving typeclass instances. It is **not trusted** — bugs here produce elaboration failures, not unsound proofs.

```
Surface syntax (parser)
        ↓
  elab_term / elab_command
        ↓
  Implicit arg insertion      ← MetavarContext
        ↓
  Typeclass inference          ← Tabled resolution (Selsam/Ullrich/de Moura 2020)
        ↓
  Unification                  ← Structural + pattern HO unification
        ↓
  Kernel term (Expr)           → ferriprove-kernel for checking
```

### Metavar Context

Metavariables represent holes filled during elaboration. Three kinds:

- `Natural` — proof terms, filled by unification or tactics
- `Synth` — typeclass instance holes, filled by tabled resolution
- `Tmp` — temporary unification variables, always solved before elaboration completes

### Unification

Structural unification with these special cases:

- `MVar ~ t` — occurs check, then assign
- Eta expansion — `f ~ λx. ?` retried with eta-expanded `f`
- Pattern HO unification — `?f x1...xn ~ t` where `xi` are distinct FVars (decidable subset)
- Full higher-order unification is undecidable — non-pattern flex-flex constraints are postponed

### Typeclass Inference

Tabled resolution (per Selsam/Ullrich/de Moura 2020):

- Memoize `synth(ty)` to break diamond and cycle problems
- Instance candidates retrieved from discrimination tree by head symbol
- `outParam` determines search direction
- Default instances have lower priority, tried after all others

---

## Tactic Engine

Tactics run in `TacticM`, a monad over the elaborator state:

```rust
type TacticM<T> = StateT<TacticState, ElabM<Result<T, TacticError>>>;

struct TacticState {
    goals: Vec<Goal>,
    mctx: MetavarContext,   // shared with elaborator
}

struct Goal {
    id: GoalId,
    ctx: LocalCtx,
    target: Expr,           // what needs to be proved
    mvar: MVarId,           // the hole this goal fills
}
```

Tactics produce proof terms assigned to goal MVars. When all goals are closed and all MVars solved, the tactic block produces a complete kernel term which is then checked by `ferriprove-kernel`.

---

## `simp` Architecture

`simp` is implemented as three independent components:

```
Discrimination tree      ← E-matching for rewrite candidate retrieval
        +
Congruence closure       ← Nieuwenhuis-Oliveras 2005, with proof extraction
        +
Rewrite loop             ← Bottom-up, fixpoint, priority-ordered
```

The discrimination tree is benchmarked independently against the simp call corpus extracted from Lean 4 / Mathlib instrumentation. Correctness gate: output term must match Lean 4 on 100% of corpus calls.

---

## LSP Architecture

```
ferriprove-lsp (tower-lsp)
        ├── textDocument/didOpen → parse → elab → diagnostics
        ├── textDocument/didChange → incremental re-elab (dependency graph)
        ├── textDocument/hover → inferred type at cursor
        └── custom notification → goal state for infoview
```

Incremental elaboration: declarations are nodes in a dependency graph. On edit, only changed nodes and their transitive dependents are re-elaborated. Independent declarations are elaborated in parallel via Rayon.

---

## Workspace Structure (planned)

```
ferriprove/
├── Cargo.toml                    (workspace)
├── rust-toolchain.toml
├── .cargo/config.toml
├── ferriprove-types/
│   ├── src/
│   │   ├── expr.rs               (Expr, Level, BinderInfo, Literal)
│   │   ├── name.rs               (Name — hierarchical, interned)
│   │   ├── level.rs              (Level operations)
│   │   └── lib.rs
│   └── tests/
├── ferriprove-export/
│   ├── src/
│   │   ├── parser.rs             (lean4export NDJSON parser)
│   │   ├── reconstruct.rs        (Expr tree reconstruction)
│   │   └── lib.rs
│   ├── fuzz/                     (cargo-fuzz targets)
│   └── tests/
├── ferriprove-kernel/
│   ├── src/
│   │   ├── env.rs                (Environment, Declaration)
│   │   ├── reduce.rs             (whnf, full_reduce)
│   │   ├── infer.rs              (infer_type, check_type)
│   │   ├── def_eq.rs             (def_eq_pure, def_eq with cache)
│   │   ├── inductive.rs          (inductive validator, recursor gen)
│   │   └── lib.rs
│   ├── benches/
│   │   ├── def_eq.rs
│   │   └── baselines/
│   └── tests/
├── ferriprove-elab/
│   ├── src/
│   │   ├── mctx.rs               (MetavarContext)
│   │   ├── unify.rs              (unification)
│   │   ├── typeclass.rs          (tabled resolution)
│   │   ├── elab.rs               (elab_term, elab_command)
│   │   ├── inductive_elab.rs
│   │   └── lib.rs
│   └── tests/
├── ferriprove-tactic/
│   ├── src/
│   │   ├── monad.rs              (TacticM, Goal)
│   │   ├── basic.rs              (exact, intro, apply, rfl, assumption)
│   │   ├── cases.rs              (cases, induction, rcases)
│   │   ├── rewrite.rs            (rw, simp)
│   │   ├── simp/
│   │   │   ├── discr_tree.rs
│   │   │   ├── congr_closure.rs
│   │   │   └── rewrite_loop.rs
│   │   ├── omega.rs
│   │   └── lib.rs
│   └── tests/
├── ferriprove-lsp/
│   └── src/
│       ├── server.rs
│       ├── incremental.rs
│       └── lib.rs
├── ferriprove-cli/
│   └── src/main.rs
├── editors/
│   └── vscode/
├── benches/
│   └── simp_corpus/
├── proofs/                       (Lean files: Aeneas output + soundness proofs)
│   ├── KernelModel.lean
│   ├── Soundness.lean
│   └── CacheCoherence.lean
└── docs/
    ├── README.md
    ├── ARCHITECTURE.md           (this file)
    ├── TODO.md
    ├── CONTRIBUTING.md
    ├── SECURITY.md
    ├── SOUNDNESS.md
    ├── GOVERNANCE.md
    ├── AUDIT.md                  (nanoda_lib audit, filled in M0)
    ├── NOTICE
    └── LICENSE
```

---

## Reference Papers

| Paper | Year | Relevance |
|---|---|---|
| de Moura et al., "The Lean Theorem Prover" (CADE-25) | 2015 | Kernel + elaborator spec |
| de Moura et al., "Elaboration in Dependent Type Theory" | 2015 | Metavar/unification algorithm |
| Ullrich, de Moura, "Beyond Notations: Hygienic Macro Expansion" (IJCAR) | 2020 | Macro system design |
| Selsam, Ullrich, de Moura, "Tabled Typeclass Resolution" | 2020 | Typeclass inference algorithm |
| de Moura et al., "Functional But In-Place" (FBIP) | 2020 | Memory model (maps to Rust ownership) |
| de Moura, Ullrich, "The Lean 4 Theorem Prover" (CADE-28) | 2021 | System overview |
| Ullrich, "An Extensible Theorem Proving Frontend" (PhD thesis) | 2023 | Complete frontend blueprint |
| Carneiro, "Lean4Lean" (arXiv 2403.14064) | 2024/2025 | Proof oracle, soundness bug precedent |
| Nieuwenhuis, Oliveras, "Fast Congruence Closure" | 2005 | `simp` congruence closure algorithm |
| Ho et al., "Aeneas: Rust Verification by Functional Translation" (ICFP) | 2022 | Aeneas translation tool |

---

*Last updated: 2026-04-08 | Stage: pre-M0*
