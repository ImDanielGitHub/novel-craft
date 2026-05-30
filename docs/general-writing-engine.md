# General Writing Support

Novel Craft is fiction-first. Its main job is to help a writing agent turn a user prompt into a stronger novel-writing brief, draft with clearer constraints, review the draft, and revise with evidence.

The writing-support layer is smaller on purpose. It supports the prose around a novel project: names, READMEs, release notes, docs, short explanations, and claim checks. It is not a separate product line and it does not replace the novel-specific skills.

## Current Scope

Use the bundled `novel-craft-writing-support` skill when an agent needs to:

- name a feature, command, file, or section plainly
- clean up README or docs wording
- make instructions easier for another agent to follow
- keep claims concrete and honest
- remove stiff generated phrasing from release notes or project notes

For fiction drafting, scene planning, chapter review, continuity, dialogue, character, or memory work, use the novel-specific skills first.

## Plain Naming

A good public name is usually the phrase a busy person would ask for.

Prefer:

- `eval gate`
- `creative brief`
- `novel-craft-writing-support`
- `reader-check`
- `must-include`
- `must-avoid`

Avoid names that are clever but hard to search, architecture-heavy, or vague about the command's job.

## Writing Rules

Supporting prose should:

- start with the command, output, or decision the reader needs
- explain what is packaged and what runs locally
- separate deterministic checks from creative judgement
- use concrete claims instead of broad product vision
- keep network and publishing boundaries explicit
- use natural language without over-polishing

The CLI can point out likely issues, required facts, forbidden claims, and awkward wording patterns, but the agent still decides how to revise.
