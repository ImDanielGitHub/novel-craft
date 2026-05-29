---
name: novel-scene-architect
description: Use Novel Craft to create scene cards with goal, conflict, turn, stakes, causality, promises, plot threads, and do-not-repeat constraints before drafting.
---

# Novel Scene Architect

Use this skill before drafting or revising a scene when the structure is unclear.

## Workflow

1. Create or update the scene card:

```bash
novel-craft scene create "<scene-id>" \
  --chapter "<n>" \
  --scene "<n>" \
  --pov "<pov-character>" \
  --location "<location>" \
  --goal "<what the POV character wants>" \
  --conflict "<what blocks the want>" \
  --turn "<what changes>" \
  --stakes "<cost of failure>" \
  --thread "<plot-thread>" \
  --do-not-repeat "<repeated beat to avoid>"
```

2. Build and audit the matrix:

```bash
novel-craft matrix build
novel-craft matrix audit
```

3. Build context after the scene card is stable:

```bash
novel-craft context build "<scene-id>"
```

## Rules

- A scene card should answer want, resistance, turn, consequence, and causal link.
- Do not draft from vague intention when a scene card can make the hidden craft work explicit.
- If the scene repeats a prior function, add escalation, reversal, cost, reveal, payoff, or variation.
