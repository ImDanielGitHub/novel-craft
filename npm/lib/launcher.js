"use strict";

const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const PLATFORM_SUFFIXES = Object.freeze({
  darwin: "apple-darwin",
  linux: "unknown-linux-gnu",
  win32: "pc-windows-msvc",
});

const ARCH_PREFIXES = Object.freeze({
  arm64: "aarch64",
  x64: "x86_64",
});

function platformTriple(platform, arch) {
  const platformSuffix = PLATFORM_SUFFIXES[platform];
  const archPrefix = ARCH_PREFIXES[arch];
  if (!platformSuffix || !archPrefix) {
    return null;
  }
  return `${archPrefix}-${platformSuffix}`;
}

function executableName(platform) {
  return platform === "win32" ? "novel-craft.exe" : "novel-craft";
}

function buildCandidates({
  binDir,
  projectRoot,
  platform,
  arch,
  env,
  pathModule = path,
}) {
  const candidates = [];
  const override = env.NOVEL_CRAFT_BINARY && env.NOVEL_CRAFT_BINARY.trim();
  if (override) {
    candidates.push(pathModule.resolve(override));
  }

  const triple = platformTriple(platform, arch);
  const extension = platform === "win32" ? ".exe" : "";
  if (triple) {
    candidates.push(
      pathModule.join(binDir, `novel-craft-${triple}${extension}`),
    );
  }

  const binaryName = executableName(platform);
  candidates.push(pathModule.join(projectRoot, "target", "release", binaryName));
  candidates.push(pathModule.join(projectRoot, "target", "debug", binaryName));

  return [...new Set(candidates)];
}

function inspectCandidate(candidate, { platform, fsModule = fs }) {
  let stats;
  try {
    stats = fsModule.statSync(candidate);
  } catch (error) {
    if (error && error.code === "ENOENT") {
      return { path: candidate, status: "missing" };
    }
    return {
      path: candidate,
      status: "unreadable",
      detail: error && error.message ? error.message : String(error),
    };
  }

  if (!stats.isFile()) {
    return { path: candidate, status: "not a file" };
  }

  if (platform !== "win32") {
    try {
      fsModule.accessSync(candidate, fsModule.constants.X_OK);
    } catch (error) {
      return {
        path: candidate,
        status: "not executable",
        detail: error && error.message ? error.message : String(error),
      };
    }
  }

  return { path: candidate, status: "usable" };
}

function listPackagedBinaries(binDir, { fsModule = fs, pathModule = path } = {}) {
  let entries;
  try {
    entries = fsModule.readdirSync(binDir, { withFileTypes: true });
  } catch {
    return [];
  }

  return entries
    .filter(
      (entry) =>
        entry.isFile() &&
        /^novel-craft-.+/.test(entry.name) &&
        !entry.name.endsWith(".sha256"),
    )
    .map((entry) => pathModule.join(binDir, entry.name))
    .sort();
}

function resolveBinary({
  binDir,
  projectRoot,
  platform,
  arch,
  env,
  fsModule = fs,
  pathModule = path,
}) {
  const candidates = buildCandidates({
    binDir,
    projectRoot,
    platform,
    arch,
    env,
    pathModule,
  });
  const inspections = candidates.map((candidate) =>
    inspectCandidate(candidate, { platform, fsModule }),
  );
  const usable = inspections.find((inspection) => inspection.status === "usable");

  return {
    binary: usable ? usable.path : null,
    candidates,
    inspections,
    packagedBinaries: listPackagedBinaries(binDir, { fsModule, pathModule }),
    triple: platformTriple(platform, arch),
  };
}

function formatResolutionError({ platform, arch, resolution }) {
  const lines = [
    "Novel Craft could not find a usable binary.",
    "",
    `Detected platform: ${platform}`,
    `Detected architecture: ${arch}`,
    `Expected Rust target: ${resolution.triple || "unsupported"}`,
    "",
    "Searched paths:",
  ];

  for (const inspection of resolution.inspections) {
    const detail = inspection.detail ? `: ${inspection.detail}` : "";
    lines.push(`  - ${inspection.path} (${inspection.status}${detail})`);
  }

  lines.push("", "Packaged binaries found:");
  if (resolution.packagedBinaries.length === 0) {
    lines.push("  - none");
  } else {
    for (const binary of resolution.packagedBinaries) {
      lines.push(`  - ${binary}`);
    }
  }

  lines.push(
    "",
    "To use a specific build, set NOVEL_CRAFT_BINARY to its file path.",
    "For a source checkout, run: cargo build --release",
    "For an npm install, report the diagnostics above at:",
    "  https://github.com/ImDanielGitHub/novel-craft/issues",
  );

  return `${lines.join("\n")}\n`;
}

function formatExecutionError(binary, error) {
  const code = error && error.code ? ` (${error.code})` : "";
  const message = error && error.message ? error.message : String(error);
  return `Novel Craft could not launch ${binary}${code}: ${message}\n`;
}

function runLauncher({
  argv = process.argv.slice(2),
  env = process.env,
  platform = process.platform,
  arch = process.arch,
  binDir,
  projectRoot,
  wrapperPath,
  fsModule = fs,
  pathModule = path,
  spawnSyncFn = spawnSync,
  processObject = process,
  stderr = process.stderr,
} = {}) {
  if (!binDir) {
    throw new TypeError("runLauncher requires binDir");
  }

  const resolvedProjectRoot = projectRoot || pathModule.resolve(binDir, "..", "..");
  const resolvedWrapperPath = wrapperPath || pathModule.join(binDir, "novel-craft.js");
  const resolution = resolveBinary({
    binDir,
    projectRoot: resolvedProjectRoot,
    platform,
    arch,
    env,
    fsModule,
    pathModule,
  });

  if (!resolution.binary) {
    stderr.write(formatResolutionError({ platform, arch, resolution }));
    return 127;
  }

  const result = spawnSyncFn(resolution.binary, argv, {
    stdio: "inherit",
    env: {
      ...env,
      NOVEL_CRAFT_NPM_WRAPPER: "1",
      NOVEL_CRAFT_NPM_WRAPPER_PATH: resolvedWrapperPath,
    },
  });

  if (result.error) {
    stderr.write(formatExecutionError(resolution.binary, result.error));
    return 127;
  }

  if (result.signal) {
    try {
      processObject.kill(processObject.pid, result.signal);
      return null;
    } catch (error) {
      stderr.write(
        `Novel Craft child process ended with ${result.signal}, but the wrapper could not preserve that signal: ${error.message}\n`,
      );
      return 1;
    }
  }

  return Number.isInteger(result.status) ? result.status : 1;
}

module.exports = {
  buildCandidates,
  executableName,
  formatExecutionError,
  formatResolutionError,
  inspectCandidate,
  listPackagedBinaries,
  platformTriple,
  resolveBinary,
  runLauncher,
};
