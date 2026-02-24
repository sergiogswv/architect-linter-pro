#!/usr/bin/env node
// Downloads the correct pre-built binary for this platform on npm postinstall.

'use strict';

const https = require('https');
const fs = require('fs');
const path = require('path');

const VERSION = require('./package.json').version;
const GITHUB_BASE = `https://github.com/sergiogswv/architect-linter-pro/releases/download/v${VERSION}`;

const PLATFORM_BINARIES = {
  'linux-x64':    'architect-linter-pro-linux-x86_64',
  'linux-arm64':  'architect-linter-pro-linux-aarch64',
  'darwin-x64':   'architect-linter-pro-macos-x86_64',
  'darwin-arm64': 'architect-linter-pro-macos-aarch64',
  'win32-x64':    'architect-linter-pro-windows-x86_64.exe',
};

const platformKey = `${process.platform}-${process.arch}`;
const artifactName = PLATFORM_BINARIES[platformKey];

if (!artifactName) {
  console.error(`\n❌ Unsupported platform: ${platformKey}`);
  console.error('   Supported: linux-x64, linux-arm64, darwin-x64, darwin-arm64, win32-x64');
  console.error('   Build from source: https://github.com/sergiogswv/architect-linter-pro\n');
  process.exit(1);
}

const binDir = path.join(__dirname, 'bin');
const binName = process.platform === 'win32'
  ? 'architect-linter-pro.exe'
  : 'architect-linter-pro';
const binPath = path.join(binDir, binName);

// Create bin directory
fs.mkdirSync(binDir, { recursive: true });

const downloadUrl = `${GITHUB_BASE}/${artifactName}`;

/**
 * Downloads a URL to a file, following up to 10 redirects.
 */
function download(url, destPath, redirectCount = 0) {
  return new Promise((resolve, reject) => {
    if (redirectCount > 10) {
      return reject(new Error('Too many redirects'));
    }

    const file = fs.createWriteStream(destPath);

    https.get(url, { headers: { 'User-Agent': 'architect-linter-npm-installer' } }, (res) => {
      if (res.statusCode === 301 || res.statusCode === 302) {
        file.close();
        fs.unlinkSync(destPath);
        const redirectUrl = res.headers.location;
        if (!redirectUrl) return reject(new Error('Redirect with no location header'));
        return resolve(download(redirectUrl, destPath, redirectCount + 1));
      }

      if (res.statusCode !== 200) {
        file.close();
        fs.unlinkSync(destPath);
        return reject(new Error(`HTTP ${res.statusCode}: ${url}`));
      }

      res.pipe(file);
      file.on('finish', () => { file.close(); resolve(); });
      file.on('error', (err) => { fs.unlinkSync(destPath); reject(err); });
    }).on('error', (err) => {
      if (fs.existsSync(destPath)) fs.unlinkSync(destPath);
      reject(err);
    });
  });
}

console.log(`\n📦 Installing architect-linter-pro v${VERSION} for ${platformKey}...`);
console.log(`   Downloading from: ${downloadUrl}\n`);

download(downloadUrl, binPath)
  .then(() => {
    if (process.platform !== 'win32') {
      fs.chmodSync(binPath, 0o755);
    }
    console.log(`✅ architect-linter-pro installed successfully\n`);
  })
  .catch((err) => {
    console.error(`\n❌ Failed to download binary: ${err.message}`);
    console.error('   Try building from source:');
    console.error('   https://github.com/sergiogswv/architect-linter-pro#installation\n');
    // Exit 0 to not block npm install on download failure (graceful degradation)
    process.exit(0);
  });
