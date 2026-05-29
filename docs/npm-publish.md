# npm Publish Process

Novel Craft is published to npm as `novel-craft`. Do not publish from a developer machine for v1. The intended release path is GitHub Actions trusted publishing with provenance.

## Publish Boundary

Do not publish until all are true:

- the public tree contains only the Rust CLI, docs, rules, skills, fixtures, npm wrapper, and release metadata
- unsupported implementation files are absent from the public root commit
- leak checks pass
- CI passes on the private GitHub repository
- npm package contents have been reviewed with `npm pack --dry-run`
- maintainer explicitly approves the release

## npm Account Setup

1. Own or create the npm account that will publish `novel-craft`.
2. Enable two-factor authentication on the npm account.
3. Check package availability immediately before first publish:

```bash
npm view novel-craft
```

If npm returns `404 Not Found`, the unscoped package name is available at that moment. Package names can change, so check again immediately before release.

## Trusted Publishing

Use npm trusted publishing instead of long-lived npm tokens.

In npm:

1. Open the `novel-craft` package publishing settings after the first package setup flow is available.
2. Add the GitHub repository as a trusted publisher.
3. Point it at the release workflow that runs `npm publish --provenance --access public`.

In GitHub:

1. Create an `npm` environment.
2. Require maintainer approval for deployments to that environment.
3. Restrict publishing to protected release tags.
4. Keep workflow permissions minimal.

The unscoped package is public by default, but the workflow still uses `--access public` for clarity.

## Release Flow

From a clean `main` branch:

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
cargo build --release
npm pack --dry-run
```

Create and push a signed tag:

```bash
git tag -s v0.1.0 -m "Novel Craft v0.1.0"
git push origin v0.1.0
```

The release workflow should:

- build release binaries for Linux, macOS, and Windows
- copy binaries into `npm/bin/`
- smoke-test `node npm/bin/novel-craft.js --version`
- run `npm pack --dry-run`
- wait for the protected `npm` environment approval
- publish with provenance:

```bash
npm publish --provenance --access public
```

## Package Verification

After release:

```bash
npm view novel-craft version
npx --yes novel-craft --version
npx --yes novel-craft start --no-input --defaults --json
```

Also inspect the GitHub Release artifacts, checksums, and attestation/provenance output.

## Rollback Or Deprecation

npm packages cannot be treated like a private branch after publish. If a bad release escapes:

1. stop further publish jobs
2. create a fixed patch release if possible
3. deprecate the bad version with a clear message:

```bash
npm deprecate novel-craft@0.1.0 "Use 0.1.1; 0.1.0 has a packaging issue."
```

Only unpublish if the package is inside npm's narrow unpublish policy window and the release creates a serious security or accidental-publication issue.

## What Never Goes In npm

- `.novel/` project state
- `target/`
- `node_modules/`
- local tarballs
- local binaries not meant for release
- credentials, tokens, SSH keys, private notes, or API keys
- unsupported implementation files
- hidden network-call or model-call code
