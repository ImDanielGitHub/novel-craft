# Skill Guide

Novel Craft skills are compact instructions for using the CLI.

They should:

- run `novel-craft` commands
- include generated packets only when they help the current task
- keep rule cards and metrics advisory
- avoid provider-specific assumptions
- preserve human approval for canon, voice, plot, and memory changes

Useful flows:

```bash
novel-craft setup
novel-craft setup --yes --target ~/.codex/skills --json
novel-craft setup --no-skills --json
novel-craft skills list
novel-craft skills export --out ./skills-export
novel-craft skills install --target ~/.codex/skills --dry-run
```

`setup` is the recommended first run. It shows the skills below, explains why they are crucial for Novel Craft to work correctly in an agent workflow, and lets the user opt out before anything is installed.

The bundled skills cover:

- `novel-craft-agentic-writer` for prompt-to-plan-to-finished-chapter loops
- `novel-craft-creativity-engine` for atlas, brief, and contender generation
- `novel-craft-draft-review` for chapter/story review, gates, and comparison
- `novel-craft-next-chapter` for continuation work
- `novel-craft-scene-planner` for scene cards and context
- `novel-craft-line-review`, `novel-craft-dialogue-review`, and `novel-craft-character-review`
- `novel-craft-continuity-sync`, `novel-craft-memory-sync`, and `novel-craft-book-audit`
- `novel-craft-rulebook-review`
- `novel-craft-writing-support` for plain naming, natural wording, and docs cleanup

Older skill names are shipped only as deprecated alias stubs for one release.
