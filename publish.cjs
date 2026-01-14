const fs = require('fs');
const { execSync } = require('child_process');

// Read version from package.json
const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
const version = packageJson.version;

console.log(`📦 Preparing to publish version ${version}...`);

// Check if tag already exists
try {
    execSync(`git rev-parse v${version}`, { stdio: 'pipe' });
    console.log(`⚠️  Tag v${version} already exists locally`);

    const readline = require('readline').createInterface({
        input: process.stdin,
        output: process.stdout
    });

    readline.question('Do you want to delete and recreate it? (y/N): ', (answer) => {
        readline.close();
        if (answer.toLowerCase() === 'y') {
            execSync(`git tag -d v${version}`, { stdio: 'inherit' });
            execSync(`git push origin :refs/tags/v${version}`, { stdio: 'inherit' });
            createAndPushTag();
        } else {
            console.log('❌ Aborted');
            process.exit(1);
        }
    });
} catch (error) {
    // Tag doesn't exist, create it
    createAndPushTag();
}

function createAndPushTag() {
    try {
        // Create tag
        console.log(`🏷️  Creating tag v${version}...`);
        execSync(`git tag -a v${version} -m "Release ${version}"`, { stdio: 'inherit' });

        // Push tag
        console.log(`📤 Pushing tag to GitHub...`);
        execSync(`git push origin v${version}`, { stdio: 'inherit' });

        console.log('');
        console.log('✅ Tag pushed successfully!');
        console.log('');
        console.log('🤖 GitHub Actions will now:');
        console.log('   1. Build for Windows and Linux');
        console.log('   2. Sign the builds');
        console.log('   3. Create a GitHub release');
        console.log('   4. Upload all artifacts');
        console.log('   5. Generate update manifest');
        console.log('');
        console.log(`🔗 Track progress at: https://github.com/B3n00n/Arceus/actions`);
        console.log(`🔗 Release will be at: https://github.com/B3n00n/Arceus/releases/tag/v${version}`);

    } catch (error) {
        console.error('❌ Failed to create/push tag');
        console.error(error.message);
        process.exit(1);
    }
}
