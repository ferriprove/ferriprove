# Using Scripts in Pre-commit Hooks

This guide shows how to integrate custom scripts into pre-commit hooks for the Ferriprove project.

## Overview

Pre-commit hooks can run custom scripts alongside standard hooks. This allows you to:
- Validate project-specific requirements
- Run custom performance benchmarks
- Check documentation coverage
- Enforce coding standards beyond standard linters

## Script Integration Methods

### 1. Local Hooks (Recommended)

Add custom scripts to the `local` repo section in `.pre-commit-config.yaml`:

```yaml
# Performance and optimization checks
- repo: local
  hooks:
    - id: custom-script
      name: Custom Script
      description: Description of what the script does
      entry: ./scripts/my-script.sh
      language: script
      files: '\.rs$'  # Optional: only run on specific files
      pass_filenames: false
      always_run: false
```

### 2. Hook Configuration Options

| Option | Description | Example |
|--------|-------------|---------|
| `id` | Unique identifier for the hook | `workspace-validation` |
| `name` | Display name in output | `Workspace Validation` |
| `description` | Detailed description | `Validate Cargo workspace structure` |
| `entry` | Script command to run | `./scripts/validate-workspace.sh` |
| `language` | Script type | `script`, `system`, `python` |
| `files` | File pattern to match | `'\.rs$'`, `'^Cargo\.toml$'` |
| `types` | File types to match | `rust`, `yaml`, `markdown` |
| `pass_filenames` | Pass file names to script | `true` or `false` |
| `always_run` | Run even if no files match | `true` or `false` |

## Current Custom Scripts

### 1. Workspace Validation (`scripts/validate-workspace.sh`)

**Purpose**: Validates Cargo workspace structure and consistency

**When it runs**: When Cargo.toml files change

**What it checks**:
- Workspace configuration exists
- All workspace members exist and have Cargo.toml
- No duplicate package names
- Version consistency across workspace

**Hook configuration**:
```yaml
- id: workspace-validation
  name: workspace validation
  description: Validate Cargo workspace structure
  entry: ./scripts/validate-workspace.sh
  language: script
  files: '^Cargo\.toml$|^.*/Cargo\.toml$'
  pass_filenames: false
```

### 2. Documentation Coverage (`scripts/check-doc-coverage.sh`)

**Purpose**: Ensures adequate documentation coverage

**When it runs**: On Rust file changes

**What it checks**:
- Documentation builds without warnings
- Undocumented public items
- README files exist
- Examples directory exists

**Hook configuration**:
```yaml
- id: documentation-coverage
  name: documentation coverage
  description: Check documentation coverage for Rust code
  entry: ./scripts/check-doc-coverage.sh
  language: script
  types: [rust]
  pass_filenames: false
```

### 3. Performance Benchmarks (`scripts/run-benchmarks.sh`)

**Purpose**: Runs performance tests to catch regressions

**When it runs**: Optional, on Rust file changes

**What it checks**:
- Expression creation performance
- Substitution performance
- Compilation time
- Binary size

**Hook configuration**:
```yaml
- id: performance-benchmark
  name: performance benchmark
  description: Run performance benchmarks on changes
  entry: ./scripts/run-benchmarks.sh
  language: script
  types: [rust]
  pass_filenames: false
  always_run: false
```

### 4. Dev Script Check (`./scripts/dev.sh check`)

**Purpose**: Validates that the dev script is working

**When it runs**: Optional, on any commit

**What it checks**:
- Dev script functionality
- Basic project health

**Hook configuration**:
```yaml
- id: dev-script-check
  name: dev script check
  description: Check if dev script is working correctly
  entry: ./scripts/dev.sh check
  language: script
  pass_filenames: false
  always_run: false
```

## Creating Custom Scripts

### Script Requirements

1. **Executable**: Scripts must be executable (`chmod +x`)
2. **Shebang**: Include appropriate shebang line
3. **Exit codes**: Return 0 for success, non-zero for failure
4. **Output**: Use stdout for info, stderr for errors

### Script Template

```bash
#!/bin/bash
# Description of what the script does

set -euo pipefail  # Exit on error, undefined vars, pipe failures

# Colors for output (optional)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Main script logic
print_status "Running custom validation..."

# Your validation logic here
if [[ condition ]]; then
    print_error "Validation failed"
    exit 1
fi

print_status "Validation completed successfully!"
```

## Advanced Usage

### Conditional Hooks

Run hooks only when specific conditions are met:

```yaml
- id: conditional-hook
  name: Conditional Hook
  entry: ./scripts/conditional.sh
  language: script
  files: '^src/.*\.rs$'  # Only on src Rust files
  exclude: '^src/tests/.*'  # Exclude test files
```

### Environment Variables

Pass environment variables to scripts:

```yaml
- id: env-hook
  name: Environment Hook
  entry: ./scripts/env-check.sh
  language: script
  env:
    - name: CUSTOM_VAR
      value: "custom_value"
    - name: PATH
      value: "$PATH:/usr/local/bin"
```

### File Arguments

Pass filenames to scripts for processing:

```yaml
- id: file-processor
  name: File Processor
  entry: ./scripts/process-files.sh
  language: script
  files: '\.md$'
  pass_filenames: true
```

The script receives filenames as arguments:
```bash
#!/bin/bash
for file in "$@"; do
    echo "Processing $file"
    # Process each file
done
```

## Best Practices

### 1. Performance
- Keep scripts fast (under 30 seconds)
- Use caching for expensive operations
- Skip expensive checks on CI with `always_run: false`

### 2. Reliability
- Handle errors gracefully
- Provide clear error messages
- Use `set -euo pipefail` for robust error handling

### 3. Maintainability
- Document script purpose clearly
- Use consistent output formatting
- Make scripts idempotent (safe to run multiple times)

### 4. Integration
- Test scripts independently before adding to hooks
- Use the dev script to test hook integration
- Consider CI/CD implications

## Testing Custom Hooks

### Manual Testing

```bash
# Test individual hooks
pre-commit run workspace-validation --all-files

# Test all hooks
pre-commit run --all-files

# Test specific files
pre-commit run --files src/lib.rs
```

### Using the Dev Script

```bash
# Test all pre-commit hooks
./scripts/dev.sh precommit

# Test specific functionality
./scripts/validate-workspace.sh
./scripts/check-doc-coverage.sh
./scripts/run-benchmarks.sh
```

## Troubleshooting

### Common Issues

1. **Script not found**: Ensure script path is correct and executable
2. **Permission denied**: Run `chmod +x scripts/your-script.sh`
3. **Hook fails silently**: Check pre-commit logs with `pre-commit run --verbose`
4. **Performance issues**: Use `always_run: false` for expensive hooks

### Debugging

```bash
# Run with verbose output
pre-commit run --all-files --verbose

# Check hook configuration
pre-commit validate-config

# Test specific hook
pre-commit run your-hook-id --all-files
```

## Examples in Ferriprove

The Ferriprove project includes several examples of custom scripts:

1. **Workspace Validation**: Ensures workspace consistency
2. **Documentation Coverage**: Checks documentation quality
3. **Performance Benchmarks**: Catches performance regressions
4. **Dev Script Integration**: Validates tooling setup

These examples demonstrate different approaches to script integration and can serve as templates for your own custom hooks.
