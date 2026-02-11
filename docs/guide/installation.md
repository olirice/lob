# Installation

## Prerequisites

- **Rust toolchain** (1.70+) - Required for now (embedded toolchain coming soon)
- **Git** - For cloning the repository

### Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or visit [https://rustup.rs/](https://rustup.rs/) for other installation methods.

## Install lob

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/olirice/lob.git
cd lob

# Build and install
cargo install --path crates/lob-cli

# Verify installation
lob --version
```

### Build for Development

```bash
# Clone the repository
git clone https://github.com/olirice/lob.git
cd lob

# Build in debug mode
cargo build

# Run tests
cargo test --all

# Run lob in dev mode
cargo run -- '_.take(5)'
```

## Verify Installation

Test that lob is working correctly:

```bash
# Simple test
echo -e "hello\nworld" | lob '_.map(|x| x.to_uppercase())'

# Should output:
# "HELLO"
# "WORLD"
```

## Troubleshooting

### "rustc not found"

Make sure Rust is installed and in your PATH:

```bash
rustc --version
cargo --version
```

### Build Errors

Try updating Rust to the latest stable version:

```bash
rustup update stable
```

### Permission Denied

On Unix systems, ensure the binary is executable:

```bash
chmod +x ~/.cargo/bin/lob
```

## Next Steps

- [Quick Start Guide](quickstart.md) - Learn basic usage
- [CLI Usage](cli.md) - Explore all CLI options
- [API Reference](../api/overview.md) - Discover available operations
