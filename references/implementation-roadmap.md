# Novel Craft Implementation Roadmap

## Phase 1: Spec And Rulebook

Done in v0.1:

- PRD, SDS, source policy, roadmap.
- Machine-readable effect-first rule cards with break conditions, good/bad contexts, and classification doctrine.
- Evaluation fixtures.
- Plugin metadata and Codex skills.

## Phase 2: Deterministic CLI

Done in v0.1:

- `novel-craft start`
- `novel-craft init`
- `novel-craft doctor`
- `novel-craft character add/update`
- `novel-craft scene create/show/from-text`
- `novel-craft context build`
- `novel-craft draft`
- `novel-craft next`
- `novel-craft analyse`
- `novel-craft lint line|scene|plot`
- `novel-craft review`
- `novel-craft revise`
- `novel-craft diff`
- `novel-craft memory extract/commit`
- `novel-craft full-book`

Added in the fiction-engineering pass:

- `novel-craft scene create/show/from-text`
- `novel-craft plot thread`
- `novel-craft context build`
- `novel-craft draft`
- `novel-craft analyse`
- `novel-craft review --rubric`
- `novel-craft matrix build/audit`
- `novel-craft audit continuity/repetition/causality`
- `novel-craft memory extract/commit`
- dynamic `.novel/state/*.yml`
- `.novel/scene-cards/*.yml`
- `.novel/plot-threads/*.yml`
- `.novel/pending-memory/*.diff.yml`

Added in the creative-evaluation gap pass:

- `novel-craft creative novelty`
- `novel-craft creative trope-check`
- `novel-craft creative tournament`
- `novel-craft eval reader-profiles`
- `novel-craft eval reader-check`
- `novel-craft eval voice-drift`
- `novel-craft eval feedback-add/feedback-report`
- `novel-craft eval calibrate-add/calibrate-report`
- `novel-craft eval reward-export/reward-report`
- `novel-craft matrix heatmap`
- `.novel/state/beta-feedback.yml`
- `.novel/state/taste-profile.yml`
- `.novel/evals/reward-pairs.jsonl`

## Phase 3: Model-Neutral Skill Packets

Next:

- Keep the core CLI model-neutral and provider-free.
- Expand skills that call the CLI, then hand prompt packets to the user's chosen model.
- Add explicit author approval gates for high-risk voice, plot, canon, or memory changes.
- Add skill modes: architect, draft, critic, revision, continuity, line-editor, memory.
- Use feedback/taste profiles as optional context inside generated prompt packets.
- Use reward-pair exports as the boundary for external evaluator experiments.

## Phase 4: Better NLP

Next:

- Add dependency-based passive voice and stimulus/reaction checks.
- Add semantic repeated-beat comparison.
- Keep semantic/embedding adapters optional and outside the default install.
- Add stronger knowledge-state contradiction extraction.
- Add semantic promise/payoff age and reminder-without-progress detection.
- Compare character voice drift checks against voice matrices.

## Phase 5: Evaluation And Tuning

Next:

- Expand fixtures to 50+ examples.
- Add expected rule IDs per fixture.
- Add regression tests for false positives and intentional rule-breaking.
- Tune thresholds per genre profile.
- Add long-context synthetic 100-chapter audit fixture.
