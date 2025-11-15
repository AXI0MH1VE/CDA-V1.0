# Troubleshooting Guide

This guide helps you resolve common issues with AxiomHive Assistant. If you can't find a solution here, check the [FAQ](faq.md) or open an issue on GitHub.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Startup Problems](#startup-problems)
- [Performance Issues](#performance-issues)
- [Constitutional Compliance Errors](#constitutional-compliance-errors)
- [Model Loading Problems](#model-loading-problems)
- [File Processing Issues](#file-processing-issues)
- [Update Problems](#update-problems)
- [Platform-Specific Issues](#platform-specific-issues)
- [Getting Help](#getting-help)

## Installation Issues

### Rust Installation Problems

**Problem:** `rustup` command not found after installation

**Solutions:**
```bash
# On Windows
# Add Rust to PATH or restart terminal
# Check if C:\Users\%USERNAME%\.cargo\bin is in PATH

# On Linux/macOS
source ~/.cargo/env
# Or add to ~/.bashrc: export PATH="$HOME/.cargo/bin:$PATH"
```

**Problem:** Rust version too old

**Solution:**
```bash
rustup update
rustc --version  # Should show 1.75+
```

### Node.js Issues

**Problem:** npm install fails with permission errors

**Solutions:**
```bash
# Clear cache and reinstall
npm cache clean --force
rm -rf node_modules package-lock.json
npm install

# Or use a Node version manager
nvm install 20
nvm use 20
```

**Problem:** Node.js version incompatible

**Solution:**
```bash
# Check version
node --version  # Should be 20.x

# Update if needed
nvm install 20
nvm use 20
```

### Tauri CLI Problems

**Problem:** `tauri` command not found

**Solutions:**
```bash
# Install globally
cargo install tauri-cli

# Or run via cargo
cargo tauri --help
```

**Problem:** Tauri build fails on Linux

**Solution:** Install system dependencies
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.0-devel openssl-devel curl wget libappindicator-gtk3 librsvg2-devel
```

## Startup Problems

### Application Won't Launch

**Problem:** Window opens but immediately closes

**Debug Steps:**
1. Run in development mode: `npm run tauri:dev`
2. Check console output for errors
3. Verify model files are present
4. Check system resources (RAM, disk space)

**Common Causes:**
- Insufficient RAM (<8GB)
- Missing AI model files
- Corrupted installation
- Antivirus blocking the app

### Model Loading Errors

**Problem:** "Failed to load model" error

**Solutions:**
```bash
# Verify model files exist
ls -la models/
# Should see: mistral-7b-4bit.gguf, embeddings.onnx

# Check file integrity
# Download models again if corrupted

# Check file permissions
chmod 644 models/*.gguf
```

**Problem:** Out of memory during model loading

**Solutions:**
- Close other applications
- Increase system RAM
- Use a smaller model (if available)
- Check for memory leaks in other processes

### Constitutional Engine Failures

**Problem:** "Constitutional validation failed" on startup

**Debug Steps:**
1. Check CONSTITUTION.md is present
2. Verify Lambda Core initialization
3. Check for corrupted configuration files
4. Reset application settings

## Performance Issues

### Slow Inference

**Problem:** Responses are very slow (>30 seconds)

**Causes and Solutions:**

**High CPU Usage:**
- Close other CPU-intensive applications
- Check for background processes
- Consider GPU acceleration (future feature)

**Memory Issues:**
```bash
# Check available RAM
free -h  # Linux
# or
systeminfo | findstr Memory  # Windows

# Close memory-hungry applications
# Restart the application
```

**Model Configuration:**
- Verify correct model quantization
- Check sparse activation is working
- Monitor Lambda/Phi compute ratio (<1% is good)

### High Memory Usage

**Problem:** Application uses too much RAM

**Solutions:**
- Monitor with system tools
- Check for memory leaks
- Close and restart application
- Verify model unloading when not in use

**Expected Usage:**
- Base application: 40-80 MB
- With model loaded: 3-4 GB total
- During inference: May spike to 4-5 GB

### Poor Response Quality

**Problem:** Responses are irrelevant or low quality

**Causes:**
- Constitutional filtering too aggressive
- Model temperature too low
- Context window limitations
- Input preprocessing issues

**Solutions:**
- Adjust settings (if available)
- Rephrase queries more clearly
- Provide more context
- Check for constitutional overrides

## Constitutional Compliance Errors

### Identity Violation Errors

**Problem:** "Identity claim detected" errors

**Causes:**
- User queries triggering false positives
- Overly sensitive validation rules
- Context misunderstanding

**Solutions:**
- Rephrase queries to avoid identity-related terms
- Use the application as intended (tool, not entity)
- Report false positives as bugs

### Transparency Mandate Issues

**Problem:** Missing constitutional disclosure

**Solutions:**
- Check if disclosure was accepted on first launch
- Clear application data and restart
- Verify localStorage is not corrupted

### Safety Protocol Violations

**Problem:** Harm prevention blocking legitimate queries

**Debug Steps:**
1. Review the blocked query
2. Check if it contains prohibited keywords
3. Rephrase to avoid triggering filters
4. Report over-filtering as issue

## Model Loading Problems

### File Not Found Errors

**Problem:** Model files not found at startup

**Solutions:**
```bash
# Check correct file names
ls models/
# Expected: mistral-7b-4bit.gguf, embeddings.onnx

# Verify file paths in configuration
cat .env | grep MODEL_PATH

# Re-download models if missing
./scripts/download-models.sh
```

### Corrupted Model Files

**Problem:** Model loading fails with corruption errors

**Solutions:**
- Verify download integrity (checksums)
- Re-download corrupted files
- Check available disk space
- Ensure stable internet during download

### Version Mismatch

**Problem:** Model version incompatible

**Solutions:**
- Check model version compatibility
- Update to latest compatible models
- Verify configuration matches model format

## File Processing Issues

### Upload Failures

**Problem:** Files fail to upload or process

**Common Issues:**
- File size too large (max 100MB)
- Unsupported file type
- Corrupted file
- Permission issues

**Solutions:**
```bash
# Check file size
ls -lh filename

# Verify supported formats
# Supported: PDF, TXT, MD, DOCX, JPG, PNG, MP3, WAV

# Check file permissions
chmod 644 filename
```

### Processing Errors

**Problem:** File processes but gives wrong results

**Causes:**
- OCR quality issues (images)
- Document parsing problems
- Encoding issues (text files)
- Model limitations

**Solutions:**
- Try different file formats
- Ensure text is clearly readable
- Check for special characters/encoding
- Report processing bugs

## Update Problems

### Update Download Failures

**Problem:** Automatic updates fail

**Solutions:**
- Check internet connection
- Verify firewall allows connections
- Try manual download from website
- Check available disk space

### Installation Failures

**Problem:** Update installs but application doesn't work

**Solutions:**
- Check for conflicting processes
- Restart computer
- Verify file integrity
- Rollback to previous version

### Rollback Issues

**Problem:** Cannot rollback after bad update

**Solutions:**
- Manual installation of previous version
- Clear application data
- Check for leftover files
- Contact support for assistance

## Platform-Specific Issues

### Windows Issues

**Problem:** Windows Defender blocks the application

**Solutions:**
- Add exception in Windows Security
- Run as administrator
- Submit false positive report to Microsoft

**Problem:** Visual Studio Build Tools required

**Solution:**
```bash
# Install via winget
winget install Microsoft.VisualStudio.2022.BuildTools
```

### macOS Issues

**Problem:** "App is damaged" error

**Solutions:**
- Check Gatekeeper settings
- Re-download from official source
- Check code signature

**Problem:** Notarization issues

**Solution:** Use development builds or wait for Apple approval

### Linux Issues

**Problem:** Missing dependencies

**Solution:** Install required packages
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-dev libappindicator3-dev

# Fedora
sudo dnf install webkit2gtk4.0-devel libappindicator-gtk3
```

**Problem:** AppImage won't run

**Solutions:**
```bash
# Make executable
chmod +x AxiomHive*.AppImage

# Install fuse if needed
sudo apt install fuse
```

## Getting Help

### Diagnostic Information

When reporting issues, include:

```bash
# System information
uname -a  # Linux/macOS
systeminfo  # Windows

# Application version
./AxiomHive --version

# Rust/Node versions
rustc --version
node --version

# Error logs
# Check console output or log files
```

### Support Channels

1. **GitHub Issues:** For bugs and feature requests
2. **GitHub Discussions:** For questions and community help
3. **Documentation:** Check all docs/ files first
4. **Discord:** Real-time community support

### Emergency Contacts

- **Security Issues:** security@axiomhive.co
- **Constitutional Concerns:** constitution@axiomhive.co
- **General Support:** support@axiomhive.co

### Contributing Fixes

Found a bug? Consider contributing:

1. Fork the repository
2. Create a fix branch
3. Test thoroughly
4. Submit a pull request
5. Follow [Contributing Guidelines](CONTRIBUTING.md)

---

**Still having issues?** Check the [FAQ](faq.md) or create a detailed issue report on GitHub with your system information and error logs.

