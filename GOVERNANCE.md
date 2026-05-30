# Governance

Novel Craft is maintainer-led.

## Maintainers

The initial maintainer is the repository owner, `ImDanielGitHub`.

## Decision Model

- Maintainers set product direction, release timing, and merge policy.
- Public issues and PRs are welcome.
- Major changes should start as an issue or design discussion.
- Maintainers may reject changes that add provider lock-in, scraping risk, hidden network behavior, fragile dependencies, or unclear UX.

## Merge Policy

- `main` is protected.
- All PRs require CI.
- Source, release, workflow, security, and skill changes require CODEOWNER review.
- Maintainers are the only people allowed to publish releases.

## Release Policy

Releases are tagged from `main` after the release checklist passes. npm publishing uses trusted publishing with provenance.
