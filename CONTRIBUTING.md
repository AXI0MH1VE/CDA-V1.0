# Contributing Guidelines

Welcome! We appreciate your interest in contributing to the AxiomHive Assistant project. This document outlines the process and standards for contributing to ensure we maintain high quality, constitutional compliance, and collaborative development.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Constitutional Compliance](#constitutional-compliance)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Community](#community)

## Code of Conduct

### Our Standards

**Be Respectful:** Treat all contributors with respect and kindness. Disagreements are fine, but personal attacks are not.

**Be Inclusive:** Welcome contributors from all backgrounds and experience levels. Help newcomers learn and grow.

**Be Constructive:** Focus on solutions and improvements rather than criticism. Provide actionable feedback.

**Be Transparent:** Communicate clearly and honestly. Keep discussions open and accessible.

### Constitutional Alignment

All contributions must align with CDA-v1.0 principles:

- **No Consciousness Claims:** Do not suggest or implement features that simulate consciousness
- **Transparency First:** Clearly document all functionality and limitations
- **Safety Priority:** Implement robust safeguards and validation
- **User Sovereignty:** Respect user control and privacy

## Constitutional Compliance

### Pre-Contribution Checklist

Before contributing, ensure your work:

- [ ] Does not violate Article I (Identity and Purpose) prohibitions
- [ ] Implements Article II (Operational Principles) requirements
- [ ] Enforces Article III (Ethical Boundaries) safeguards
- [ ] Maintains transparency about AI limitations
- [ ] Respects user privacy and data sovereignty

### Constitutional Review

All pull requests undergo constitutional review:

1. **Automated Checks:** CI/CD pipeline validates constitutional compliance
2. **Manual Review:** Core contributors review for alignment with CDA-v1.0
3. **Legal Review:** Significant changes may require legal consultation

## Getting Started

### Prerequisites

- Rust 1.75+ ([Installation Guide](https://rustup.rs/))
- Node.js 20+ ([Download](https://nodejs.org/))
- Tauri CLI (`cargo install tauri-cli`)
- Git

### Setup

1. **Fork the repository** on GitHub
2. **Clone your fork:**
   ```bash
   git clone https://github.com/your-username/CDA-V1.0.git
   cd CDA-v1.0
   ```
3. **Add upstream remote:**
   ```bash
   git remote add upstream https://github.com/AXI0MH1VE/CDA-V1.0.git
   ```
4. **Create a branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```
5. **Install dependencies:**
   ```bash
   cd axiomhive-assistant
   npm install
   ```
6. **Download models** (for testing):
   ```bash
   mkdir models
   # Download required AI models
   ```

## Development Workflow

### Branch Naming

Use descriptive branch names:

```
feature/add-web-search
bugfix/fix-crash-on-startup
docs/update-installation-guide
refactor/optimize-memory-usage
```

### Commit Messages

Follow conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Testing
- `chore`: Maintenance

**Examples:**
```
feat(lambda-core): add Z3 solver integration

Fixes issue with formal constraint validation in constitutional engine.

Closes #123
```

```
docs(readme): update installation instructions

Updated Node.js version requirement and added troubleshooting section.
```

### Development Process

1. **Sync with upstream:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Make changes** following code standards

3. **Test thoroughly:**
   ```bash
   npm test
   cargo test
   npm run tauri:build
   ```

4. **Update documentation** if needed

5. **Commit changes:**
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

6. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create pull request**

## Code Standards

### Rust Code Standards

- **Follow Rust idioms** and best practices
- **Use `cargo fmt`** for formatting
- **Use `cargo clippy`** for linting
- **Document public APIs** with rustdoc
- **Handle errors properly** with `Result` and `?`
- **Write tests** for all public functions

**Example:**
```rust
/// Validates query against constitutional prohibitions
///
/// # Arguments
/// * `query` - The user query to validate
///
/// # Returns
/// * `Ok(ValidatedQuery)` if validation passes
/// * `Err(ValidationError)` if validation fails
pub fn validate_query(&self, query: &str) -> Result<ValidatedQuery, ValidationError> {
    // Implementation
    Ok(validated_query)
}
```

### TypeScript/React Standards

- **Use TypeScript** for all new code
- **Follow React best practices**
- **Use functional components** with hooks
- **Implement proper error boundaries**
- **Write comprehensive types**

**Example:**
```typescript
interface ChatMessage {
  id: string;
  content: string;
  role: 'user' | 'assistant';
  timestamp: Date;
  constitutional_check?: ValidationResult;
}

const ChatMessage: React.FC<{ message: ChatMessage }> = ({ message }) => {
  return (
    <div className="message">
      <div className="content">{message.content}</div>
      {message.constitutional_check && (
        <ValidationBadge result={message.constitutional_check} />
      )}
    </div>
  );
};
```

### General Standards

- **Write clear, readable code**
- **Use meaningful variable names**
- **Add comments for complex logic**
- **Keep functions small and focused**
- **Follow DRY principle**
- **Use consistent formatting**

## Testing

### Testing Requirements

- **Unit tests** for all public functions
- **Integration tests** for API endpoints
- **Constitutional compliance tests**
- **End-to-end tests** for critical user flows
- **Performance tests** for benchmarks

### Running Tests

```bash
# Run all tests
npm test

# Run Rust tests only
cargo test

# Run with coverage
npm run test:coverage

# Run integration tests
npm run test:integration
```

### Test Coverage

Maintain >80% code coverage:

- **Rust:** Use `cargo tarpaulin`
- **TypeScript:** Use `jest` with coverage
- **Integration:** Use Playwright for E2E tests

### Constitutional Testing

Special tests for CDA compliance:

```rust
#[test]
fn test_identity_prohibitions() {
    let validator = AxiomValidator::new();
    assert!(validator.check_identity_prohibitions("I am conscious").is_err());
    assert!(validator.check_identity_prohibitions("Hello world").is_ok());
}
```

## Pull Request Process

### PR Checklist

- [ ] **Constitutional compliance** verified
- [ ] **Tests pass** locally
- [ ] **Code formatted** and linted
- [ ] **Documentation updated**
- [ ] **Breaking changes** clearly marked
- [ ] **Security review** completed

### PR Template

Use the provided PR template:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Constitutional Compliance
- [ ] No violations of CDA-v1.0
- [ ] Transparency maintained
- [ ] Safety features intact

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Screenshots (if applicable)
Add screenshots for UI changes

## Additional Notes
Any additional information
```

### Review Process

1. **Automated checks** run on CI
2. **Peer review** by at least one maintainer
3. **Constitutional review** if needed
4. **Approval and merge**

### Review Guidelines

**Reviewers should check:**
- Code quality and standards
- Constitutional compliance
- Test coverage
- Documentation updates
- Security implications

**Provide constructive feedback:**
- Explain reasoning
- Suggest improvements
- Ask questions for clarification
- Acknowledge good work

## Issue Reporting

### Bug Reports

Use the bug report template:

```markdown
## Bug Description
Clear description of the issue

## Steps to Reproduce
1. Go to '...'
2. Click on '...'
3. See error

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: [e.g., Windows 10]
- Version: [e.g., 1.0.0]
- Browser: [if applicable]

## Additional Context
Screenshots, logs, etc.
```

### Feature Requests

Use the feature request template:

```markdown
## Feature Summary
Brief description of the feature

## Problem Statement
What problem does this solve?

## Proposed Solution
How should it work?

## Constitutional Considerations
How does this align with CDA-v1.0?

## Alternatives Considered
Other solutions you've considered

## Additional Context
Mockups, examples, etc.
```

### Security Issues

**Do not report security issues publicly.** Email security@axiomhive.co with details.

## Community

### Communication Channels

- **GitHub Issues:** Bug reports and feature requests
- **GitHub Discussions:** General questions and community discussion
- **Discord:** Real-time chat (link in README)
- **Email:** For sensitive matters

### Getting Help

- **Documentation:** Check docs/ folder first
- **Issues:** Search existing issues
- **Discussions:** Ask the community
- **Discord:** Real-time help

### Recognition

Contributors are recognized through:
- GitHub contributor statistics
- Mention in release notes
- Special recognition for significant contributions

## License and IP

### Contributor License Agreement

By contributing, you agree to:
- License your contributions under the project license
- Warrant that you have rights to contribute the code
- Allow the project to use your contribution

### Intellectual Property

- **Constitutional AI technology** is proprietary
- **CDA-v1.0** implementation is protected
- **Contributions** must not infringe on third-party IP

## Recognition

We value all contributions, big and small. Contributors may be:

- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Invited to join the core team
- Featured in project communications

## Questions?

If you have questions about contributing:

1. Check this document
2. Search existing issues/discussions
3. Ask in GitHub Discussions
4. Contact the maintainers

Thank you for contributing to AxiomHive Assistant! Your efforts help build safer, more transparent AI.

---

**Last updated:** January 2025