# Arceus Project Setup Guide

Complete guide to set up the Arceus project on a new computer.

## Prerequisites

### Required Software

1. **Node.js** (v18 or later)
   - Download: https://nodejs.org/
   - Verify: `node --version` and `npm --version`

2. **Rust** (latest stable)
   - Download: https://rustup.rs/
   - Verify: `rustc --version` and `cargo --version`

3. **Git**
   - Download: https://git-scm.com/
   - Verify: `git --version`

4. **Tauri Prerequisites** (for Arkeus desktop app)
   - **Windows**:
     - Microsoft Visual Studio C++ Build Tools
     - WebView2 (usually pre-installed on Windows 11)
   - **macOS**:
     - Xcode Command Line Tools: `xcode-select --install`
   - **Linux**:
     - `sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`

5. **Google Cloud SDK** (for GCS operations)
   - Download: https://cloud.google.com/sdk/docs/install
   - Verify: `gcloud --version`

## Project Structure

```
Arceus/
├── src-tauri/          # Arkeus - Tauri desktop app (Rust + React)
├── alakazam/           # Alakazam - Backend API server (Rust)
├── giratina/           # Giratina - Admin web dashboard (React)
├── calyrex/            # Calyrex - Clipboard utility (Rust)
└── arceus.key          # Tauri update signing key (DO NOT SHARE)
```

## Setup Steps

### 1. Clone the Repository

```bash
git clone <your-repo-url> Arceus
cd Arceus
```

### 2. Set Up Arkeus (Desktop App)

```bash
# Install Node.js dependencies (root + src-tauri)
npm install

# Build the desktop app (or run in dev mode)
npm run tauri dev      # Development mode with hot reload
npm run tauri build    # Production build
```

**Environment Variables** (optional for local dev):
- No specific env vars needed for basic development
- For signing updates, see "Code Signing" section below

### 3. Set Up Alakazam (Backend API)

```bash
cd alakazam

# Build the project
cargo build --release

# Set up environment variables
cp .env.example .env    # If you have an example file
# OR create .env manually:
```

**Required `.env` file** (`alakazam/.env`):
```env
# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=43571

# Database
DATABASE_URL=sqlite://alakazam.db

# Google Cloud Storage
GCS_BUCKET_NAME=your-bucket-name
GCS_SIGNED_URL_DURATION_SECS=3600

# CORS (for Giratina frontend)
CORS_ALLOWED_ORIGIN=http://localhost:5173

# GCP Service Account (for local development)
# In production, use GCP metadata server instead
GCP_SERVICE_ACCOUNT_EMAIL=your-service-account@project.iam.gserviceaccount.com
```

**Database Setup:**
```bash
# Alakazam uses SQLite with sqlx
# Run migrations (if you have a migrations/ folder)
cargo install sqlx-cli
sqlx migrate run

# OR the database will auto-initialize on first run
cargo run
```

**GCP Service Account Setup:**

For local development, you need GCP credentials:

1. Go to GCP Console → IAM & Admin → Service Accounts
2. Create a service account (or use existing)
3. Grant permissions:
   - `Storage Object Admin` (for GCS uploads)
   - `Service Account Token Creator` (for signing URLs)
4. **For local dev only**: Download JSON key → save as `alakazam-sa-key.json`
5. Authenticate locally:
   ```bash
   gcloud auth application-default login
   # OR set environment variable:
   export GOOGLE_APPLICATION_CREDENTIALS=/path/to/alakazam-sa-key.json
   ```

**Important**: In production (Cloud Run), use workload identity - no key file needed!

### 4. Set Up Giratina (Admin Dashboard)

```bash
cd giratina

# Install dependencies
npm install

# Set up environment variables
cp .env.example .env    # If you have an example file
# OR create .env manually:
```

**Required `.env` file** (`giratina/.env`):
```bash
# Copy the example file and edit it
cp .env.example .env
# Then edit .env with your actual values
```

See `giratina/.env.example` for all configuration options.

**Run development server:**
```bash
npm run dev
# Opens at http://localhost:5173
```

**Build for production:**
```bash
npm run build
# Output in giratina/dist/
```

### 5. Set Up Calyrex (Optional - Clipboard Utility)

```bash
cd calyrex

# Build
cargo build --release

# Run
cargo run
```

## Code Signing (Arkeus Updates)

The `arceus.key` file is used to sign Tauri app updates.

**On the original computer:**
1. The key pair already exists:
   - Private key: `arceus.key` (keep secret!)
   - Public key: `arceus.key.pub`
   - Public key is also embedded in `src-tauri/tauri.conf.json`

**On a new computer:**

**Option A - Copy existing keys** (recommended):
```bash
# Copy both files to the new computer
arceus.key
arceus.key.pub

# Place them in the project root
# They're already in .gitignore (DO NOT commit to git!)
```

**Option B - Generate new keys** (only if keys are lost):
```bash
# Install Tauri CLI
cargo install tauri-cli

# Generate new key pair
cargo tauri signer generate -w arceus.key

# Update tauri.conf.json with the new public key
# (The command will show you the public key to paste in)
```

**Important**: If you generate new keys, old app versions won't accept updates!

## Running the Full Stack Locally

1. **Start Alakazam** (backend):
   ```bash
   cd alakazam
   cargo run
   # Running on http://localhost:43571
   ```

2. **Start Giratina** (admin dashboard):
   ```bash
   cd giratina
   npm run dev
   # Running on http://localhost:5173
   ```

3. **Start Arkeus** (desktop app):
   ```bash
   cd ..  # back to root
   npm run tauri dev
   # Desktop app will open
   ```

## Deployment

### Alakazam (Backend)
- Deploy to GCP Cloud Run
- Set environment variables in Cloud Run
- Use GCP Secret Manager for sensitive values
- No service account key file needed (uses workload identity)

### Giratina (Admin Dashboard)
- Build: `npm run build`
- Deploy to GCP Cloud Run or static hosting
- Set `VITE_API_URL` to production Alakazam URL

### Arkeus (Desktop App)
- Build: `npm run tauri build`
- Sign updates with `arceus.key`
- Publish to GitHub releases (see `publish.cjs`)

## Common Issues

### Issue: `sqlx` migration errors
**Solution**: Ensure SQLite database exists or run migrations

### Issue: GCP authentication failed
**Solution**: Run `gcloud auth application-default login`

### Issue: Tauri build fails on Windows
**Solution**: Install Visual Studio C++ Build Tools

### Issue: CORS errors in Giratina
**Solution**:
1. Check `CORS_ALLOWED_ORIGIN` in Alakazam `.env`
2. Ensure GCS bucket has CORS configured (see main README)

## Security Checklist

- [ ] Never commit `.env` files
- [ ] Never commit `arceus.key` or service account JSON keys
- [ ] Use GCP Secret Manager in production
- [ ] Use workload identity for Cloud Run (no key files)
- [ ] Keep `arceus.key` backed up securely (encrypted backup)

## Next Steps

1. Test all three projects locally
2. Configure GCS bucket CORS (if needed)
3. Deploy to production
4. Set up CI/CD (optional)

## Need Help?

- Check existing documentation: `IMPLEMENTATION.md`, `SYSTEM_ARCHITECTURE.md`
- Review deployment guides: `PRODUCTION_READINESS.md`
