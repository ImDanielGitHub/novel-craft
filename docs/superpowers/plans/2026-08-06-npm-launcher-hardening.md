# npm Launcher Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Harden the npm launcher with deterministic binary resolution, actionable diagnostics, hermetic tests, and package-contract verification.

**Architecture:** Extract launcher logic into a dependency-free CommonJS module and retain a minimal executable entry point. Test observable launcher behaviour with Node's built-in test runner, then enforce the package contract in CI and release workflows.

**Tech Stack:** Node.js 18+, CommonJS, `node:test`, npm pack JSON output, GitHub Actions.

## Global Constraints

- Do not add runtime or development dependencies.
- Preserve the Rust CLI arguments, stdout, stderr, exit codes, and wrapper environment metadata.
- Preserve `novel-craft` and `novel` npm aliases.
- Keep existing Rust target binary names and source checkout fallbacks.
- Use exit code 127 for launcher resolution or execution failures.
- Keep all diagnostics on stderr.

---

### Task 1: Specify launcher behaviour with failing tests

**Files:**
- Create: `npm/test/launcher.test.js`
- Test: `npm/bin/novel-craft.js`

**Interfaces:**
- Consumes: executable wrapper path `npm/bin/novel-craft.js`
- Produces: black-box behaviour requirements for binary override, diagnostics, exit propagation, and signals

- [x] **Step 1: Write tests for explicit binary override, missing-binary diagnostics, non-executable rejection, child exit status, wrapper metadata, and POSIX signal propagation.**
- [x] **Step 2: Run `node --test npm/test/launcher.test.js`.**
- [x] **Step 3: Confirm the new override, diagnostic, permission, and signal tests fail against the current launcher for the expected reasons.**

### Task 2: Extract and harden launcher logic

**Files:**
- Create: `npm/lib/launcher.js`
- Modify: `npm/bin/novel-craft.js`
- Test: `npm/test/launcher.test.js`

**Interfaces:**
- Consumes: `process.argv`, `process.env`, `process.platform`, `process.arch`, `__dirname`
- Produces: `runLauncher(options): number | null`, `platformTriple(platform, arch): string | null`, `buildCandidates(options): string[]`, and `resolveBinary(options): { binary: string | null, inspections: object[], packagedBinaries: string[], triple: string | null }`

- [x] **Step 1: Implement the minimum launcher module needed to satisfy the failing tests.**
- [x] **Step 2: Replace the bin script with a thin call into `runLauncher`.**
- [x] **Step 3: Run `node --test npm/test/launcher.test.js` and confirm all launcher tests pass.**
- [x] **Step 4: Refactor duplicated diagnostics and candidate inspection while keeping the suite green.**

### Task 3: Add package-contract verification

**Files:**
- Create: `npm/scripts/verify-package.js`
- Create: `npm/test/package-contract.test.js`
- Modify: `package.json`

**Interfaces:**
- Consumes: npm pack JSON file entries shaped as `{ path: string }`
- Produces: `validatePackedFiles(files, options): { binaries: string[], files: string[] }` and CLI verification through `npm run verify:package`

- [x] **Step 1: Write failing tests for required launcher files, excluded development paths, and release binary presence.**
- [x] **Step 2: Run `node --test npm/test/package-contract.test.js` and confirm failure because the verifier is absent.**
- [x] **Step 3: Implement `validatePackedFiles` and the `npm pack --dry-run --json` CLI path.**
- [x] **Step 4: Update package files and scripts to ship `npm/lib/launcher.js`, run wrapper tests, and verify package contents.**
- [x] **Step 5: Run both Node test files and confirm they pass.**

### Task 4: Enforce the contract in CI and releases

**Files:**
- Modify: `.github/workflows/ci.yml`
- Modify: `.github/workflows/release.yml`

**Interfaces:**
- Consumes: npm scripts `test:wrapper` and `verify:package:release`
- Produces: cross-platform launcher validation before build acceptance and npm publishing

- [x] **Step 1: Add wrapper tests to the CI matrix after Node setup.**
- [x] **Step 2: Add wrapper tests and package verification to the npm release job before publish.**
- [x] **Step 3: Validate both workflow files as YAML and inspect the resulting diff.**

### Task 5: Document the operational changes

**Files:**
- Modify: `docs/npm-install.md`
- Modify: `CHANGELOG.md`

**Interfaces:**
- Consumes: launcher environment variable and failure output contract
- Produces: contributor and user instructions for troubleshooting and local package verification

- [x] **Step 1: Document `NOVEL_CRAFT_BINARY`, searched-path diagnostics, executable permission recovery, wrapper tests, and package verification.**
- [x] **Step 2: Add an Unreleased changelog section describing the launcher and release safeguards.**
- [x] **Step 3: Run the full Node test suite, syntax checks, package verification with a fixture binary, and a final diff review.**
