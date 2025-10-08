const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Set environment variables for signing
process.env.TAURI_SIGNING_PRIVATE_KEY = 'D:\\Data\\GitProjects\\Arceus\\arceus.key';
process.env.TAURI_SIGNING_PRIVATE_KEY_PASSWORD = 'temp_password';

console.log('🔧 Setting up Tauri signing environment...');

// Read version from package.json
const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
const version = packageJson.version;

console.log(`📦 Building version ${version}...`);

// Build the app
try {
    execSync('npm run tauri build', { stdio: 'inherit' });
} catch (error) {
    console.error('❌ Build failed');
    process.exit(1);
}

console.log('✅ Build completed');

// Generate manifest
console.log('📝 Generating update manifest...');

const sigPath = `src-tauri/target/release/bundle/nsis/arceus_${version}_x64-setup.exe.sig`;
const exePath = `src-tauri/target/release/bundle/nsis/arceus_${version}_x64-setup.exe`;

if (!fs.existsSync(sigPath)) {
    console.error(`❌ Signature file not found: ${sigPath}`);
    process.exit(1);
}

if (!fs.existsSync(exePath)) {
    console.error(`❌ Executable not found: ${exePath}`);
    process.exit(1);
}

const signature = fs.readFileSync(sigPath, 'utf8').trim();

const manifest = {
    version: version,
    notes: `Release ${version}`,
    pub_date: new Date().toISOString(),
    platforms: {
        "windows-x86_64": {
            signature: signature,
            url: `https://github.com/B3n00n/Arceus/releases/download/v${version}/arceus_${version}_x64-setup.exe`
        }
    }
};

fs.writeFileSync('update-manifest.json', JSON.stringify(manifest, null, 2));
console.log('✅ Generated update-manifest.json');

// Create GitHub release using gh CLI
console.log('🚀 Creating GitHub release...');

try {
    // Create release (this will create the tag automatically)
    execSync(`gh release create v${version} --title "Release ${version}" --notes "Release ${version}" --target main`, { stdio: 'inherit' });

    // Upload files
    console.log('📤 Uploading files...');
    execSync(`gh release upload v${version} "${exePath}" "${sigPath}" "update-manifest.json"`, { stdio: 'inherit' });

    console.log('🎉 Release published successfully!');
    console.log(`🔗 View at: https://github.com/B3n00n/Arceus/releases/tag/v${version}`);

} catch (error) {
    console.error('❌ Failed to create release. Make sure you have gh CLI installed and authenticated.');
    console.log('📁 Files are ready for manual upload:');
    console.log(`   - ${exePath}`);
    console.log(`   - ${sigPath}`);
    console.log(`   - update-manifest.json`);
    process.exit(1);
}

// Clean up
fs.unlinkSync('update-manifest.json');
console.log('🧹 Cleaned up temporary files');
console.log('✨ All done!');