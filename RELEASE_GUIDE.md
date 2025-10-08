# Arceus Release Guide

## Quick Release Process

### 1. Update Version
```bash
# Update version in these files:
# - package.json
# - src-tauri/Cargo.toml
# - src-tauri/tauri.conf.json

# Example: changing to 0.1.2
```

### 2. Build Release
```bash
# Set signing environment variables
set TAURI_SIGNING_PRIVATE_KEY=D:\Data\GitProjects\Arceus\arceus.key
set TAURI_SIGNING_PRIVATE_KEY_PASSWORD=temp_password

# Build the release
npm run tauri build
```

### 3. Sign the Installer (if build signing fails)
```bash
# Sign manually if needed
npx tauri signer sign -f "arceus.key" -p "temp_password" "src-tauri/target/release/bundle/nsis/arceus_X.X.X_x64-setup.exe"
```

### 4. Create GitHub Release
1. Go to GitHub → Releases → Create new release
2. Tag: `vX.X.X` (e.g., `v0.1.2`)
3. Upload files:
   - `arceus_X.X.X_x64-setup.exe`
   - `arceus_X.X.X_x64-setup.exe.sig`
4. Publish release

**That's it!** The app will automatically detect the new version from GitHub Releases API.

## Key Files

### Signing Keys
- **Private Key**: `arceus.key` (keep secret!)
- **Public Key**: `arceus.pub` (already in tauri.conf.json)
- **Password**: `temp_password`

### Auto-Updater Config
- **Endpoint**: GitHub Releases API (automatic)
- **Install Mode**: `quiet` (silent, no UI)
- **Update Flow**: Automatic (no user confirmation)

## Generating New Keys (if needed)

```bash
# Generate new keypair
npm run tauri signer generate

# This creates:
# - New private key file
# - New public key (copy to tauri.conf.json)
```

## No More Manual Steps!

✅ **No more update-manifest.json** - Uses GitHub Releases API
✅ **No manual signature updates** - Automatically reads .sig files
✅ **No UI messages** - Silent when up-to-date
✅ **Automatic updates** - No user confirmation needed

## File Locations

```
src-tauri/target/release/bundle/nsis/
├── arceus_X.X.X_x64-setup.exe     ← Upload to GitHub
└── arceus_X.X.X_x64-setup.exe.sig ← Upload to GitHub
```

## Version Update Checklist

- [ ] Update `package.json` version
- [ ] Update `src-tauri/Cargo.toml` version
- [ ] Update `src-tauri/tauri.conf.json` version
- [ ] Run `npm run tauri build`
- [ ] Sign installer (if needed)
- [ ] Create GitHub release with both files
- [ ] Test update on older version