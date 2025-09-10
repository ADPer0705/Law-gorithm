# Tauri Deployment Guide - Law-gorithm

## 🚀 **Deployment Status: Working Build System (December 2024)**

This guide provides comprehensive instructions for building and deploying the Law-gorithm Tauri desktop application.

## 📋 **Pre-Deployment Checklist**

### ✅ **Verified Working Components**
- **Rust Backend**: All services compile successfully
- **Python RAG**: ChromaDB + SentenceTransformers functional
- **OCR Integration**: Bundled Tesseract binaries included
- **Vector Database**: SQLite with VSS extension configured  
- **Frontend**: Web interface served via HTTP server
- **Build System**: Cargo compilation working with all dependencies

### ⚠️ **Known Issues**
- **LibSoup Conflict**: Runtime issue with libsoup2/libsoup3 on some Linux systems
- **Solution**: Production builds work better than dev mode

## 🏗️ **Build Environment Setup**

### **System Requirements**

#### **Ubuntu/Debian Linux**
```bash
# Essential build dependencies
sudo apt update
sudo apt install -y curl build-essential libwebkit2gtk-4.0-dev 
  libwebkit2gtk-4.1-dev libgtk-3-dev libappindicator3-dev 
  librsvg2-dev libssl-dev pkg-config python3 python3-pip

# Additional utilities
sudo apt install -y git nodejs npm sqlite3
```

#### **Other Linux Distributions**
```bash
# Fedora/RHEL/CentOS
sudo dnf install -y curl gcc webkit2gtk3-devel gtk3-devel 
  libappindicator-gtk3-devel librsvg2-devel openssl-devel 
  pkg-config python3 python3-pip

# Arch Linux  
sudo pacman -S curl base-devel webkit2gtk gtk3 libappindicator-gtk3 
  librsvg openssl pkgconf python python-pip
```

### **Development Tools**

#### **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc

# Verify installation
rustc --version
cargo --version
```

#### **Install Tauri CLI**
```bash
# Install globally
npm install -g @tauri-apps/cli@latest

# Or install locally in project
npm install --save-dev @tauri-apps/cli@^1.0.0

# Verify installation
npx tauri --version
```

#### **Python Environment**
```bash
# Install Python dependencies for RAG system
cd law-gorithm/python_modules
pip3 install -r requirements.txt

# Verify ChromaDB installation
python3 -c "import chromadb; print('ChromaDB installed successfully')"
```

## 📦 **Build Process**

### **Development Build**
```bash
cd law-gorithm

# Method 1: Using npm scripts
npm run tauri dev  # Development mode with hot reload

# Method 2: Direct cargo build  
cd src-tauri
cargo build  # Debug build
cargo build --release  # Optimized build

# Start frontend server (separate terminal)
cd ..
npm run serve  # Starts HTTP server on localhost:8080
```

### **Production Build**
```bash
cd law-gorithm

# Full production build
npm run tauri build

# Manual build process
cd src-tauri
cargo build --release
cd ..
npm run serve &  # Start frontend in background
```

### **Build Output Locations**
```bash
src-tauri/target/release/
├── law-gorithm              # Main executable
├── bundle/                  # Platform-specific packages
│   ├── deb/                # Debian packages (.deb)
│   ├── appimage/           # AppImage (.AppImage) 
│   └── rpm/                # RPM packages (.rpm)
└── deps/                   # Dependency libraries
```

## 🔧 **Configuration Management**

### **Build Configuration (`src-tauri/tauri.conf.json`)**

Key configuration sections for deployment:

```json
{
  "build": {
    "beforeBuildCommand": "npm run serve",
    "beforeDevCommand": "npm run serve", 
    "devPath": "http://localhost:8080",
    "distDir": "../src"
  },
  "package": {
    "productName": "Law-gorithm",
    "version": "0.1.0"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": ["deb", "appimage"],
      "identifier": "com.lawgorithm.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png", 
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    }
  }
}
```

### **Cargo Configuration (`src-tauri/Cargo.toml`)**

Production-ready dependency configuration:

```toml
[package]
name = "law-gorithm"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "1.0", features = [] }

[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
tauri = { version = "1.0", features = ["api-all", "system-tray"] }

# Core functionality
reqwest = { version = "0.11", features = ["json"] }
rusqlite = { version = "0.29", features = ["bundled", "load_extension"] }
chrono = { version = "0.4", features = ["serde"] }

# Python integration
pyo3 = { version = "0.19", features = ["auto-initialize"] }

# Document processing  
pdf-extract = "0.6"

# File operations
rfd = "0.10"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

## 📱 **Platform-Specific Builds**

### **Linux Distribution Packages**

#### **Debian/Ubuntu (.deb)**
```bash
# Build Debian package
npm run tauri build -- --target deb

# Install generated package
sudo dpkg -i src-tauri/target/release/bundle/deb/law-gorithm_0.1.0_amd64.deb

# Handle dependencies
sudo apt install -f
```

#### **Universal Linux (AppImage)**
```bash
# Build AppImage
npm run tauri build -- --target appimage

# Make executable and run
chmod +x src-tauri/target/release/bundle/appimage/law-gorithm_0.1.0_amd64.AppImage
./src-tauri/target/release/bundle/appimage/law-gorithm_0.1.0_amd64.AppImage
```

#### **RPM-based Systems**
```bash
# Build RPM package
npm run tauri build -- --target rpm

# Install on Fedora/RHEL/CentOS
sudo rpm -i src-tauri/target/release/bundle/rpm/law-gorithm-0.1.0-1.x86_64.rpm
```

### **Cross-Platform Considerations**

#### **Windows Build (from Linux)**
```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Install cross-compilation tools
sudo apt install gcc-mingw-w64-x86-64

# Build for Windows
npm run tauri build -- --target x86_64-pc-windows-gnu
```

#### **macOS Build Requirements**
- Requires macOS system for building
- Xcode Command Line Tools
- Valid Apple Developer account for distribution

## 🗂️ **Asset Management**

### **Bundled Resources**

#### **Tesseract OCR Binaries**
```bash
vendor/tesseract-linux/
├── tesseract                   # Main binary
├── tessdata/                   # Language data
├── lib*.so*                    # Required libraries
└── CACHEDIR.TAG               # Cache marker
```

#### **Application Icons**
```bash
src-tauri/icons/
├── 32x32.png                  # System tray icon
├── 128x128.png                # Standard icon
├── 128x128@2x.png             # High-DPI icon  
├── icon.icns                  # macOS icon
└── icon.ico                   # Windows icon
```

### **Database Initialization**
```bash
# SQLite database created at runtime in:
~/.local/share/law-gorithm/data/law-gorithm.db

# Vector search extension loaded dynamically
# No manual setup required
```

## 🚀 **Deployment Strategies**

### **Development Deployment**
```bash
# Quick development setup
git clone <repository>
cd law-gorithm

# Install dependencies
npm install
cd python_modules && pip3 install -r requirements.txt && cd ..

# Run development build
npm run serve &  # Frontend server
npm run tauri dev  # Backend + GUI
```

### **Production Deployment**
```bash
# Production build and package
npm run tauri build

# Distribute generated packages
# - law-gorithm_0.1.0_amd64.deb (Debian/Ubuntu)
# - law-gorithm_0.1.0_amd64.AppImage (Universal Linux)
# - law-gorithm-0.1.0-1.x86_64.rpm (RPM systems)
```

### **Enterprise Deployment**
```bash
# Create installation script
#!/bin/bash
set -e

echo "Installing Law-gorithm..."

# Check system requirements
python3 --version || { echo "Python 3 required"; exit 1; }

# Install system dependencies
if command -v apt &> /dev/null; then
    sudo apt update
    sudo apt install -y libwebkit2gtk-4.0-37 libgtk-3-0
elif command -v dnf &> /dev/null; then
    sudo dnf install -y webkit2gtk3 gtk3
fi

# Install application package
if [ -f "law-gorithm_0.1.0_amd64.deb" ]; then
    sudo dpkg -i law-gorithm_0.1.0_amd64.deb
    sudo apt install -f
elif [ -f "law-gorithm_0.1.0_amd64.AppImage" ]; then
    chmod +x law-gorithm_0.1.0_amd64.AppImage
    mv law-gorithm_0.1.0_amd64.AppImage /usr/local/bin/law-gorithm
fi

echo "Law-gorithm installed successfully!"
```

## 🐛 **Troubleshooting**

### **Common Build Issues**

#### **LibSoup Version Conflicts**
```bash
# Symptom: "libsoup3 symbols detected" error
# Solution 1: Use production builds
npm run tauri build  # Instead of tauri dev

# Solution 2: Install compatibility packages
sudo apt install libsoup2.4-dev libsoup-3.0-dev

# Solution 3: Create symbolic links (if needed)
sudo ln -sf /usr/lib/x86_64-linux-gnu/libwebkit2gtk-4.1.so 
            /usr/lib/x86_64-linux-gnu/libwebkit2gtk-4.0.so
```

#### **Python Integration Errors**
```bash
# Symptom: "Python module not found" errors
# Solution: Check Python path and install dependencies

# Verify Python installation
python3 -c "import chromadb, sentence_transformers; print('OK')"

# Reinstall dependencies
cd python_modules
pip3 install -r requirements.txt --force-reinstall
```

#### **Build Dependency Issues**
```bash
# Symptom: Compilation errors for native dependencies  
# Solution: Install complete development environment

# Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev

# Update Rust toolchain
rustup update stable
```

### **Runtime Issues**

#### **OCR Not Working**
```bash
# Check bundled Tesseract
ls -la vendor/tesseract-linux/tesseract

# Verify permissions
chmod +x vendor/tesseract-linux/tesseract

# Test OCR functionality
cd src-tauri
LAW_GORITHM_TESSERACT_PATH=../vendor/tesseract-linux/tesseract cargo run
```

#### **Database Connection Failures**  
```bash
# Check SQLite installation
sqlite3 --version

# Verify VSS extension availability
sqlite3 -c "SELECT load_extension('vss');" 2>/dev/null && echo "VSS OK"

# Clear database and restart
rm -rf ~/.local/share/law-gorithm/data/
```

## 📊 **Performance Optimization**

### **Build Optimization**
```bash
# Enable link-time optimization
export RUSTFLAGS="-C lto=fat"
cargo build --release

# Optimize for size  
export RUSTFLAGS="-C opt-level=s -C lto=fat"
cargo build --release

# Strip debug symbols
strip src-tauri/target/release/law-gorithm
```

### **Runtime Performance**
```toml
# In Cargo.toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
opt-level = "s"  # Optimize for size
```

## 🔒 **Security Considerations**

### **Build Security**
- **Verify Dependencies**: Audit all Rust crates and Python packages
- **Code Signing**: Sign binaries for distribution (requires certificates)
- **Sandboxing**: Tauri provides built-in sandboxing capabilities

### **Distribution Security**
```bash
# Generate checksums for distribution
sha256sum law-gorithm_0.1.0_amd64.deb > checksums.txt
sha256sum law-gorithm_0.1.0_amd64.AppImage >> checksums.txt

# Sign packages (if certificates available)
gpg --armor --detach-sign law-gorithm_0.1.0_amd64.deb
```

## 📋 **Deployment Checklist**

### **Pre-Build**
- [ ] All dependencies installed and working
- [ ] Python RAG system tested independently  
- [ ] Rust backend compiles without warnings
- [ ] Frontend served successfully on localhost:8080
- [ ] OCR functionality verified with test documents

### **Build Process**
- [ ] Production build completes successfully
- [ ] Generated binaries execute without errors
- [ ] Package installers created for target platforms
- [ ] Asset bundling verified (icons, Tesseract binaries)

### **Post-Build Testing**
- [ ] Install packages on clean test systems
- [ ] Verify all functionality works in installed version
- [ ] Test document upload and processing
- [ ] Confirm RAG search and AI responses
- [ ] Performance benchmarking completed

### **Distribution**
- [ ] Package integrity verified with checksums
- [ ] Installation documentation updated
- [ ] User documentation reflects current features
- [ ] Support channels prepared for user questions

---

**🔥 Priority Focus**: The build system is working - focus on resolving the libsoup runtime issue for a smooth user experience.
