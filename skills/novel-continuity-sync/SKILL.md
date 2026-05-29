---
name: novel-continuity-sync
description: "Use Novel Craft to maintain story continuity memory: character sheets, wounds, dress, smell, powers, motives, knowledge state, open promises, plot matrix scenes, and progression changes."
---

# Novel Continuity Sync

Use this skill when the user asks to update character sheets, story bible, plot matrix, canon, memory, promises, payoffs, wounds, power levels, or long-context continuity.

## Workflow

Add or update characters:

```bash
novel-craft character add "<name>" --appearance "<facts>" --motive "<motive>" --wound "<wound>"
```

Track scene structure:

```bash
novel-craft scene create "<scene-id>" \
  --chapter "<n>" \
  --scene "<n>" \
  --goal "<goal>" \
  --conflict "<conflict>" \
  --turn "<changed state>" \
  --stakes "<cost of failure>" \
  --thread "<plot-thread-id>"
```

Track plot threads as state machines:

```bash
novel-craft plot thread "<thread-id>" \
  --owner "<character>" \
  --stage "<current stage>" \
  --expected-payoff-window "<chapters>" \
  --risk "<do not repeat this clue without new information>"
```

Track promises:

```bash
novel-craft plot add-promise "<promise>" --source "<chapter/scene>"
novel-craft plot payoff "<promise-id>" --note "<payoff>"
```

Extract lightweight memory from a draft:

```bash
novel-craft memory extract <draft.md> --scene-id "<scene-id>"
```

Commit only after the diff is approved:

```bash
novel-craft memory commit .novel/pending-memory/<scene-id>.diff.yml
```

Audit continuity:

```bash
novel-craft audit continuity <draft.md>
novel-craft matrix build
novel-craft matrix audit
```

## Rules

- Stable canon changes require explicit on-page cause or user approval.
- Treat `.novel/characters`, `.novel/scene-cards`, `.novel/plot-threads`, `.novel/state`, `.novel/plot-matrix.yml`, and `.novel/memory.sqlite` as project-local memory, not global Codex memory.
- Track who knows what. Do not let reader knowledge, antagonist knowledge, and POV knowledge collapse together.
- Do not silently mutate canon; use pending memory diffs and commit only approved changes.
