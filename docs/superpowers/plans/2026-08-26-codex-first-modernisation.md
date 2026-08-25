# Codex-first Novel Craft Modernisation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the 26 August 2026 200-item audit as a smaller set of coherent, tested Codex-first subsystems and merge each green subsystem into `main`.

**Architecture:** Preserve the mature CLI surface while hardening its boundaries. Introduce machine contracts, schema/state safety, evaluation benchmarks, narrative graph exports, and agent-native commands as focused modules, then incrementally reduce the monolithic Rust file behind behavioural tests rather than performing a risky rewrite.

**Tech Stack:** Rust 2021, clap, serde/serde_json/serde_yaml, rusqlite, Node.js 18+, CommonJS launcher, GitHub Actions, npm trusted publishing, OpenSSF/RustSec tooling.

**Spec:** `docs/superpowers/specs/2026-08-26-codex-first-modernisation-design.md`

## Global Constraints

- Preserve `novel-craft` and `novel` aliases.
- Preserve local-first/provider-neutral behaviour.
- No telemetry, scraping, hidden network calls, or bundled model-provider SDKs.
- Existing `.novel/` projects remain readable.
- High-risk canon, memory, voice, plot, and taste changes remain reviewable rather than silently accepted.
- New machine contracts use stable schema/error identifiers.
- GitHub Actions is the authoritative Rust/cross-platform proof boundary in this environment.
- No pull request merges until all required checks for that pull request are green.

---

### Task 1: Complete npm launcher hardening

**Files:**
- Modify: `npm/test/launcher.test.js` on PR #24
- Existing: `npm/lib/launcher.js`
- Existing: `npm/scripts/verify-package.js`
- Existing: `.github/workflows/ci.yml`
- Existing: `.github/workflows/release.yml`

**Interfaces:**
- Consumes: Node filesystem/process semantics and platform binaries
- Produces: deterministic wrapper resolution, package contract validation, cross-platform tests

- [ ] Canonicalise the wrapper metadata path in the macOS regression test with `fs.realpathSync`.
- [ ] Push the fix to `agent/harden-npm-launcher`.
- [ ] Verify Linux, macOS, and Windows PR jobs are all green.
- [ ] Squash-merge PR #24 into `main`.

### Task 2: Repair supply-chain security

**Files:**
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `.github/workflows/ci.yml`
- Modify: `.github/workflows/security.yml`
- Modify: `.github/workflows/scorecard.yml`
- Modify: `.github/workflows/release.yml`
- Modify: `SECURITY.md`
- Create: `docs/security/release-integrity.md`

**Interfaces:**
- Consumes: RustSec advisory database, GitHub Actions, npm OIDC
- Produces: audit-clean locked dependencies, pinned workflow dependencies, least-privilege jobs, actionable reporting route

- [ ] Raise `anyhow` requirement to `>=1.0.103` and regenerate the lockfile.
- [ ] Regenerate the dependency graph so `crossbeam-epoch >=0.9.20` is selected.
- [ ] Run `cargo audit` and `cargo deny check` on pull requests.
- [ ] Pin every workflow action to a full commit SHA while retaining a version comment.
- [ ] Pin the zizmor install version.
- [ ] Make Scorecard top-level permissions read-only and move `security-events`/`id-token` writes to the minimum job/step-compatible scope.
- [ ] Reduce release permissions so build jobs are read-only and publishing/release jobs receive only required write scopes.
- [ ] Add a concrete private GitHub Security Advisory reporting link/instruction to `SECURITY.md`.
- [ ] Verify CodeQL/static analysis runs on pull requests and main.
- [ ] Verify security and Scorecard workflows no longer fail because of repository-controlled permission configuration.

### Task 3: Materialise a testable source workspace

**Files:**
- Create temporarily: `.github/workflows/codex-workspace-export.yml`
- Delete before merge: `.github/workflows/codex-workspace-export.yml`

**Interfaces:**
- Consumes: repository checkout in GitHub Actions
- Produces: source archive and regenerated lockfile workflow artefacts for this execution environment

- [ ] Add a pull-request-only workflow that checks out the branch, runs dependency regeneration, archives the source tree, and uploads both artefacts.
- [ ] Download the workflow artefact into the execution container.
- [ ] Use the materialised tree for local Node/Python/static editing.
- [ ] Delete the temporary workflow before the modernisation PR can merge.

### Task 4: Introduce machine contracts and project path safety

**Files:**
- Create: `src/contracts.rs`
- Create: `src/project_paths.rs`
- Create: `tests/contracts.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `MachineEnvelope<T>`, `MachineError`, stable error codes, `ProjectPaths`, safe ID-to-path helpers

- [ ] Add tests for schema version `1`, stable error serialisation, traversal rejection, absolute-path rejection, Unicode normalisation, and project-root containment.
- [ ] Implement `MachineEnvelope<T>` with `schema_version`, `command`, `ok`, `data`, `warnings`, and `errors`.
- [ ] Implement machine errors with stable `NC_*` codes.
- [ ] Implement `ProjectPaths` and one canonical identifier normalisation/safety path.
- [ ] Wire new commands to these contracts without changing legacy JSON responses yet.

### Task 5: Add project schema inspection and migration

**Files:**
- Create: `src/schema.rs`
- Create: `tests/schema.rs`
- Create: `tests/fixtures/projects/v0/`
- Create: `tests/fixtures/projects/v1/`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: legacy-v0 detection, v1 project schema, migration preview/apply functions

- [ ] Add legacy fixture where `schema_version` is absent.
- [ ] Add v1 fixture with `schema_version: "1"`.
- [ ] Add tests proving unknown fields survive migration.
- [ ] Add `novel-craft schema status --json`.
- [ ] Add `novel-craft schema migrate --dry-run --json` and apply mode.
- [ ] Ensure migration writes are atomic and snapshot the original project metadata.

### Task 6: Add atomic state writes, snapshots, and repair checks

**Files:**
- Create: `src/state_io.rs`
- Create: `src/integrity.rs`
- Create: `tests/state_integrity.rs`
- Modify: state-writing call sites in `src/lib.rs`

**Interfaces:**
- Produces: atomic YAML/JSON writes, snapshot metadata, `novel-craft repair --check|--fix-safe`

- [ ] Test interrupted-write recovery with a preserved original file.
- [ ] Test safe repair does not delete unknown state.
- [ ] Implement same-directory temp write plus fsync/rename where supported.
- [ ] Create snapshots before high-risk migration/memory operations.
- [ ] Add integrity checks for required project directories, parseable state files, duplicate IDs, invalid paths, and SQLite openability.

### Task 7: Harden SQLite memory semantics

**Files:**
- Create: `src/memory_store.rs`
- Create: `tests/memory_store.rs`
- Modify: memory command paths in `src/lib.rs`

**Interfaces:**
- Produces: foreign-key enforcement, busy timeout, idempotent commits, event history

- [ ] Test `PRAGMA foreign_keys = ON` is active.
- [ ] Test duplicate memory-diff application is idempotent.
- [ ] Test locked-database errors return a stable machine code.
- [ ] Add a bounded busy timeout.
- [ ] Add canonical memory event IDs and timestamps.
- [ ] Preserve reviewable pending-memory semantics.

### Task 8: Build evaluation fixture contracts and benchmark command

**Files:**
- Modify: `evals/fixtures/manifest.yml`
- Add: at least 40 additional fixture files under `evals/fixtures/`
- Create: `src/benchmark.rs`
- Create: `tests/benchmark.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: expected/forbidden rule-ID fixture contract and `novel-craft benchmark --json`

- [ ] Extend fixture manifest entries with `expected_rules` and `forbidden_rules`.
- [ ] Add paired false-positive fixtures for passive voice, filter words, emotion labels, dialogue exposition, opening exposition, repeated gestures, paragraph length, trope saturation, and continuity.
- [ ] Add intentional-rule-breaking fixtures mapped to documented `break_when` cases.
- [ ] Add Unicode, smart-quote, CRLF, Markdown frontmatter, fenced-code, nested-dialogue, and abbreviation fixtures.
- [ ] Add benchmark aggregation for fixture pass rate and per-rule regression counts.
- [ ] Explicitly label benchmark values as detector-regression metrics, not literary quality.

### Task 9: Improve text segmentation boundaries

**Files:**
- Create: `src/text.rs`
- Create: `tests/text.rs`
- Modify: detector call sites in `src/lib.rs`

**Interfaces:**
- Produces: normalised prose spans, sentence/paragraph helpers, dialogue spans, Markdown exclusion spans

- [ ] Test curly quotes and apostrophes.
- [ ] Test abbreviations do not create false sentence boundaries.
- [ ] Test YAML frontmatter and fenced code are excluded from prose analysis.
- [ ] Test LF and CRLF yield equivalent results.
- [ ] Centralise Unicode-aware word segmentation around `unicode-segmentation`.

### Task 10: Add narrative graph and provenance exports

**Files:**
- Create: `src/story_graph.rs`
- Create: `tests/story_graph.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: typed nodes/edges for scenes, causes, goals, knowledge, relationships, payoffs and provenance

- [ ] Add typed causal edges: causes, enables, prevents, motivates.
- [ ] Add character goal adoption/change records.
- [ ] Add fact provenance references to source scene/chapter/state records.
- [ ] Add payoff prerequisites and loop age metadata.
- [ ] Add agency trace showing which protagonist decisions changed later state.
- [ ] Add `novel-craft graph story --json` and `--format dot`.
- [ ] Add `novel-craft graph knowledge --json` and `graph relationships --json`.

### Task 11: Add Codex-native status, explain, and context inspection

**Files:**
- Create: `src/status.rs`
- Create: `src/explain.rs`
- Create: `tests/codex_workflow.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: structured `status`, rule explanation, context inclusion reasons

- [ ] Add `novel-craft status --json` returning schema version, project health, pending memory, stale loops, contradictions, schema state, and suggested next commands.
- [ ] Add `novel-craft explain rule <id> --json` returning evidence strength, detector limitation, break conditions, and examples.
- [ ] Add context packet inclusion metadata: item, source, reason, salience, recency.
- [ ] Add `context explain <target> --json` without mutating project state.
- [ ] Ensure all new errors use stable `NC_*` machine codes.

### Task 12: Add provider-neutral stdio agent protocol

**Files:**
- Create: `src/stdio_protocol.rs`
- Create: `tests/stdio_protocol.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Consumes: one JSON request per line on stdin
- Produces: one `MachineEnvelope` JSON response per line on stdout

- [ ] Define protocol request `{ "id": string, "command": string, "args": object }`.
- [ ] Add `novel-craft serve --stdio`.
- [ ] Support read-only operations first: `doctor`, `status`, `context-explain`, `rules-list`, `graph-story`.
- [ ] Return `NC_PROTOCOL_BAD_REQUEST` for malformed requests without terminating the server.
- [ ] Keep provider/network functionality out of the protocol.

### Task 13: Add shell completion and guided discovery

**Files:**
- Modify: `Cargo.toml` if clap completion support is required
- Create: `tests/completions.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `novel-craft completions <shell>` and `novel-craft guide --json|text`

- [ ] Generate Bash, Zsh, Fish, and PowerShell completions.
- [ ] Add a workflow-oriented `guide` that groups commands into start, plan, draft, review, state, and maintenance paths.
- [ ] Keep `guide --json` machine-readable for Codex discovery.

### Task 14: Add creative genealogy and reproducible tournament data

**Files:**
- Create: `src/creative_graph.rs`
- Create: `tests/creative_graph.rs`
- Modify: creative command paths in `src/lib.rs`

**Interfaces:**
- Produces: seeded candidate IDs, parent IDs, transform operators, separated novelty/utility evidence

- [ ] Add an optional deterministic seed to creative tournament packet generation.
- [ ] Record candidate genealogy and transformation operator.
- [ ] Separate novelty and utility/fit evidence instead of one aggregate creative score.
- [ ] Preserve existing behaviour when no seed/genealogy output is requested.

### Task 15: Add feedback/intentional-exception calibration

**Files:**
- Create: `src/feedback.rs`
- Create: `tests/feedback.rs`
- Modify: evaluation feedback paths in `src/lib.rs`

**Interfaces:**
- Produces: accept/reject/edit evidence and intentional-exception suppression records

- [ ] Add local records for accepted, rejected, and edited recommendations.
- [ ] Add rule-specific intentional exception records scoped by project/profile.
- [ ] Ensure suppressed findings remain inspectable with `--show-suppressed`.
- [ ] Use calibration to rank findings, never to silently rewrite text.

### Task 16: Add large-context synthetic fixture and performance guard

**Files:**
- Create: `tests/fixtures/long-context/`
- Create: `tests/long_context.rs`
- Create: `benches/` only if stable benchmark infrastructure is justified

**Interfaces:**
- Produces: generated 100-chapter fixture, deterministic aggregate expectations

- [ ] Generate synthetic chapters from repository-owned templates rather than copyrighted fiction.
- [ ] Encode known continuity, loop-age, repetition, and causality defects.
- [ ] Assert full-book and matrix audits find the expected defects.
- [ ] Record a generous runtime ceiling in CI to catch catastrophic regressions without introducing flaky microbenchmarks.

### Task 17: Add property/fuzz-ready test surfaces

**Files:**
- Create: `tests/properties.rs`
- Create: `fuzz/` only if `cargo-fuzz` integration is viable in CI

**Interfaces:**
- Consumes: schemas, identifiers, text parsers, state serialisers
- Produces: round-trip and malformed-input resilience checks

- [ ] Property-test identifier normalisation invariants.
- [ ] Property-test YAML/JSON parse-serialise-parse invariants for public schemas.
- [ ] Fuzz or bounded-random-test rule YAML, project YAML, JSONL preference records, and stdio protocol requests.
- [ ] Ensure malformed inputs produce errors rather than panics.

### Task 18: Clean open-source documentation and contribution surface

**Files:**
- Modify: `README.md`
- Modify: `CONTRIBUTING.md`
- Modify: `SECURITY.md`
- Modify: `GOVERNANCE.md`
- Modify: `references/implementation-roadmap.md`
- Create: `docs/architecture.md`
- Create: `docs/benchmarking.md`
- Create: `.github/ISSUE_TEMPLATE/good_first_issue.yml` if useful

**Interfaces:**
- Produces: contributor-visible architecture, verification commands, benchmark policy, security route

- [ ] Remove statements that no longer match the implementation.
- [ ] Document the machine protocol and schema compatibility policy.
- [ ] Document how research/evidence labels work.
- [ ] Document local and CI verification boundaries.
- [ ] Add clear issue labels/backlog guidance for contributions.
- [ ] Keep public docs concise and command-first.

### Task 19: Final repository and release verification

**Files:**
- Modify as failures require
- Delete: `.github/workflows/codex-workspace-export.yml`

**Interfaces:**
- Produces: clean merge candidates and green main

- [ ] Run formatting, check, Clippy, all Rust tests, Node tests, package-contract tests, security scans, benchmark fixtures, and release build on all supported operating systems.
- [ ] Run npm packed-tarball install/execute smoke tests on all supported operating systems.
- [ ] Verify no temporary workflow, generated archive, secret, build product, or fixture cache is committed.
- [ ] Verify docs refer only to implemented commands.
- [ ] Verify GitHub release workflow remains trusted-publishing compatible.

### Task 20: Merge programme PRs in dependency order

**Files:** none

**Interfaces:**
- Consumes: green pull requests
- Produces: verified `main`

- [ ] Merge npm hardening first.
- [ ] Merge security/release integrity second.
- [ ] Merge contracts/state foundations before commands that consume them.
- [ ] Merge evaluation/text changes before narrative/Codex interfaces that expose their data.
- [ ] Merge open-source hygiene last after all documented behaviour is real.
- [ ] Fetch final `main` workflow runs and confirm green status at the requested proof boundary.
