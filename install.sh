#!/bin/bash

# PingTest Installation Script
# This script installs PingTest from source

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check Rust installation
check_rust() {
    if ! command_exists cargo; then
        print_error "Rust is not installed!"
        print_status "Please install Rust from https://rustup.rs/"
        print_status "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    local rust_version=$(rustc --version | cut -d' ' -f2)
    print_success "Rust $rust_version is installed"
}

# Function to check if we're in the right directory
check_directory() {
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml not found!"
        print_status "Please run this script from the PingTest project directory"
        exit 1
    fi
    
    if [ ! -f "src/main.rs" ]; then
        print_error "Source files not found!"
        print_status "Please ensure you're in the correct PingTest directory"
        exit 1
    fi
    
    print_success "PingTest project directory found"
}

# Function to build the project
build_project() {
    print_status "Building PingTest..."
    
    if cargo build --release; then
        print_success "Build completed successfully"
    else
        print_error "Build failed!"
        exit 1
    fi
}

# Function to run tests
run_tests() {
    print_status "Running tests..."
    
    if cargo test; then
        print_success "All tests passed"
    else
        print_error "Tests failed!"
        exit 1
    fi
}

# Function to install the binary
install_binary() {
    print_status "Installing PingTest..."
    
    if cargo install --path .; then
        print_success "PingTest installed successfully"
    else
        print_error "Installation failed!"
        exit 1
    fi
}

# Function to verify installation
verify_installation() {
    if command_exists pingtest; then
        local version=$(pingtest --version 2>/dev/null || echo "unknown")
        print_success "PingTest $version is now installed and ready to use!"
        print_status "Run 'pingtest' to start the application"
        print_status "Run 'pingtest --help' to see all available options"
    else
        print_error "Installation verification failed!"
        exit 1
    fi
}

# Function to show usage information
show_usage() {
    echo "PingTest Installation Script"
    echo "==========================="
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --help, -h          Show this help message"
    echo "  --no-test           Skip running tests"
    echo "  --no-install        Only build, don't install"
    echo "  --test-only         Only run tests"
    echo "  --build-only        Only build the project"
    echo ""
    echo "Examples:"
    echo "  $0                  # Full installation with tests"
    echo "  $0 --no-test        # Install without running tests"
    echo "  $0 --build-only     # Only build the project"
    echo "  $0 --test-only      # Only run tests"
}

# Main installation function
main() {
    local run_tests=true
    local install_binary=true
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                show_usage
                exit 0
                ;;
            --no-test)
                run_tests=false
                shift
                ;;
            --no-install)
                install_binary=false
                shift
                ;;
            --test-only)
                install_binary=false
                shift
                ;;
            --build-only)
                run_tests=false
                install_binary=false
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    print_status "Starting PingTest installation..."
    echo ""
    
    # Check prerequisites
    check_rust
    check_directory
    echo ""
    
    # Build the project
    build_project
    echo ""
    
    # Run tests if requested
    if [ "$run_tests" = true ]; then
        run_tests
        echo ""
    fi
    
    # Install binary if requested
    if [ "$install_binary" = true ]; then
        install_binary
        echo ""
        verify_installation
    else
        print_success "Build completed successfully"
        print_status "Binary is available at: target/release/pingtest"
    fi
    
    echo ""
    print_success "Installation process completed!"
    
    if [ "$install_binary" = true ]; then
        echo ""
        print_status "Quick start:"
        echo "  pingtest                    # Run interactive speed test"
        echo "  pingtest --quick            # Run quick test"
        echo "  pingtest --theme dracula    # Run with specific theme"
        echo "  pingtest --help             # Show all options"
    fi
}

# Run main function with all arguments
main "$@"