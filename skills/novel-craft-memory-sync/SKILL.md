---
name: novel-craft-memory-sync
description: Use Novel Craft to extract, inspect, and commit approved story-memory diffs after a draft changes canon.
---

# Novel Craft Memory Sync

```bash
novel-craft memory extract <draft.md> --scene-id "<scene-id>"
novel-craft memory commit .novel/pending-memory/<scene-id>.diff.yml
novel-craft matrix build
```

Commit only approved facts. If a fact is uncertain, leave it pending or mark it as suspicion rather than canon.
