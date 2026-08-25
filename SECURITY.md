# Security Policy

## Supported Versions

Novel Craft is pre-1.0. Security fixes target the latest public release.

## Reporting A Vulnerability

Do not open a public issue for a suspected vulnerability.

Use GitHub's private vulnerability reporting route:

https://github.com/ImDanielGitHub/novel-craft/security/advisories/new

Include:

- affected version or commit;
- operating system;
- command used;
- impact;
- a minimal reproduction;
- whether secrets, local files, or project data are exposed.

If the private-report form is unavailable, do not publish exploit details. Contact the maintainer through the GitHub profile linked from the repository and ask for a private reporting channel first.

The maintainer will acknowledge a usable report as soon as practical, coordinate disclosure while a fix is prepared, and publish remediation information once users can update safely. There is no guaranteed response SLA for this pre-1.0 volunteer project.

## Security Defaults

Novel Craft is local-first:

- no built-in provider integrations;
- no API-key storage;
- no telemetry;
- no hidden network calls;
- no scraping workflows;
- no training or embedding pipeline over copyrighted hosted fiction.

Release workflows use least-privilege GitHub tokens and npm trusted publishing/OIDC rather than long-lived npm tokens. GitHub Release binaries are checksummed and attested. See [docs/security/release-integrity.md](docs/security/release-integrity.md).
