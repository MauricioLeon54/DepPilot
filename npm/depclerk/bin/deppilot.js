#!/usr/bin/env node

"use strict";

// This wrapper locates the platform-specific native binary installed
// as an optional dependency, then exec-spawns it with forwarded arguments.
// Pattern adapted from esbuild's npm distribution.

const path = require("path");
const childProcess = require("child_process");
const fs = require("fs");

const PLATFORM_PACKAGES = {
  "darwin-arm64": { pkg: "deppilot-darwin-arm64", bin: "deppilot" },
  "darwin-x64":   { pkg: "deppilot-darwin-x64",   bin: "deppilot" },
  "linux-arm64":  { pkg: "deppilot-linux-arm64",  bin: "deppilot" },
  "linux-x64":    { pkg: "deppilot-linux-x64",    bin: "deppilot" },
  "win32-x64":    { pkg: "deppilot-windows-x64",  bin: "deppilot.exe" },
};

function resolveBinary() {
  const key = `${process.platform}-${process.arch}`;
  const entry = PLATFORM_PACKAGES[key];

  if (!entry) {
    throw new Error(
      `DepPilot does not have a pre-built binary for ${key}.\n` +
      `Supported platforms: ${Object.keys(PLATFORM_PACKAGES).join(", ")}.\n` +
      `To build from source: cargo install deppilot`
    );
  }

  // 1. Try the optional dependency package (normal npm install path).
  try {
    return require.resolve(`${entry.pkg}/bin/${entry.bin}`);
  } catch (_) {
    // optional dependency not installed — fall through
  }

  // 2. Fall back to a binary placed next to this file (manual installs).
  const local = path.join(__dirname, entry.bin);
  if (fs.existsSync(local)) {
    return local;
  }

  throw new Error(
    `DepPilot binary not found for ${key}.\n` +
    `Expected optional dependency '${entry.pkg}' to be installed.\n` +
    `Try reinstalling: npm install -g deppilot`
  );
}

let binaryPath;
try {
  binaryPath = resolveBinary();
} catch (err) {
  process.stderr.write(`${err.message}\n`);
  process.exit(1);
}

const result = childProcess.spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit",
  windowsHide: false,
});

if (result.error) {
  process.stderr.write(`Failed to run DepPilot: ${result.error.message}\n`);
  process.exit(1);
}

process.exitCode = result.status ?? 1;
