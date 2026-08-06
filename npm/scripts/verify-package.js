#!/usr/bin/env node
"use strict";

const { execFileSync } = require("node:child_process");

const REQUIRED_FILES = Object.freeze([
  "LICENSE",
  "README.md",
  "npm/bin/novel-craft.js",
  "npm/lib/launcher.js",
  "package.json",
]);

const FORBIDDEN_PREFIXES = Object.freeze([
  ".git/",
  ".github/",
  ".novel/",
  "evals/",
  "node_modules/",
  "npm/scripts/",
  "npm/test/",
  "references/",
  "rules/",
  "skills/",
  "src/",
  "target/",
  "tests/",
]);

const FORBIDDEN_FILES = new Set([
  "Cargo.lock",
  "Cargo.toml",
  "deny.toml",
]);

function normalisePackedPath(filePath) {
  return filePath
    .replace(/\\/g, "/")
    .replace(/^\.\//, "")
    .replace(/^package\//, "");
}

function isPlatformBinary(filePath) {
  return (
    /^npm\/bin\/novel-craft-.+/.test(filePath) &&
    !filePath.endsWith(".sha256") &&
    !filePath.endsWith(".js")
  );
}

function validatePackedFiles(files, { requireBinary = false } = {}) {
  if (!Array.isArray(files)) {
    throw new TypeError("npm pack output must contain a files array");
  }

  const paths = files
    .map((file) => normalisePackedPath(typeof file === "string" ? file : file.path))
    .sort();
  const pathSet = new Set(paths);

  for (const required of REQUIRED_FILES) {
    if (!pathSet.has(required)) {
      throw new Error(`Missing required package file: ${required}`);
    }
  }

  for (const filePath of paths) {
    if (FORBIDDEN_FILES.has(filePath)) {
      throw new Error(`Forbidden package path: ${filePath}`);
    }
    if (FORBIDDEN_PREFIXES.some((prefix) => filePath.startsWith(prefix))) {
      throw new Error(`Forbidden package path: ${filePath}`);
    }
    if (filePath.endsWith(".tgz")) {
      throw new Error(`Forbidden package path: ${filePath}`);
    }
  }

  const binaries = paths.filter(isPlatformBinary);
  if (requireBinary && binaries.length === 0) {
    throw new Error("No platform binary was included in the npm package");
  }

  return { binaries, files: paths };
}

function readPackManifest() {
  const npmExecPath = process.env.npm_execpath;
  const command = npmExecPath ? process.execPath : process.platform === "win32" ? "npm.cmd" : "npm";
  const args = npmExecPath
    ? [npmExecPath, "pack", "--dry-run", "--json"]
    : ["pack", "--dry-run", "--json"];
  const output = execFileSync(command, args, {
    cwd: process.cwd(),
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
  const manifests = JSON.parse(output);
  if (!Array.isArray(manifests) || manifests.length !== 1) {
    throw new Error("Expected npm pack to return exactly one package manifest");
  }
  return manifests[0];
}

function main(argv = process.argv.slice(2)) {
  const requireBinary = argv.includes("--require-binary");
  const unknown = argv.filter((arg) => arg !== "--require-binary");
  if (unknown.length > 0) {
    throw new Error(`Unknown argument: ${unknown[0]}`);
  }

  const manifest = readPackManifest();
  const result = validatePackedFiles(manifest.files, { requireBinary });
  process.stdout.write(
    `${JSON.stringify(
      {
        status: "ok",
        package: manifest.name,
        version: manifest.version,
        file_count: result.files.length,
        binaries: result.binaries,
      },
      null,
      2,
    )}\n`,
  );
}

if (require.main === module) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`Package verification failed: ${error.message}\n`);
    process.exit(1);
  }
}

module.exports = {
  FORBIDDEN_FILES,
  FORBIDDEN_PREFIXES,
  REQUIRED_FILES,
  isPlatformBinary,
  main,
  normalisePackedPath,
  readPackManifest,
  validatePackedFiles,
};
