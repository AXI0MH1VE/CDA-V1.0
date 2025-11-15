# Deployment and Distribution Guide

This guide covers building, packaging, and distributing the AxiomHive Assistant across different platforms.

## Table of Contents

- [Build Process](#build-process)
- [Platform-Specific Builds](#platform-specific-builds)
- [Code Signing](#code-signing)
- [Distribution Channels](#distribution-channels)
- [Update Mechanism](#update-mechanism)
- [CI/CD Pipeline](#cicd-pipeline)

## Build Process

### Prerequisites

- Rust 1.75+ with targets installed
- Node.js 20+ and npm
- Tauri CLI 2.0+
- Platform-specific build tools

### Development Build

For testing during development:

```bash
cd axiomhive-assistant
npm run tauri:dev
```

This creates a development build with:
- Hot reloading enabled
- Debug symbols included
- Development tools accessible

### Production Build

For distribution:

```bash
cd axiomhive-assistant
npm run tauri:build
```

This process:
1. Compiles Rust code in release mode
2. Builds React frontend optimized
3. Bundles everything into platform-specific packages
4. Applies code signing (if configured)

### Build Configuration

Key settings in `tauri.conf.json`:

```json
{
  "productName": "AxiomHive Assistant",
  "version": "1.0.0",
  "identifier": "co.axiomhive.assistant",
  "bundle": {
    "active": true,
    "targets": ["msi", "dmg", "appimage", "deb"],
    "createUpdaterBundle": true,
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com"
    },
    "macOS": {
      "frameworks": [],
      "minimumSystemVersion": "10.15"
    }
  }
}
```

## Platform-Specific Builds

### Windows

**Requirements:**
- Windows 10/11
- Visual Studio Build Tools 2019+
- Windows SDK

**Build Command:**
```bash
npm run tauri:build -- --target x86_64-pc-windows-msvc
```

**Output:**
- `src-tauri/target/release/bundle/msi/AxiomHive Assistant_1.0.0_x64.msi`
- `src-tauri/target/release/bundle/msi/AxiomHive Assistant_1.0.0_x64.msi.zip` (updater bundle)

**Distribution:**
- Microsoft Store (requires developer account)
- Direct download from axiomhive.co
- Winget package manager

### macOS

**Requirements:**
- macOS 10.15+
- Xcode 12+
- Apple Developer Program membership (for signing)

**Build Command:**
```bash
npm run tauri:build -- --target x86_64-apple-darwin
# Or for Apple Silicon:
npm run tauri:build -- --target aarch64-apple-darwin
```

**Output:**
- `src-tauri/target/release/bundle/dmg/AxiomHive Assistant_1.0.0_x64.dmg`
- `src-tauri/target/release/bundle/dmg/AxiomHive Assistant_1.0.0_x64.dmg.zip` (updater)

**Distribution:**
- Mac App Store (requires Apple Developer Program)
- Direct download (.dmg files)
- Homebrew cask

### Linux

**Requirements:**
- Ubuntu 18.04+ or equivalent
- Build essentials
- WebKitGTK development libraries

**Build Commands:**
```bash
# Debian/Ubuntu
npm run tauri:build -- --target x86_64-unknown-linux-gnu

# AppImage (universal)
npm run tauri:build -- --target x86_64-unknown-linux-gnu --bundles appimage
```

**Output:**
- `src-tauri/target/release/bundle/deb/axiomhive-assistant_1.0.0_amd64.deb`
- `src-tauri/target/release/bundle/appimage/AxiomHive Assistant_1.0.0_amd64.AppImage`
- `src-tauri/target/release/bundle/appimage/AxiomHive Assistant_1.0.0_amd64.AppImage.zip` (updater)

**Distribution:**
- Snap Store
- FlatHub (Flathub)
- Direct downloads (.deb, .AppImage)
- Package managers (apt, dnf, etc.)

## Code Signing

### Windows Code Signing

1. **Obtain certificate:**
   - Purchase from trusted CA (DigiCert, GlobalSign, etc.)
   - Or use self-signed for development

2. **Configure signing:**
   ```json
   {
     "bundle": {
       "windows": {
         "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
         "digestAlgorithm": "sha256",
         "timestampUrl": "http://timestamp.digicert.com"
       }
     }
   }
   ```

3. **Sign manually** (if needed):
   ```bash
   signtool sign /fd sha256 /t http://timestamp.digicert.com "path/to/installer.msi"
   ```

### macOS Code Signing

1. **Apple Developer Program:**
   - Required for App Store distribution
   - Provides signing certificates

2. **Configure signing:**
   ```json
   {
     "bundle": {
       "macOS": {
         "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)",
         "providerShortName": "your-provider"
       }
     }
   }
   ```

3. **Notarization** (required for App Store):
   ```bash
   xcrun notarytool submit "path/to/app.dmg" --keychain-profile "your-profile" --wait
   ```

### Linux Code Signing

Linux applications typically don't require code signing for distribution, but you can:

- Sign .deb packages with GPG
- Include checksums for verification
- Use reproducible builds

## Distribution Channels

### Official Website

**Setup:**
- Host downloads on axiomhive.co
- Provide checksums for verification
- Include release notes

**Implementation:**
```html
<!-- Download page example -->
<div class="downloads">
  <h2>Download AxiomHive Assistant</h2>
  <div class="platform">
    <h3>Windows</h3>
    <a href="/downloads/AxiomHive Assistant_1.0.0_x64.msi">Download MSI</a>
    <small>SHA256: abc123...</small>
  </div>
  <!-- Similar for macOS and Linux -->
</div>
```

### App Stores

#### Microsoft Store

1. **Partner Center setup:**
   - Create developer account ($19 one-time fee)
   - Reserve app name
   - Configure app metadata

2. **Package submission:**
   - Upload signed .msi file
   - Provide screenshots and descriptions
   - Set pricing and availability

3. **Store benefits:**
   - Automatic updates
   - Discovery through store
   - Trust and security validation

#### Mac App Store

1. **App Store Connect:**
   - Configure app record
   - Upload signed .pkg
   - Set up sandbox entitlements

2. **Review process:**
   - Apple review (typically 24-48 hours)
   - Constitutional compliance documentation
   - Privacy policy submission

#### Linux Stores

**Snap Store:**
```yaml
# snapcraft.yaml
name: axiomhive-assistant
version: '1.0.0'
summary: Constitutional AI Assistant
description: |
  AxiomHive Assistant implements CDA-v1.0 for safe, transparent AI interaction.

grade: stable
confinement: strict

apps:
  axiomhive-assistant:
    command: bin/axiomhive-assistant
    plugs: [home, network, removable-media]
```

**Flathub:**
- Submit flatpak manifest
- Follow Flathub review process
- Automatic updates through Flathub infrastructure

### Package Managers

#### Windows (WinGet)

```yaml
# winget manifest
Id: AxiomHive.AxiomHiveAssistant
Version: 1.0.0
Name: AxiomHive Assistant
Publisher: AxiomHive
Description: Constitutional AI Assistant implementing CDA-v1.0
License: Proprietary

Installers:
- Architecture: x64
  InstallerType: msi
  InstallerUrl: https://axiomhive.co/downloads/AxiomHive Assistant_1.0.0_x64.msi
  InstallerSha256: ABC123...
```

#### Linux (APT/DNF)

**Ubuntu/Debian:**
```bash
# Add repository
echo "deb [signed-by=/usr/share/keyrings/axiomhive.gpg] https://repo.axiomhive.co/apt stable main" | sudo tee /etc/apt/sources.list.d/axiomhive.list
wget -qO- https://repo.axiomhive.co/apt/gpg | sudo tee /usr/share/keyrings/axiomhive.gpg > /dev/null
sudo apt update
sudo apt install axiomhive-assistant
```

## Update Mechanism

### Tauri Updater

Built-in automatic updates:

```rust
// In main.rs
tauri::Builder::default()
    .plugin(tauri_plugin_updater::Builder::new().build())
    .setup(|app| {
        let handle = app.handle();
        tauri::async_runtime::spawn(async move {
            update_app(handle).await;
        });
        Ok(())
    })
```

### Update Process

1. **Check for updates:**
   - Application checks GitHub releases API
   - Compares current version with latest
   - Downloads update in background

2. **Install updates:**
   - Windows: MSI installer
   - macOS: DMG with auto-updater
   - Linux: Package manager or AppImage replacement

3. **User experience:**
   - Silent updates when possible
   - User notification for major updates
   - Optional manual update checks

### Update Server

Host update metadata on GitHub Releases:

```json
// update.json on GitHub releases
{
  "version": "1.0.1",
  "notes": "Bug fixes and performance improvements",
  "pub_date": "2025-01-15T10:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "signature-here",
      "url": "https://github.com/AXI0MH1VE/CDA-V1.0/releases/download/v1.0.1/AxiomHive.Assistant_1.0.1_x64.msi.zip"
    },
    "darwin-x86_64": {
      "signature": "signature-here",
      "url": "https://github.com/AXI0MH1VE/CDA-V1.0/releases/download/v1.0.1/AxiomHive.Assistant_1.0.1_x64.dmg.zip"
    }
  }
}
```

## CI/CD Pipeline

### GitHub Actions Workflow

```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [windows-latest, macos-latest, ubuntu-latest]

    steps:
    - uses: actions/checkout@v4

    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '20'

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Install dependencies
      run: npm ci

    - name: Build application
      run: npm run tauri:build

    - name: Upload artifacts
      uses: actions/upload-artifact@v4
      with:
        name: bundles-${{ matrix.os }}
        path: src-tauri/target/release/bundle/
```

### Automated Testing

```yaml
# .github/workflows/test.yml
name: Test
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '20'

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Install dependencies
      run: npm ci

    - name: Run tests
      run: npm test

    - name: Run Rust tests
      run: cargo test
```

### Release Process

1. **Version bump:**
   ```bash
   npm version patch  # or minor/major
   git tag v1.0.1
   git push --tags
   ```

2. **Automated build:**
   - GitHub Actions builds for all platforms
   - Creates release with artifacts
   - Generates update metadata

3. **Distribution:**
   - Upload to app stores
   - Update website downloads
   - Notify users of new version

## Security Considerations

### Binary Security

- **Reproducible builds:** Ensure consistent builds
- **Supply chain security:** Audit dependencies
- **Vulnerability scanning:** Regular security audits

### Distribution Security

- **HTTPS downloads:** All downloads over secure connections
- **Checksum verification:** SHA256 hashes for integrity
- **Certificate validation:** Proper code signing certificates

### Update Security

- **Signature verification:** Updates must be signed
- **Rollback protection:** Prevent downgrade attacks
- **Staging environment:** Test updates before release

## Monitoring and Analytics

### Optional Telemetry

```typescript
// Optional usage analytics (user consent required)
interface TelemetryData {
  version: string;
  platform: string;
  session_duration: number;
  features_used: string[];
}

// Only send if user opts in
if (userConsentsToAnalytics) {
  sendTelemetry(data);
}
```

### Error Reporting

```rust
// Automatic error reporting (anonymized)
tauri::Builder::default()
    .plugin(tauri_plugin_sentry::init(
        sentry::init(("https://your-dsn@sentry.io/project", sentry::ClientOptions {
            release: env!("CARGO_PKG_VERSION").into(),
            ..Default::default()
        }))
    ))
```

## Troubleshooting Deployment

### Build Failures

**Common issues:**
- Missing system dependencies
- Rust target not installed
- Node.js version mismatch

**Solutions:**
```bash
# Install missing targets
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-apple-darwin

# Clear build cache
cargo clean
npm run clean
```

### Distribution Issues

**App store rejections:**
- Ensure constitutional compliance documentation
- Provide clear privacy policy
- Test on all supported platforms

**Download issues:**
- Check file integrity with checksums
- Verify HTTPS certificates
- Test download speeds and reliability

### Update Problems

**Failed updates:**
- Check network connectivity
- Verify file permissions
- Clear application cache

**Rollback procedure:**
```bash
# Manual rollback
# 1. Download previous version
# 2. Uninstall current version
# 3. Install previous version
# 4. Disable auto-updates temporarily
```

## Performance Optimization

### Bundle Size Optimization

- **Tree shaking:** Remove unused code
- **Asset optimization:** Compress images and fonts
- **Dependency analysis:** Minimize included libraries

### Build Time Optimization

- **Incremental builds:** Use cargo's incremental compilation
- **Parallel processing:** Utilize multiple CPU cores
- **Caching:** Cache dependencies between builds

## Legal and Compliance

### Distribution Licenses

- **Proprietary software:** Commercial licensing required
- **Third-party dependencies:** Ensure compatible licenses
- **Export controls:** Check encryption and AI regulations

### Platform Requirements

- **Microsoft Store:** Follow Windows app certification
- **Mac App Store:** Comply with App Store Review Guidelines
- **Linux stores:** Follow respective store policies

For more information, see the [Legal Documentation](legal.md) or contact legal@axiomhive.co.
