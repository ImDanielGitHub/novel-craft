"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const projectRoot = path.resolve(__dirname, "..", "..");
const verifierPath = path.join(projectRoot, "npm", "scripts", "verify-package.js");

function loadVerifier() {
  assert.equal(
    fs.existsSync(verifierPath),
    true,
    "npm/scripts/verify-package.js must exist",
  );
  delete require.cache[require.resolve(verifierPath)];
  return require(verifierPath);
}

function validFiles() {
  return [
    { path: "package/package.json" },
    { path: "package/README.md" },
    { path: "package/LICENSE" },
    { path: "package/npm/bin/novel-craft.js" },
    { path: "package/npm/bin/novel-craft-x86_64-unknown-linux-gnu" },
    { path: "package/npm/lib/launcher.js" },
  ];
}

test("validatePackedFiles accepts the required launcher contract", () => {
  const { validatePackedFiles } = loadVerifier();
  const result = validatePackedFiles(validFiles(), { requireBinary: true });

  assert.deepEqual(result.binaries, [
    "npm/bin/novel-craft-x86_64-unknown-linux-gnu",
  ]);
});

test("validatePackedFiles rejects a missing launcher library", () => {
  const { validatePackedFiles } = loadVerifier();
  const files = validFiles().filter(
    (file) => file.path !== "package/npm/lib/launcher.js",
  );

  assert.throws(
    () => validatePackedFiles(files, { requireBinary: true }),
    /missing required package file: npm\/lib\/launcher\.js/i,
  );
});

test("validatePackedFiles rejects development-only paths", () => {
  const { validatePackedFiles } = loadVerifier();
  const files = [...validFiles(), { path: "package/npm/test/launcher.test.js" }];

  assert.throws(
    () => validatePackedFiles(files, { requireBinary: true }),
    /forbidden package path: npm\/test\/launcher\.test\.js/i,
  );
});

test("validatePackedFiles can require at least one release binary", () => {
  const { validatePackedFiles } = loadVerifier();
  const files = validFiles().filter(
    (file) => !file.path.includes("novel-craft-x86_64"),
  );

  assert.throws(
    () => validatePackedFiles(files, { requireBinary: true }),
    /no platform binary was included/i,
  );
});

test("validatePackedFiles permits a source dry run without release binaries", () => {
  const { validatePackedFiles } = loadVerifier();
  const files = validFiles().filter(
    (file) => !file.path.includes("novel-craft-x86_64"),
  );

  const result = validatePackedFiles(files, { requireBinary: false });
  assert.deepEqual(result.binaries, []);
});
