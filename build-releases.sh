#!/bin/bash

# Build pre-compiled binaries for different platforms
# This script helps create release binaries for distribution

set -e

echo "🏗️ Building SpaceCleaner for Multiple Platforms"
echo "==============================================="

# Ensure we're in the project directory
cd "$(dirname "$0")"

# Create releases directory
mkdir -p releases

# Build for current platform (will be either macOS or Linux)
echo "📦 Building for current platform..."
cargo build --release

# Determine current platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    CURRENT_PLATFORM="macos-universal"
    BINARY_EXT=""
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    CURRENT_PLATFORM="linux-x86_64"
    BINARY_EXT=""
else
    echo "❌ Unsupported platform: $OSTYPE"
    exit 1
fi

# Package current platform
echo "📦 Packaging $CURRENT_PLATFORM..."
cp target/release/spacecleaner releases/spacecleaner-$CURRENT_PLATFORM
tar -czf releases/spacecleaner-$CURRENT_PLATFORM.tar.gz -C releases spacecleaner-$CURRENT_PLATFORM
rm releases/spacecleaner-$CURRENT_PLATFORM

# Create installation package with installer
echo "📦 Creating complete installation package..."
mkdir -p releases/spacecleaner-installer
cp target/release/spacecleaner releases/spacecleaner-installer/
cp install.sh releases/spacecleaner-installer/
cp spacecleaner-gui.sh releases/spacecleaner-installer/
cp README.md releases/spacecleaner-installer/

# Create simple installer README
cat > releases/spacecleaner-installer/INSTALL.md << 'EOF'
# SpaceCleaner - Installation Instructions

## 🚀 One-Click Install (Recommended)

Just run this command in Terminal:

```bash
./install.sh
```

This will:
- Install SpaceCleaner to your system
- Add it to your PATH
- Create desktop shortcuts (macOS)
- Handle all dependencies automatically

## 📱 Manual Install

1. Copy `spacecleaner` to `/usr/local/bin/` or `~/.local/bin/`
2. Make it executable: `chmod +x spacecleaner`
3. Run: `spacecleaner --help`

## 🖥️ GUI Version (macOS)

Run `./spacecleaner-gui.sh` for a simple point-and-click interface.

## 🛡️ Safety First

Always run `spacecleaner --dry-run` first to preview what will be cleaned!
EOF

tar -czf releases/spacecleaner-installer-$CURRENT_PLATFORM.tar.gz -C releases spacecleaner-installer
rm -rf releases/spacecleaner-installer

echo ""
echo "✅ Build Complete!"
echo ""
echo "📦 Created packages:"
ls -la releases/
echo ""
echo "🚀 Ready for distribution!"
echo ""
echo "For non-technical users, share: spacecleaner-installer-$CURRENT_PLATFORM.tar.gz"
echo "This includes the binary + easy installer + GUI wrapper."