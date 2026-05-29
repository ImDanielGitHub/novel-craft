#!/usr/bin/env node
"use strict";

const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");

function platformTriple() {
  const platformMap = {
    darwin: "apple-darwin",
    linux: "unknown-linux-gnu",
    win32: "pc-windows-msvc",
  };
  const archMap = {
    arm64: "aarch64",
    x64: "x86_64",
  };
  const platform = platformMap[process.platform];
  const arch = archMap[process.arch];
  if (!platform || !arch) {
    return null;
  }
  return `${arch}-${platform}`;
}

const ext = process.platform === "win32" ? ".exe" : "";
const triple = platformTriple();
const candidates = [];

if (triple) {
  candidates.push(path.join(__dirname, `novel-craft-${triple}${ext}`));
}

candidates.push(path.join(__dirname, "..", "..", "target", "release", `novel-craft${ext}`));
candidates.push(path.join(__dirname, "..", "..", "target", "debug", `novel-craft${ext}`));

const binary = candidates.find((candidate) => fs.existsSync(candidate));

if (!binary) {
  console.error("Novel Craft binary was not found for this platform.");
  console.error("");
  console.error("If you are running from a source checkout, build it first:");
  console.error("  cargo build --release");
  console.error("");
  console.error("If this came from npm, please open an issue with your OS and architecture:");
  console.error("  https://github.com/ImDanielGitHub/novel-craft/issues");
  process.exit(127);
}

const result = spawnSync(binary, process.argv.slice(2), {
  stdio: "inherit",
  env: {
    ...process.env,
    NOVEL_CRAFT_NPM_WRAPPER: "1",
    NOVEL_CRAFT_NPM_WRAPPER_PATH: __filename,
  },
});

if (result.error) {
  console.error(result.error.message);
  process.exit(127);
}

process.exit(result.status === null ? 1 : result.status);
