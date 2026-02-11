#!/usr/bin/env node

const { existsSync, mkdirSync, chmodSync } = require('fs');
const { join } = require('path');
const { get } = require('https');
const { pipeline } = require('stream');
const { promisify } = require('util');
const { createWriteStream, createReadStream } = require('fs');
const { createGunzip } = require('zlib');
const tar = require('tar');

const streamPipeline = promisify(pipeline);

const VERSION = require('./package.json').version;

// Map Node's platform/arch to Rust target triples
function getTarget() {
  const platform = process.platform;
  const arch = process.arch;

  const targets = {
    'darwin-x64': 'x86_64-apple-darwin',
    'darwin-arm64': 'aarch64-apple-darwin',
    'linux-x64': 'x86_64-unknown-linux-gnu',
    'linux-arm64': 'aarch64-unknown-linux-gnu',
    'win32-x64': 'x86_64-pc-windows-msvc',
  };

  const key = `${platform}-${arch}`;
  const target = targets[key];

  if (!target) {
    throw new Error(
      `Unsupported platform: ${platform} ${arch}\n` +
      `Supported: ${Object.keys(targets).join(', ')}`
    );
  }

  return target;
}

async function download(url, dest) {
  return new Promise((resolve, reject) => {
    get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        return download(response.headers.location, dest).then(resolve, reject);
      }

      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: ${response.statusCode}`));
        return;
      }

      const file = createWriteStream(dest);
      response.pipe(file);
      file.on('finish', () => {
        file.close();
        resolve();
      });
      file.on('error', reject);
    }).on('error', reject);
  });
}

async function install() {
  try {
    const target = getTarget();
    const binDir = join(__dirname, 'bin');
    const ext = process.platform === 'win32' ? '.exe' : '';
    const binaryName = `lob${ext}`;
    const binaryPath = join(binDir, binaryName);

    // Create bin directory
    if (!existsSync(binDir)) {
      mkdirSync(binDir, { recursive: true });
    }

    // Download URL
    const archiveName = process.platform === 'win32'
      ? `lob-${VERSION}-${target}.zip`
      : `lob-${VERSION}-${target}.tar.gz`;

    const url = `https://github.com/olirice/lob/releases/download/v${VERSION}/${archiveName}`;

    console.log(`Downloading lob ${VERSION} for ${target}...`);
    console.log(`URL: ${url}`);

    const archivePath = join(__dirname, archiveName);

    // Download the archive
    await download(url, archivePath);

    console.log('Extracting...');

    // Extract based on platform
    if (process.platform === 'win32') {
      const AdmZip = require('adm-zip');
      const zip = new AdmZip(archivePath);
      zip.extractEntryTo('lob.exe', binDir, false, true);
    } else {
      // Extract tar.gz
      await tar.x({
        file: archivePath,
        cwd: binDir,
      });
    }

    // Make executable on Unix
    if (process.platform !== 'win32') {
      chmodSync(binaryPath, 0o755);
    }

    console.log('✓ lob installed successfully!');
    console.log(`Binary location: ${binaryPath}`);

    // Cleanup
    const { unlinkSync } = require('fs');
    unlinkSync(archivePath);

  } catch (error) {
    console.error('Installation failed:', error.message);
    process.exit(1);
  }
}

install();
