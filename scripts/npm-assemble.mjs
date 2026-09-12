// Assembles the npm platform packages + main package from release artifacts.
// Usage: node scripts/npm-assemble.mjs <version> <artifacts-dir> <out-dir>
import { execSync } from "node:child_process";
import { mkdirSync, writeFileSync, copyFileSync, chmodSync, readFileSync, rmSync } from "node:fs";

const [version, artifacts, out] = process.argv.slice(2);
if (!version || !artifacts || !out) {
  console.error("usage: npm-assemble.mjs <version> <artifacts-dir> <out-dir>");
  process.exit(1);
}

const targets = {
  "aarch64-apple-darwin": { key: "darwin-arm64", os: "darwin", cpu: "arm64", archive: "tar" },
  "x86_64-apple-darwin": { key: "darwin-x64", os: "darwin", cpu: "x64", archive: "tar" },
  "x86_64-unknown-linux-gnu": { key: "linux-x64", os: "linux", cpu: "x64", archive: "tar" },
  "x86_64-pc-windows-msvc": { key: "win32-x64", os: "win32", cpu: "x64", archive: "zip" },
};

const optional = {};
for (const [target, t] of Object.entries(targets)) {
  const name = `@pirol/charta-${t.key}`;
  const dir = `${out}/charta-${t.key}`;
  const binName = t.os === "win32" ? "charta.exe" : "charta";
  mkdirSync(`${dir}/bin`, { recursive: true });
  const tmp = `${dir}/.extract`;
  mkdirSync(tmp, { recursive: true });
  if (t.archive === "tar") execSync(`tar -xzf "${artifacts}/charta-${target}.tar.gz" -C "${tmp}"`);
  else execSync(`unzip -q "${artifacts}/charta-${target}.zip" -d "${tmp}"`);
  copyFileSync(`${tmp}/${binName}`, `${dir}/bin/${binName}`);
  if (t.os !== "win32") chmodSync(`${dir}/bin/${binName}`, 0o755);
  rmSync(tmp, { recursive: true, force: true });
  writeFileSync(
    `${dir}/package.json`,
    JSON.stringify(
      {
        name,
        version,
        description: `charta binary for ${t.key}`,
        os: [t.os],
        cpu: [t.cpu],
        files: ["bin"],
        license: "Apache-2.0",
        repository: { type: "git", url: "git+https://github.com/Pirol-ai/company-as-code.git" },
      },
      null,
      2
    ) + "\n"
  );
  optional[name] = version;
}

const mainDir = `${out}/charta`;
mkdirSync(`${mainDir}/bin`, { recursive: true });
copyFileSync("npm/charta/bin/charta.js", `${mainDir}/bin/charta.js`);
copyFileSync("npm/charta/README.md", `${mainDir}/README.md`);
const base = JSON.parse(readFileSync("npm/charta/package.json", "utf8"));
base.version = version;
base.optionalDependencies = optional;
writeFileSync(`${mainDir}/package.json`, JSON.stringify(base, null, 2) + "\n");

// `npm init @pirol/charta` — thin initializer that calls `charta init`
const createDir = `${out}/create-charta`;
mkdirSync(`${createDir}/bin`, { recursive: true });
copyFileSync("npm/create-charta/bin/create-charta.js", `${createDir}/bin/create-charta.js`);
copyFileSync("npm/create-charta/README.md", `${createDir}/README.md`);
const createBase = JSON.parse(readFileSync("npm/create-charta/package.json", "utf8"));
createBase.version = version;
createBase.dependencies = { "@pirol/charta": version };
writeFileSync(`${createDir}/package.json`, JSON.stringify(createBase, null, 2) + "\n");

console.log(
  `assembled ${Object.keys(targets).length} platform packages + @pirol/charta@${version} + @pirol/create-charta@${version} in ${out}`
);
