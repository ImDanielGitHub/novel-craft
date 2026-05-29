---
name: novel-memory-diff
description: Use Novel Craft to extract, inspect, and commit approved story-memory diffs for facts, promises, character state, wounds, choices, and progression changes.
---

# Novel Memory Diff

Use this skill after a draft is accepted or when canon/state needs to be synchronised.

## Workflow

1. Extract a pending diff:

```bash
novel-craft memory extract <draft.md> --scene-id "<scene-id>"
```

2. Inspect `.novel/pending-memory/<scene-id>.diff.yml`.

3. Commit only approved changes:

```bash
novel-craft memory commit .novel/pending-memory/<scene-id>.diff.yml
```

4. Rebuild matrix if promises or scene state changed:

```bash
novel-craft matrix build
```

## Rules

- Do not silently mutate canon.
- If a fact is uncertain, leave it in pending memory or mark it as a suspicion rather than canon.
- Track who knows what separately from what the reader knows.
