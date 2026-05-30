---
name: novel-craft-draft-review
description: Use Novel Craft to review drafted chapters or stories with craft checks, gates, reader profiles, and pairwise comparisons.
---

# Novel Craft Draft Review

Use this skill after the agent has written a chapter, story extract, or revision.

## Workflow

```bash
novel-craft eval chapter <draft.md> --genre <genre-or-profile> --profile fast-webnovel --json
novel-craft eval story <draft.md> --genre <genre-or-profile> --profile fast-webnovel --json
novel-craft eval gate <draft.md> --profile fast-webnovel --must-include "<required fact>" --must-avoid "<forbidden claim>" --json
novel-craft eval compare old.md new.md --json
```

## Review Focus

- chapter spine: hook, orientation, goal, obstacle, escalation, turn, cost, exit hook
- scene change: plot, character, relationship, knowledge, stakes, promise
- reader retention: open question, attachment, payoff expectation, ending momentum
- prose: clarity, rhythm, filter words, abstract emotion, sensory grounding
- voice: what the POV notices, ignores, values, fears, and misreads
- open loops: introduced, paid off, unresolved, or transformed

## Rules

- `eval chapter` and `eval story` are not pass/fail gates.
- Use `eval gate` only when hard constraints need status.
- Cite text evidence before judging a revision better.
- Never choose a winner from lexical novelty signals alone.
- Treat findings as guidance unless the agent or human decides they harm the intended reader effect.
