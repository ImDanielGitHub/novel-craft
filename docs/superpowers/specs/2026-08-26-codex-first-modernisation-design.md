# Codex-first Novel Craft Modernisation Design

## Context

Novel Craft is a local-first Rust CLI and npm-distributed native binary for agent-assisted long-form fiction. The existing product already has a broad CLI, structured `.novel/` state, deterministic and heuristic craft checks, agent skills, evaluation fixtures, cross-platform CI, npm trusted publishing, and security automation. The audit on 26 August 2026 identified two classes of work: 100 research-backed product/quality improvements and 100 bug fixes, hardening tasks, and functionality build-outs.

This programme implements that audit as a coherent Codex-first release rather than 200 unrelated patches.

## Goal

Make Novel Craft a trustworthy Codex-native writing operator: deterministic where possible, explicit about judgement, safe with local manuscript state, easy for agents to call through stable machine contracts, strongly tested across platforms, and maintainable as an open-source Rust project.

## Codex-first principles

1. Machine-readable output is a first-class API. JSON output must be versioned, stable, and accompanied by machine error identifiers.
2. Commands should compose. Agents should be able to inspect state, plan, execute a bounded operation, verify it, and continue without scraping prose.
3. Mutations are previewable. State-changing commands should expose dry-run or reviewable-diff behaviour where practical.
4. Context is deliberate. Packet builders should surface why material was included and avoid dumping whole manuscripts into agent context.
5. Human judgement remains authoritative for high-risk voice, plot, canon, taste, and memory changes.
6. Local-first remains the default. No telemetry, provider calls, scraping, or hidden network operations are introduced.
7. The core remains provider-neutral. Optional semantic adapters must live behind explicit feature or process boundaries.
8. Research-backed recommendations remain hypotheses and review aids, not objective literary scores.

## Programme decomposition

### Programme A: supply-chain security and release integrity

Repair current RustSec and OpenSSF findings, pin workflow dependencies, reduce token permissions, run dependency and workflow security checks on pull requests, add machine-verifiable release metadata, and establish an actionable security reporting route. The release pipeline must continue to use npm trusted publishing and OIDC rather than long-lived npm tokens.

### Programme B: npm and cross-platform distribution reliability

Complete the npm launcher hardening work, canonicalise filesystem paths where platforms expose aliases, validate package contents, install and execute packed tarballs in CI, preserve exit/signal semantics, and expand supported build targets without changing existing CLI aliases.

### Programme C: Rust architecture and command contracts

Reduce the single-file architecture by extracting self-contained modules only behind behaviour-preserving tests. The initial extraction targets shared schemas, machine-response envelopes, project paths, filesystem safety, and evaluation/pack metadata because those boundaries can be introduced without rewriting the mature CLI surface. Large command handlers move incrementally after contracts exist.

### Programme D: state, schema, migration, and filesystem integrity

Introduce explicit schema versions, strict validation, Unicode-safe identifiers, root-containment checks, atomic writes, snapshots, state integrity checks, migration fixtures, and transactional/traceable memory changes. Existing `.novel/` projects remain readable.

### Programme E: text-processing and evaluation science

Expand the evaluation fixture corpus, encode expected findings and intentional non-findings, add benchmark reporting, improve Unicode/Markdown boundaries, build false-positive regressions, and add property/fuzz-ready pure interfaces. Novel Craft should measure detector behaviour rather than only asserting that commands execute.

### Programme F: narrative intelligence and external cognition

Represent causality, intentions, state changes, provenance, knowledge state, relationship state, payoff prerequisites, and agency as inspectable data. Build exports that Codex can consume and humans can inspect. Avoid a GUI dependency; structured JSON/Markdown/DOT output comes before an interactive TUI.

### Programme G: Codex-native agent interface and workflow ergonomics

Add stable response envelopes, `status`, `guide`, `--explain`, safer dry-run surfaces, context-composition explanations, shell completion support, and a provider-neutral stdio protocol for structured invocation. The protocol must call existing deterministic command handlers rather than duplicating writing logic.

### Programme H: open-source repository hygiene

Make the repository easier to trust and contribute to: fix failing workflows, align docs and real behaviour, publish a roadmap and benchmark process, add good-first-issue scaffolding, improve SECURITY/CONTRIBUTING guidance, add repository metadata/topics where supported, and remove stale/deprecated material only when compatibility allows.

## Data contracts

### Machine response envelope

New structured commands should use an envelope shaped like:

```json
{
  "schema_version": "1",
  "command": "status",
  "ok": true,
  "data": {},
  "warnings": [],
  "errors": []
}
```

Errors use stable objects:

```json
{
  "code": "NC_PROJECT_NOT_FOUND",
  "message": "No .novel project was found",
  "detail": null
}
```

Existing JSON response shapes remain compatible in this release unless a command explicitly opts into the envelope. A future major version may standardise all output after migration tooling and fixture coverage exist.

### Project schema

`.novel/project.yml` gains `schema_version`. Missing versions are interpreted as the legacy v0 schema and can be inspected/migrated. Migration never silently deletes unknown fields.

### Identifier safety

IDs used as file names are normalised, reject absolute paths and traversal segments, and must resolve inside the intended project subdirectory after canonicalisation where the target exists.

### State mutation

File mutations use write-to-temporary-file plus atomic rename where the platform permits. Higher-risk commands create reviewable snapshots or diffs. SQLite and related file-state updates must either commit coherently or leave recoverable evidence.

## Evaluation contract

Each evaluation fixture can declare:

- expected rule IDs
- forbidden rule IDs
- profile/mode
- intentional exception notes
- expected machine status where relevant

A benchmark command aggregates detector precision-oriented regression data from the fixture manifest. This is not a literary quality score.

## Research-backed product changes

The 100 research recommendations are implemented by prioritising mechanisms rather than slogans:

- planning/revision separation becomes explicit workflow state and packets
- feedback becomes ranked goal/evidence/action output
- creative divergence becomes reproducible candidate generation and genealogy
- external cognition becomes structured graphs, provenance, timelines, and context-composition inspection
- human-AI collaboration becomes initiative modes, reviewable diffs, locked constraints, and accept/reject evidence
- long-context quality becomes salience-aware packet composition rather than raw manuscript dumping

Items whose complete user experience would require a separate GUI or provider integration are implemented as provider-neutral data contracts and exports first. This preserves the local CLI thesis and gives Codex a better interface immediately.

## Testing boundary

A change is not complete until its relevant proof boundary passes:

- Node launcher/package tests locally where possible
- Rust formatting, check, Clippy, unit/integration tests in GitHub Actions
- Linux, macOS, and Windows CI
- package contract and packed-tarball execution
- security checks on the pull request
- fixture benchmark/regression checks
- release workflow syntax/security scan

The current shell has no Rust toolchain and cannot reach GitHub directly. Therefore GitHub Actions is the authoritative Rust/cross-platform execution boundary for this programme. Local Node/Python validation is supplemental only.

## Compatibility

- Preserve `novel-craft` and `novel` command aliases.
- Preserve local-first/provider-neutral behaviour.
- Preserve existing `.novel/` projects through legacy reads and migration tooling.
- Do not silently auto-accept canon, memory, voice, plot, or taste changes.
- Do not add telemetry.
- Do not copy or train on copyrighted hosted fiction.
- Keep npm Node support at `>=18` unless CI evidence requires a deliberate major-version change.

## Merge strategy

Land the programme in independently green pull requests, then merge them into `main` in dependency order. PR #24 is treated as Programme B's first change and is fixed/merged before later npm work. The modernisation branch is used for specifications and temporary CI materialisation only; temporary export workflows are removed before any merge.

## Completion criteria

The programme is complete when:

1. all programme PRs are merged to `main`;
2. `main` CI is green on Linux, macOS, and Windows;
3. current scheduled security and Scorecard failures caused by repository configuration are resolved or explicitly documented when caused by external integration limitations;
4. current known RustSec advisories are removed from the locked dependency graph;
5. npm package tests validate actual tarball execution;
6. project schemas and machine errors are versioned;
7. fixture and benchmark coverage materially exceeds the current corpus and includes false-positive/intentional-exception cases;
8. Codex has structured status, explanation, context, and invocation surfaces;
9. repository documentation describes what is actually implemented and how contributors verify changes;
10. temporary implementation-only workflows/files are removed.