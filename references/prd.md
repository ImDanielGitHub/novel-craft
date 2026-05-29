# Novel Craft PRD

## Goal

Novel Craft helps AI agents help writers draft, review, revise, and continue fiction without losing purpose, reader effect, voice, clarity, structure, or long-context state.

The current product is a local-first CLI with bundled agent skills. Its public domain pack is long-form fiction, especially webnovel and progression fantasy.

That domain is a useful starting point because chapter cadence, hooks, power growth, recap control, kingdom-building logistics, and continuity all matter at once.

The product is not merely a grammar checker. It is a stateful writing-quality engine. In the fiction pack, rule cards describe reader effects, legitimate rule-breaking, character memory, plot progression, knowledge state, repetition risks, and long-context packet construction.

Supporting prose work, such as names, docs, and release notes, is handled by the small `novel-craft-writing-support` skill. Broader non-fiction domain packs are outside the current release.

Public serial-fiction research is folded in as abstract profiles such as `breakout-serial`, `nightmare-survival`, `rational-magus`, `beast-bond-progression`, `vr-cultivation`, `monster-evolution`, and `high-drama-romance`. These profiles encode reader-grip and serial-retention signals without copying title-specific text or claiming guaranteed popularity.

The broad `creative atlas` supplies 50 genres, 50 subgenres, 50 tropes, and 50 sub-tropes so agents can mix story ingredients before narrowing. The high bar is global: every fiction packet should aim for a strong first chapter, costly advantages, chapter structure, wider story engine, and fair continuation pressure.

Opening guidance is micro before macro when it improves reader grip: agents should usually dramatise the smallest working unit of the premise before leaning on the full roadmap. This is especially important for systems, kingdom-building, progression ladders, and dense worldbuilding, but examples are indicators rather than limits.

## Product Thesis

Good writing is not one universal voice. Good writing fits the reader, purpose, constraints, evidence, and desired effect.

Novel Craft should therefore encode effect-first rule cards, purpose profiles, examples, counterexamples, rubrics, comparison prompts, and revision passes. Those tools should guide any model toward better writing without treating metrics as taste.

## Users

- A writer drafting serial chapters.
- An AI agent helping with next-chapter planning, line editing, continuity sync, or whole-book review.
- A maintainer extending craft rule cards, genre profiles, purpose profiles, or LLM-facing packet generation.

## Non-Goals

- Do not claim objective literary quality.
- Do not bulk-scrape or train on hosted copyrighted fiction.
- Do not silently rewrite a whole book without review.
- Do not replace author judgement on voice, subtext, rhythm, theme, or intentional rule-breaking.
- Do not force every writing purpose into the public release.
- Do not claim one generic "humanized" style is good for every audience or task.
- Do not claim Novel Craft can guarantee an award, ranking, contract, adaptation, popularity, or reader response.

## Primary Workflow

1. `novel-craft agent plan` turns a user prompt into an agent-facing chapter workflow with contender generation, comparison, chapter cards, and post-write checks.
2. `novel-craft init` creates `.novel/` project state.
3. `novel-craft character add/update` records durable facts; advanced character sheets can also hold surface, sensory signature, psychology, behaviour, relationships, continuity, voice matrix, current state, and do-not-overuse gestures.
4. `novel-craft scene create` records the target scene's goal, conflict, turn, stakes, exit state, threads, promises, and repetition warnings.
5. `novel-craft plot thread` maintains state-machine records for mysteries, powers, debts, rivalries, and long-running promises.
6. `novel-craft matrix build/audit` maintains story progression and checks repetition, promise load, and therefore/because causality.
7. `novel-craft matrix heatmap` shows hot, stale, or unpaid promises and threads.
8. `novel-craft creative atlas/brief/tournament/novelty/trope-check` widens premise options, audits genericity, and gives the LLM creativity questions before drafting.
9. `novel-craft eval chapter/story/gate/compare/reader-check/voice-drift/feedback/calibrate/reward` checks drafted Markdown, hard constraints, reader fit, voice consistency, beta feedback, taste anchors, and pairwise preference records.
10. `novel-craft context build` creates a layered context packet for long-context drafting.
11. `novel-craft draft` prepares a drafting prompt from the packet.
12. `novel-craft analyse` and `novel-craft lint line|scene|plot` run deterministic and hybrid craft checks.
13. `novel-craft review --rubric` creates a mode-specific review for prose, scene, character, dialogue, continuity, or the full stack.
14. `novel-craft revise --pass ...` creates targeted revision reports or applies safe line fixes.
15. `novel-craft audit continuity/repetition/causality` runs focused long-form checks.
16. `novel-craft memory extract` creates a reviewable memory diff.
17. `novel-craft memory commit` commits approved facts, promises, and state changes.
18. `novel-craft full-book` audits a folder or manuscript for repeated issues, long-form continuity, and promise drift.

## Source Policy

Novel Craft may use hosted fiction as browser-observed reference for high-level patterns only. Persistent corpora should be user-owned, public-domain, licensed, or explicitly approved. See `references/source-policy.md`.

## Acceptance Criteria

- A user can create a `.novel` project with config, rule cards, plot matrix, reports folder, character folder, and SQLite memory.
- The public docs describe Novel Craft as a fiction-first agent CLI, with `novel-craft-writing-support` clearly scoped to supporting names, docs, release notes, and claim checks.
- A user can add/update character sheets with appearance, dress, smell, motives, powers, wounds, secrets, voice, knowledge, and advanced dynamic-state fields.
- A user can create scene cards and plot-thread state machines.
- A user can run high-variance creative premise generation before drafting, including a broad story atlas, trope matrices, tournament prompts, novelty/specificity checks, and trope-saturation checks.
- Every creative brief and tournament packet carries the always-on novel excellence standard: first-chapter pull, reader promise, costly advantage, scene turns, chapter structure, wider story engine, and fair continuation pressure.
- A user can run `eval gate` and see opening-promise warnings when a first chapter front-loads macro-scale labels before scene-level action, choice, cost, or consequence.
- A user can run `agent plan --idea "<prompt>" --chapters 1 --json` and receive task facts, missing story questions, contender rules, chapter cards, drafting instructions, revision loop, and post-write commands.
- A user can run `eval chapter <draft.md>` or `eval story <draft.md>` after writing an existing Markdown file and receive structured review guidance without pass/fail gate language.
- A user can use reader-profile matrices to test opening wound, reader promise, costly power, world-depth signal, and serial-retention hook before drafting.
- A user can check a draft against reader-level profiles, storing warnings about readability, paragraph length, dialogue ratio, and profile-specific review focus.
- A user can compare dialogue voice fingerprints across chapters and hand the output to the LLM for voice-matrix review.
- A user can store beta-reader feedback and liked/disliked taste samples instead of losing reader evidence in chat history.
- A user can export pairwise preference records for external evaluator experiments without claiming automatic model training.
- A user can produce a promise heat map from the story matrix so the writer can escalate, pay off, reframe, or intentionally defer open loops.
- A user can lint a draft at line, scene, and plot levels.
- A user can run full analysis with agency, narrative distance, motivation-reaction order, scene function, dialogue power/subtext, rhythm, paragraphing, exposition, and repetition signals.
- A user can generate a context packet that includes project config, characters, knowledge state, relationship state, scene card, recent scenes, open promises, plot threads, style profile, and memory events.
- A user can extract and approve memory diffs instead of silently mutating canon.
- A user can run a full-book audit over `.md` or `.txt` files.
- Bundled skills can call the CLI with explicit commands and workflow boundaries.
- Plugin validation and CLI smoke tests pass.
