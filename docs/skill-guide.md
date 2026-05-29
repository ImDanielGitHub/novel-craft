# Skill Guide

Novel Craft skills are model-neutral instructions that call the CLI.

They should:

- run `novel-craft` commands
- include generated packets in the model context
- keep rule cards and metrics advisory
- avoid provider-specific assumptions
- preserve human approval for canon, voice, plot, and memory changes

Useful flows:

```bash
novel-craft skills list
novel-craft skills export --out ./skills-export
novel-craft skills install --target ~/.codex/skills --dry-run
```

The bundled skills cover:

- next chapter context
- line editing
- dialogue review
- character review
- continuity sync
- memory diff
- rulebook review
- full-book review
- scene architecture
- creative architecture
- evaluation review

Model boundary: the skill may ask a model to draft, review, compare, or revise, but the CLI itself should not call a model in v1.
