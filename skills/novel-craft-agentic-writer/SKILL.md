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
8. Compare revisions if needed.
9. Finalise only after the agent or human has judged the result.

## Post-Draft Checks

```bash
novel-craft eval chapter <draft.md> --genre <genre-or-profile> --profile fast-webnovel --json
novel-craft eval gate <draft.md> --must-include "<required fact>" --must-avoid "<forbidden claim>" --json
novel-craft eval compare old.md new.md --json
```

## Rules

- Novel Craft guides the agent; it does not call a model or write prose by itself.
- A first chapter should prove the promise through a concrete scene before explaining the whole roadmap.
- Treat examples, trope axes, and promise language as indicators, not limits.
- Every chapter should have a visible want, resistance, a turn, cost, and continuation reason.
- Do not use lexical novelty as a quality score.
- Keep platform, award, publishing, rights, monetisation, and AI-disclosure facts out of this craft loop unless the user asks for fresh research.
