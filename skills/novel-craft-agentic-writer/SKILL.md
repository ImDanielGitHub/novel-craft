---
name: novel-craft-agentic-writer
description: Use Novel Craft as an agentic chapter-writing guide: plan from a user prompt, compare contenders, draft finished Markdown chapters, review with the CLI, revise, and finalise.
---

# Novel Craft Agentic Writer

Use this skill when the user wants a new story, first chapter, next chapter, or several chapters from a prompt.

## Core Loop

```bash
novel-craft agent plan --idea "<user prompt>" --chapters 1 --genre <genre-or-profile> --profile fast-webnovel --json
```

1. Restate the task facts.
2. Ask missing story questions if the human is in an IDE or plan conversation.
3. Generate and compare several directions before drafting.
4. Choose the best direction with evidence.
5. Draft finished prose in Markdown.
6. Run `novel-craft eval chapter <draft.md> --genre <genre-or-profile> --profile <profile> --json`.
7. Revise for the strongest useful signals.
8. Extract reviewable canon changes if the chapter is accepted.
9. Compare revisions if needed.
10. Finalise only after the agent or human has judged the result.

## Stateful Project Loop

Use this when the story will continue across chapters:

```bash
novel-craft start --no-input --idea "<user prompt>" --genre <genre-or-profile> --json
novel-craft story set --premise "<refined premise>" --protagonist "<name>" --power-system "<power/system rule>" --json
novel-craft matrix build --json
novel-craft context build <scene-id> --out .novel/context/<scene-id>.md
novel-craft draft <scene-id> --word-count "1200-2500 words" --json
```

After drafting:

```bash
novel-craft eval chapter <draft.md> --genre <genre-or-profile> --profile fast-webnovel --json
novel-craft memory extract <draft.md> --review --json
novel-craft next <next-scene-id> --from <draft.md> --json
```

Review extracted memory before committing it as canon.

## Post-Draft Checks

```bash
novel-craft eval chapter <draft.md> --genre <genre-or-profile> --profile fast-webnovel --json
novel-craft eval gate <draft.md> --must-include "<required fact>" --must-avoid "<forbidden claim>" --json
novel-craft eval compare old.md new.md --json
```

## Rules

- Novel Craft guides the agent with planning, review, and continuity packets.
- A first chapter should prove the core appeal through a concrete scene before explaining the whole roadmap.
- Treat examples, trope axes, and reader-effect language as indicators, not limits. Do not default to literal oath, vow, or contract magic unless the user asks for it or it clearly wins after comparison.
- Every chapter should have a visible want, resistance, a turn, cost, and continuation reason.
- Do not use lexical novelty as a quality score.
- Keep platform, award, publishing, rights, monetisation, and AI-disclosure facts out of this craft loop unless the user asks for fresh research.
