#!/usr/bin/env bash
set -e

echo "🚀 Installing Vyzorix CLI..."

OS="$(uname -s)"
ARCH="$(uname -m)"

if [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
        TARGET="vyzorix-macos-arm64"
    else
        TARGET="vyzorix-macos-intel"
    fi
elif [ "$OS" = "Linux" ]; then
    TARGET="vyzorix-linux-amd64"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

DOWNLOAD_URL="https://github.com/vyzorix/vyzorix-cli/releases/latest/download/${TARGET}"
BIN_DIR="/usr/local/bin"

echo "Downloading $TARGET..."
curl -fsSL -o vyzorix "$DOWNLOAD_URL"

echo "Installing to $BIN_DIR..."
chmod +x vyzorix
sudo mv vyzorix "$BIN_DIR/"

echo "✅ Vyzorix CLI successfully installed!"
echo "Run 'vyzorix --help' to get started."
