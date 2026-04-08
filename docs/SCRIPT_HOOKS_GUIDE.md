# Complete Guide: Using Scripts in Pre-commit Hooks

This guide demonstrates how to integrate and use custom scripts within pre-commit hooks for the Ferriprove project.

## Quick Start

### 1. Add Scripts to Pre-commit Configuration

Edit `.pre-commit-config.yaml` to add custom scripts:

```yaml
# Performance and optimization checks
- repo: local
  hooks:
    # Custom script hooks examples
    - id: workspace-validation
      name: workspace validation
      description: Validate Cargo workspace structure
      entry: ./scripts/validate-workspace.sh
      language: script
      files: '^Cargo\.toml$|^.*/Cargo\.toml$'
      pass_filenames: false

    - id: documentation-coverage
      name: documentation coverage
      description: Check documentation coverage for Rust code
      entry: ./scripts/check-doc-coverage.sh
      language: script
      types: [rust]
      pass_filenames: false

    - id: dev-script-check
      name: dev script check
      description: Check if dev script is working correctly
      entry: ./scripts/dev.sh check
      language: script
      pass_filenames: false
      always_run: false
```

### 2. Make Scripts Executable

```bash
chmod +x scripts/your-script.sh
```

### 3. Test the Integration

```bash
# Test individual hooks
pre-commit run workspace-validation --all-files
pre-commit run documentation-coverage --all-files

# Test all hooks
pre-commit run --all-files

# Use the dev script
./scripts/dev.sh precommit
```

## Working Examples in Ferriprove

### Example 1: Workspace Validation

**Script**: `scripts/validate-workspace.sh`

**Purpose**: Ensures Cargo workspace structure is valid

**Hook Configuration**:
```yaml
- id: workspace-validation
  name: workspace validation
  description: Validate Cargo workspace structure
  entry: ./scripts/validate-workspace.sh
  language: script
  files: '^Cargo\.toml$|^.*/Cargo\.toml$'
  pass_filenames: false
```

**Usage**:
```bash
# Run manually
./scripts/validate-workspace.sh

# Run via pre-commit
pre-commit run workspace-validation --all-files
```

**Output**:
```
[INFO] Validating Cargo workspace structure...
[INFO] ✓ Workspace member ferriprove-types exists
[INFO] ✓ Workspace member ferriprove-export exists
[INFO] ✓ Workspace member ferriprove-kernel exists
[INFO] ✓ Workspace member ferriprove-elab exists
[INFO] ✓ Workspace member ferriprove-tactic exists
[INFO] ✓ Workspace member ferriprove-lsp exists
[INFO] ✓ Workspace member ferriprove-cli exists
[INFO] Root version: 0.0.1
[INFO] ✓ ferriprove-types inherits version from workspace
[INFO] ✓ ferriprove-export inherits version from workspace
[INFO] ✓ ferriprove-kernel inherits version from workspace
[INFO] ✓ ferriprove-elab inherits version from workspace
[INFO] ✓ ferriprove-tactic inherits version from workspace
[INFO] ✓ ferriprove-lsp inherits version from workspace
[INFO] ✓ ferriprove-cli inherits version from workspace
[INFO] Workspace validation completed successfully!
```

### Example 2: Documentation Coverage

**Script**: `scripts/check-doc-coverage.sh`

**Purpose**: Ensures adequate documentation coverage

**Hook Configuration**:
```yaml
- id: documentation-coverage
  name: documentation coverage
  description: Check documentation coverage for Rust code
  entry: ./scripts/check-doc-coverage.sh
  language: script
  types: [rust]
  pass_filenames: false
```

**Usage**:
```bash
# Run manually
./scripts/check-doc-coverage.sh

# Run via pre-commit
pre-commit run documentation-coverage --all-files
```

### Example 3: Dev Script Integration

**Script**: `scripts/dev.sh`

**Purpose**: Provides unified interface to development tools

**Hook Configuration**:
```yaml
- id: dev-script-check
  name: dev script check
  description: Check if dev script is working correctly
  entry: ./scripts/dev.sh check
  language: script
  pass_filenames: false
  always_run: false
```

**Usage**:
```bash
# Run via pre-commit
pre-commit run dev-script-check --all-files

# Use dev script directly
./scripts/dev.sh check
./scripts/dev.sh test
./scripts/dev.sh fmt
```

## Hook Configuration Options

### File-based Triggers

```yaml
# Run only on specific files
- id: rust-specific
  files: '\.rs$'
  
# Run only on configuration files
- id: config-check
  files: '^Cargo\.toml$|\.yaml$|\.json$'
  
# Run on all files of a type
- id: markdown-check
  types: [markdown]
```

### Conditional Execution

```yaml
# Always run (even if no files match)
- id: always-check
  always_run: true
  
# Skip on CI (for expensive operations)
- id: expensive-check
  always_run: false
```

### Environment Variables

```yaml
- id: env-check
  env:
    - name: CUSTOM_VAR
      value: "custom_value"
    - name: DEBUG
      value: "1"
```

## Script Development Best Practices

### 1. Script Structure

```bash
#!/bin/bash
# Description of script purpose

set -euo pipefail  # Robust error handling

# Colors for output
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

# Main logic
print_status "Starting validation..."

# Your validation code here

print_status "Validation completed successfully!"
exit 0
```

### 2. Error Handling

```bash
# Check for required tools
if ! command -v cargo &> /dev/null; then
    print_error "cargo is required but not installed"
    exit 1
fi

# Validate environment
if [[ ! -f "Cargo.toml" ]]; then
    print_error "This script must be run from a Cargo project directory"
    exit 1
fi

# Handle errors gracefully
if ! cargo check --quiet; then
    print_error "Cargo check failed"
    exit 1
fi
```

### 3. Performance Considerations

```bash
# Use caching for expensive operations
CACHE_FILE=".cache/validation-cache"
if [[ -f "$CACHE_FILE" && "$CACHE_FILE" -nt "Cargo.toml" ]]; then
    print_status "Using cached validation results"
    exit 0
fi

# Run expensive operations
print_status "Running expensive validation..."
# ... validation code ...

# Update cache
touch "$CACHE_FILE"
```

## Advanced Integration

### 1. File Processing Hooks

```yaml
- id: file-processor
  name: Process Files
  entry: ./scripts/process-files.sh
  language: script
  files: '\.md$'
  pass_filenames: true
```

**Script**:
```bash
#!/bin/bash
# Process each file passed as argument
for file in "$@"; do
    echo "Processing $file"
    # Process the file
done
```

### 2. Conditional Hooks Based on Content

```yaml
- id: conditional-check
  name: Conditional Check
  entry: ./scripts/conditional.sh
  language: script
  files: '\.rs$'
  pass_filenames: true
```

**Script**:
```bash
#!/bin/bash
# Only run if files contain specific patterns
for file in "$@"; do
    if grep -q "TODO\|FIXME" "$file"; then
        echo "Found TODO/FIXME in $file"
        # Run additional checks
    fi
done
```

### 3. Integration with External Tools

```yaml
- id: external-tool
  name: External Tool Check
  entry: ./scripts/external-tool.sh
  language: script
  types: [rust]
```

**Script**:
```bash
#!/bin/bash
# Integrate with external tools like linters, formatters
if command -v custom-linter &> /dev/null; then
    custom-linter "$@"
else
    echo "custom-linter not found, skipping"
fi
```

## Testing and Debugging

### Manual Testing

```bash
# Test individual hooks
pre-commit run workspace-validation --all-files

# Test with verbose output
pre-commit run --all-files --verbose

# Test specific files
pre-commit run --files src/lib.rs README.md

# Validate configuration
pre-commit validate-config
```

### Using the Dev Script

```bash
# Test all pre-commit hooks
./scripts/dev.sh precommit

# Test specific functionality
./scripts/validate-workspace.sh
./scripts/check-doc-coverage.sh
./scripts/dev.sh check
```

### Debugging Script Issues

```bash
# Run script directly to see errors
./scripts/validate-workspace.sh

# Check script permissions
ls -la scripts/

# Test with bash debugging
bash -x scripts/validate-workspace.sh
```

## Troubleshooting Common Issues

### 1. Script Not Found

**Problem**: `Error: ./scripts/my-script.sh: No such file or directory`

**Solution**: 
- Check script path in `.pre-commit-config.yaml`
- Ensure script exists and is executable
- Use absolute paths if needed

### 2. Permission Denied

**Problem**: `Error: Permission denied`

**Solution**:
```bash
chmod +x scripts/your-script.sh
```

### 3. Hook Fails Silently

**Problem**: Hook fails but no error output

**Solution**:
```bash
# Run with verbose output
pre-commit run --all-files --verbose

# Check hook logs
pre-commit run your-hook --all-files --verbose
```

### 4. Performance Issues

**Problem**: Hooks take too long to run

**Solution**:
- Use `always_run: false` for expensive hooks
- Add caching to scripts
- Optimize script performance
- Consider running expensive checks only on CI

## Complete Workflow Example

### 1. Setup

```bash
# Install pre-commit
./scripts/dev.sh setup

# Validate configuration
pre-commit validate-config
```

### 2. Development

```bash
# Make changes to code
echo "// New code" >> src/lib.rs

# Run pre-commit hooks
./scripts/dev.sh precommit

# Or run specific hooks
pre-commit run workspace-validation --all-files
```

### 3. Testing Integration

```bash
# Test all hooks work together
pre-commit run --all-files

# Test specific scenarios
echo "invalid" > Cargo.toml
pre-commit run workspace-validation --all-files
git checkout Cargo.toml
```

### 4. CI Integration

The hooks automatically integrate with GitHub Actions through `.github/workflows/pre-commit.yml`.

## Summary

Using scripts in pre-commit hooks provides:

✅ **Custom Validation**: Project-specific checks beyond standard linters
✅ **Automation**: Automatic validation on every commit
✅ **Consistency**: Ensures all team members follow the same standards
✅ **Integration**: Seamless integration with existing development workflow
✅ **Flexibility**: Can be tailored to specific project needs

The Ferriprove project demonstrates several practical examples:
- Workspace structure validation
- Documentation coverage checking
- Performance benchmarking
- Development tool integration

These examples serve as templates for creating your own custom pre-commit hooks.
