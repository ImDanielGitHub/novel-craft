# Release integrity

Novel Craft releases use short-lived GitHub Actions identity rather than long-lived package tokens.

## npm

The npm package is published through npm trusted publishing with GitHub OIDC. npm provenance is enabled for the public package.

## GitHub release binaries

Each release binary has:

- a SHA-256 checksum next to the binary;
- a GitHub artifact attestation signed with the workflow's short-lived OIDC identity;
- an offline `.jsonl` attestation bundle attached to the GitHub Release.

Verify a downloaded binary online:

```bash
gh attestation verify ./novel-craft-x86_64-unknown-linux-gnu \
  --repo ImDanielGitHub/novel-craft \
  --signer-workflow ImDanielGitHub/novel-craft/.github/workflows/release.yml
```

Verify the checksum separately:

```bash
sha256sum -c novel-craft-x86_64-unknown-linux-gnu.sha256
```

On macOS, use `shasum -a 256 -c <checksum-file>` if `sha256sum` is not installed.

For offline verification, use the `.jsonl` bundle shipped with the release plus a current Sigstore trusted-root file. GitHub documents the `gh attestation trusted-root` and `gh attestation verify --bundle` flow.

## Workflow dependencies

GitHub Actions are pinned to full commit SHAs. The adjacent version comments are documentation only. Dependabot remains responsible for proposing future action updates, which must pass CI and security scans before merge.
