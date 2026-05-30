# Changelog

All notable changes to Novel Craft are recorded here.

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
