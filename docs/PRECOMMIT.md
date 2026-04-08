# Pre-commit Hooks Setup

This document describes the pre-commit hooks configuration used in Ferriprove to ensure code quality and consistency.

## Overview

Ferriprove uses [pre-commit](https://pre-commit.com/) to run various checks before commits are made. This includes formatting, linting, security checks, and more.

## Quick Start

### 1. Install Dependencies

```bash
# Run the setup script (recommended)
./scripts/dev.sh setup

# Or run the pre-commit setup script directly
./scripts/setup-precommit.sh
```

### 2. Manual Installation

If you prefer to install dependencies manually:

```bash
# Install pre-commit
pip install pre-commit

# Install Rust components
rustup component add rustfmt clippy

# Install cargo tools
cargo install cargo-audit cargo-deny

# Install hooks
pre-commit install
```

## Available Hooks

### Code Quality

- **rustfmt**: Formats Rust code according to style guidelines
- **clippy**: Runs Rust lints to catch common mistakes
- **cargo-check**: Ensures code compiles without errors
- **cargo-test**: Runs all tests before committing

### Security

- **cargo-audit**: Checks for security vulnerabilities in dependencies
- **cargo-deny**: Validates licenses and dependency policies

### Documentation

- **cargo-doc**: Ensures documentation builds without warnings
- **markdownlint**: Checks Markdown files for style consistency

### File Management

- **trailing-whitespace**: Removes trailing whitespace
- **end-of-file-fixer**: Ensures files end with newlines
- **check-yaml/check-toml/check-json**: Validates configuration files

## Usage

### Running Hooks Manually

```bash
# Run on all files
pre-commit run --all-files

# Run on specific files
pre-commit run --files src/lib.rs

# Run specific hook
pre-commit run rustfmt --all-files
```

### Development Workflow

```bash
# Use the dev script for common tasks
./scripts/dev.sh fmt          # Format code
./scripts/dev.sh clippy       # Run lints
./scripts/dev.sh test         # Run tests
./scripts/dev.sh precommit    # Run all hooks

# Watch for changes and run checks
./scripts/dev.sh watch-test   # Watch and test
./scripts/dev.sh watch-clippy # Watch and lint
```

## Configuration Files

- `.pre-commit-config.yaml`: Main pre-commit configuration
- `deny.toml`: Cargo-deny configuration for license/security checks
- `.cz.toml`: Commitizen configuration for conventional commits
- `.markdownlint.json`: Markdown linting rules

## Commit Message Format

Ferriprove uses [conventional commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes
- `build`: Build system changes

Examples:

```
feat(types): add expression interning support
fix: resolve hash collision vulnerability
docs: update pre-commit setup guide
```

## CI Integration

The pre-commit hooks are integrated into GitHub Actions:

- **pre-commit.yml**: Runs all hooks on pull requests
- **Security checks**: Separate job for cargo-audit and cargo-deny
- **Matrix testing**: Tests across multiple OS and Rust versions

## Skipping Hooks

If you need to skip hooks (not recommended):

```bash
# Skip all hooks
git commit --no-verify

# Skip specific hook
SKIP=rustfmt git commit
```

## Troubleshooting

### Common Issues

1. **Hook fails on formatting**

   ```bash
   # Auto-fix formatting issues
   ./scripts/dev.sh fmt
   ```

2. **Security audit fails**

   ```bash
   # Update dependencies
   cargo update
   # Or review and accept the risk
   ```

3. **License check fails**

   ```bash
   # Review deny.toml configuration
   cargo deny check
   ```

### Performance Tips

- Use `cargo watch` for continuous checking during development
- Run specific hooks instead of all hooks when possible
- Cache dependencies to speed up CI runs

## Contributing

When contributing to Ferriprove:

1. Fork and clone the repository
2. Run `./scripts/dev.sh setup` to install hooks
3. Create a feature branch
4. Make your changes
5. Run `./scripts/dev.sh precommit` before committing
6. Submit a pull request

The pre-commit hooks will ensure your contribution meets Ferriprove's quality standards.
