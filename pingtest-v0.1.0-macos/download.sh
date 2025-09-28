#!/bin/bash

# PingTest Download Script
# Downloads and installs PingTest executable

set -e

echo "🚀 PingTest - Internet Speed Test"
echo "=================================="
echo

# Detect platform
PLATFORM=""
ARCH=""
if [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macos"
    ARCH="x86_64"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux"
    ARCH="x86_64"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    PLATFORM="windows"
    ARCH="x86_64"
else
    echo "❌ Unsupported platform: $OSTYPE"
    echo "Please build from source or contact support."
    exit 1
fi

EXECUTABLE_NAME="pingtest-${PLATFORM}-${ARCH}"
if [[ "$PLATFORM" == "windows" ]]; then
    EXECUTABLE_NAME="${EXECUTABLE_NAME}.exe"
fi

echo "📋 Platform detected: $PLATFORM ($ARCH)"
echo "📦 Executable: $EXECUTABLE_NAME"
echo

# Check if executable exists in releases directory
if [[ -f "releases/$EXECUTABLE_NAME" ]]; then
    echo "✅ Found executable in releases directory"
    
    # Copy to local directory
    cp "releases/$EXECUTABLE_NAME" "./pingtest"
    chmod +x "./pingtest"
    
    echo "📁 Installed to: $(pwd)/pingtest"
    echo
    echo "🎉 Installation complete!"
    echo
    echo "Usage:"
    echo "  ./pingtest --help          # Show help"
    echo "  ./pingtest --quick         # Quick test"
    echo "  ./pingtest --duration 10   # 10-second test"
    echo "  ./pingtest --export results.json  # Export results"
    echo
    
    # Test the installation
    echo "🧪 Testing installation..."
    ./pingtest --version
    echo
    echo "✅ PingTest is ready to use!"
    
else
    echo "❌ Executable not found: releases/$EXECUTABLE_NAME"
    echo
    echo "Available executables:"
    ls -la releases/ 2>/dev/null || echo "No releases directory found"
    echo
    echo "Please build from source:"
    echo "  1. Install Rust: https://rustup.rs/"
    echo "  2. Clone repository: git clone <repo-url>"
    echo "  3. Build: cargo build --release"
    echo "  4. Run: ./target/release/pingtest"
    exit 1
fi