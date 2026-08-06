# npm Launcher Hardening Design

## Context

Novel Craft is a Rust CLI distributed through npm as a JavaScript launcher plus platform binaries. The writing engine has broad integration coverage, but the launcher is only smoke-tested with `--version`. The current launcher checks whether candidate paths exist, then runs the first match. It does not distinguish a missing file from a directory or non-executable file, cannot use an explicit binary for hermetic tests or unusual installations, reports little platform detail, and converts signal termination into exit code 1.

## Goal

Make the npm boundary deterministic, diagnosable, testable, and safe without adding dependencies or changing the Rust CLI contract.

## Chosen Approach

Move launcher logic into `npm/lib/launcher.js` and keep `npm/bin/novel-craft.js` as a thin executable entry point. The library will expose small pure helpers for platform target resolution, candidate construction, packaged binary discovery, and diagnostics. The runtime path will accept injected process, filesystem, path, and spawn dependencies for tests, while the public executable will use Node built-ins.

The launcher will:

- support `NOVEL_CRAFT_BINARY` as the highest-priority explicit binary path
- map supported operating system and architecture pairs to Rust target triples
- inspect files, not merely path existence
- require executable permission on non-Windows systems
- list searched paths and packaged binaries in failures
- include detected operating system, architecture, and expected target in failures
- preserve child exit codes
- re-raise child termination signals where the host supports that signal
- keep `NOVEL_CRAFT_NPM_WRAPPER` and `NOVEL_CRAFT_NPM_WRAPPER_PATH` environment metadata

## Package Contract

Add a dependency-free package verifier under `npm/scripts/verify-package.js`. It will parse `npm pack --dry-run --json` output and fail when required launcher files are absent or development-only paths leak into the tarball. The package must contain:

- `npm/bin/novel-craft.js`
- `npm/lib/launcher.js`
- `README.md`
- `LICENSE`
- at least one platform binary when release verification runs

The package must not contain source trees, tests, GitHub workflow files, local caches, or generated tarballs.

## Testing

Use Node's built-in `node:test` module so the project gains no new runtime or development dependency. Black-box launcher tests will use temporary executable fixtures and child processes. Pure helper tests will inject platform and architecture values. Platform-specific permission and signal tests will be skipped on Windows.

CI will run launcher tests on Linux, macOS, and Windows after Node setup. Release publishing will run launcher tests, wrapper smoke checks, and package-contract verification before npm publish.

## Error Handling

Missing or unusable binaries exit with code 127 and print actionable diagnostics. A child process exit code passes through unchanged. A child process signal is re-raised on the wrapper process when possible; if the host rejects the signal, the wrapper exits with code 1 and reports the signal.

## Compatibility

- Node.js remains `>=18`.
- No npm dependencies are added.
- Existing binary naming remains `novel-craft-<rust-target-triple>[.exe]`.
- Existing source checkout fallbacks remain `target/release/novel-craft` then `target/debug/novel-craft`.
- Existing `novel-craft` and `novel` bin aliases remain unchanged.
