# GitHub Repository Settings

Configure these settings after creating `github.com/ImDanielGitHub/novel-craft` as a private repository. Keep it private until the clean root commit, CI, npm package dry run, and leak checks have passed.

## General

- Default branch: `main`
- Visibility: private until release readiness review passes, then public only by explicit maintainer action
- Issues: enabled
- Discussions: optional
- Wiki: disabled unless maintainers want it
- Projects: optional

## Branch Protection For `main`

Enable:

- require a pull request before merging
- require at least one approval
- require CODEOWNER review
- dismiss stale approvals after new commits
- require status checks to pass
- require branches to be up to date before merging
- require linear history
- disallow force pushes
- disallow branch deletion
- restrict who can push directly

Required status checks:

- Rust / ubuntu-latest
- Rust / macos-latest
- Rust / windows-latest

## Security

Enable:

- Dependabot alerts
- Dependabot security updates
- Secret scanning
- Push protection
- Code scanning alerts

## Environments

Create an `npm` environment:

- required reviewers: maintainer
- deployment branches/tags: protected tags only
- no long-lived npm token secrets required when trusted publishing is configured

## npm Trusted Publishing

Configure npm trusted publishing for this repository and the `Release` workflow. The release workflow uses OIDC and `npm publish --provenance --access public`.
