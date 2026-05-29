# CLI Reference

Novel Craft exposes a human-friendly CLI and an agent-friendly CLI.

Human-friendly commands may prompt. Agent-friendly usage should pass `--json`, `--out`, `--no-input`, or `--defaults` where available.

The public install path is:

```bash
npx novel-craft start
```

Run `novel-craft --help` or `novel-craft help <command>` for built-in help.

## Setup

```bash
novel-craft start
novel-craft start --no-input --defaults --json
novel-craft init --title "My Novel" --genre system-isekai
novel-craft doctor --json
```

`doctor` is read-only. It checks the Rust binary version, embedded rules, embedded skills, `.novel/` project state, npm wrapper detection, and platform target triple.

## Creative Planning

```bash
novel-craft creative methods
novel-craft creative tropes --json
novel-craft creative brief --idea "weak-to-strong kingdom-building isekai"
novel-craft creative tournament --idea "weak-to-strong kingdom-building isekai" --json
novel-craft creative novelty chapter.md --json
novel-craft creative trope-check chapter.md --json
```

## Evaluation

```bash
novel-craft eval rubric
novel-craft eval sheet chapter.md --json
novel-craft eval compare old.md new.md --json
novel-craft eval reader-check chapter.md --profile fast-webnovel --json
novel-craft eval voice-drift chapter01.md chapter02.md --character Mara --json
```

## Story State

```bash
novel-craft character add Mara --trait guarded --motive "clear her family name"
novel-craft scene create chapter_01_scene_01 --goal "Find shelter" --conflict "No one trusts her"
novel-craft plot thread missing_brother --owner Mara --stage clue
novel-craft plot add-promise "Who opened the western gate?" --source chapter_01_scene_01 --json
novel-craft matrix build
novel-craft matrix audit --json
novel-craft matrix heatmap --json
novel-craft context build chapter_01_scene_01 --out .novel/context/ch01s01.md
```

## Review Loop

```bash
novel-craft analyse chapter.md --json
novel-craft lint line chapter.md --json
novel-craft review chapter.md --rubric dialogue
novel-craft revise chapter.md --pass prose
novel-craft memory extract chapter.md --scene-id chapter_01_scene_01
novel-craft memory commit .novel/pending-memory/chapter-01-scene-01.diff.yml
```

## Skills

```bash
novel-craft skills list
novel-craft skills export --out ./skills-export
novel-craft skills install --target ~/.codex/skills --dry-run
novel-craft skills doctor --target ~/.codex/skills --json
```
