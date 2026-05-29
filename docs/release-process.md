# Release Process

Novel Craft releases are maintainer-only.

## Before Release

Confirm the tree is Rust-only and clean:

```bash
git ls-files
git grep -n -I -E 'sk-|ghp_|github_pat_|npm_|AKIA|BEGIN .*PRIVATE KEY|password|api[_-]?key|token|secret' HEAD -- .
find . -type f -size +5M -not -path './.git/*' -print
```

Run:

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
cargo build --release
target/release/novel-craft doctor --json
```

Run security checks when installed:

```bash
cargo audit
cargo deny check
zizmor .github/workflows
```

Check the currently published npm version before tagging:

```bash
npm view novel-craft version
```

Test the local npm package without publishing:

```bash
HOST_TRIPLE="$(rustc -vV | awk '/host:/ {print $2}')"
cp target/release/novel-craft "npm/bin/novel-craft-${HOST_TRIPLE}"
chmod +x "npm/bin/novel-craft-${HOST_TRIPLE}"
npm pack --dry-run
npm pack
npm install -g ./novel-craft-0.1.1.tgz
novel-craft --help
novel-craft doctor --json
novel-craft start --no-input --defaults --json
novel --version
npm uninstall -g novel-craft
npx --yes ./novel-craft-0.1.1.tgz --version
rm -f novel-craft-*.tgz npm/bin/novel-craft-*
```

See [npm-install.md](npm-install.md) for PATH troubleshooting and alternate `npm exec` syntax.

## Tag

Create a signed release tag from `main`:

```bash
git tag -s v0.1.1 -m "Novel Craft v0.1.1"
git push origin v0.1.1
```

## GitHub Actions

The release workflow should:

- build macOS, Linux, and Windows binaries
- copy binaries into `npm/bin/`
- run the npm wrapper smoke test
- upload checksums and artifacts to GitHub Releases
- publish to npm through trusted publishing with provenance

The `npm` GitHub environment must require manual maintainer approval.

See [npm-publish.md](npm-publish.md) for trusted publishing setup, provenance, and rollback/deprecation notes.

## After Release

- Confirm `npx novel-craft --version`.
- Confirm `npx novel-craft doctor --json`.
- Confirm `npx novel-craft start --no-input --defaults --json`.
- Update `CHANGELOG.md` or release notes.
- Open a tracking issue for any release follow-ups.
