---
name: novel-craft
description: Create original webnovels from scratch, continue a serial, or revise an existing manuscript with source-grounded context, inspectable proposals and recoverable local state. Use when a user wants fiction writing, continuation, editing, continuity review or book export.
---

# Novel Craft

The writer owns intention and taste. You provide reading, invention and judgement. The CLI provides trustworthy state and transactions, not objective literary scores.

## Discover before guessing

Run `novel-craft schema --json` and `novel-craft status --json`. Use `novel-craft genres <category> --examples --json` only when examples help; do not transplant their objects or plots. Unknown categories are errors, not fantasy defaults. `novel-craft guide --json` explains the craft principles and source policy.

## Starting from scratch

With an existing authenticated Codex CLI: `novel-craft create ./book --idea "<request>" --genre <category> --chapters 3 --runner codex --json` runs actual planning, prose generation, review and bounded revision. Another writing agent can implement the documented JSON stdin/stdout runner protocol. It produces a proposal, not accepted canon. Do not launch an inner agent when you can do the writing directly in this session.

When you are the writing agent: run `init`, obtain `context`, write original prose into a temporary Markdown file, and `import --file <draft> --chapter 1`. This route needs no additional model or keys. A vague prompt permits modest inference, not compulsory questionnaires or twelve competing premises.

## Continuing or revising

Load `context --chapter <n> --json` before writing. Respect the context's coverage and warnings. Request `review --chapter <n> --packet --json` for the full current target and review schema. Use earlier passages, not only summaries, when a claim matters. Preserve world facts, character beliefs, reader knowledge and plans as separate concepts.

Write at the requested scale. A local edit must not restart the novel. Show and tell deliberately; quiet chapters and resolved endings are valid. Review findings need real excerpts, an intentional alternative reading and a bounded suggestion. A preference is not a defect. The writer can store stable choices using `preferences add --instruction ... --expect <revision>`.

## Approval and continuity

Inspect `diff <proposal-id> --json`. Commit only within explicit or delegated writer authority with `commit <proposal-id> --expect <base-revision>`. Extracted facts remain candidates unless `--accept-facts` was explicitly authorised or individual records are accepted using `canon accept <fact-id> --expect <revision>`.

Use `status`, `canon` and `audit` to read back what changed. A successful command proves storage or checks, not literary quality. For manual manuscript edits, `import --adopt --file manuscript/chapter-0001.md --chapter 1` records a proposal without overwriting them. Source conflicts require a new proposal, never a force overwrite.

## Recovery and delivery

`runs` lists generation checkpoints. Resume with `generate --resume <run-id>` and the same explicit runner configuration; model-call budgets remain enforced. `recover --yes` completes a previously authorised interrupted transaction. `unlock --yes` only releases a lock whose process has stopped. Inspect history before `undo --expect <revision> --yes`.

Export committed text with `export --format md|html|epub --out <new-file>`. Existing files are not overwritten. Read the output as a reader before calling it finished. No scraping, hidden publishing, protected-chapter copying or model credential storage is part of this workflow.
