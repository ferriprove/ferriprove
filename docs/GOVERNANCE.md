# Ferriprove Governance

## Project Leadership

### Maintainers
- **@SHA888** - Project lead, kernel architecture
- Additional maintainers will be added as the project grows

### Roles and Responsibilities

#### Project Lead (@SHA888)
- Overall project direction and vision
- Kernel architecture and soundness oversight
- Release management and versioning
- Community coordination and conflict resolution

#### Maintainers
- Code review and quality assurance
- Issue triage and prioritization
- Documentation maintenance
- Mentorship and community support

## Decision Making

### Technical Decisions
- **Kernel changes**: Require project lead approval
- **API changes**: Require maintainer consensus
- **Performance changes**: Require benchmark validation
- **Bug fixes**: Can be merged by any maintainer

### Process Decisions
- **Tooling changes**: Require maintainer discussion
- **Documentation changes**: Can be merged by any maintainer
- **Community guidelines**: Require project lead approval

### Release Decisions
- **Patch releases**: Can be made by any maintainer
- **Minor releases**: Require maintainer consensus
- **Major releases**: Require project lead approval

## License Decision Rationale

### Chosen License: MIT OR Apache-2.0

#### Why Dual License?

1. **Ecosystem Compatibility**: 
   - Lean 4 uses Apache-2.0
   - nanoda_lib uses MIT
   - Rust ecosystem prefers permissive licenses

2. **User Flexibility**:
   - MIT for minimal restrictions
   - Apache-2.0 for patent protection
   - Users can choose either license

3. **Commercial Adoption**:
   - Both licenses are business-friendly
   - Clear patent grant in Apache-2.0
   - No copyleft restrictions

#### License Compatibility

All dependencies are compatible with MIT OR Apache-2.0:

- **nanoda_lib** (MIT) ✅
- **Lean4Lean** (Apache-2.0) ✅
- **lean-kernel-arena** (Apache-2.0) ✅
- **Rust stdlib** (MIT/Apache-2.0) ✅
- **Planned dependencies** all compatible ✅

## Contribution Policy

### Who Can Contribute?

Anyone can contribute! We welcome:
- Code contributions
- Documentation improvements
- Bug reports and feature requests
- Community participation
- Security research

### Contribution Requirements

- Follow the [Code of Conduct](CODE_OF_CONDUCT.md)
- Sign the DCO (Developer Certificate of Origin)
- Accept the project license terms
- Follow contribution guidelines in CONTRIBUTING.md

### Intellectual Property

- Contributors retain copyright to their work
- Contributions are licensed under MIT OR Apache-2.0
- No patent grants required beyond Apache-2.0 terms
- No contributor license agreements needed

## Release Process

### Versioning Policy

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes to kernel or public APIs
- **MINOR**: New features, backward-compatible changes
- **PATCH**: Bug fixes, documentation updates

### Release Schedule

- **M0**: Foundation (no release)
- **M1**: Kernel parity (v0.1.0)
- **M2**: Elaborator (v0.2.0)
- **M3**: Core tactics (v0.3.0)
- **M4**: `simp` (v0.4.0)
- **M5**: LSP + VS Code (v0.5.0)

### Release Criteria

Each milestone must meet:
- All acceptance tests pass
- Documentation is complete
- Performance benchmarks meet targets
- Security audit passes
- Community review period completed

## Community Guidelines

### Code of Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct):

- Be inclusive and respectful
- Focus on what is best for the community
- Show empathy towards other community members

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: General questions, ideas
- **GitHub PRs**: Code contributions, reviews
- **Twitter**: @ks_sha888 (security issues)

### Meeting Schedule

- **Weekly**: Maintainer sync (as needed)
- **Monthly**: Community update (as needed)
- **Quarterly**: Roadmap review
- **Ad-hoc**: Security incident response

## Conflict Resolution

### Dispute Resolution Process

1. **Direct Discussion**: Try to resolve directly between parties
2. **Mediation**: Involve a neutral maintainer
3. **Escalation**: Project lead makes final decision
4. **Appeal**: Community discussion for major decisions

### Code of Conduct Enforcement

- **First offense**: Warning and education
- **Second offense**: Temporary suspension
- **Third offense**: Permanent ban
- **Severe violations**: Immediate ban

## Security and Soundness

### Security Team
- **@ks_sha888** (Twitter) - Security vulnerabilities
- Confidential reporting and coordinated disclosure
- Security advisories and patches

### Soundness Team
- **@ks_sha888** (Twitter) - Soundness bugs
- P0 bug response and fixes
- Formal verification oversight

## Financial Policy

### Funding
- **Open source**: No commercial funding required
- **Infrastructure**: GitHub provides free hosting
- **Domains**: Project maintains ferriprove.org
- **Services**: No paid services currently

### Expenses
- **Domain registration**: ~$15/year
- **Infrastructure**: $0 (GitHub free tier)
- **Legal**: $0 (permissive licenses)
- **Marketing**: $0 (community-driven)

## Legal Structure

### Current Status
- **Unincorporated**: Individual project
- **Liability**: Project lead assumes responsibility
- **Intellectual Property**: Maintained by contributors
- **Trademarks**: "Ferriprove" name protected by use

### Future Considerations
- **Foundation**: Possible non-profit structure
- **Trademark**: Formal registration if needed
- **Liability**: Limited liability entity if required
- **Insurance**: Professional liability if commercial use grows

## Amendment Process

### Governance Changes

1. **Proposal**: Create GitHub issue with proposed changes
2. **Discussion**: Minimum 2-week discussion period
3. **Vote**: Maintainer consensus required
4. **Implementation**: Update documentation and processes

### License Changes

License changes require:
- **Supermajority**: 2/3 of all contributors
- **Notice**: 90-day notice period
- **Migration**: Clear migration path for existing code
- **Legal**: Legal review of compatibility

## Contact Information

### General Inquiries
- **GitHub Issues**: Public questions and discussions
- **GitHub Discussions**: Community conversations
- **Twitter**: @ks_sha888

### Urgent Matters
- **Security**: security@ferriprove.org (future)
- **Soundness**: soundness@ferriprove.org (future)
- **Legal**: legal@ferriprove.org (future)

### Current Contact Channel
- **X (Twitter)**: @ks_sha888 - Current primary contact method

### Future Contact Methods (Planned)
- **General**: maintainers@ferriprove.org (future)

### Project Lead
- **GitHub**: @SHA888
- **Email**: Available to maintainers and security team

---

This governance document is a living document and will be updated as the project evolves. All changes will be made transparently with community input.
