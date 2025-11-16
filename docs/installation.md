# Installation and Setup Guide

This guide provides detailed instructions for installing and setting up the AxiomHive Assistant on Windows, macOS, and Linux.

## Table of Contents

- [System Requirements](#system-requirements)
- [Prerequisites](#prerequisites)
- [Installation Steps](#installation-steps)
- [Model Setup](#model-setup)
- [Configuration](#configuration)
- [Verification](#verification)
- [Platform-Specific Notes](#platform-specific-notes)
- [Troubleshooting](#troubleshooting)
- [Next Steps](#next-steps)

## System Requirements

### Minimum Requirements

- **Operating System**: Windows 10 (64-bit), macOS 10.15+, Ubuntu 18.04+ or equivalent Linux distribution
- **Processor**: Intel/AMD x64 or Apple Silicon (M1/M2/M3)
- **Memory (RAM)**: 8 GB minimum, 16 GB recommended
- **Storage**: 10 GB free space (including 4-5 GB for AI models)
- **Graphics**: No GPU required (CPU inference), but NVIDIA/AMD GPU recommended for better performance

### Recommended Specifications

- **Processor**: Modern multi-core CPU (Intel i5/Ryzen 5 or better)
- **Memory (RAM)**: 16-32 GB
- **Storage**: SSD with 20+ GB free space
- **Graphics**: NVIDIA RTX 30-series or newer, or AMD Radeon RX 6000-series or newer (optional but recommended)

## Prerequisites

### 1. Rust Programming Language

The AxiomHive Assistant backend is written in Rust. Install Rust using rustup:

#### Windows
```bash
# Download and run the installer
# Visit: https://rustup.rs/
# Or use winget:
winget install --id Rustlang.Rustup
```

#### macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Linux (Ubuntu/Debian)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:
```bash
rustc --version
cargo --version
```

Expected output:
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

### 2. Node.js and npm

The frontend is built with React and TypeScript.

#### Windows
- Download from: https://nodejs.org/
- Choose LTS version (20.x recommended)
- Run the installer

#### macOS
```bash
# Using Homebrew
brew install node

# Or download from nodejs.org
```

#### Linux (Ubuntu/Debian)
```bash
# Using NodeSource repository
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
```

Verify installation:
```bash
node --version
npm --version
```

Expected output:
```
v20.10.0
10.2.3
```

### 3. Tauri CLI

Tauri is the framework used to build the desktop application.

```bash
cargo install tauri-cli
```

Verify installation:
```bash
tauri --version
```

Expected output:
```
tauri-cli 2.0.0
```

### 4. Git

Required for cloning the repository.

#### Windows
- Download from: https://git-scm.com/download/win
- Or use winget: `winget install --id Git.Git`

#### macOS
```bash
# Using Homebrew
brew install git

# Or Xcode Command Line Tools
xcode-select --install
```

#### Linux
```bash
sudo apt update
sudo apt install git
```

Verify installation:
```bash
git --version
```

## Installation Steps

### Step 1: Clone the Repository

```bash
git clone https://github.com/AXI0MH1VE/CDA-V1.0.git
cd CDA-v1.0
```

### Step 2: Install Dependencies

Navigate to the axiomhive-assistant directory and install Node.js dependencies:

```bash
cd axiomhive-assistant
npm install
```

This will install all required Node.js packages including React, TypeScript, and Tauri dependencies.

### Step 3: Download AI Models

The application requires pre-trained AI models. Create a `models` directory and download the required models:

```bash
# Create models directory
mkdir models
cd models
```

#### Required Models

1. **Mistral-7B Base Model** (4-bit quantized, ~4GB)
   - Download from: [Hugging Face](https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.3-GGUF)
   - File: `mistral-7b-instruct-v0.3.Q4_K_M.gguf`
   - Place as: `models/mistral-7b-4bit.gguf`

2. **Embedding Model** (~100MB)
   - Download from: [Hugging Face](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2)
   - File: `model.onnx`
   - Place as: `models/embeddings.onnx`

#### Automated Download Commands

Download directly from Hugging Face without leaving the terminal:

- **macOS/Linux**
  ```bash
  curl -L -o models/mistral-7b-4bit.gguf \
    https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.3-GGUF/resolve/main/mistral-7b-instruct-v0.3.Q4_K_M.gguf
  curl -L -o models/embeddings.onnx \
    https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/onnx/model.onnx
  ```

- **Windows PowerShell**
  ```powershell
  Invoke-WebRequest -Uri https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.3-GGUF/resolve/main/mistral-7b-instruct-v0.3.Q4_K_M.gguf -OutFile .\models\mistral-7b-4bit.gguf
  Invoke-WebRequest -Uri https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/onnx/model.onnx -OutFile .\models\embeddings.onnx
  ```

#### Direct Download + Integrity Checks

| Model            | URL                                                                                                        | SHA256 (expected)                                   |
|------------------|------------------------------------------------------------------------------------------------------------|-----------------------------------------------------|
| Mistral-7B 4-bit | https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.3-GGUF/resolve/main/mistral-7b-instruct-v0.3.Q4_K_M.gguf | `54d02f5c5d431c4b1d9d29d3990b1fc3f01966cb03a41f64c74f0b551fea2a23` |
| Embedding model  | https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/onnx/model.onnx                | `c2f6a9d24db53842c48f2a16897b9a0c6f5f19fc71b22006c23ef6c64339377f` |

Verify after downloading:

- **macOS/Linux**
  ```bash
  sha256sum mistral-7b-instruct-v0.3.Q4_K_M.gguf
  sha256sum model.onnx
  ```

- **Windows PowerShell**
  ```powershell
  Get-FileHash -Path .\mistral-7b-instruct-v0.3.Q4_K_M.gguf -Algorithm SHA256
  Get-FileHash -Path .\model.onnx -Algorithm SHA256
  ```

If the hash differs, re-download before running the application.

### Step 4: Build the Application

#### Development Build

For development and testing:

```bash
npm run tauri:dev
```

This will:
- Compile the Rust backend
- Build the React frontend
- Launch the application in development mode

#### Production Build

For final distribution:

```bash
npm run tauri:build
```

This creates optimized binaries in `src-tauri/target/release/bundle/`.

## Configuration

### Environment Variables

Create a `.env` file in the `axiomhive-assistant` directory:

```env
# AI Model Configuration
MODEL_PATH=./models/mistral-7b-4bit.gguf
EMBEDDING_MODEL_PATH=./models/embeddings.onnx

# Application Settings
MAX_TOKENS=2048
TEMPERATURE=0.7
TOP_P=0.9

# Safety Settings
ENABLE_CONSTITUTIONAL_CHECK=true
ENABLE_SANDBOX=true

# Optional: Web Search API (if using search features)
# BRAVE_API_KEY=your_api_key_here
```

### Tauri Configuration

The main configuration is in `src-tauri/tauri.conf.json`. Key settings:

```json
{
  "productName": "AxiomHive Assistant",
  "version": "1.0.0",
  "identifier": "co.axiomhive.assistant",
  "bundle": {
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": null
    }
  }
}
```

## Verification

### Test the Installation

1. **Start the application:**
   ```bash
   npm run tauri:dev
   ```

2. **Check the console output:**
   - Should show successful compilation
   - No error messages
   - Application window opens

3. **Test basic functionality:**
   - Type a simple query: "Hello, what can you help me with?"
   - Verify constitutional disclosure appears
   - Check that responses are appropriate

### Performance Verification

Run the built-in benchmark:

```bash
# From the application, use the /benchmark command
/benchmark
```

Expected performance:
- Startup time: <0.5 seconds
- Memory usage: 40-80 MB (base) + 3-4 GB (model loaded)
- Inference speed: 20-50 tokens/second

## Platform-Specific Notes

### Windows

- **Visual Studio Build Tools**: May be required for Rust compilation
  ```bash
  # Install with Chocolatey
  choco install visualstudio2019buildtools
  ```

- **WebView2**: Ensure Microsoft Edge WebView2 is installed (usually comes with Windows 10/11)

- **Firewall**: The application may request firewall access for local networking

### macOS

- **Xcode**: Full Xcode installation may be required
  ```bash
  xcode-select --install
  ```

- **Code Signing**: For distribution, configure code signing in `tauri.conf.json`

### Linux

- **WebKit**: Install WebKitGTK
  ```bash
  # Ubuntu/Debian
  sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

  # Fedora
  sudo dnf install webkit2gtk4.0-devel openssl-devel curl wget libappindicator-gtk3 librsvg2-devel
  ```

- **AppImage**: For distribution, ensure fuse is installed
  ```bash
  sudo apt install fuse
  ```

## Troubleshooting

### Common Issues

#### Rust Compilation Errors

**Problem**: `error: linker 'link.exe' not found`

**Solution** (Windows):
```bash
# Install Visual Studio Build Tools
choco install visualstudio2019buildtools
```

**Problem**: `error: could not find 'cc'`

**Solution** (Linux/macOS):
```bash
# Install build tools
sudo apt install build-essential  # Ubuntu/Debian
brew install gcc                 # macOS
```

#### Node.js Dependencies Fail

**Problem**: `npm install` fails with permission errors

**Solution**:
```bash
# Clear npm cache and reinstall
npm cache clean --force
rm -rf node_modules package-lock.json
npm install
```

#### Model Loading Errors

**Problem**: `Failed to load model: File not found`

**Solution**:
- Verify model files are in the correct location
- Check file permissions
- Ensure model files are not corrupted (verify checksums)

#### Application Won't Start

**Problem**: Application builds but doesn't launch

**Solution**:
- Check console output for error messages
- Verify all dependencies are installed
- Try running in development mode: `npm run tauri:dev`

### Getting Help

If you encounter issues not covered here:

1. Check the [Troubleshooting Guide](troubleshooting.md)
2. Review the [FAQ](faq.md)
3. Open an issue on GitHub with:
   - Your operating system and version
   - Rust and Node.js versions
   - Full error messages
   - Steps to reproduce

### Performance Optimization

For better performance:

1. **Use SSD storage** for models
2. **Close other applications** during model loading
3. **Consider GPU acceleration** (if available)
4. **Increase RAM** if experiencing slowdowns
5. **Use smaller models** for testing

## Next Steps

Once installation is complete:

1. Read the [User Manual](user-manual.md) to learn how to use the application
2. Explore the [Developer Documentation](developer.md) if you plan to contribute
3. Join our community for support and updates

---

**Installation completed successfully?** Proceed to the [User Manual](user-manual.md) to start using AxiomHive Assistant.
