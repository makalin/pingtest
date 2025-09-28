# 🚀 PingTest

A beautiful, fast, and feature-rich terminal-based internet speed test application written in Rust.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg?style=for-the-badge)

## ✨ Features

### 🚀 Core Speed Testing
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

### 🛠️ Advanced Network Tools
- 🔍 **Network Diagnostics** - Comprehensive network health assessment
- 📊 **Bandwidth Monitoring** - Real-time bandwidth usage tracking
- 🛤️ **Traceroute Analysis** - Advanced route tracing with path optimization
- 🌐 **DNS Testing** - Multi-record DNS resolution and server benchmarking
- 🔍 **Port Scanning** - Advanced port scanning with service detection
- 🌐 **Network Discovery** - Comprehensive network host discovery
- 📶 **WiFi Analysis** - WiFi network scanning and security assessment
- 📈 **Quality Analysis** - Network quality assessment with trend analysis
- 📊 **Statistics Collection** - System and network statistics monitoring
- 🗺️ **Topology Discovery** - Network topology mapping and analysis

## 🎥 Demo

![PingTest Demo](https://via.placeholder.com/800x400/1e1e2e/89b4fa?text=PingTest+TUI+Demo+Animation)

## 📦 Installation

### 🚀 Quick Download (Recommended)

**Download and run the pre-built executable:**

```bash
# Download the installation script
curl -O https://raw.githubusercontent.com/makalin/pingtest/main/download.sh
chmod +x download.sh
./download.sh
```

This will automatically detect your platform and download the appropriate executable.

### 📥 Manual Download

**Download the executable for your platform:**

- **macOS (Intel)**: `pingtest-macos-x86_64`
- **Linux (x86_64)**: `pingtest-linux-x86_64` 
- **Windows (x86_64)**: `pingtest-windows-x86_64.exe`

After downloading, make it executable and run:

```bash
chmod +x pingtest-macos-x86_64
./pingtest-macos-x86_64 --help
```

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

### Basic Speed Testing

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

### 🛠️ Advanced Network Tools

```bash
# Network Diagnostics - Comprehensive health check
pingtest --tools diagnose --format json --output diagnostics.json

# Bandwidth Monitoring - Real-time usage tracking
pingtest --tools monitor --interface eth0 --duration 60 --export-csv bandwidth.csv

# Traceroute Analysis - Route tracing with optimization
pingtest --tools trace google.com --max-hops 20 --format json --output trace.json

# DNS Testing - Resolution and benchmarking
pingtest --tools dns google.com --record-type A --benchmark

# Port Scanning - Service detection and security assessment
pingtest --tools scan-ports 192.168.1.1 --ports common --scan-type service

# Network Discovery - Host discovery and analysis
pingtest --tools scan-network 192.168.1.0/24 --scan-type comprehensive

# WiFi Analysis - Network scanning and security assessment
pingtest --tools wifi --interface wlan0 --scan-type comprehensive

# Quality Analysis - Network performance assessment
pingtest --tools quality --duration 120 --interval 10 --format json

# Statistics Collection - System and network monitoring
pingtest --tools stats --duration 300 --export-all stats.json
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

#### Speed Testing Options
```
USAGE:
    pingtest [OPTIONS]

SPEED TEST OPTIONS:
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
        --tools                 Access advanced network tools
    -h, --help                  Print help information
    -V, --version               Print version information
```

#### Advanced Network Tools
```
NETWORK TOOLS:
    diagnose                    Run comprehensive network diagnostics
    monitor                     Monitor bandwidth usage in real-time
    trace                       Perform advanced traceroute analysis
    dns                         DNS resolution testing and benchmarking
    scan-ports                  Advanced port scanning with service detection
    scan-network                Network host discovery and analysis
    wifi                        WiFi network scanning and analysis
    quality                     Network quality assessment and analysis
    stats                       System and network statistics collection

EXAMPLES:
    pingtest --tools diagnose --format json --output diagnostics.json
    pingtest --tools monitor --interface eth0 --duration 60 --export-csv bandwidth.csv
    pingtest --tools trace google.com --max-hops 20 --timeout 5
    pingtest --tools dns google.com --record-type A --benchmark
    pingtest --tools scan-ports 192.168.1.1 --ports common --scan-type service
    pingtest --tools scan-network 192.168.1.0/24 --scan-type comprehensive
    pingtest --tools wifi --interface wlan0 --scan-type comprehensive
    pingtest --tools quality --duration 120 --interval 10 --format json
    pingtest --tools stats --duration 300 --export-all stats.json
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

## 📚 Documentation

### 📖 Detailed Tool Documentation
For comprehensive documentation of all advanced network tools, see:
- **[TOOLS.md](TOOLS.md)** - Complete guide to all network analysis tools
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development and contribution guidelines

### 🔧 Tool Categories

#### 🔍 **Diagnostic Tools**
- **Network Diagnostics** - Comprehensive network health assessment
- **Quality Analysis** - Network performance evaluation with trend analysis
- **Statistics Collection** - System and network monitoring

#### 🌐 **Discovery Tools**
- **Network Scanner** - Host discovery and network mapping
- **Port Scanner** - Service detection and security assessment
- **WiFi Analyzer** - Wireless network analysis and security

#### 📊 **Monitoring Tools**
- **Bandwidth Monitor** - Real-time bandwidth usage tracking
- **Traceroute** - Advanced route analysis and optimization
- **DNS Resolver** - DNS testing and server benchmarking

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

## 🎯 Use Cases

### 👨‍💼 **Network Administrators**
- Comprehensive network health monitoring
- Performance optimization and capacity planning
- Troubleshooting connectivity issues
- Network security assessment

### 🔒 **Security Professionals**
- Network vulnerability scanning
- Port scanning and service detection
- WiFi security analysis
- Network topology mapping

### 👨‍💻 **Developers**
- Network debugging and diagnostics
- Performance testing and optimization
- API endpoint testing
- Infrastructure monitoring

### 🏢 **Enterprise Use**
- Network infrastructure monitoring
- SLA compliance verification
- Capacity planning and optimization
- Security compliance auditing

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
