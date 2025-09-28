# Contributing to PingTest

Thank you for your interest in contributing to PingTest! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git
- A terminal that supports TUI (most modern terminals)

### Development Setup

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/pingtest.git
   cd pingtest
   ```

3. Install development dependencies:
   ```bash
   cargo install cargo-watch cargo-audit cargo-fmt cargo-clippy
   ```

4. Build the project:
   ```bash
   cargo build
   ```

5. Run tests:
   ```bash
   cargo test
   ```

## 🛠️ Development Workflow

### Code Style

We use `rustfmt` and `clippy` for code formatting and linting:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test
```

### Running the Application

```bash
# Run in debug mode
cargo run

# Run with specific options
cargo run -- --quick --theme dracula

# Run with debug logging
RUST_LOG=debug cargo run
```

### Watching for Changes

During development, you can use `cargo watch` to automatically rebuild and run the application:

```bash
cargo watch -x run
```

## 📝 Making Changes

### Branch Naming

Use descriptive branch names:
- `feature/add-new-theme`
- `fix/ping-calculation-bug`
- `docs/update-readme`
- `refactor/ui-components`

### Commit Messages

Follow conventional commit format:
- `feat: add new dracula theme`
- `fix: resolve ping calculation error`
- `docs: update installation instructions`
- `test: add integration tests for speed test`
- `refactor: simplify UI component structure`

### Pull Request Process

1. Create a feature branch from `main`
2. Make your changes with appropriate tests
3. Ensure all tests pass: `cargo test`
4. Run benchmarks if applicable: `cargo bench`
5. Update documentation if needed
6. Submit a pull request with a clear description

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_speed_test_basic

# Run integration tests
cargo test --test integration_tests

# Run benchmarks
cargo bench
```

### Test Coverage

We aim for high test coverage. When adding new features:
- Add unit tests for individual functions
- Add integration tests for complete workflows
- Add benchmarks for performance-critical code

## 🎨 UI Development

### Theme Development

To add a new theme:

1. Add the theme to `src/ui/theme.rs`
2. Implement the `ThemeColors` struct
3. Add the theme to the `ThemeManager::new()` method
4. Update the theme list in the README
5. Add tests for the new theme

### Component Development

When adding new UI components:

1. Add the component to `src/ui/components.rs`
2. Follow the existing patterns for styling and layout
3. Ensure components are responsive to terminal size
4. Add tests for component rendering

## 🌐 Network Testing

### Speed Test Implementation

The speed test implementation is in `src/network/speedtest.rs`. When modifying:

1. Ensure compatibility with speedtest.net API
2. Handle network errors gracefully
3. Provide accurate measurements
4. Support different connection counts and durations

### Ping Analysis

Ping analysis is in `src/ping/analyzer.rs`. When enhancing:

1. Maintain accuracy of ping measurements
2. Calculate jitter and packet loss correctly
3. Support different ping targets
4. Handle network timeouts appropriately

## 📊 Performance

### Benchmarking

We use Criterion for benchmarking. To add benchmarks:

1. Add benchmark functions to `benches/network_bench.rs`
2. Focus on performance-critical code paths
3. Include various input sizes and scenarios
4. Document performance expectations

### Performance Guidelines

- Minimize allocations in hot paths
- Use async/await appropriately
- Cache expensive computations
- Profile before optimizing

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Environment**: OS, Rust version, terminal type
2. **Steps to reproduce**: Clear, minimal steps
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Logs**: Include relevant log output
6. **Screenshots**: If UI-related

## 💡 Feature Requests

When suggesting features:

1. **Use case**: Describe the problem you're trying to solve
2. **Proposed solution**: How you envision it working
3. **Alternatives**: Other approaches you've considered
4. **Implementation**: Any technical considerations

## 📚 Documentation

### Code Documentation

- Use Rust doc comments for public APIs
- Include examples in documentation
- Document error conditions
- Explain complex algorithms

### User Documentation

- Keep README.md up to date
- Document all command-line options
- Provide clear examples
- Include troubleshooting section

## 🔒 Security

### Security Considerations

- Validate all user input
- Handle network errors safely
- Don't log sensitive information
- Use secure defaults

### Reporting Security Issues

For security vulnerabilities, please email makalin@gmail.com instead of creating a public issue.

## 📄 License

By contributing to PingTest, you agree that your contributions will be licensed under the MIT License.

## 🙏 Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Email**: makalin@gmail.com for private matters

## 🎯 Contribution Areas

We welcome contributions in:

- **Code**: Bug fixes, new features, performance improvements
- **Documentation**: README updates, code comments, tutorials
- **Testing**: Test cases, benchmarks, integration tests
- **UI/UX**: Themes, component improvements, accessibility
- **Platform Support**: Windows, macOS, Linux specific improvements
- **Localization**: Translations for different languages

Thank you for contributing to PingTest! 🚀