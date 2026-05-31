#!/bin/bash

# NFlux Setup Script
# Initializes the development environment

echo "🚀 NFlux Setup"
echo "=============="
echo ""

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Please install Node.js 18+"
    exit 1
fi

if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Please install Rust"
    exit 1
fi

echo "✅ Node.js: $(node --version)"
echo "✅ Rust: $(rustc --version)"
echo ""

# Install npm dependencies
echo "📦 Installing npm dependencies..."
npm install
if [ $? -ne 0 ]; then
    echo "❌ Failed to install npm dependencies"
    exit 1
fi
echo "✅ npm dependencies installed"
echo ""

# Fetch Rust dependencies
echo "📦 Fetching Rust dependencies..."
cd src-tauri
cargo fetch
if [ $? -ne 0 ]; then
    echo "❌ Failed to fetch Rust dependencies"
    exit 1
fi
echo "✅ Rust dependencies fetched"
echo ""

# Check compilation
echo "🔨 Checking Rust compilation..."
cargo check
if [ $? -ne 0 ]; then
    echo "❌ Rust compilation check failed"
    exit 1
fi
echo "✅ Rust compilation verified"
echo ""

cd ..

echo "✨ Setup complete!"
echo ""
echo "Available commands:"
echo "  npm run tauri dev    - Start development server"
echo "  npm run tauri build  - Build for production"
echo ""
echo "Happy coding! 🎉"
