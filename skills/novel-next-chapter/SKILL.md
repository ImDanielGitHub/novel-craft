---
name: novel-next-chapter
description: Use Novel Craft to build a long-context packet and prepare or review the next webnovel/progression-fantasy chapter with continuity, open promises, and serial momentum preserved.
---

# Novel Next Chapter

Use this skill when the user wants the next chapter, next scene, chapter planning, continuation, or serial/progression drafting support.

## Workflow

1. Work from the story repo/folder that contains `.novel/`. If it does not exist, run:

```bash
novel-craft init --title "<title>"
```

2. If a scene card does not exist, create one before drafting:

```bash
novel-craft scene create "<scene-id>" \
  --chapter "<chapter>" \
  --scene "<scene>" \
  --goal "<goal>" \
  --conflict "<conflict>" \
  --turn "<required turn>" \
  --stakes "<cost of failure>"
```

3. Build the layered context packet:

```bash
novel-craft context build "<scene-id>"
```

4. Build the LLM rule guide and drafting prompt:

```bash
novel-craft rules guide
novel-craft draft "<scene-id>"
```

5. If there is a previous draft to learn from, run:

```bash
novel-craft next --chapter "<chapter>" --scene "<scene>" --draft <draft.md>
```

6. Use `.novel/context/<scene-id>.md` and `.novel/reports/<scene-id>-draft-prompt.md` as the drafting brief. Preserve canon and source policy.

## Rules

- Do not bulk-ingest hosted copyrighted fiction.
- Use `novel-craft analyse`, `novel-craft review --rubric scene`, `novel-craft review --rubric dialogue`, `novel-craft audit continuity`, `novel-craft audit repetition`, and `novel-craft memory extract` after drafting.
- Commit memory only after the diff is approved with `novel-craft memory commit`.
- For webnovel/progression work, check hook, payoff, progression delta, cost, rival pressure, and unresolved promises.
- Treat craft findings as effects: likely mistake, possibly intentional, or clearly functional.
