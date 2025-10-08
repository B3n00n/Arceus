# Auto-Update Setup Guide

This document explains how to set up and use the auto-updater for Arceus.

## Overview

The Arceus auto-updater uses Tauri's built-in updater plugin to check for and install updates from GitHub releases.

## Setup Requirements

### 1. Private Key Security

The private key (`arceus.key`) is already generated and must be kept secure:
- **Never commit the private key to version control** (already in .gitignore)
- Store it securely for signing releases
- Set these environment variables when building releases:
  ```bash
  TAURI_SIGNING_PRIVATE_KEY=path/to/arceus.key
  TAURI_SIGNING_PRIVATE_KEY_PASSWORD=temp_password
  ```

### 2. GitHub Release Setup

To create a signed update:

1. **Build the release:**
   ```bash
   npm run tauri build
   ```

2. **The build process will:**
   - Create signed binaries in `src-tauri/target/release/`
   - Generate update manifests
   - Package everything for distribution

3. **Create GitHub Release:**
   - Upload the built artifacts to a GitHub release
   - The updater checks: `https://api.github.com/repos/B3n00n/arceus/releases/latest`

### 3. Update Process Flow

1. **Check for Updates:** App queries GitHub API for latest release
2. **Compare Versions:** Uses semantic versioning to determine if update is available
3. **Download:** If update available, downloads the signed update package
4. **Verify:** Validates signature using the public key in `tauri.conf.json`
5. **Install:** Applies update and restarts the application

## Configuration

### Current Settings

- **Repository:** `B3n00n/arceus`
- **Endpoint:** GitHub Releases API
- **Public Key:** Embedded in `src-tauri/tauri.conf.json`
- **Verification:** Signature validation enabled

### Customization

To change the update source, modify `src-tauri/tauri.conf.json`:

```json
{
  "plugins": {
    "updater": {
      "endpoints": ["https://api.github.com/repos/YOUR_USER/YOUR_REPO/releases/latest"],
      "pubkey": "YOUR_PUBLIC_KEY"
    }
  }
}
```

## Troubleshooting

### Common Issues

1. **"Invalid public key" error:**
   - Ensure the public key in `tauri.conf.json` matches the one used for signing
   - Regenerate keys if needed: `npm run tauri signer generate`

2. **"No updates available" when update exists:**
   - Check version number in `package.json` and `src-tauri/Cargo.toml`
   - Ensure GitHub release is marked as "latest"
   - Verify release contains properly signed artifacts

3. **Download/Install failures:**
   - Check network connectivity
   - Verify GitHub release assets are publicly accessible
   - Ensure sufficient disk space for update

### Debug Mode

To test the updater in development:

1. Create a test release on GitHub
2. Temporarily modify the version in your local build to be lower
3. Run `npm run tauri dev` to test update detection

## Security Notes

- Updates are cryptographically signed and verified
- Only releases signed with the correct private key will be accepted
- The updater validates the signature before applying any updates
- Keep your private key secure and backed up safely