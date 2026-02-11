#!/bin/sh
# Installation script for lob
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)
            OS="unknown-linux-gnu"
            ;;
        Darwin)
            OS="apple-darwin"
            ;;
        *)
            echo "${RED}Unsupported OS: $OS${NC}"
            exit 1
            ;;
    esac

    case "$ARCH" in
        x86_64)
            ARCH="x86_64"
            ;;
        arm64|aarch64)
            ARCH="aarch64"
            ;;
        *)
            echo "${RED}Unsupported architecture: $ARCH${NC}"
            exit 1
            ;;
    esac

    PLATFORM="${ARCH}-${OS}"
}

# Get latest version from GitHub
get_latest_version() {
    VERSION="${LOB_VERSION:-latest}"

    if [ "$VERSION" = "latest" ]; then
        VERSION=$(curl -s https://api.github.com/repos/olirice/lob/releases/latest | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/')
    fi

    if [ -z "$VERSION" ]; then
        echo "${RED}Failed to get latest version${NC}"
        exit 1
    fi
}

# Download and install
install_lob() {
    detect_platform
    get_latest_version

    echo "${GREEN}Installing lob v${VERSION} for ${PLATFORM}...${NC}"

    INSTALL_DIR="${LOB_INSTALL_DIR:-$HOME/.local/bin}"
    mkdir -p "$INSTALL_DIR"

    TMP_DIR=$(mktemp -d)
    cd "$TMP_DIR"

    ARCHIVE="lob-${VERSION}-${PLATFORM}.tar.gz"
    URL="https://github.com/olirice/lob/releases/download/v${VERSION}/${ARCHIVE}"

    echo "Downloading from $URL..."

    if command -v curl > /dev/null 2>&1; then
        curl -fsSL "$URL" -o "$ARCHIVE"
    elif command -v wget > /dev/null 2>&1; then
        wget -q "$URL" -O "$ARCHIVE"
    else
        echo "${RED}Error: curl or wget required${NC}"
        exit 1
    fi

    echo "Extracting..."
    tar -xzf "$ARCHIVE"

    echo "Installing to $INSTALL_DIR..."
    mv lob "$INSTALL_DIR/lob"
    chmod +x "$INSTALL_DIR/lob"

    cd -
    rm -rf "$TMP_DIR"

    echo "${GREEN}✓ lob installed successfully!${NC}"
    echo ""
    echo "Location: $INSTALL_DIR/lob"
    echo ""

    # Check if in PATH
    case ":$PATH:" in
        *":$INSTALL_DIR:"*)
            echo "Run 'lob --help' to get started"
            ;;
        *)
            echo "${YELLOW}Note: $INSTALL_DIR is not in your PATH${NC}"
            echo "Add this to your shell profile:"
            echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
            ;;
    esac
}

install_lob
