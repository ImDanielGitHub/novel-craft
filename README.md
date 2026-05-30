# Novel Craft

![Novel Craft CLI shown beside a story map, rule cards, and manuscript pages.](assets/readme/hero.png)

[![CI](https://github.com/ImDanielGitHub/novel-craft/actions/workflows/ci.yml/badge.svg)](https://github.com/ImDanielGitHub/novel-craft/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![npm](https://img.shields.io/npm/v/novel-craft.svg)](https://www.npmjs.com/package/novel-craft)

Novel Craft is a local CLI that helps a writing agent turn a user prompt into better fiction.

The agent uses it to plan a chapter, compare possible directions, draft in Markdown, review the draft, revise, and compare versions.

## Install

Run the startup wizard first. It lists the bundled Novel Craft skills, explains why they matter, and asks before installing them into your agent skills folder.

```bash
npx novel-craft setup
```

For unattended setup:

```bash
npx novel-craft setup --yes --target ~/.codex/skills --json
```

To opt out of skill installation and only use the binary:

```bash
npx novel-craft setup --no-skills --json
```

You can install the skills later:

```bash
novel-craft skills install --target ~/.codex/skills
```

Check the installed binary:

```bash
npx novel-craft doctor --json
```

Start a project when the agent needs local story state:

```bash
npx novel-craft start \
  --no-input \
  --title "The First Handhold" \
  --idea "weak-to-strong isekai tower climbing" \
  --genre tower-climb \
  --json
```

From a source checkout:

```bash
cargo run --bin novel-craft -- doctor --json
```

More install notes: [docs/npm-install.md](docs/npm-install.md).

## Agent Flow

The user says what they want. The agent runs Novel Craft before and after drafting.

1. Turn the user prompt into an agent plan.

```bash
novel-craft agent plan \
  --idea "weak-to-strong kingdom-building system" \
  --chapters 1 \
  --genre system-isekai \
  --profile fast-webnovel \
  --json
```

The packet tells the agent what facts to preserve, what story questions are missing, how to generate contenders, how to compare them, and how to review the finished chapter.

2. Pull a broad ingredient map when the prompt is thin or too familiar.

```bash
novel-craft creative atlas --json
```

3. Convert the request into a draft brief when the agent needs a more detailed instruction packet.

```bash
novel-craft creative brief \
  --idea "write a launch-night tech-fantasy story for a newly published CLI" \
  --genre tech-fantasy-celebration \
  --must-include "package name: novel-craft" \
  --must-include "the CLI ships bundled craft checks" \
  --must-avoid "wrong version number"
```

4. Draft finished prose in Markdown.

For stateful projects, the agent can build a draft packet from `.novel/` before drafting:

```bash
novel-craft draft chapter_01_scene_01 \
  --word-count "1800 words" \
  --must-include "first safe handhold" \
  --avoid "status dump" \
  --json
```

5. Save the draft, then run the normal chapter review. The review now includes action-ranked revision priorities so the agent can fix the highest-impact issue first.

```bash
novel-craft eval chapter draft.md \
  --genre tech-fantasy-celebration \
  --profile fast-webnovel \
  --json
```

6. If hard facts matter, run the deterministic gate.

```bash
novel-craft eval gate draft.md \
  --must-include "package name: novel-craft" \
  --must-avoid "wrong version number" \
  --json
```

7. Revise for useful craft signals. Treat findings as guidance, not automatic rewrite orders.

8. Compare alternatives.

```bash
novel-craft eval compare draft-a.md draft-b.md \
  --must-include "package name: novel-craft" \
  --must-avoid "wrong version number" \
  --json
```

`eval compare` never chooses the winner. It gives the agent evidence so the reviewer can make the call.

## Commands Agents Usually Use

```bash
novel-craft creative brief --idea "<user request>" --genre <genre-or-profile> --must-include "<fact>" --must-avoid "<bad claim>"
novel-craft agent plan --idea "<user request>" --chapters 1 --genre <genre-or-profile> --profile fast-webnovel --json
novel-craft creative atlas --json
novel-craft creative tournament --idea "<user request>" --count 8 --json
novel-craft eval chapter draft.md --genre <genre-or-profile> --profile fast-webnovel --json
novel-craft eval story draft.md --genre <genre-or-profile> --json
novel-craft eval gate draft.md --must-include "<fact>" --must-avoid "<bad claim>" --json
novel-craft eval compare old.md new.md --json
novel-craft lint line draft.md --json
novel-craft eval reader-check draft.md --profile breakout-serial --json
novel-craft creative novelty draft.md --json
novel-craft context build chapter_01_scene_01 --out .novel/context/ch01s01.md
novel-craft next chapter_02 --from chapter-01.md --json
novel-craft memory extract chapter-01.md --review --json
novel-craft writing guide
novel-craft skills list --json
```

`creative novelty` reports lexical signals only. It is not a quality score.

Useful genre/profile values include `tower-climb`, `progression-fantasy`, `isekai-survival`, `dungeon-core`, `breakout-serial`, `nightmare-survival`, `rational-magus`, `beast-bond-progression`, `vr-cultivation`, `monster-evolution`, `high-drama-romance`, `system-isekai`, and `general-fiction`.

`creative atlas` gives agents 50 genres, 50 subgenres, 50 tropes, and 50 sub-tropes for broad mix-and-match planning. Briefs and tournaments also carry the always-on novel standard: a strong first chapter, costly advantages, scene turns, chapter-end continuation, and a wider story engine.

Opening guidance: often show the big story through a small dramatic unit first. For example, a kingdom-building novel might begin with one room, meal, door, protected person, dispute, boundary, or scarce resource before the prose leans on kingdoms, empires, domains, or future upgrade ladders. These examples are loose indicators, not premise seeds.

## What The Package Contains

The npm package ships:

- a small Node wrapper at `npm/bin/novel-craft.js`
- release binaries under `npm/bin/`
- embedded craft rules
- embedded `novel-craft-*` agent skills, plus one-release deprecated alias stubs
- embedded writing-support profile and reader checks
- a craft reference packet for story/chapter planning and review

The rules and skills are compiled into the binary. The agent does not need to read this repository at runtime.

The setup wizard can copy the bundled skills into `~/.codex/skills` or another target folder. They are crucial for Novel Craft to work correctly in an agent workflow because they teach when to run `agent plan`, when to draft, when to review a chapter, when to compare revisions, and when to extract story memory. You can opt out during setup and install them later with `novel-craft skills install --target <dir>`.

## What It Does

Novel Craft helps the agent:

- make a vague prompt more concrete before drafting
- build an agent-facing chapter plan with `agent plan`
- widen a narrow premise with a broad story atlas
- review an existing `.md` or `.txt` story file after drafting
- review a single chapter with `eval chapter`
- carry required facts into the draft
- block forbidden claims
- create scene cards and context packets
- read back characters, plot threads, open loops, scene cards, and story seed through `matrix build` and `context build`
- generate draft and next-chapter packets from project state
- extract reviewable canon changes from a written draft
- check reader fit
- flag likely line issues
- surface trope and novelty signals
- compare revisions without pretending metrics are taste
- export bundled Novel Craft skill files

## What It Checks

The checks are deterministic signals:

- required facts missing
- forbidden claims present
- passive voice
- filter words
- abstract emotion labels
- weak reader fit
- opening that announces the macro premise before showing micro-action
- trope saturation
- repeated beats
- voice drift
- open loops and payoff pressure
- power without cost
- weak world-depth signals
- missing chapter-end continuation reason

These checks tell the agent where to look. They do not replace creative judgement, and they should not force the story into a rigid template.

## Project State

When state is useful, `novel-craft start` creates `.novel/`:

- project settings
- craft rules
- scene cards
- character notes
- plot threads
- context packets
- review reports
- local memory files

Use `story set` to store the real seed after the user clarifies the project:

```bash
novel-craft story set \
  --title "The First Handhold" \
  --genre tower-climb \
  --premise "weak-to-strong isekai tower climbing" \
  --protagonist "Kai Marlow" \
  --power-system "Gripseed anchors cost body heat when other climbers use them" \
  --json
```

`matrix build` hydrates the current project state from scene cards, characters, plot threads, open loops, payoffs, progression notes, and the story seed. `context build` packages that state for the drafting agent.

For one-off prompt-to-draft work, an agent can use `creative brief`, `eval gate`, and `eval compare` without creating a full project.

## Limits

Novel Craft does not:

- store API keys
- scrape hosted fiction
- train on or imitate copyrighted novels
- claim objective literary quality
- guarantee awards, rankings, publishing outcomes, platform eligibility, or reader response

Source policy: [docs/source-policy.md](docs/source-policy.md).

## Development

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
npm run pack:dry
```

More docs:

- [docs/cli-reference.md](docs/cli-reference.md)
- [docs/release-process.md](docs/release-process.md)
- [docs/npm-publish.md](docs/npm-publish.md)
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [SECURITY.md](SECURITY.md)
