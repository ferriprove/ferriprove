# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Foundation setup and project structure
- Cargo workspace with 7 crates
- Governance and documentation framework
- Branch protection and CI/CD preparation

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.0.1] - 2026-04-08

### Added
- Initial project scaffold
- README.md with project overview
- ARCHITECTURE.md with technical specifications
- TODO.md with comprehensive milestone planning
- .gitignore for Rust development
- CODEOWNERS with maintainer assignment
- LICENSE (MIT OR Apache-2.0 dual license)
- NOTICE file with attributions
- DOMAIN.md configuration documentation
- Placeholder crates for all 7 ferriprove packages

### Infrastructure
- GitHub repository creation
- Branch protection configuration
- crates.io namespace reservation
- Initial documentation structure

---

## Future Releases

### [0.1.0] - Milestone 1: Kernel Parity
*Planned for kernel compatibility with Lean 4*

- ferriprove-types: Core type definitions
- ferriprove-export: Lean export format parser
- ferriprove-kernel: Trusted type checking kernel
- Arena test compatibility
- Performance benchmarks
- Aeneas verification setup

### [0.2.0] - Milestone 2: Elaborator
*Planned for type inference and elaboration*

- ferriprove-elab: Elaboration engine
- Implicit argument inference
- Type class resolution
- Unification algorithm
- Error reporting improvements

### [0.3.0] - Milestone 3: Core Tactics
*Planned for basic tactic support*

- ferriprove-tactic: Tactic engine
- Basic tactics (intro, apply, cases)
- Tactic state management
- Proof script parsing

### [0.4.0] - Milestone 4: Simplifier
*Planned for simplification support*

- Simplifier implementation
- Rewrite rules database
- Congruence closure
- Performance optimizations

### [0.5.0] - Milestone 5: LSP + VS Code
*Planned for IDE integration*

- ferriprove-lsp: Language Server Protocol
- ferriprove-cli: Enhanced command line interface
- VS Code extension
- Documentation website
- Public release

---

## Version Policy

### Pre-1.0 Releases
- Breaking changes may occur in any release
- API stability not guaranteed
- Focus on feature completeness
- Performance may vary significantly

### Post-1.0 Releases
- Semantic versioning strictly followed
- Breaking changes only in major releases
- Backward compatibility maintained
- Performance improvements in minor releases

### Release Cadence
- Major releases: As needed for breaking changes
- Minor releases: Feature-complete milestones
- Patch releases: Bug fixes and security updates

---

## Categories

### Added
- New features
- New crates or modules
- New API endpoints
- Documentation improvements

### Changed
- Changes in existing functionality
- Performance improvements
- API modifications (backward compatible)
- Configuration changes

### Deprecated
- Features that will be removed in future
- API changes scheduled for removal
- Configuration options being replaced

### Removed
- Features removed in this release
- Deprecated features now removed
- API endpoints removed

### Fixed
- Bug fixes
- Security fixes
- Performance regressions
- Documentation fixes

### Security
- Security vulnerabilities
- Security improvements
- Dependency updates for security

---

## Contributing to Changelog

This changelog is maintained using [git-cliff](https://github.com/orhun/git-cliff).

### For Contributors
- Use conventional commit messages
- Include issue numbers in commits
- Document breaking changes
- Add security notes for security fixes

### For Maintainers
- Update changelog before each release
- Review all entries for accuracy
- Include migration guides for breaking changes
- Add release notes for user-facing changes

---

## Links

- [GitHub Releases](https://github.com/ferriprove/ferriprove/releases)
- [GitHub Issues](https://github.com/ferriprove/ferriprove/issues)
- [Documentation](https://ferriprove.org)
- [Project Roadmap](./TODO.md)
