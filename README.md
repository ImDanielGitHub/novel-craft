# Novel Craft

**Start an original webnovel, continue a serial, or revise an existing manuscript with your writing agent. Keep the story, not a pile of prompts.**

Novel Craft 0.2 is a local, dependency-free Node CLI. It runs real planning, drafting, review and bounded revision through an authenticated Codex CLI or a runner you supply. It also works as the workspace for an agent already writing in your editor: no second agent or API key is required for that route.

Prose stays in Markdown. Proposed changes, accepted chapters, candidate facts, approvals and recoverable history are separate. There is no literary quality score.

## Install

Node **22.14 or later** is required. No Rust compiler, platform binary download, install hook or runtime npm dependency is needed.

```bash
npm install -g novel-craft@0.2.0
novel-craft --version
novel-craft doctor --json
```

A release tarball can also be installed directly with `npm install -g ./novel-craft-0.2.0.tgz`. For source development, run `node cli/index.mjs --help`.

## Write from scratch

Install and authenticate [Codex CLI](https://developers.openai.com/codex/cli/) separately, or configure the JSON runner described in [the CLI reference](docs/CLI.md). Model access and any usage charges belong to that runner, not this package.

```bash
novel-craft create ./harbour-of-unwritten-laws \
  --title "The Unwritten Harbour" \
  --idea "A harbour translator discovers that tomorrow's laws arrive as undelivered letters" \
  --genre fantasy --genre mystery \
  --voice "Restrained close third person; dry humour; concrete observations" \
  --chapters 3 --words 1800 --min-words 1200 --max-words 2400 \
  --passes 1 --runner codex --json
```

This invokes a model. It does not print a template and call it a novel. The pipeline creates an editable plan, writes each chapter with preceding context, checks explicit limits, obtains excerpt-backed review, and makes at most the requested revision passes. It checkpoints progress between stages.

The result includes a `proposal_id` and `base_revision`. Inspect the proposal before accepting it:

```bash
novel-craft diff <proposal-id> --project ./harbour-of-unwritten-laws
novel-craft commit <proposal-id> --expect <base-revision> \
  --project ./harbour-of-unwritten-laws
novel-craft export --format html --out ./reading-copy.html \
  --project ./harbour-of-unwritten-laws
```

Replace the angle-bracket values with those returned by the command. `--accept` on generation explicitly delegates manuscript acceptance when no major review issue or measurable failure remains. `--accept-facts` is separate permission to approve extracted facts. Without it, they remain candidates.

To continue, run `generate --chapters 3` inside the project. To resume a failed call, use `runs`, then `generate --resume <run-id>` with the same explicit runner configuration. Call and revision budgets do not reset silently.

## Work with the agent you already have

An editor agent can do the writing itself rather than launch Codex inside another agent. Install the entry skill into its supported skills folder:

```bash
novel-craft setup --target .agents/skills
novel-craft schema --json
novel-craft init --title "My serial" --genre romance --genre historical
novel-craft context --chapter 1 --json
```

The agent writes a temporary draft, then uses `import`, `diff` and `commit`. The same loop handles an existing book:

```bash
novel-craft import --file ./existing-chapter.md --chapter 1 --json
novel-craft diff <proposal-id>
novel-craft commit <proposal-id> --expect <base-revision>
novel-craft revise --chapter 1 \
  --instruction "Strengthen the subtext without making either speaker openly hostile" \
  --runner codex --json
```

`review --chapter 1 --packet --json` gives an existing agent the actual chapter, context and review schema without invoking another model. Project intentions can be refined with `project update --voice "..." --expect <revision>`; updates merge only supplied fields and preserve history.

## Broad categories, specific guidance

The package contains **47 profiles**, resolving **57 observed catalogue labels**, including action, adventure, comedy, romance, mystery, horror, historical, sports, science fiction, military, mecha, slice of life, psychological fiction, xianxia, wuxia, xuanhuan, systems, games, reincarnation and isekai. Genre, setting, format, audience and content labels remain distinguishable. Multiple profiles can be combined.

```bash
novel-craft genres
novel-craft genres xianxia --examples
novel-craft genres romance --examples --json
novel-craft genres coverage --json
novel-craft guide
```

Every profile includes reader appeal, a serial engine, common failure modes, review questions, attributable reading or craft references, and an **original illustrative vignette** with an explanation. Examples are opt-in in drafting context to reduce accidental imitation. They are not copied chapters or proof that a particular book follows every recommendation.

Coverage was checked on **7 September 2026** against the accessible [NovelFull catalogue](https://novelfull.net/). **NovelFullbook was unreachable**, so an exact match to its current taxonomy is not claimed. `genres coverage` records that limitation and the label-to-profile mapping. Unknown labels fail clearly; `--genre-file` supports an explicit custom profile instead of silently switching to fantasy. Explicit-content catalogue labels provide non-graphic adult relationship guidance, not pornography; sexual-minor categories are unsupported.

## What protects the work

Accepted text is stored as immutable content objects and materialised as ordinary `manuscript/chapter-0001.md` files. A single revision pointer selects the authoritative state. Source hashes stop stale proposals from overwriting external edits. A write journal supports recovery after interruption; undo creates a new revision rather than erasing history.

Facts cite exact manuscript excerpts and distinguish world facts, character beliefs, reader knowledge and author plans. Candidates require approval. Context includes accepted, source-valid facts, a full recent chapter, relevant older summaries, the current intention and explicit coverage warnings. Budget estimates are labelled estimates; essential text is not silently truncated.

Useful checks remain deterministic: file integrity, schema validity, exact excerpts, explicit literal constraints and word counts. Semantic reviews are model judgements with evidence and possible alternative readings. They can be wrong. A successful run does not establish that readers will enjoy the novel.

## Compatibility and limits

Version 0.2 replaces the npm CLI surface; it is not a drop-in replacement for every 0.1 command. The previous Rust implementation and its tests remain in the repository as legacy source, but are not shipped in the new npm package. Existing `.novel/` projects are never silently migrated or overwritten. Create a separate 0.2 workspace and import manuscript copies; review old notes before adding them as candidate facts. See [migration and scope](docs/REBIRTH.md).

Markdown and HTML exports use a deliberately small text renderer, not full CommonMark. EPUB exports use a self-contained EPUB 3 archive. Read exported output in your target reader before distribution. No remote publishing or scraping is performed.

## Develop and contribute

```bash
npm ci --ignore-scripts
npm test
npm run verify:package
```

Tests cover the actual subprocess protocol, continuation context, canon approval, manual edits, stale revisions, bounded generation, resume, corrupted state, interrupted commits, undo, exports, category coverage and an offline installation of the packed tarball. The model protocol fixture is explicitly synthetic; it is **not** a writing-quality benchmark. Real-model and reader evaluations are separate evidence.

Contributions need a failing regression or a reproducible writing task, not another unsupported score. For genre guidance, submit original examples, primary references and the circumstances where the advice should not apply. Do not contribute copyrighted chapter corpora or private manuscripts without permission.

[CLI reference](docs/CLI.md) · [Architecture, research and migration](docs/REBIRTH.md) · [MIT licence](LICENSE)
