#!/usr/bin/env node
// Thin launcher: resolves the platform binary package (esbuild pattern) and execs it.
const { spawnSync } = require("child_process");

const platforms = {
  "darwin-arm64": "@pirol/charta-darwin-arm64",
  "darwin-x64": "@pirol/charta-darwin-x64",
  "linux-x64": "@pirol/charta-linux-x64",
  "win32-x64": "@pirol/charta-win32-x64",
};

const key = `${process.platform}-${process.arch}`;
const pkg = platforms[key];
if (!pkg) {
  console.error(`charta: unsupported platform ${key} — download a binary from GitHub releases instead`);
  process.exit(1);
}

let bin;
try {
  bin = require.resolve(`${pkg}/bin/charta${process.platform === "win32" ? ".exe" : ""}`);
} catch {
  console.error(
    `charta: platform package ${pkg} is missing.\n` +
      `Reinstall with optional dependencies enabled (do not use --no-optional / --omit=optional).`
  );
  process.exit(1);
}

const result = spawnSync(bin, process.argv.slice(2), { stdio: "inherit" });
process.exit(result.status ?? 1);
