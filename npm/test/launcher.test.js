"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const projectRoot = path.resolve(__dirname, "..", "..");
const launcherModulePath = path.join(projectRoot, "npm", "lib", "launcher.js");
const wrapperSourcePath = path.join(projectRoot, "npm", "bin", "novel-craft.js");

function expectedTriple(platform, arch) {
  const platformMap = {
    darwin: "apple-darwin",
    linux: "unknown-linux-gnu",
    win32: "pc-windows-msvc",
  };
  const archMap = {
    arm64: "aarch64",
    x64: "x86_64",
  };
  if (!platformMap[platform] || !archMap[arch]) {
    return null;
  }
  return `${archMap[arch]}-${platformMap[platform]}`;
}

function loadLauncherModule() {
  assert.equal(
    fs.existsSync(launcherModulePath),
    true,
    "npm/lib/launcher.js must exist",
  );
  delete require.cache[require.resolve(launcherModulePath)];
  return require(launcherModulePath);
}

function createPackageFixture(t) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "novel-craft-wrapper-"));
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));

  const binDir = path.join(root, "npm", "bin");
  const libDir = path.join(root, "npm", "lib");
  fs.mkdirSync(binDir, { recursive: true });
  fs.mkdirSync(libDir, { recursive: true });
  fs.copyFileSync(wrapperSourcePath, path.join(binDir, "novel-craft.js"));

  if (fs.existsSync(launcherModulePath)) {
    fs.copyFileSync(launcherModulePath, path.join(libDir, "launcher.js"));
  }

  return {
    root,
    binDir,
    wrapperPath: path.join(binDir, "novel-craft.js"),
  };
}

function writeNodeFixture(root, source) {
  const fixturePath = path.join(root, "fixture.js");
  fs.writeFileSync(fixturePath, source, "utf8");
  return fixturePath;
}

function wrapperEnv(overrides = {}) {
  const env = { ...process.env };
  delete env.NOVEL_CRAFT_BINARY;
  return { ...env, ...overrides };
}

test("platformTriple maps supported Node platforms to Rust targets", () => {
  const { platformTriple } = loadLauncherModule();

  assert.equal(platformTriple("linux", "x64"), "x86_64-unknown-linux-gnu");
  assert.equal(platformTriple("linux", "arm64"), "aarch64-unknown-linux-gnu");
  assert.equal(platformTriple("darwin", "x64"), "x86_64-apple-darwin");
  assert.equal(platformTriple("darwin", "arm64"), "aarch64-apple-darwin");
  assert.equal(platformTriple("win32", "x64"), "x86_64-pc-windows-msvc");
  assert.equal(platformTriple("freebsd", "x64"), null);
  assert.equal(platformTriple("linux", "riscv64"), null);
});

test("buildCandidates prioritises an explicit binary override", () => {
  const { buildCandidates } = loadLauncherModule();
  const candidates = buildCandidates({
    binDir: path.join("pkg", "npm", "bin"),
    projectRoot: "pkg",
    platform: "linux",
    arch: "x64",
    env: { NOVEL_CRAFT_BINARY: path.join("custom", "novel-craft") },
  });

  assert.deepEqual(candidates, [
    path.resolve(path.join("custom", "novel-craft")),
    path.join("pkg", "npm", "bin", "novel-craft-x86_64-unknown-linux-gnu"),
    path.join("pkg", "target", "release", "novel-craft"),
    path.join("pkg", "target", "debug", "novel-craft"),
  ]);
});

test("NOVEL_CRAFT_BINARY launches the explicit binary with unchanged arguments", (t) => {
  const fixture = createPackageFixture(t);
  const child = writeNodeFixture(
    fixture.root,
    "console.log(JSON.stringify({ argv: process.argv.slice(2) }));\n",
  );

  const result = spawnSync(
    process.execPath,
    [fixture.wrapperPath, child, "chapter one", "--json"],
    {
      encoding: "utf8",
      env: wrapperEnv({ NOVEL_CRAFT_BINARY: process.execPath }),
    },
  );

  assert.equal(result.status, 0, result.stderr);
  assert.deepEqual(JSON.parse(result.stdout.trim()).argv, ["chapter one", "--json"]);
});

test("the child receives wrapper metadata", (t) => {
  const fixture = createPackageFixture(t);
  const child = writeNodeFixture(
    fixture.root,
    "console.log(JSON.stringify({ enabled: process.env.NOVEL_CRAFT_NPM_WRAPPER, wrapperPath: process.env.NOVEL_CRAFT_NPM_WRAPPER_PATH }));\n",
  );

  const result = spawnSync(process.execPath, [fixture.wrapperPath, child], {
    encoding: "utf8",
    env: wrapperEnv({ NOVEL_CRAFT_BINARY: process.execPath }),
  });

  assert.equal(result.status, 0, result.stderr);
  const data = JSON.parse(result.stdout.trim());
  assert.equal(data.enabled, "1");
  assert.equal(data.wrapperPath, fixture.wrapperPath);
});

test("a missing binary reports the detected platform, target, and searched paths", (t) => {
  const fixture = createPackageFixture(t);
  const result = spawnSync(process.execPath, [fixture.wrapperPath, "--version"], {
    encoding: "utf8",
    env: wrapperEnv(),
  });

  assert.equal(result.status, 127);
  assert.match(result.stderr, new RegExp(`platform: ${process.platform}`));
  assert.match(result.stderr, new RegExp(`architecture: ${process.arch}`));
  const triple = expectedTriple(process.platform, process.arch);
  if (triple) {
    assert.match(result.stderr, new RegExp(triple.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
  assert.match(result.stderr, /Searched paths:/);
  assert.match(result.stderr, /NOVEL_CRAFT_BINARY/);
});

test(
  "a non-executable packaged binary is rejected with an actionable reason",
  { skip: process.platform === "win32" },
  (t) => {
    const triple = expectedTriple(process.platform, process.arch);
    if (!triple) {
      t.skip("current platform has no packaged target mapping");
      return;
    }

    const fixture = createPackageFixture(t);
    const binaryPath = path.join(fixture.binDir, `novel-craft-${triple}`);
    fs.writeFileSync(binaryPath, "not executable", { mode: 0o644 });
    fs.chmodSync(binaryPath, 0o644);

    const result = spawnSync(process.execPath, [fixture.wrapperPath], {
      encoding: "utf8",
      env: wrapperEnv(),
    });

    assert.equal(result.status, 127);
    assert.match(result.stderr, /not executable/i);
    assert.match(result.stderr, new RegExp(path.basename(binaryPath)));
  },
);

test("a directory at a candidate path is rejected as not a file", (t) => {
  const triple = expectedTriple(process.platform, process.arch);
  if (!triple) {
    t.skip("current platform has no packaged target mapping");
    return;
  }

  const fixture = createPackageFixture(t);
  const ext = process.platform === "win32" ? ".exe" : "";
  const binaryPath = path.join(fixture.binDir, `novel-craft-${triple}${ext}`);
  fs.mkdirSync(binaryPath, { recursive: true });

  const result = spawnSync(process.execPath, [fixture.wrapperPath], {
    encoding: "utf8",
    env: wrapperEnv(),
  });

  assert.equal(result.status, 127);
  assert.match(result.stderr, /not a file/i);
  assert.match(result.stderr, new RegExp(path.basename(binaryPath).replace(".", "\\.")));
});

test("the wrapper preserves the child exit status", (t) => {
  const fixture = createPackageFixture(t);
  const child = writeNodeFixture(fixture.root, "process.exit(23);\n");

  const result = spawnSync(process.execPath, [fixture.wrapperPath, child], {
    encoding: "utf8",
    env: wrapperEnv({ NOVEL_CRAFT_BINARY: process.execPath }),
  });

  assert.equal(result.status, 23, result.stderr);
});

test(
  "the wrapper preserves POSIX child termination signals",
  { skip: process.platform === "win32" },
  (t) => {
    const fixture = createPackageFixture(t);
    const child = writeNodeFixture(
      fixture.root,
      'process.kill(process.pid, "SIGTERM");\n',
    );

    const result = spawnSync(process.execPath, [fixture.wrapperPath, child], {
      encoding: "utf8",
      env: wrapperEnv({ NOVEL_CRAFT_BINARY: process.execPath }),
    });

    assert.equal(result.status, null);
    assert.equal(result.signal, "SIGTERM");
  },
);
