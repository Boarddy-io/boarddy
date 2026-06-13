# Boarddy GitHub Configuration & Release Guide

This document guides the Huna Inc. team through setting up the official GitHub presence for **Boarddy** and managing automated cross-platform releases and package manager distributions.

---

## 1. Organization & Repository Setup Checklist

### Step 1: Create the Organization & Repository
1. Navigate to [GitHub](https://github.com) and create a new organization: **`boarddy-io`** (or `getboarddy` / `boarddy-dev` if taken).
2. Set the organization Display Name to `Boarddy`.
3. Create a new public repository under this organization named **`boarddy`**.
4. Set the **Repository Description** to:
   > *The Input & Memory Layer for Your Computer. Clipboard Memory, Quick Paste, Autocomplete, Autocorrect, Personal Dictionary, Notes, Multilingual Typing, and Developer Productivity Tools.*
5. Set the **Repository URL** / Homepage link to: `https://huna.io/boarddy` (or your official product website).

### Step 2: Configure Organization Profile
Update the organization settings with the following:
* **Tagline**: `The Input & Memory Layer for Your Computer.`
* **Short Description**: `Your clipboard remembers. Your keyboard learns.`
* **Long Description**:
  > *Boarddy helps users type faster, remember more, and retrieve anything they have copied, typed, or saved. Boarddy combines clipboard memory, smart typing, personal dictionaries, notes, and keyboard productivity features into a single local-first desktop experience.*

### Step 3: Configure Repository Topics
Add the following topics in the repository **About** section on the right-hand panel of the main repository page:
```
clipboard-manager, clipboard-history, autocomplete, autocorrect, productivity, desktop-app, tauri, rust, react, typescript, keyboard, typing, memory, notes, search, local-first, offline-first, developer-tools, windows, macos, linux, knowledge-management, personal-knowledge-management, pkm, second-brain, input-layer, keyboard-productivity, developer-productivity
```

### Step 4: Enable & Configure GitHub Discussions
1. Go to repository **Settings** -> **General** -> scroll down to the **Features** section.
2. Check the box for **Discussions**.
3. Click on the **Discussions** tab in the repository menu bar to customize categories:
   * **Announcements**: For official releases, updates, and news from Huna Inc.
   * **Ideas**: For community members to brainstorm enhancements.
   * **Feature Requests**: Formally request new features.
   * **Bug Reports**: General bug reports or reproduction queries before opening formal issues.
   * **Developer Mode**: For configuring custom expansions, keyboard mappings, and shell plugins.
   * **Product Feedback**: Direct feedback regarding user experience.
   * **Showcase**: Where users share screenshot setups or video walkthroughs.
   * **Community**: For general chit-chat and tips.

### Step 5: Enable Private Vulnerability Reporting
1. Navigate to repository **Settings** -> **Security** -> **Code security and analysis**.
2. Enable **Private vulnerability reporting** to allow researchers to report security issues privately before a public disclosure.

---

## 2. Package Manager Submissions & Distribution

To allow users to install Boarddy using command-line commands, follow these package-specific instructions once the first release is generated.

### A. Windows Package Manager (Winget)
Boarddy can be submitted to the [Microsoft Winget Community Repository](https://github.com/microsoft/winget-pkgs).
1. Install the `wingetcreate` CLI tool:
   ```bash
   winget install Microsoft.WingetCreate
   ```
2. Generate the submission manifest using the URL of the `.msi` installer from your latest GitHub release:
   ```bash
   wingetcreate new https://github.com/boarddy-io/boarddy/releases/download/v0.1.0/Boarddy_0.1.0_x64_en-US.msi
   ```
3. Follow the interactive prompts to fill out details:
   * Publisher: `Huna Inc.`
   * License: `Proprietary` (referencing `https://github.com/boarddy-io/boarddy/blob/main/LICENSE.md`)
   * Short Description: `The Input & Memory Layer for Your Computer.`
4. Submit the pull request. Once approved, users can install via:
   ```bash
   winget install Boarddy
   ```

### B. macOS Homebrew Cask
To distribute Boarddy for macOS (Intel & Apple Silicon DMG installers):
1. Create a public tap repository under your organization called **`homebrew-tap`** (`github.com/boarddy-io/homebrew-tap`).
2. Add a formula/cask definition file `Casks/boarddy.rb`:
   ```ruby
   cask "boarddy" do
     version "0.1.0"
     sha256 "LATEST_RELEASE_DMG_SHA256"

     url "https://github.com/boarddy-io/boarddy/releases/download/v#{version}/Boarddy_#{version}_universal.dmg"
     name "Boarddy"
     desc "The Input & Memory Layer for Your Computer"
     homepage "https://huna.io/boarddy"

     app "Boarddy.app"

     zap trash: [
       "~/Library/Application Support/com.boarddy.app",
       "~/Library/Saved Application State/com.boarddy.app.savedState",
       "~/Library/Preferences/com.boarddy.app.plist",
     ]
   end
   ```
3. Users can tap and install:
   ```bash
   brew tap boarddy-io/tap
   brew install --cask boarddy
   ```

### C. Chocolatey (Windows)
For standard Windows installations:
1. Create a Chocolatey package folder with `boarddy.nuspec` and `tools/chocolateyinstall.ps1`.
2. Configure `chocolateyinstall.ps1` to download and install the MSI:
   ```powershell
   $toolsDir   = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
   $packageArgs = @{
     packageName   = 'boarddy'
     fileType      = 'msi'
     url           = 'https://github.com/boarddy-io/boarddy/releases/download/v0.1.0/Boarddy_0.1.0_x64_en-US.msi'
     silentArgs    = '/qn /norestart'
     validExitCodes= @(0, 3010, 1641)
   }
   Install-ChocolateyPackage @packageArgs
   ```
3. Push to Chocolatey community feed using `choco push`. Once approved, users install via:
   ```bash
   choco install boarddy
   ```

### D. Scoop (Windows Portable)
Scoop is ideal for portable, non-admin Windows installations:
1. Create a Scoop bucket repository `scoop-bucket` or submit to the `extras` bucket.
2. Create `bucket/boarddy.json`:
   ```json
   {
     "version": "0.1.0",
     "description": "The Input & Memory Layer for Your Computer",
     "homepage": "https://huna.io/boarddy",
     "license": "Proprietary",
     "architecture": {
       "64bit": {
         "url": "https://github.com/boarddy-io/boarddy/releases/download/v0.1.0/Boarddy_0.1.0_x64_portable.exe",
         "hash": "LATEST_PORTABLE_SHA256"
       }
     },
     "bin": "Boarddy.exe",
     "shortcuts": [
       [
         "Boarddy.exe",
         "Boarddy"
       ]
     ],
     "persist": "data"
   }
   ```
3. Users install via:
   ```bash
   scoop install boarddy
   ```

---

## 3. GitHub Actions CI/CD Configurations

Our GitHub Actions workflows are configured in the `.github/workflows/` directory:
1. **`ci.yml`**: Triggers on pull requests to run frontend TypeScript verification (`npm run build`), Rust clippy lints, formatting checks, and Cargo tests.
2. **`release.yml`**: Triggers on tag pushes (e.g., `v0.1.0`). Builds Windows (MSI, Portable), macOS (Intel, Apple Silicon), and Linux (AppImage, Deb, RPM), signs them, computes SHA256 checksums, and uploads them to a draft release automatically.

### Configuring Signing Certificates & Secrets (Optional but Recommended)
For a premium experience, developers should sign the desktop binaries.
1. Add the following secrets to repository **Settings** -> **Secrets and variables** -> **Actions**:
   * **Windows Code Signing**: `SIGNING_CERTIFICATE` (Base64-encoded PFX cert) and `CERTIFICATE_PASSWORD`.
   * **macOS Notarization**: `APPLE_CERTIFICATE` (p12 developer cert), `APPLE_CERTIFICATE_PASSWORD`, `APPLE_API_KEY_ID`, and `APPLE_API_KEY_ISSUER`.
2. Update the release step in `.github/workflows/release.yml` to consume these environment variables during compilation.
