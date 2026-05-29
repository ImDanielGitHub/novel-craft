---
name: novel-craft-continuity-sync
description: Use Novel Craft to maintain story continuity: character state, powers, knowledge, promises, payoffs, plot matrix rows, and approved memory diffs.
---

# Novel Craft Continuity Sync

```bash
novel-craft scene create "<scene-id>" --goal "<goal>" --conflict "<conflict>" --turn "<changed state>" --stakes "<cost>"
novel-craft plot add-promise "<promise>" --source "<chapter/scene>"
novel-craft memory extract <draft.md> --scene-id "<scene-id>"
novel-craft memory commit .novel/pending-memory/<scene-id>.diff.yml
novel-craft matrix build
novel-craft matrix audit
```

Keep canon local to `.novel/`. Do not silently mutate character facts, power levels, promises, or knowledge state.
