---
name: novel-creativity-architect
description: Use Novel Craft to widen weak or generic story ideas with creativity methods, trope matrices, reading-level checks, word-choice diagnostics, and LLM-facing premise briefs.
---

# Novel Creativity Architect

Use this skill when the user says a story feels boring, too narrow, too complex, unrelatable, generic, not creative enough, or stuck in the wrong premise frame.

## Workflow

1. Generate the LLM creativity guide:

```bash
novel-craft creative methods
novel-craft creative tropes --genre system-isekai
```

2. Build a creative brief:

```bash
novel-craft creative brief \
  --idea "<seed idea>" \
  --genre system-isekai \
  --reading-grade "6-8" \
  --avoid "<stale or narrow frame>"
```

3. For serious premise work, run a prompt tournament before drafting:

```bash
novel-craft creative tournament \
  --idea "<seed idea>" \
  --genre system-isekai \
  --count 8 \
  --avoid "<stale or narrow frame>"
```

4. Diagnose existing prose when available:

```bash
novel-craft creative diagnose <draft.md>
novel-craft creative novelty <draft.md> --genre system-isekai
novel-craft creative trope-check <draft.md> --genre system-isekai
novel-craft eval reader-check <draft.md> --profile fast-webnovel
```

5. Only after divergence/convergence should the agent draft.

## Rules

- Do not polish a premise that is too narrow. Widen first.
- Mix familiar tropes with one fresh pressure point.
- Make the first page broadly relatable through desire, danger, injustice, fear, hunger, shame, wonder, or a concrete problem.
- Keep reading surface simple when the world/system is complex.
- Flag word-choice clusters that trap the story in a niche frame before the fantasy promise appears.
- Treat trope familiarity as a feature only when it is paired with twist, cost, contradiction, or consequence.
- Use novelty checks as review prompts, not as proof that a story is original.
