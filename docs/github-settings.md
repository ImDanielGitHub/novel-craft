# GitHub Repository Settings

Use these settings for the public `github.com/ImDanielGitHub/novel-craft` repository.

## General

- Default branch: `main`
- Visibility: public
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
