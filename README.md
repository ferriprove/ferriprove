# Ferriprove

> A Lean 4-compatible interactive theorem prover and type checker, implemented in Rust.
> Kernel-first. Elaborator-complete. Formally verified against Lean's own metatheory.

---

## What is Ferriprove?

Ferriprove is a ground-up Rust implementation of the Lean 4 type theory stack — from the trusted kernel through elaboration, tactics, and language server. It is designed to be:

- **Correct by construction** — the kernel is formally verified for soundness using [Lean4Lean](https://github.com/digama0/lean4lean) as the proof oracle
- **Lean 4-compatible** — reads `.lean4export` format, targets Lean 4 kernel language semantics
- **Performance-competitive** — benchmarked against the reference C++ kernel and Lean 4's `simp` on the full Mathlib corpus
- **Independent** — a truly external checker with no shared trust assumptions with Lean's C++ runtime

## Relationship to Existing Work

| Project | Role in Ferriprove |
|---|---|
| [`leanprover/lean4`](https://github.com/leanprover/lean4) | Reference implementation (C++). Defines the target semantics. |
| [`ammkrn/nanoda_lib`](https://github.com/ammkrn/nanoda_lib) | Existing Rust kernel checker. Audited as prior art in M0. |
| [`digama0/lean4lean`](https://github.com/digama0/lean4lean) | Lean kernel in Lean. Proof oracle for Ferriprove's soundness proofs. |
| [`leanprover/lean-kernel-arena`](https://github.com/leanprover/lean-kernel-arena) | Official test arena. Ferriprove is registered as a checker. |
| [Aeneas](https://github.com/AeneasVerif/aeneas) | Translates Ferriprove's pure Rust kernel to Lean for formal verification. |

## Crate Structure

```
ferriprove/                  (workspace root)
├── ferriprove-types/        (shared types: Expr, Level, Name, BinderInfo)
├── ferriprove-export/       (lean4export NDJSON parser)
├── ferriprove-kernel/       (trusted type checker — published to crates.io)
├── ferriprove-elab/         (elaborator — implicit args, unification, typeclass)
├── ferriprove-tactic/       (tactic engine — intro, apply, cases, simp, ...)
├── ferriprove-lsp/          (Language Server Protocol server)
├── ferriprove-cli/          (binary — published as `ferriprove` on crates.io)
└── editors/vscode/          (VS Code extension)
```

## Status

| Milestone | Version | Status |
|---|---|---|
| M0: Foundation + audit | pre-release | 🔲 Not started |
| M1: Kernel parity + verification | `v0.1.0` | 🔲 Not started |
| M2: Elaborator | `v0.2.0` | 🔲 Not started |
| M3: Core tactics | `v0.3.0` | 🔲 Not started |
| M4: `simp` | `v0.4.0` | 🔲 Not started |
| M5: LSP + VS Code | `v0.5.0` | 🔲 Not started |

## Building

```bash
# Requires: Rust stable (see rust-toolchain.toml for pinned version)
git clone https://github.com/YOUR_ORG/ferriprove
cd ferriprove
cargo build --release

# Run the kernel checker on a lean4export file
./target/release/ferriprove check path/to/export.ndjson

# Run all tests
cargo test --workspace

# Run benchmarks
cargo bench --workspace
```

## Install

```bash
cargo install ferriprove
```

## Documentation

- [ARCHITECTURE.md](./ARCHITECTURE.md) — layer design, trust boundary, memory model, Aeneas strategy
- [TODO.md](./TODO.md) — full atomic task list with milestones and SemVer gates
- [CONTRIBUTING.md](./docs/CONTRIBUTING.md) — code style, Conventional Commits, PR process, issue templates
- [SECURITY.md](./docs/SECURITY.md) — trust model, soundness bug policy, disclosure process
- [SOUNDNESS.md](./docs/SOUNDNESS.md) — list of all soundness assumptions (updated per kernel change)
- [GOVERNANCE.md](./docs/GOVERNANCE.md) — project leadership, decision making, license rationale
- [AUDIT.md](./docs/AUDIT.md) — nanoda_lib audit framework and results
- [CHANGELOG.md](./docs/CHANGELOG.md) — release history and future plans
- [docs.rs/ferriprove-kernel](https://docs.rs/ferriprove-kernel) — API documentation

## Reference Papers

See [ARCHITECTURE.md § Reference Papers](./ARCHITECTURE.md#reference-papers) for the full annotated list.

## License

> **License is decided before any code is written.** See [TODO.md § License Decision](./TODO.md#license-decision) for the decision task and [LICENSE](./LICENSE) once resolved.

Candidate: MIT OR Apache-2.0 (Rust convention, compatible with Lean 4's Apache-2.0 and nanoda_lib's MIT).

## Project Identity

| Field | Value |
|---|---|
| Name | `ferriprove` |
| Tagline | Lean-compatible ITP kernel and elaborator in Rust |
| Rust crates (lib) | `ferriprove-types`, `ferriprove-export`, `ferriprove-kernel`, `ferriprove-elab`, `ferriprove-tactic`, `ferriprove-lsp` |
| Rust crate (bin) | `ferriprove` (published), `ferriprove-cli` (workspace name) |
| Target | Lean 4 kernel language (`.lean4export` format) |
| Reference impl | `leanprover/lean4` (C++), `ammkrn/nanoda_lib` (Rust kernel) |
| Proof oracle | `digama0/lean4lean` (Lean4Lean metatheory) |
| Test corpus | `leanprover/lean-kernel-arena` |
| SemVer baseline | `v0.1.0` at kernel parity |

---

*Last updated: 2026-04-08 | Stage: pre-M0*
