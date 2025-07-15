#!/usr/bin/env node

const { spawn } = require('child_process');
const os = require('os');
const path = require('path');
const fs = require('fs');

function getBinaryName() {
  const platform = os.platform();
  let bin = 'gigli-';

  if (platform === 'win32') {
    bin += 'win.exe';
  } else if (platform === 'darwin') {
    bin += 'macos';
  } else if (platform === 'linux') {
    bin += 'linux';
  } else {
    console.error('Unsupported platform:', platform);
    process.exit(1);
  }
  return path.join(__dirname, bin);
}

const binPath = getBinaryName();

if (!fs.existsSync(binPath)) {
  console.error('Binary not found for your platform:', binPath);
  process.exit(1);
}

// Ensure the binary is executable
if (os.platform() !== 'win32') {
  fs.chmodSync(binPath, 0o755);
}

const args = process.argv.slice(2);
const child = spawn(binPath, args, { stdio: 'inherit' });

child.on('exit', code => process.exit(code));
