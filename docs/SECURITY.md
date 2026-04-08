# Security Policy

## Supported Versions

| Version | Supported          | Security Updates |
|---------|--------------------|------------------|
| 0.1.x   | ✅ Yes             | ✅ Yes           |
| 0.0.x   | ❌ No              | ❌ No            |

## Reporting a Vulnerability

### Security vs Soundness

Ferriprove distinguishes between **security vulnerabilities** and **soundness bugs**:

- **Security vulnerabilities**: Issues that could lead to remote code execution, data corruption, or other security impacts
- **Soundness bugs**: Issues where the kernel might accept an invalid proof (false positive) - these are **P0** bugs but not security issues

### Reporting Security Vulnerabilities

For security vulnerabilities, please contact: **security@ferriprove.org** (future)

### Current Contact Channel
- **X (Twitter)**: @ks_sha888 - Current primary contact method

Please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Any proof-of-concept code

We will respond within 48 hours and provide a timeline for resolution.

### Reporting Soundness Bugs

For soundness bugs, please use the regular GitHub issue tracker with the `soundness` label. These are treated as P0 bugs but don't require private disclosure.

## Security Model

### Trust Boundaries

```
┌──────────────────────────────────────────────────────┐
│                 User Input                           │  Untrusted
├──────────────────────────────────────────────────────┤
│              ferriprove-lsp                           │  Untrusted
│              ferriprove-cli                           │  Untrusted
│              ferriprove-tactic                        │  Untrusted
│              ferriprove-elab                          │  Untrusted
├──────────────────────────────────────────────────────┤
│              ferriprove-export                        │  Untrusted
├──────────────────────────────────────────────────────┤
│              ferriprove-types                         │  ✅ Trusted
├──────────────────────────────────────────────────────┤
│              ferriprove-kernel                        │  ✅ Trusted
└──────────────────────────────────────────────────────┘
```

### Security Assumptions

1. **Kernel is trusted**: `ferriprove-kernel` and `ferriprove-types` are the only trusted components
2. **Input validation**: All external input is validated before reaching trusted components
3. **Memory safety**: No unsafe code in trusted components without formal verification
4. **Dependency hygiene**: All dependencies are audited for security issues

### Threat Model

#### In Scope
- Remote code execution via malformed input
- Denial of service attacks
- Memory corruption vulnerabilities
- Path traversal in file operations
- Side-channel attacks in cryptographic operations

#### Out of Scope
- Soundness bugs (handled separately as P0 issues)
- Performance degradation (handled as regular bugs)
- Social engineering attacks
- Physical attacks on running systems

## Security Best Practices

### Development

- All code must be reviewed before merge
- Use `cargo audit` to check for vulnerable dependencies
- No `unsafe` code in trusted components without formal verification
- Regular security reviews of critical components
- Fuzz testing of all input parsers

### Dependencies

```bash
# Audit dependencies
cargo audit

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

### Deployment

- Use latest stable releases
- Enable all relevant security features
- Monitor for security advisories
- Regular updates and patches

## Known Security Considerations

### Parser Security

The `ferriprove-export` crate handles external NDJSON files and includes:
- Input size limits
- Memory usage bounds
- Comprehensive error handling
- Fuzz testing targets

### LSP Security

The `ferriprove-lsp` crate includes:
- Input validation for all LSP messages
- Resource limits for large files
- Safe path handling for file operations
- No arbitrary code execution

### CLI Security

The `ferriprove-cli` crate includes:
- Safe file path handling
- Resource limits for large inputs
- No network access by default
- Secure temporary file handling

## Security Updates

### Update Process

1. Vulnerability is reported and assessed
2. Fix is developed and tested
3. Security advisory is prepared
4. Update is released
5. Users are notified via GitHub advisories

### Notification Channels

- GitHub Security Advisories
- Email notifications to registered users (future)
- Release notes with security details
- Blog posts for critical issues

## Security Team

The Ferriprove security team can be contacted at:
- **@ks_sha888** (Twitter) - Security vulnerabilities
- GitHub Issues - General security questions

### Future Contact Methods (Planned)
- **security@ferriprove.org** - Security vulnerabilities (future)
- **maintainers@ferriprove.org** - General security questions (future)

## Security Badges

- ![Security audit](https://github.com/ferriprove/ferriprove/workflows/Security%20audit/badge.svg)
- ![Dependabot](https://github.com/ferriprove/ferriprove/workflows/Dependabot/badge.svg)

## Acknowledgments

We thank security researchers who help us keep Ferriprove secure. All valid security reports will be acknowledged and credited according to reporter preferences.
