#!/usr/bin/env node
// `npm init @pirol/charta` — runs `charta init` with whatever arguments follow.
// Example: npm init @pirol/charta -- --name "Acme GmbH"
const { spawnSync } = require("child_process");

let shim;
try {
  shim = require.resolve("@pirol/charta/bin/charta.js");
} catch {
  console.error("create-charta: could not find @pirol/charta — try: npm i -g @pirol/charta");
  process.exit(1);
}

const result = spawnSync(process.execPath, [shim, "init", ...process.argv.slice(2)], {
  stdio: "inherit",
});
process.exit(result.status ?? 1);
