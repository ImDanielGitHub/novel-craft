# Novel Craft PRD

## Goal

Novel Craft helps writers and Codex agents draft, review, revise, and continue writing without losing purpose, reader effect, voice, clarity, structure, or long-context state.

The v1 product is a local-first CLI wrapped by Codex skills. Its first public domain pack is long-form fiction, especially webnovel/progression fantasy.

That domain is a useful starting point because chapter cadence, hooks, power growth, recap control, kingdom-building logistics, and continuity all matter at once.

The product is not merely a grammar checker. It is a stateful writing-quality engine. In the fiction pack, rule cards describe reader effects, legitimate rule-breaking, character memory, plot progression, knowledge state, repetition risks, and long-context packet construction.

Future packs should reuse the same machinery for copywriting, essays, reports, proposals, emails, product writing, technical docs, and other writing jobs.

## Product Thesis

Good writing is not one universal voice. Good writing fits the reader, purpose, constraints, evidence, and desired effect.

Novel Craft should therefore encode effect-first rule cards, purpose profiles, examples, counterexamples, rubrics, comparison prompts, and revision passes. Those tools should guide any model toward better writing without treating metrics as taste.

## Users

- A writer drafting serial chapters.
- A Codex agent helping with next-chapter planning, line editing, continuity sync, or whole-book review.
- A future maintainer extending craft rule cards, genre profiles, purpose profiles, domain packs, or LLM rewrite adapters.
- A future user improving copy, reports, emails, proposals, essays, product copy, or technical docs with the same rule-card and prompt-packet engine.

## Non-Goals

- Do not claim objective literary quality.
- Do not bulk-scrape or train on hosted copyrighted fiction.
- Do not silently rewrite a whole book without review.
- Do not replace author judgement on voice, subtext, rhythm, theme, or intentional rule-breaking.
- Do not force every writing purpose into a novel-shaped workflow.
- Do not claim one generic "humanized" style is good for every audience or task.

## Primary Workflow

1. `novel-craft init` creates `.novel/` project state.
2. `novel-craft character add/update` records durable facts; advanced character sheets can also hold surface, sensory signature, psychology, behaviour, relationships, continuity, voice matrix, current state, and do-not-overuse gestures.
3. `novel-craft scene create` records the target scene's goal, conflict, turn, stakes, exit state, threads, promises, and repetition warnings.
4. `novel-craft plot thread` maintains state-machine records for mysteries, powers, debts, rivalries, and long-running promises.
5. `novel-craft matrix build/audit` maintains story progression and checks repetition, promise load, and therefore/because causality.
6. `novel-craft matrix heatmap` shows hot, stale, or unpaid promises and threads.
7. `novel-craft creative brief/tournament/novelty/trope-check` widens premise options, audits genericity, and gives the LLM creativity questions before drafting.
8. `novel-craft eval reader-check/voice-drift/feedback/calibrate/reward` checks reader fit, voice consistency, beta feedback, taste anchors, and pairwise preference records.
9. `novel-craft context build` creates a layered context packet for long-context drafting.
10. `novel-craft draft` prepares a drafting prompt from the packet.
11. `novel-craft analyse` and `novel-craft lint line|scene|plot` run deterministic and hybrid craft checks.
12. `novel-craft review --rubric` creates a mode-specific review for prose, scene, character, dialogue, continuity, or the full stack.
13. `novel-craft revise --pass ...` creates targeted revision reports or applies safe line fixes.
14. `novel-craft audit continuity/repetition/causality` runs focused long-form checks.
15. `novel-craft memory extract` creates a reviewable memory diff.
16. `novel-craft memory commit` commits approved facts, promises, and state changes.
17. `novel-craft full-book` audits a folder or manuscript for repeated issues, long-form continuity, and promise drift.

## Source Policy

Novel Craft may use hosted fiction as browser-observed reference for high-level patterns only. Persistent corpora should be user-owned, public-domain, licensed, or explicitly approved. See `references/source-policy.md`.

## Acceptance Criteria

- A user can create a `.novel` project with config, rule cards, plot matrix, reports folder, character folder, and SQLite memory.
- The public docs describe the long-term product as a general writing-quality engine, with the fiction pack clearly identified as the first domain pack rather than the full scope.
- A user can add/update character sheets with appearance, dress, smell, motives, powers, wounds, secrets, voice, knowledge, and advanced dynamic-state fields.
- A user can create scene cards and plot-thread state machines.
- A user can run high-variance creative premise generation before drafting, including trope matrices, tournament prompts, novelty/specificity checks, and trope-saturation checks.
- A user can check a draft against reader-level profiles, storing warnings about readability, paragraph length, dialogue ratio, and profile-specific review focus.
- A user can compare dialogue voice fingerprints across chapters and hand the output to the LLM for voice-matrix review.
- A user can store beta-reader feedback and liked/disliked taste samples instead of losing reader evidence in chat history.
- A user can export pairwise preference records for future evaluator or reward-model adapters without claiming automatic model training.
- A user can produce a promise heat map from the story matrix so the writer can escalate, pay off, reframe, or intentionally defer open loops.
- A user can lint a draft at line, scene, and plot levels.
- A user can run full analysis with agency, narrative distance, motivation-reaction order, scene function, dialogue power/subtext, rhythm, paragraphing, exposition, and repetition signals.
- A user can generate a context packet that includes project config, characters, knowledge state, relationship state, scene card, recent scenes, open promises, plot threads, style profile, and memory events.
- A user can extract and approve memory diffs instead of silently mutating canon.
- A user can run a full-book audit over `.md` or `.txt` files.
- Codex skills can call the CLI with explicit commands and workflow boundaries.
- Plugin validation and CLI smoke tests pass.
