# 🚀 PingTest

A beautiful, fast, and feature-rich terminal-based internet speed test application written in Rust.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg?style=for-the-badge)

![Demo](https://via.placeholder.com/800x400/000000/FFFFFF?text=PingTest+Demo)

## ✨ Features

- 🎨 **Beautiful Terminal UI** - Real-time graphs and visualizations powered by [ratatui](https://github.com/ratatui-org/ratatui)
- ⚡ **Fast & Accurate** - Multi-threaded speed testing with precise measurements
- 📊 **Real-time Charts** - Live download/upload speed graphs during tests
- 🌍 **Server Selection** - Automatic best server selection or manual server choice
- 📈 **History Tracking** - Save and compare previous test results
- 🎯 **Minimal Dependencies** - Single binary, no external requirements
- 🌈 **Colorful Themes** - Multiple color schemes (Dracula, Nord, Solarized, etc.)
- 📱 **Responsive Design** - Adapts to any terminal size
- 💾 **Export Results** - JSON/CSV export for further analysis
- 🔒 **Privacy Focused** - No data collection, everything stays local
- 🏓 **Smart Ping Testing** - Advanced latency analysis with jitter detection

## 🎥 Demo

![PingTest Demo](https://via.placeholder.com/800x400/1e1e2e/89b4fa?text=PingTest+TUI+Demo+Animation)

## 📦 Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/makalin/pingtest.git
cd pingtest

# Build and install
cargo install --path .

# Or install directly from crates.io (when published)
cargo install pingtest
```

### Pre-built Binaries

Download the latest release for your platform:

- [Linux (x86_64)](https://github.com/makalin/pingtest/releases/latest)
- [macOS (x86_64 & ARM64)](https://github.com/makalin/pingtest/releases/latest)
- [Windows (x86_64)](https://github.com/makalin/pingtest/releases/latest)

```bash
# Linux/macOS
curl -L https://github.com/makalin/pingtest/releases/latest/download/pingtest-linux-x86_64.tar.gz | tar xz
sudo mv pingtest /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/makalin/pingtest/releases/latest/download/pingtest-windows-x86_64.zip" -OutFile "pingtest.zip"
Expand-Archive -Path "pingtest.zip" -DestinationPath "C:\Program Files\pingtest"
```

### Package Managers

```bash
# Homebrew (macOS/Linux)
brew tap makalin/pingtest
brew install pingtest

# AUR (Arch Linux)
yay -S pingtest

# Scoop (Windows)
scoop bucket add pingtest https://github.com/makalin/pingtest
scoop install pingtest
```

## 🚀 Usage

### Basic Usage

```bash
# Run interactive speed test
pingtest

# Quick test with default settings
pingtest --quick

# Test with specific server
pingtest --server 12345

# Export results to JSON
pingtest --export results.json

# Run with specific theme
pingtest --theme dracula
```

### Advanced Usage

```bash
# Custom test duration (seconds)
pingtest --duration 30

# Test only download speed
pingtest --no-upload

# Test only upload speed
pingtest --no-download

# Specify number of connections
pingtest --connections 8

# Save to history file
pingtest --save --tag "home-network"

# Compare with previous tests
pingtest --compare --days 7

# Advanced ping analysis
pingtest --ping-analysis --jitter-detection
```

### Command Line Options

```
USAGE:
    pingtest [OPTIONS]

OPTIONS:
    -q, --quick                 Run quick test with minimal output
    -s, --server <ID>           Test against specific server ID
    -d, --duration <SECONDS>    Test duration in seconds [default: 15]
    -c, --connections <NUM>     Number of parallel connections [default: 4]
        --no-download           Skip download test
        --no-upload             Skip upload test
        --ping-analysis         Enable advanced ping analysis
        --jitter-detection      Calculate network jitter
    -t, --theme <THEME>         Color theme [default: auto]
    -e, --export <FILE>         Export results to JSON/CSV
        --save                  Save results to history
        --tag <TAG>             Add tag to saved result
        --compare               Compare with previous results
        --days <DAYS>           Compare with results from last N days
    -h, --help                  Print help information
    -V, --version               Print version information
```

## 🎨 Themes

Available color themes:

| Theme | Preview |
|-------|---------|
| `auto` | Automatically detect based on terminal |
| `dracula` | 🧛‍♂️ Dark theme with purple accents |
| `nord` | 🏔️ Arctic-inspired color scheme |
| `solarized-dark` | 🌅 Classic Solarized dark |
| `solarized-light` | ☀️ Classic Solarized light |
| `monokai` | 🌈 Vibrant Monokai colors |
| `github` | 🐙 GitHub's color scheme |
| `tokyo-night` | 🗼 Tokyo Night theme |

Switch themes during runtime with `Ctrl+T`.

## 📊 Visualizations

### Real-time Graphs
- **Download Speed**: Live line graph showing download progression
- **Upload Speed**: Live line graph showing upload progression  
- **Latency**: Real-time ping measurements with jitter calculation
- **Progress Bars**: Visual progress indicators for each test phase
- **Ping Heatmap**: Visual representation of ping stability

### Post-Test Summary
- **Speed Meter**: Beautiful gauge showing your connection speed
- **Comparison Chart**: Compare with previous tests or ISP averages
- **Geographic Info**: Server location and distance
- **Network Quality**: Overall connection quality score
- **Ping Analysis**: Detailed latency and jitter statistics

## 📈 History & Analytics

```bash
# Show test history
pingtest history

# Show statistics for last 30 days
pingtest stats --days 30

# Export history to CSV
pingtest export --format csv --output history.csv

# Clear history
pingtest history --clear

# Show ping trends
pingtest trends --metric latency
```

## 🔧 Configuration

Create `~/.config/pingtest/config.toml`:

```toml
[general]
theme = "dracula"
default_duration = 15
auto_save = true
default_connections = 4

[display]
show_graph = true
animation_speed = "medium"
update_interval = 100  # milliseconds
show_ping_heatmap = true

[ping]
packet_count = 10
timeout_ms = 1000
jitter_calculation = true

[servers]
preferred_countries = ["US", "CA", "GB"]
max_distance_km = 1000
auto_select = true

[export]
default_format = "json"
include_timestamp = true
include_server_info = true
include_ping_analysis = true
```

## 🏓 Advanced Ping Features

- **Jitter Detection**: Measures ping variability over time
- **Packet Loss Analysis**: Tracks dropped packets during tests
- **Latency History**: Builds latency profiles for different times of day
- **Network Quality Score**: Combines speed and ping metrics
- **Geographic Ping Mapping**: Shows ping times to different regions

## 🛠️ Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development dependencies
cargo install cargo-watch cargo-audit
```

### Building from Source

```bash
# Clone repository
git clone https://github.com/makalin/pingtest.git
cd pingtest

# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Watch for changes during development
cargo watch -x run
```

### Project Structure

```
pingtest/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Main application logic
│   ├── ui/              # UI components
│   ├── network/         # Network testing logic
│   ├── ping/            # Ping analysis features
│   ├── config.rs        # Configuration handling
│   └── utils.rs         # Utility functions
├── tests/               # Integration tests
├── benches/             # Performance benchmarks
└── Cargo.toml           # Project manifest
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

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

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) - For the amazing TUI framework
- [speedtest.net](https://www.speedtest.net/) - For providing the speed test infrastructure
- [Rust community](https://rust-lang.org/community) - For the excellent ecosystem
- [ping](https://en.wikipedia.org/wiki/Ping_(networking_utility)) - The classic network utility that inspired us

## 📞 Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/makalin/pingtest/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/makalin/pingtest/discussions)
- 📧 **Email**: makalin@gmail.com

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=makalin/pingtest&type=Date)](https://star-history.com/#makalin/pingtest&Date)

---

<div align="center">
  <p><b>Made with ❤️ and 🦀</b></p>
  <p>
    <a href="https://github.com/makalin/pingtest">⭐️ Star this project</a> •
    <a href="https://github.com/makalin/pingtest/fork">🍴 Fork this project</a>
  </p>
</div>
```
