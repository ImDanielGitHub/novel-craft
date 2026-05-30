# Fiction Pack Workflow

Novel Craft is not a grammar checker. For fiction, it behaves like a stateful writing engine for long-form AI-assisted novels.

The product thesis:

> Deterministic checks + structured story memory + AI critique + controlled revision loops can help a writing agent draft, audit, revise, and remember a novel over hundreds of thousands of words.

## Core Loop

1. Create or update scene card.
2. Build layered context packet.
3. Draft from packet.
4. Run `novel-craft analyse`.
5. Run focused reviews: scene, character, dialogue, prose, continuity.
6. Audit repetition and causality against the matrix.
7. Revise one pass at a time.
8. Extract a memory diff.
9. Commit approved memory changes.
10. Build or audit the matrix before the next scene.

## Commands

```bash
novel-craft scene create chapter_08_scene_02 \
  --chapter 8 \
  --scene 2 \
  --pov Mara \
  --location prison_archive_outer_hall \
  --goal "Mara confronts Soren about the altered transfer record" \
  --conflict "Soren blocks access and evades direct answers" \
  --turn "Soren admits someone altered the record, but denies doing it" \
  --stakes "Mara may lose the only trail to Elian" \
  --thread missing_brother \
  --thread soren_divided_loyalty \
  --do-not-repeat "brother's ring clue" \
  --do-not-repeat "Soren simply refuses to answer"
```

```bash
novel-craft context build chapter_08_scene_02
novel-craft draft chapter_08_scene_02
novel-craft eval story scenes/ch08_s02.md --genre system-isekai --json
novel-craft analyse scenes/ch08_s02.md --out .novel/reports/ch08_s02.analysis.md
novel-craft review scenes/ch08_s02.md --rubric dialogue
novel-craft audit continuity scenes/ch08_s02.md
novel-craft audit repetition scenes --recent 10
novel-craft audit causality
novel-craft memory extract scenes/ch08_s02.md --scene-id chapter_08_scene_02
novel-craft memory commit .novel/pending-memory/chapter_08_scene_02.diff.yml
novel-craft matrix build
novel-craft matrix audit
```

## Context Packet Layers

Novel Craft should never stuff the whole manuscript into one prompt. It builds a packet from:

- project policy and genre defaults
- hard canon
- character state and voice matrices
- knowledge state
- relationship state
- open plot threads
- promise/payoff state
- recent scene chain
- target scene card
- style profile
- recent memory events

## Rule-Breaking Doctrine

Every finding should be classified:

- `likely_mistake`: probably accidental and weakening agency, immediacy, clarity, causality, or tension.
- `possibly_intentional`: may be functional; keep if it serves the scene effect.
- `clearly_functional`: likely deliberate and useful.

Rules are effects, not commandments.

The agent should use the rule guide:

```bash
novel-craft rules guide
```

That guide gives each rule a problem example and a counterexample/keep-case. The deterministic detector is only the smoke alarm; the reviewer decides whether there is a fire.

Examples:

- Passive voice can be wrong in action but right for mystery, victim focus, helplessness, or institutional coldness.
- Filter words can be wrong in close POV but right when perception itself gives tactical information.
- Telling can be wrong for a major betrayal but right for recap, transition, montage, or deliberately widened narrative distance.
- Repetition can be wrong when a beat loops, but right as motif, ritual, obsession, comedy, or escalation.

## Long-Form Failure Modes

Novel Craft is especially designed to catch failures common in AI-written long fiction:

- protagonist passivity
- smooth scenes with no turn
- openings that lean on the macro premise before dramatizing the smallest working unit, when that weakens reader grip
- repeated arguments with no changed power dynamic
- promises repeated without progress
- characters using knowledge they do not have
- emotional loops that do not escalate
- progression chapters with no advancement, cost, limitation, or rival pressure
- "and then" scene chains without because/therefore causality
- repeated gestures and generic body language
- context-window drift in wounds, clothing, possessions, powers, and relationship state

## Human Control

The CLI can identify patterns and prepare diffs. The author or Codex agent should still approve high-risk changes:

- canon updates
- voice changes
- subtext changes
- plot restructuring
- theme and motif decisions
- intentional summary or narrative-distance choices
