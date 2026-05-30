# npm Publish Process

Novel Craft is published to npm as `novel-craft`. The intended release path is GitHub Actions trusted publishing with provenance. Local developer publishes are only an emergency fallback.

## Publish Boundary

Do not publish until all are true:

- the public tree contains only the Rust CLI, docs, rules, skills, fixtures, npm wrapper, and release metadata
- unsupported implementation files are absent from the public root commit
- leak checks pass
- CI passes on the protected public repository
- npm package contents have been reviewed with `npm pack --dry-run`
- maintainer explicitly approves the release

## npm Account Setup

1. Own or create the npm account that will publish `novel-craft`.
2. Enable two-factor authentication on the npm account.
3. Confirm the currently published package version before tagging:

```bash
npm view novel-craft version
```

## Trusted Publishing

Use npm trusted publishing instead of long-lived npm tokens.

Configure it with the current npm CLI. Use `npx npm@latest` so the command supports the current `--allow-publish` permission flag:

```bash
npx --yes npm@latest trust github novel-craft \
  --repo ImDanielGitHub/novel-craft \
  --file release.yml \
  --env npm \
  --allow-publish \
  -y

npx --yes npm@latest trust list novel-craft
```

Expected trust record:

- type: `github`
- repository: `ImDanielGitHub/novel-craft`
- file: `release.yml`
- environment: `npm`
- permissions: `createPackage` in `npm trust list` output, created with the `--allow-publish` flag

If `npm publish` fails with `E404 Not Found` for an existing package, treat it as a trusted-publisher permission problem first. Re-run the trust command above, approve the npm passkey prompt, then re-run the failed GitHub Actions job.

In npm's web UI, the same setting is:

1. Open the `novel-craft` package publishing settings.
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

Create and push an annotated release tag. Use a signed tag only after GPG signing is configured locally:

```bash
git tag -a v0.1.2 -m "Novel Craft v0.1.2"
git push origin v0.1.2
```

The release workflow should:

- build release binaries for Linux, macOS, and Windows
- copy binaries into `npm/bin/`
- smoke-test `node npm/bin/novel-craft.js --version`
- run `npm pack --dry-run`
- wait for the protected `npm` environment approval
- publish with provenance:

```bash
npx --yes npm@latest publish --provenance --access public
```

## Package Verification

After release:

```bash
npm view novel-craft version
npx --yes novel-craft --version
npx --yes novel-craft doctor --json
```

Also inspect the GitHub Release artifacts, checksums, and attestation/provenance output.

## Rollback Or Deprecation

npm packages cannot be treated like a private branch after publish. If a bad release escapes:

1. stop further publish jobs
2. create a fixed patch release if possible
3. deprecate the bad version with a clear message:

```bash
npm deprecate novel-craft@<bad-version> "Use <fixed-version>; this release has a confirmed issue."
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
- hidden network-call code
