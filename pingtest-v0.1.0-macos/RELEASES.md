# 📦 PingTest Releases

## Current Release: v0.1.0

### 🚀 Quick Start

**Download and run immediately:**

```bash
# Download the installation script
curl -O https://raw.githubusercontent.com/makalin/pingtest/main/download.sh
chmod +x download.sh
./download.sh
```

### 📥 Manual Downloads

| Platform | Architecture | Download | Size |
|----------|-------------|----------|------|
| macOS | x86_64 | [pingtest-macos-x86_64](releases/pingtest-macos-x86_64) | ~2.5 MB |
| Linux | x86_64 | *Coming Soon* | ~2.5 MB |
| Windows | x86_64 | *Coming Soon* | ~2.5 MB |

### 🔧 Installation Instructions

1. **Download** the appropriate executable for your platform
2. **Make executable** (Unix systems):
   ```bash
   chmod +x pingtest-macos-x86_64
   ```
3. **Run**:
   ```bash
   ./pingtest-macos-x86_64 --help
   ```

### 🎯 Usage Examples

```bash
# Quick test
./pingtest --quick

# Custom duration
./pingtest --duration 10

# Export results
./pingtest --export results.json

# Skip upload test
./pingtest --no-upload

# Show help
./pingtest --help
```

### 🏗️ Building from Source

If you prefer to build from source:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/makalin/pingtest.git
cd pingtest
cargo build --release

# Run
./target/release/pingtest
```

### 📋 System Requirements

- **OS**: macOS 10.12+, Linux (glibc 2.17+), Windows 10+
- **Architecture**: x86_64
- **Memory**: 10 MB RAM minimum
- **Storage**: 5 MB disk space

### 🔒 Security

- All executables are built from open source code
- No external dependencies or network calls during execution
- All data stays local on your machine
- No telemetry or data collection

### 🐛 Troubleshooting

**Permission denied:**
```bash
chmod +x pingtest-macos-x86_64
```

**Command not found:**
```bash
# Make sure you're in the directory with the executable
ls -la pingtest*
./pingtest-macos-x86_64 --version
```

**Build errors:**
- Ensure you have Rust installed: `rustc --version`
- Update Rust: `rustup update`

### 📞 Support

- **Issues**: [GitHub Issues](https://github.com/makalin/pingtest/issues)
- **Discussions**: [GitHub Discussions](https://github.com/makalin/pingtest/discussions)
- **Documentation**: [README.md](README.md)

---

**Last Updated**: $(date)
**Version**: 0.1.0