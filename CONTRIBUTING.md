# Contributing

Thanks for helping improve Novel Craft.

## Ground Rules

- Public PRs are welcome.
- Maintainers decide what merges.
- All source, workflow, release, security, and skill changes require maintainer review.
- Do not add model API calls, telemetry, scraping, or hidden network behavior without an accepted design issue first.
- Do not include copyrighted novel text unless you own it or it is clearly licensed for this use.

## Local Checks

Run:

```bash
cargo fmt
cargo check
cargo clippy -- -D warnings
cargo test
```

## PR Checklist

- Explain the reader/writer problem being solved.
- Add or update tests for command output, JSON shape, or fixture behavior.
- Keep outputs model-neutral.
- Update docs for new public commands.
- Avoid broad rewrites unless the issue was accepted first.

## Security

Do not report vulnerabilities in public issues. See [SECURITY.md](SECURITY.md).
