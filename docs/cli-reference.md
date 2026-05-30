# CLI Reference

Novel Craft is meant to be called by an AI agent while it is writing.

The usual pattern is:

1. read the user request
2. run `agent plan`
3. generate and compare directions
4. draft finished prose
5. save the draft as Markdown
6. run `eval chapter`
7. revise, extract approved canon, and compare

Prefer `--json` when the output will be read by an agent. Prefer `--out` when a packet should be saved and reused.

## Install And Health Check

```bash
npx novel-craft setup
npx novel-craft setup --yes --target ~/.codex/skills --json
npx novel-craft setup --no-skills --json
npx novel-craft doctor --json
novel-craft doctor --json
```

`setup` is the first-run wizard. It lists the bundled `novel-craft-*` skills, explains that they are crucial for Novel Craft to work correctly in an agent workflow, and asks before installing them. Use `--yes` for unattended installs, `--no-skills` to opt out, and `--target` to choose a skills directory. If you opt out, install later with:

```bash
novel-craft skills install --target ~/.codex/skills
```

`doctor` is read-only. It reports version, platform, embedded rules, embedded skills, npm wrapper state, and whether a `.novel/` project is present.

## Prompt To Draft

Use `agent plan` when the user has asked for a chapter, story, or multi-chapter run and the agent needs a concrete writing loop.

```bash
novel-craft agent plan \
  --idea "<user request>" \
  --chapters 1 \
  --genre <genre-or-profile> \
  --profile fast-webnovel \
  --json
```

The plan returns task facts, missing story questions, reader-effect guidance, contender rules, comparison protocol, chapter cards, drafting instructions, revision loop, and post-write commands.

For several chapters:

```bash
novel-craft agent plan --idea "<arc request>" --chapters 3 --genre progression-fantasy --json
```

Use `creative atlas` when the user gives a broad goal, a stale trope, or a prompt that needs more range before drafting.

```bash
novel-craft creative atlas --json
```

The atlas returns 50 genres, 50 subgenres, 50 tropes, 50 sub-tropes, a mixing protocol, and the always-on novel excellence standard. It is an ingredient map for the agent, not a recipe to copy mechanically.

Opening guidance is micro before macro when it helps. The agent should usually dramatise the smallest working unit of the premise before naming the full roadmap. For kingdom-building, that might be one door, meal, protected person, dispute, boundary, or scarce resource before any talk of kingdoms, domains, empires, or upgrade ladders. These are loose indicators, not premise seeds.

Use `creative brief` when the user gives a prompt and the agent needs a better drafting instruction.

```bash
novel-craft creative brief \
  --idea "<user request>" \
  --genre <genre-or-profile> \
  --audience "<reader>" \
  --reading-grade "6-8" \
  --trope "<required ingredient>" \
  --avoid "<stale frame>" \
  --must-include "<required fact>" \
  --must-avoid "<forbidden claim>" \
  --out brief.md
```

Use `creative tournament` when the agent should explore multiple directions before drafting.

```bash
novel-craft creative tournament \
  --idea "<user request>" \
  --genre tech-fantasy-celebration \
  --count 8 \
  --must-include "<required fact>" \
  --must-avoid "<forbidden claim>" \
  --json
```

Available genre/profile matrices include:

- `breakout-serial`
- `nightmare-survival`
- `rational-magus`
- `beast-bond-progression`
- `vr-cultivation`
- `monster-evolution`
- `high-drama-romance`
- `system-isekai`
- `tech-fantasy-celebration`
- `general-fiction`

These profiles are planning tools, not guarantees of awards or popularity. Use them to force stronger hooks, clearer costs, deeper world signals, and serial-retention checks before drafting.

Every profile still uses the same baseline ambition: a strong first chapter, costly advantages, scene turns, chapter-end continuation, and a wider story engine. Use `breakout-serial` only when the reader profile is specifically long-form serial fiction, not because quality is optional elsewhere.

## Draft Review

After drafting, save the text and run the story review when the agent wants craft feedback on an existing `.md` or `.txt` file:

```bash
novel-craft eval chapter draft.md \
  --genre system-isekai \
  --profile fast-webnovel \
  --must-include "<required fact>" \
  --must-avoid "<forbidden claim>" \
  --json
```

Use `eval chapter` for a single chapter and `eval story` for a broader story/extract review:

```bash
novel-craft eval story draft.md \
  --genre system-isekai \
  --profile fast-webnovel \
  --must-include "<required fact>" \
  --must-avoid "<forbidden claim>" \
  --json
```

`eval chapter` and `eval story` are not pass/fail gates. They return metrics, lint leads, reader-profile checks, constraint checks, action-ranked revision priorities, chapter spine, scene change, reader-retention signals, prose review, voice review, open loops, progression/power checks, dialogue/relationship checks, opening guidance, lexical signals, trope saturation, rubric dimensions, and review questions.

Use `eval gate` when hard facts or forbidden claims need a pass/warn/fail status:

```bash
novel-craft eval gate draft.md \
  --profile fast-webnovel \
  --must-include "<required fact>" \
  --must-avoid "<forbidden claim>" \
  --json
```

Gate status:

- `pass`: configured checks did not find a blocker.
- `warn`: review line issues or reader-fit warnings before finalising.
- `fail`: a required fact is missing or a forbidden claim appears.

The gate includes metrics, lint summary, reader-profile warnings, constraint adherence, opening guidance, lexical novelty signals, action-ranked revision priorities, and review notes.

The JSON field is `opening_guidance`: a heuristic warning for first chapters that may announce macro-scale labels too early. The fix is often to move world/system explanation into a present-tense choice, obstacle, cost, or consequence.

## Compare Revisions

```bash
novel-craft eval compare old.md new.md \
  --profile fast-webnovel \
  --must-include "<required fact>" \
  --must-avoid "<forbidden claim>" \
  --json
```

`eval compare` returns evidence for both drafts and `winner: null`. The agent should not treat metrics as taste.

To store a final reviewer choice:

```bash
novel-craft eval reward-export old.md new.md --winner b --dimension overall --note "<why>"
```

## Focused Checks

```bash
novel-craft creative atlas --json
novel-craft agent plan --idea "<user request>" --chapters 1 --json
novel-craft eval chapter draft.md --genre system-isekai --json
novel-craft eval story draft.md --genre system-isekai --json
novel-craft lint line draft.md --json
novel-craft creative novelty draft.md --json
novel-craft creative novelty draft.md --experimental-score --json
novel-craft creative trope-check draft.md --genre system-isekai --json
novel-craft eval reader-check draft.md --profile breakout-serial --json
novel-craft eval voice-drift chapter01.md chapter02.md --character Mara --json
```

`creative novelty` is lexical. The optional experimental score is not a quality score.

## Story State

Use project state when the agent is continuing a longer work.

```bash
novel-craft start \
  --no-input \
  --title "The First Handhold" \
  --idea "weak-to-strong isekai tower climbing" \
  --genre tower-climb \
  --json
novel-craft story set \
  --protagonist "Kai Marlow" \
  --protagonist-want "protect Seren and open a safe route on Floor One" \
  --world "Skyladder tower" \
  --power-system "Gripseed anchors cost body heat when other climbers use them" \
  --json
novel-craft scene create chapter_01_scene_01 --goal "Find shelter" --conflict "No one trusts her"
novel-craft character add Mara --trait guarded --motive "clear her family name"
novel-craft plot thread missing_brother --owner Mara --stage clue
novel-craft plot add-loop "Who opened the western gate?" --source chapter_01_scene_01 --json
novel-craft matrix build --json
novel-craft matrix heatmap --json
novel-craft context build chapter_01_scene_01 --out .novel/context/ch01s01.md
```

`matrix build` reads back the actual `.novel/` state: story seed, characters, plot threads, open loops, payoffs, scene cards, and progression notes. `context build` then packages that state for a drafting agent instead of only repeating the scene card.

## Draft And Continue From State

Use `draft` when the agent wants a prose brief from project state:

```bash
novel-craft draft chapter_01_scene_01 \
  --word-count "1800 words" \
  --must-include "first safe handhold" \
  --avoid "status dump" \
  --json
```

Use `next` after a chapter exists:

```bash
novel-craft next chapter_02 \
  --from .novel/drafts/chapter-01.md \
  --json
```

The next packet includes source-draft signals, likely changes, open loops, power/status deltas, continuity reminders, and a suggested next chapter card.

After a draft is approved, extract a reviewable canon diff:

```bash
novel-craft memory extract .novel/drafts/chapter-01.md --review --json
```

The agent should review the extracted facts before committing anything to memory.

## Supporting Writing

Use this for docs, names, release notes, and short explanatory prose around the novel project.

```bash
novel-craft writing guide
novel-craft writing show --json
```

## Skills

Bundled skills are embedded in the binary and can be exported into a Codex skills directory.

```bash
novel-craft skills list --json
novel-craft skills export --out ./skills-export
novel-craft skills install --target ~/.codex/skills --dry-run
novel-craft skills doctor --target ~/.codex/skills --json
```

Primary public skill names use the `novel-craft-*` prefix, for example `novel-craft-agentic-writer`, `novel-craft-creativity-engine`, `novel-craft-draft-review`, `novel-craft-next-chapter`, and `novel-craft-writing-support`. Older names are exported only as deprecated alias stubs for compatibility.
