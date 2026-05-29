# Security Policy

## Supported Versions

Novel Craft is pre-1.0. Security fixes target the latest public release.

## Reporting A Vulnerability

Please do not open a public issue for a vulnerability.

Until a dedicated security advisory channel is configured, contact the maintainer directly and include:

- affected version or commit
- operating system
- command used
- impact
- minimal reproduction
- whether secrets, local files, or project data are exposed

## Security Defaults

Novel Craft v1 is local-first:

- no built-in LLM provider calls
- no API-key storage
- no telemetry
- no hidden network calls
- no scraping workflows
- no model training or embedding pipeline over copyrighted hosted fiction

Release workflows must use least-privilege GitHub tokens and npm trusted publishing/OIDC rather than long-lived npm tokens.
