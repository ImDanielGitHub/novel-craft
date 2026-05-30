# npm Install And Local npx Testing

Novel Craft ships as a Rust binary wrapped by a small npm package. The npm package does not contain a JavaScript implementation; it only exposes the `novel-craft` and `novel` commands and launches the platform binary.

## Requirements

- Node.js with npm and npx on `PATH`
- Rust toolchain for source builds
- a built release binary for your platform

Check local tools:

```bash
node --version
npm --version
npx --version
cargo --version
```

## Source Checkout Smoke Test

From the repository root:

```bash
cargo build --release
target/release/novel-craft --help
target/release/novel-craft setup --no-skills --json
target/release/novel-craft doctor --json
target/release/novel-craft start --no-input --defaults --json
target/release/novel --version
```

## Prepare A Local npm Tarball

The npm wrapper looks for a binary named with the Rust target triple:

```bash
HOST_TRIPLE="$(rustc -vV | awk '/host:/ {print $2}')"
mkdir -p npm/bin
cp target/release/novel-craft "npm/bin/novel-craft-${HOST_TRIPLE}"
chmod +x "npm/bin/novel-craft-${HOST_TRIPLE}"
```

On Windows, use the `.exe` binary name:

```powershell
$HOST_TRIPLE = rustc -vV | Select-String "host:" | ForEach-Object { $_.ToString().Split(" ")[1] }
Copy-Item target\release\novel-craft.exe "npm\bin\novel-craft-$HOST_TRIPLE.exe"
```

Inspect the package before installing it:

```bash
npm pack --dry-run
npm pack
```

The dry run should include only:

- `npm/bin/novel-craft.js`
- the platform binary or release binaries prepared for packaging
- `README.md`
- `LICENSE`
- README-owned assets and provenance notes
- npm metadata

It should not include `.novel/`, `target/`, `node_modules/`, local tarballs, credentials, or generated caches.

## Install Globally From The Local Tarball

```bash
npm install -g ./novel-craft-0.1.2.tgz
novel-craft --help
novel-craft setup --no-skills --json
novel-craft setup --yes --target /tmp/novel-craft-skills-smoke --dry-run --json
novel-craft doctor --json
novel-craft start --no-input --defaults --json
novel --version
npm uninstall -g novel-craft
```

## Test npx From The Local Tarball

```bash
npx --yes ./novel-craft-0.1.2.tgz --version
```

If your npm version does not accept that local tarball syntax, use:

```bash
npm exec --yes --package ./novel-craft-0.1.2.tgz -- novel-craft --version
```

## Add npm To PATH

Find npm's global binary directory:

```bash
npm prefix -g
```

On macOS and Linux, the command directory is usually `$(npm prefix -g)/bin`. If `novel-craft` is installed but your shell cannot find it, add that directory to your shell PATH. For zsh:

```bash
echo 'export PATH="$(npm prefix -g)/bin:$PATH"' >> ~/.zshrc
exec zsh -l
```

For a local Node install under `~/.local/share/node`, add:

```bash
export PATH="$HOME/.local/share/node/bin:$PATH"
```

## Uninstall

```bash
npm uninstall -g novel-craft
```

Remove local packaging artifacts when finished:

```bash
rm -f novel-craft-*.tgz npm/bin/novel-craft-*
```
