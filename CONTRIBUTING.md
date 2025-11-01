# Contributing to Axionax Core

Thank you for your interest in contributing to Axionax! This document provides guidelines and instructions for contributing to the project.

## 🌟 Ways to Contribute

- **Code Contributions**: Implement features, fix bugs, optimize performance
- **Documentation**: Improve docs, write tutorials, create examples
- **Testing**: Write tests, report bugs, perform security audits
- **Design**: UI/UX improvements, diagrams, visual assets
- **Community**: Answer questions, organize events, write blog posts
- **Translations**: Help translate documentation to other languages

---

## 🚀 Getting Started

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/axionax-core.git
cd axionax-core

# Add upstream remote
git remote add upstream https://github.com/axionaxprotocol/axionax-core.git
```

### 2. Set Up Development Environment

**Prerequisites:**
- Go 1.21+ or Rust 1.75+
- Python 3.10+
- Git
- Docker (optional, for testing)

```bash
# Install Python dependencies
pip install -r deai/requirements.txt

# Build and run all tests
./run_tests.sh
```

### 3. Create a Branch

```bash
# Update your fork
git fetch upstream
git checkout main
git merge upstream/main

# Create a feature branch
git checkout -b feature/your-feature-name
```

**Branch Naming Convention:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Adding tests
- `chore/` - Maintenance tasks

---

## 💻 Development Workflow

### 1. Make Your Changes

- Follow the existing code style
- Write clear, descriptive commit messages
- Add tests for new features
- Update documentation as needed
- Keep changes focused and atomic

### 2. Code Style

**Go Code:**
> *Note: The Go implementation is now considered legacy. New contributions should focus on the Rust/Python/TypeScript architecture.*
 
**Rust Code:**
```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

**Python Code:**
```bash
# We recommend using tools like `black` for formatting and `flake8` or `pylint` for linting.
# pip install black flake8
black .
flake8 .
```

**Commit Message Format:**
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance

**Example:**
```
feat(asr): implement Top-K selection with VRF

Add VRF-weighted random selection from Top-K scored workers.
Includes quota enforcement and fairness boost calculations.

Closes #123
```

### 3. Test Your Changes

```bash
# Run all tests
make test

# Run specific test
go test ./pkg/asr -v

# Run integration tests
make test-integration

# Run benchmarks
make bench
```

### 4. Update Documentation

- Update relevant `.md` files
- Add inline code comments
- Update API documentation
- Add examples if needed

### 5. Checking for Broken Links

Before committing changes to HTML files, run the link checker script to ensure there are no broken links. This script uses `lychee` to find and report any issues.

```bash
# Run the link checker
./tools/check-links.sh
```

> **Note:** The script may report "Network error" for external links if you are in an offline environment. This is expected. Please focus on fixing any "File not found" errors for internal links.

### 6. Commit and Push

```bash
# Add your changes
git add .

# Commit with descriptive message
git commit -m "feat(asr): implement Top-K selection with VRF"

# Push to your fork
git push origin feature/your-feature-name
```

---

## 🔄 Pull Request Process

### 1. Create Pull Request

1. Go to your fork on GitHub
2. Click "New Pull Request"
3. Select base: `axionaxprotocol/axionax-core:main`
4. Select compare: `your-fork:feature/your-feature-name`
5. Fill in the PR template

### 2. PR Template

```markdown
## Description
Brief description of changes

## Related Issue
Closes #123

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] No new warnings generated
- [ ] Tests added/updated
- [ ] All tests passing
```

### 3. Review Process

- Maintainers will review your PR
- Address any requested changes
- Discussion happens in PR comments
- Once approved, PR will be merged

**Review Criteria:**
- ✅ Code quality and style
- ✅ Test coverage
- ✅ Documentation completeness
- ✅ Performance impact
- ✅ Security implications
- ✅ Backward compatibility

### 4. After Merge

```bash
# Update your local main
git checkout main
git pull upstream main

# Delete feature branch
git branch -d feature/your-feature-name
git push origin --delete feature/your-feature-name
```

---

## 🐛 Reporting Bugs

### Before Reporting

1. Check existing issues
2. Try latest version
3. Reproduce the bug consistently
4. Gather relevant information

### Bug Report Template

Use GitHub issue template or include:

```markdown
**Describe the bug**
Clear description of the bug

**To Reproduce**
Steps to reproduce:
1. Step one
2. Step two
3. See error

**Expected behavior**
What should happen

**Actual behavior**
What actually happens

**Environment:**
- OS: [e.g., Ubuntu 22.04]
- Version: [e.g., v1.5.0]
- Go/Rust version: [e.g., 1.21]

**Logs**
Relevant log output

**Additional context**
Any other relevant information
```

---

## 💡 Feature Requests

### Before Requesting

1. Check existing issues/discussions
2. Consider if it aligns with project goals
3. Think about implementation

### Feature Request Template

```markdown
**Problem Statement**
What problem does this solve?

**Proposed Solution**
Describe your proposed solution

**Alternatives Considered**
What alternatives did you consider?

**Additional Context**
Mockups, diagrams, links, etc.
```

---

## 🧪 Testing Guidelines

### Unit Tests

- Test individual functions/methods
- Mock external dependencies
- Cover edge cases
- Aim for >80% coverage

```go
func TestASRTopKSelection(t *testing.T) {
    workers := generateMockWorkers(100)
    asr := NewASR(64) // K = 64
    
    selected := asr.SelectTopK(workers)
    
    assert.Equal(t, 64, len(selected))
    // Additional assertions...
}
```

### Integration Tests

- Test component interactions
- Use testnet or local network
- Test realistic scenarios

### Performance Tests

```bash
# Run benchmarks
make bench

# Profile CPU
go test -cpuprofile=cpu.prof -bench=.

# Profile memory
go test -memprofile=mem.prof -bench=.
```

---

## 📖 Documentation Guidelines

### Code Comments

```go
// ASR implements the Auto-Selection Router for worker assignment.
// It scores workers based on Suitability, Performance, and FairnessBoost,
// then uses VRF to randomly select from the Top-K highest-scoring workers.
type ASR struct {
    k int // Top-K size (governance parameter)
    // ...
}
```

### Documentation Files

- Use Markdown format
- Include code examples
- Add diagrams when helpful
- Keep it up-to-date

---

## 🏆 Recognition

Contributors are recognized in:
- Release notes
- GitHub contributors page
- Community hall of fame
- Special Discord role (for significant contributions)

---

## 📜 Code of Conduct

### Our Pledge

We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity and expression, level of experience, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

**Positive Behavior:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other members

**Unacceptable Behavior:**
- Trolling, insulting/derogatory comments, personal attacks
- Public or private harassment
- Publishing others' private information
- Other conduct reasonably considered inappropriate

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported to conduct@axionax.org. All complaints will be reviewed and investigated promptly and fairly.

---

## 🤝 Community

### Communication Channels

- **Discord**: https://discord.gg/axionax
- **Forum**: https://forum.axionax.org
- **Twitter**: @axionaxprotocol
- **Telegram**: https://t.me/axionax

### Weekly Calls

- **Dev Call**: Thursdays 15:00 UTC
- **Community Call**: First Monday of month, 16:00 UTC

### Getting Help

- Check documentation first
- Search existing issues
- Ask in Discord #dev-help
- Tag maintainers if urgent

---

## 📝 License

By contributing, you agree that your contributions will be licensed under the same [MIT License](./LICENSE) that covers the project.

---

## 🙏 Thank You!

Every contribution, no matter how small, makes a difference. We appreciate your time and effort in making Axionax better!

---

**Questions?** Reach out in Discord or email dev@axionax.org

Last Updated: 2025-01-21 | v1.5.0
