# Novel Craft SDS

## Architecture

Novel Craft is a local-first Rust CLI distributed as a native binary and npm/npx wrapper:

```text
novel-craft/
  Cargo.toml
  src/lib.rs
  src/bin/novel-craft.rs
  src/bin/novel.rs
  npm/bin/novel-craft.js
  rules/default.yml
  references/
  evals/fixtures/
  skills/
```

The CLI is intentionally local-first. Project state lives inside each writing project under `.novel/`. The current release has no API key handling, no telemetry, and no scraping.

Architecturally, Novel Craft should be treated as a fiction-first writing engine for agents. The reusable core is rule cards, purpose profiles, context packets, deterministic approximations, review guides, comparison reports, feedback memory, and targeted revision passes.

Fiction-specific state such as characters, scene cards, promises, and plot threads should stay isolated. Supporting prose outside fiction is currently limited to the `novel-craft-writing-support` skill for names, docs, release notes, and claim checks.

Serial-success research is represented as abstract genre/profile matrices, not as copied source text. The matrices should push agents toward opening wounds, costly powers, world-depth signals, and serial-retention hooks while preserving source-policy boundaries.

## Storage

Project files:

- `.novel/project.yml`: title, genre profile, POV mode, source policy, style anchors, token budget.
- `.novel/characters/*.yml`: durable and dynamic character facts, including surface, sensory signature, psychology, behaviour, voice matrix, relationships, continuity, current state, and do-not-overuse gestures.
- `.novel/scene-cards/*.yml`: one card per scene with goal, conflict, turn, entry/exit state, open threads, promises, emotional beat, scene shape, causality, and do-not-repeat items.
- `.novel/plot-threads/*.yml`: state-machine records for mysteries, promises, arcs, powers, debts, rivalries, and other long-running threads.
- `.novel/plot-matrix.yml`: arcs, scenes, promises, payoffs, progression beats, and compatibility rows for older projects.
- `.novel/story-matrix.yml`: generated matrix built from scene cards, plot rows, and plot threads.
- `.novel/state/knowledge-state.yml`: who knows what, including reader knowledge.
- `.novel/state/dynamic-state.yml`: current scene, character state, objects, injuries, locations, and volatile state.
- `.novel/state/style-profile.yml`: distance, voice, motifs, rhythm, and overuse watchlist.
- `.novel/state/beta-feedback.yml`: stored beta-reader or human feedback, grouped later by target and dimension.
- `.novel/state/taste-profile.yml`: liked/disliked/mixed sample records used to calibrate future comparisons.
- `.novel/rules/default.yml`: copied rule cards so projects can customise rules.
- `.novel/memory.sqlite`: extracted names, wounds, promises, choices, progression events.
- `.novel/evals/reward-pairs.jsonl`: optional pairwise preference records for future evaluator/reward adapters.
- `.novel/pending-memory/*.diff.yml`: reviewable state changes before canon commit.
- `.novel/context/*.md`: layered context packets for a target scene or chapter.
- `.novel/reports/*.md`: context packets, revision reports, full-book audits.

If broader project state is added later, it should use a separate layer rather than overloading `.novel/`. For now, `.novel/` is the supported public state.

## Rule Card Schema

Each rule card contains:

- `id`: stable rule identifier.
- `level`: `line`, `scene`, or `plot`.
- `name`: short human label.
- `description`: human-readable craft rule.
- `effect`: what reader/narrative effect the rule is protecting.
- `priority`: `P1` always run, `P2` default review, `P3` optional or genre-specific.
- `applies_when`: genre, POV, mode, or scene conditions.
- `detect`: deterministic, hybrid, or AI-review detection hint.
- `break_when`: legitimate exceptions.
- `usual_problem`: what goes wrong when the pattern is accidental.
- `good_when`: cases where the pattern is probably useful.
- `bad_when`: cases where the pattern is probably a mistake.
- `severity`: `S1` informational through `S4` human review required.
- `automation_risk`: `R1` safe suggestion through `R3` human review only.
- `rewrite_strategy`: how to improve the passage.
- `rewrite_strategies`: optional list of pass-specific revision tactics.
- `classification_hints`: optional lexical or structural hints for classifying intent.
- `review_guidance`: how the reviewer should use the rule.
- `review_questions`: questions to ask before recommending revision.
- `deterministic_limitations`: why the detector is only an approximation.
- `examples`: short bad/better examples.
- `counterexamples`: a keep-case showing when the pattern should not be "fixed."

If broader rule cards are added later, they should also support:

- `domain`: fiction, docs, or another explicitly supported writing domain.
- `purpose`: the writing job or reader task the rule helps.
- `reader_effect`: the practical effect to protect, such as trust, desire, clarity, suspense, actionability, empathy, or decision confidence.
- `source_requirements`: whether claims need citations, examples, evidence, or approval.

The analyzer classifies findings into:

- `likely_mistake`: the pattern probably weakens the intended effect.
- `possibly_intentional`: the pattern may be serving a legitimate effect and should be reviewed.
- `clearly_functional`: the pattern likely supports the scene effect, such as helplessness, concealment, institutional coldness, ritual repetition, or shock.

## Detection Pipeline

Line checks are mostly deterministic:

- Passive voice regex.
- Filter word density.
- Emotion labels.
- Abstract judgement labels without nearby sensory evidence.
- Weak phrases, adverbs, repeated words, rhythm repetition, ornate image clusters.

Scene checks are hybrid:

- Exposition block detection by paragraph length and backstory terms.
- Dialogue subtext and expository-dialogue patterns.
- Scene purpose heuristics for goal, resistance, and changed state.
- Agency and protagonist passivity signals.
- Motivation-reaction ordering signals.
- Dialogue leverage, evasion, direct-emotion, and hidden-agenda signals.
- Paragraphing as camera-control signals.
- Serial hook and mobile-readability checks.

Plot checks use project state:

- Character fact contradiction checks from `.novel/characters`.
- Gesture repetition.
- Progression signal checks.
- Plot-matrix repeated scene function checks.
- Open promise count checks.
- Therefore/because scene-transition audit.
- Knowledge-state and plot-thread review.
- Emotional-beat repetition and stalled arc warnings.

## CLI Surface

Primary commands use `novel-craft`; `novel` is a convenience alias where the installer supports it:

- `novel-craft start`: run the guided story setup wizard.
- `novel-craft init`: create `.novel/` non-interactively.
- `novel-craft doctor`: run read-only install, wrapper, asset, and project checks.
- `novel-craft agent plan`: build an agent-facing chapter or multi-chapter workflow from a user prompt.
- `novel-craft scene create`: create a structured scene card.
- `novel-craft scene from-text`: draft a scene card from manuscript signals.
- `novel-craft context build`: build a layered context packet for a scene or chapter.
- `novel-craft draft`: write a drafting prompt from the context packet.
- `novel-craft analyse`: run deterministic writing-quality analysis for fiction projects.
- `novel-craft review --rubric`: write a focused prose, scene, character, dialogue, continuity, or all-pass review packet.
- `novel-craft matrix build`: generate `.novel/story-matrix.yml`.
- `novel-craft matrix audit`: find repeated functions, weak causality, and promise load.
- `novel-craft matrix heatmap`: show open promise/thread heat, stale mentions, and payoff risk.
- `novel-craft audit continuity`: check a draft against character facts and knowledge state.
- `novel-craft audit repetition`: check text and the recent matrix for repeated words, gestures, functions, locations, endings, and emotional beats.
- `novel-craft audit causality`: run the therefore/because plot test.
- `novel-craft memory extract`: produce a reviewable memory diff.
- `novel-craft memory commit`: commit an approved diff to SQLite, dynamic state, and promise tracking.
- `novel-craft rules list`: inspect active project-local rule cards.
- `novel-craft rules guide`: emit a rule guide with examples, counterexamples, and approximation warnings.
- `novel-craft rules refresh`: replace a project's copied rule cards with CLI defaults.
- `novel-craft creative methods`: list creativity methods such as divergence/convergence, trope twist, matrix mixing, inversion, SCAMPER, self-refine, and branching.
- `novel-craft creative tropes`: emit trope axes for mix-and-match narrative generation.
- `novel-craft creative atlas`: emit 50 genres, 50 subgenres, 50 tropes, 50 sub-tropes, a mixing protocol, and the always-on novel excellence standard.
- `novel-craft creative brief`: build a premise-generation prompt that avoids over-narrow frames and scores options.
- `novel-craft creative diagnose`: report readability, word-choice watchlists, and premise-narrowing language.
- `novel-craft creative novelty`: estimate novelty/specificity with generic phrase, concrete detail, freshness, and trope-saturation signals.
- `novel-craft creative trope-check`: audit whether familiar tropes have enough cost, twist, limitation, or consequence.
- `novel-craft creative tournament`: generate a high-variance prompt tournament pack before drafting.
- `novel-craft eval rubric`: emit a multi-dimensional creative-writing rubric.
- `novel-craft eval sheet`: create an evidence-based scoring sheet for a draft.
- `novel-craft eval chapter`: review a drafted chapter file with chapter spine, scene change, reader retention, prose, voice, open-loop, progression, and dialogue checks.
- `novel-craft eval story`: review a drafted story or extract with the same post-writing checks.
- `novel-craft eval gate`: run hard-constraint pass/warn/fail checks for required facts and forbidden claims.
- `novel-craft eval compare`: create a pairwise comparison prompt for two revisions.
- `novel-craft eval reader-profiles`: list built-in readability/reader-fit profiles.
- `novel-craft eval reader-check`: check a draft against a target reader-level profile.
- `novel-craft eval voice-drift`: compare dialogue fingerprints across drafts to catch character voice drift.
- `novel-craft eval feedback-add`: store human or beta-reader feedback as reader-effect evidence.
- `novel-craft eval feedback-report`: summarize stored feedback by target and dimension.
- `novel-craft eval calibrate-add`: add liked, disliked, or mixed samples to a local taste profile.
- `novel-craft eval calibrate-report`: summarize the local taste profile for future comparisons.
- `novel-craft eval reward-export`: append a pairwise preference record to JSONL for evaluator/reward adapters.
- `novel-craft eval reward-report`: summarize exported pairwise records.

Later domain-pack commands should reuse the same command families instead of adding provider-specific wrappers. The current public release keeps that surface fiction-first.

Those commands should stay provider-free.

Supported public profile names include `system-isekai`, `breakout-serial`, `nightmare-survival`, `rational-magus`, `beast-bond-progression`, `vr-cultivation`, `monster-evolution`, `high-drama-romance`, `tech-fantasy-celebration`, and `general-fiction`.

`breakout-serial` is a reader-profile matrix, not the only high-quality mode. The shared novel excellence standard is injected into brief, tournament, and start packets so agents always check first-chapter pull, chapter structure, wider story engine, costly advantages, and continuation pressure.

Opening-promise checks run inside `eval chapter`, `eval story`, and `eval gate`. They inspect the opening window for macro-scale announcement risk and ask the agent whether early world/system explanation would work better as scene-level action, choice, cost, or consequence.

## Extension Boundary

Any future adapters should:

- Use context packets, not raw whole-book dumps.
- Preserve source policy.
- Produce diffs and rule-linked notes.
- Never auto-accept high-risk voice, plot, or canon changes.

## Long-Context Strategy

The context packet is the main long-context defence. It carries:

- Stable canon and source policy.
- Character state, psychology, sensory signatures, voice matrices, injuries, possessions, secrets, and do-not-overuse lists.
- Knowledge state for characters and reader.
- Relationship state.
- Open plot threads and promise/payoff windows.
- Recent scene chain with goal, conflict, turn, outcome, and progression.
- Required scene turn and do-not-do list from the scene card.
- Style profile and recent memory events.
- Token budget.

Full-book review chunks files and reports aggregate rule counts rather than loading every chapter into one context window. Long novels should move from raw manuscript to scene summaries, chapter summaries, arc summaries, story bible, state ledgers, and target-specific context packets.
