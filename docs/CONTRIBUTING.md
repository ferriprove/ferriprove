# Contributing to Ferriprove

Thank you for your interest in contributing to Ferriprove! This document provides guidelines for contributing to the project.

## Code of Conduct

This project follows the Rust Code of Conduct. Please read and follow it in all interactions.

## Development Setup

### Prerequisites

- Rust stable (see `rust-toolchain.toml` for pinned version)
- Git
- GitHub account (for PRs and issues)

### Getting Started

```bash
git clone https://github.com/ferriprove/ferriprove.git
cd ferriprove
cargo build --workspace
cargo test --workspace
```

## Code Style

### Formatting

All code must be formatted with `rustfmt`:

```bash
cargo fmt --all
```

### Linting

All code must pass `clippy` with strict warnings:

```bash
cargo clippy --workspace -- -D warnings
```

### Conventions

- Use `snake_case` for variables and functions
- Use `PascalCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Document all public APIs with `///` doc comments
- Include examples in documentation where appropriate

## Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/) for all commits:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or modifying tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI/CD changes

### Examples

```
feat(kernel): add definitional equality checker
fix(elab): handle recursive let bindings
docs(readme): update installation instructions
```

## Pull Request Process

### Before Submitting

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Ensure all tests pass: `cargo test --workspace`
5. Ensure code is formatted: `cargo fmt --all`
6. Ensure linting passes: `cargo clippy --workspace -- -D warnings`
7. Commit with conventional commit messages
8. Push to your fork
9. Create a pull request

### PR Requirements

- Title must follow conventional commit format
- Description must clearly explain the change
- All tests must pass
- Code must be formatted and linted
- Documentation must be updated if relevant
- Include links to any related issues

### Review Process

1. Automated checks must pass
2. At least one maintainer review required
3. Address all review comments
4. Update PR as needed
5. Maintainer will merge when ready

## Testing

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p ferriprove-kernel

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test --workspace test_name
```

### Writing Tests

- Unit tests go in the same module as the code
- Integration tests go in `tests/` directory
- Use descriptive test names
- Test both success and failure cases
- Use `#[should_panic]` for expected panics

### Benchmarks

```bash
# Run benchmarks
cargo bench --workspace

# Run specific benchmark
cargo bench -p ferriprove-kernel def_eq
```

## Issue Reporting

### Bug Reports

Use the bug report template and include:

- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version)
- Any relevant logs or error messages

### Feature Requests

Use the feature request template and include:

- Clear description of the feature
- Use case and motivation
- Proposed implementation (if known)
- Any alternatives considered

## Development Areas

### Core Areas

- **ferriprove-types**: Core type definitions
- **ferriprove-export**: Lean export format parsing
- **ferriprove-kernel**: Trusted type checking kernel
- **ferriprove-elab**: Elaboration and type inference
- **ferriprove-tactic**: Tactic engine
- **ferriprove-lsp**: Language Server Protocol
- **ferriprove-cli**: Command line interface

### Contributing Guidelines

#### Kernel (ferriprove-kernel)

- All kernel code must be pure and verifiable
- No `unsafe` code without explicit justification
- All functions must have formal verification in mind
- Include soundness arguments in documentation

#### Parser (ferriprove-export)

- Handle malformed input gracefully
- Include comprehensive error messages
- Add fuzz targets for robustness

#### LSP (ferriprove-lsp)

- Follow LSP specification exactly
- Handle all client requests gracefully
- Include proper error handling

## Performance

### Guidelines

- Profile before optimizing
- Use `criterion` for benchmarks
- Focus on hot paths identified by profiling
- Consider memory usage as well as CPU time

### Tools

```bash
# Install profiling tools
cargo install cargo-flamegraph
cargo install cargo-criterion

# Generate flamegraph
cargo flamegraph --bin ferriprove

# Run criterion benchmarks
cargo criterion --workspace
```

## Security

### Reporting Security Issues

For security issues, please email security@ferriprove.org rather than using public issues.

### Security Guidelines

- Never commit secrets or API keys
- Use `cargo audit` to check dependencies
- Review all external dependencies carefully
- Follow secure coding practices

## Getting Help

- GitHub Issues: For bug reports and feature requests
- GitHub Discussions: For general questions and discussion
- Documentation: Check `ARCHITECTURE.md` and `README.md`
- Code: Read existing code for examples

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 license.
