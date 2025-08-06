#!/bin/bash

# SpaceCleaner - Quick Install (Pre-built Binary)
# Downloads and installs pre-built binary for faster installation

set -e

echo "🧹 SpaceCleaner - Quick Binary Installer"
echo "======================================="
echo ""

# Detect OS
OS=""
ARCH=""
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    ARCH="universal"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    ARCH="x86_64"
else
    echo "❌ Unsupported operating system: $OSTYPE"
    echo "SpaceCleaner currently supports macOS and Linux only."
    exit 1
fi

echo "✅ Detected: $OS ($ARCH)"
echo ""

# Create installation directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Download and install binary
BINARY_URL="https://github.com/andrapra-work/spacecleaner/releases/latest/download/spacecleaner-$OS-$ARCH"
TEMP_BINARY="/tmp/spacecleaner"

echo "📥 Downloading SpaceCleaner binary..."
echo "From: $BINARY_URL"

if command -v curl &> /dev/null; then
    curl -sSL -o "$TEMP_BINARY" "$BINARY_URL"
elif command -v wget &> /dev/null; then
    wget -q -O "$TEMP_BINARY" "$BINARY_URL"
else
    echo "❌ Neither curl nor wget found. Please install one of them."
    exit 1
fi

if [ ! -f "$TEMP_BINARY" ] || [ ! -s "$TEMP_BINARY" ]; then
    echo "❌ Download failed or file is empty."
    echo "💡 Try the source installation instead:"
    echo "   curl -sSL https://raw.githubusercontent.com/andrapra-work/spacecleaner/main/install.sh | bash"
    exit 1
fi

# Make executable and install
chmod +x "$TEMP_BINARY"
mv "$TEMP_BINARY" "$INSTALL_DIR/spacecleaner"

echo "✅ Binary downloaded and installed!"
echo ""

# Add to PATH if not already there
SHELL_RC=""
case $SHELL in
    */bash)
        SHELL_RC="$HOME/.bashrc"
        ;;
    */zsh)
        SHELL_RC="$HOME/.zshrc"
        ;;
    *)
        SHELL_RC="$HOME/.profile"
        ;;
esac

# Check if PATH is already configured
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo "🔧 Adding SpaceCleaner to your PATH..."
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
    export PATH="$HOME/.local/bin:$PATH"
    echo "✅ Added to $SHELL_RC"
else
    echo "✅ PATH already configured"
fi

echo ""
echo "🎉 Quick Installation Complete!"
echo "==============================="
echo ""
echo "📍 SpaceCleaner is installed at: $INSTALL_DIR/spacecleaner"
echo ""
echo "🚀 Quick Start:"
echo "   1. Open a new Terminal window (to load PATH)"
echo "   2. Run: spacecleaner"
echo ""
echo "💡 Common Commands:"
echo "   spacecleaner              # Interactive mode"
echo "   spacecleaner scan         # Check storage usage"  
echo "   spacecleaner quick        # Quick safe cleanup"
echo "   spacecleaner --dry-run    # Preview mode"
echo ""
echo "🛡️  Always run 'spacecleaner --dry-run' first to preview changes!"
echo ""

# Test installation
if command -v spacecleaner &> /dev/null; then
    echo "✅ Installation verified - SpaceCleaner is ready to use!"
    echo ""
    echo "Would you like to run a quick scan now? (y/n)"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        echo ""
        spacecleaner scan
    fi
else
    echo "⚠️  Please restart your terminal or run: source $SHELL_RC"
fi