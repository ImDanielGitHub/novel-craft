# Changelog

All notable changes to Novel Craft are recorded here.

## Unreleased

- Harden the npm launcher with an explicit `NOVEL_CRAFT_BINARY` override, candidate validation, platform-aware diagnostics, and child signal preservation.
- Add dependency-free Node tests for target mapping, argument forwarding, wrapper metadata, missing and unusable binaries, exit codes, and POSIX signals.
- Add an npm package-contract verifier that checks required launcher files, rejects development-only paths, and requires release binaries before publishing.
- Run launcher and package checks in cross-platform CI and before npm release publishing.

## 0.1.2 - 2026-05-30

- Add the guided setup flow for bundled Novel Craft skills, including opt-out and unattended installation modes.
- Add tower-climb, progression-fantasy, isekai-survival, and dungeon-core planning profiles.
- Strengthen chapter review for opening exposition, backstory density, invented-noun grounding, declared competence, requested word counts, avoid terms, memory noise, and action-ranked revision priorities.
- Prefer open-loop terminology while retaining the deprecated `plot add-promise` alias for compatibility.
- Honour JSON output files consistently across draft, next, review, and audit workflows.

## 0.1.1 - 2026-05-30

- Add `agent plan` for prompt-to-plan-to-finished-chapter agent workflows.
- Add `eval chapter <file>` with chapter-spine, scene-change, reader-retention, prose, voice, open-loop, progression, and dialogue review sections.
- Add gate-first agent workflows for creative briefs, tournament packets, draft gates, and revision comparisons.
- Add `eval gate` with constraint checks, lint summary, reader warnings, lexical novelty signals, and external judgement reminders.
- Update `eval compare` so it reports evidence without choosing a fake winner.
- Rename bundled skills to `novel-craft-*` public names and keep old names as deprecated alias stubs.
- Add a bundled `novel-craft-writing-support` skill for plain naming, natural wording, docs, and release notes around novel projects.
- Add `creative atlas` with 50 genres, 50 subgenres, 50 tropes, and 50 sub-tropes for broader agent premise mixing.
- Add always-on novel excellence checks for first-chapter pull, costly advantages, chapter structure, wider story engine, and serial retention.
- Add opening micro-scene checks so first chapters show a small dramatic unit before announcing macro-scale systems, kingdoms, domains, or upgrade ladders.
- Add `eval story <file>` for post-writing review of existing Markdown/text drafts without pass/fail gate language.
- Add serial-grip planning profiles for costly power, world depth, serial retention, beast bonds, rational magus progression, VR cultivation, monster evolution, and romance hooks.
- Tighten README and CLI docs around agent usage, packaged contents, and package scope.

## 0.1.0 - 2026-05-29

- Start Rust-first public CLI.
- Add `novel-craft` and `novel` binaries.
- Add deep guided `start` command with non-interactive defaults.
- Add bundled rule, creative, eval, matrix, memory, export, and skills commands.
- Add npm package metadata for `npx novel-craft` usage.
- Add open-source governance, security, and release scaffolding.
