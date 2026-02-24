#!/usr/bin/env node
// Proxy entry point — delegates to the downloaded binary.

'use strict';

const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const binName = process.platform === 'win32'
  ? 'architect-linter-pro.exe'
  : 'architect-linter-pro';

const binPath = path.join(__dirname, 'bin', binName);

if (!fs.existsSync(binPath)) {
  console.error('❌ architect-linter-pro binary not found.');
  console.error('   Run `npm install` to re-download it, or build from source.');
  process.exit(1);
}

const result = spawnSync(binPath, process.argv.slice(2), {
  stdio: 'inherit',
  windowsHide: true,
});

if (result.error) {
  console.error(`Failed to run architect-linter-pro: ${result.error.message}`);
  process.exit(1);
}

process.exit(result.status ?? 1);
