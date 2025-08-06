#!/bin/bash

# SpaceCleaner - One-Click Installer
# For macOS and Linux users

set -e

echo "🧹 SpaceCleaner - Easy Storage Cleanup Tool"
echo "============================================="
echo ""

# Detect OS
OS=""
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
else
    echo "❌ Unsupported operating system: $OSTYPE"
    echo "SpaceCleaner currently supports macOS and Linux only."
    exit 1
fi

echo "✅ Detected: $OS"
echo ""

# Check if we need to install Rust
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust (required to build SpaceCleaner)..."
    echo "This is a one-time setup that won't affect your system."
    echo ""
    
    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    
    echo "✅ Rust installed successfully!"
    echo ""
else
    echo "✅ Rust already installed"
    source ~/.cargo/env 2>/dev/null || true
    echo ""
fi

# Create installation directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Build SpaceCleaner
echo "🔨 Building SpaceCleaner..."
echo "This may take a few minutes on first install..."
echo ""

# Build in release mode
cargo build --release --quiet

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi

# Copy binary to user's local bin
cp target/release/spacecleaner "$INSTALL_DIR/"

echo ""
echo "📦 Installing SpaceCleaner to $INSTALL_DIR..."

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
    echo ""
    echo "🔧 Adding SpaceCleaner to your PATH..."
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
    export PATH="$HOME/.local/bin:$PATH"
    echo "✅ Added to $SHELL_RC"
else
    echo "✅ PATH already configured"
fi

# Create desktop shortcut for macOS
if [[ "$OS" == "macos" ]]; then
    echo ""
    echo "🖥️  Creating desktop shortcuts..."
    
    # Create a simple AppleScript app
    DESKTOP_APP_DIR="$HOME/Desktop/SpaceCleaner.app"
    mkdir -p "$DESKTOP_APP_DIR/Contents/MacOS"
    
    # Create the executable script
    cat > "$DESKTOP_APP_DIR/Contents/MacOS/SpaceCleaner" << 'EOF'
#!/bin/bash
export PATH="$HOME/.local/bin:$PATH"
open -a Terminal.app "$HOME/.local/bin/spacecleaner"
EOF
    
    chmod +x "$DESKTOP_APP_DIR/Contents/MacOS/SpaceCleaner"
    
    # Create Info.plist
    cat > "$DESKTOP_APP_DIR/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>SpaceCleaner</string>
    <key>CFBundleIconFile</key>
    <string></string>
    <key>CFBundleIdentifier</key>
    <string>com.spacecleaner.app</string>
    <key>CFBundleName</key>
    <string>SpaceCleaner</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
</dict>
</plist>
EOF
    
    echo "✅ Created desktop app: ~/Desktop/SpaceCleaner.app"
fi

echo ""
echo "🎉 Installation Complete!"
echo "========================"
echo ""
echo "📍 SpaceCleaner is installed at: $INSTALL_DIR/spacecleaner"
echo ""
echo "🚀 Quick Start:"
echo "   1. Open a new Terminal window (to load PATH)"
echo "   2. Run: spacecleaner"
echo "   3. Or double-click SpaceCleaner.app on your Desktop (macOS)"
echo ""
echo "💡 Common Commands:"
echo "   spacecleaner              # Interactive mode"
echo "   spacecleaner scan         # Check storage usage"  
echo "   spacecleaner quick        # Quick safe cleanup"
echo "   spacecleaner --help       # Show all options"
echo ""
echo "🛡️  Always run 'spacecleaner --dry-run' first to preview changes!"
echo ""
echo "Need help? Check the README: $HOME/Code/spacecleaner/README.md"
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