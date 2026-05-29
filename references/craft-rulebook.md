# Novel Craft Rulebook

The executable rule cards live in `rules/default.yml`. This reference explains how to use them.

## Severity

- `S1`: informational polish note.
- `S2`: warning; likely worth checking.
- `S3`: strong warning; likely affects reader experience.
- `S4`: human review required; do not auto-rewrite blindly.

## Automation Risk

- `R1`: safe to lint and usually safe to suggest or rewrite conservatively.
- `R2`: safe to suggest; rewrite needs review.
- `R3`: human review only.

## Core Doctrine

The tool should not enforce craft rules as absolutes. It should ask: what is the scene trying to do?

- Show high-stakes turns, betrayals, confrontations, discoveries, irreversible choices, and emotional pivots.
- Tell for transitions, recap, montage, logistical compression, reflective voice, and deliberate narrative distance.
- Prefer active voice in hot action, but allow passive when the actor is hidden, unknown, or less important than the object.
- Remove filter words when they add distance without purpose, but keep them when uncertainty or perception itself matters.
- Replace emotion labels with embodied evidence in hot scenes, but allow direct emotional naming in reflective or intentionally plain voice.

The newer rule cards are effect-first. A finding should be classified before revision:

- `likely_mistake`: probably accidental and probably weakening the intended reader effect.
- `possibly_intentional`: could be useful, but the scene needs author or agent judgement.
- `clearly_functional`: likely serving a valid craft effect and should usually be kept.

Example: passive voice is a likely mistake in `The blade was swung by Kael` because the actor and physical action matter. It may be functional in `Mara was dragged through the ash` because the sentence focuses helplessness and victimhood. It may also be useful for mystery (`The door had been unlocked`) or dehumanising bureaucracy (`The prisoner was processed, catalogued, and transferred`).

## LLM-First Rule Use

Novel Craft's deterministic checks are approximations. They are meant to guide the user's LLM, not replace it. A regex can find a possible passive construction, filter word, repeated opening, or long paragraph; it cannot know the author's intended effect by itself.

Every rule guide entry should give the LLM:

- the reader/story effect being protected
- what the detector approximates
- questions to ask before revising
- a problem example
- a counterexample or keep-case
- rewrite strategies that preserve voice

Use:

```bash
novel-craft rules guide
```

The LLM should follow this decision path:

1. Read the scene card and intended effect.
2. Treat detected issues as leads.
3. Ask the rule's LLM questions.
4. Compare the flagged passage to the example and counterexample.
5. Classify the pattern as `likely_mistake`, `possibly_intentional`, or `clearly_functional`.
6. Revise only if the change improves the intended effect without flattening voice, genre, rhythm, or character.

## Creativity Before Drafting

If the premise feels narrow, generic, or unrelatable, do not line-edit it first. Run a creativity pass:

```bash
novel-craft creative brief --idea "<seed>" --avoid "<stale frame>"
novel-craft creative tropes --genre system-isekai
novel-craft creative methods
```

The LLM should generate several alternatives before committing. It should check:

- Does the first page offer wonder, danger, injustice, desire, or a concrete problem?
- Does the first chapter show the smallest working unit of the premise before leaning on the whole roadmap, if that would improve reader grip?
- Is the protagonist's situation broadly relatable?
- Is the frame too niche, such as extended office/accounting mechanics before fantasy appears?
- Can this premise sustain 50+ chapters through growth, costs, rivals, locations, promises, and relationship turns?
- Is the language easy enough to read while the story ideas stay rich?

## Expanded Craft Modules

The executable rulebook now covers these modules:

- **Agency:** track whether the protagonist acts, chooses, and causes consequences, or only observes, feels, and waits.
- **Narrative distance:** classify filters as redundant distance or functional perception.
- **Motivation-reaction order:** prefer external stimulus, internal reaction, physical response, then speech/action in close POV.
- **Scene goal/conflict/turn:** every scene should usually contain a want, resistance, and changed state.
- **Promises and payoffs:** track hooks, mysteries, powers, debts, wounds, threats, prophecies, and relationship tensions as open loops.
- **Knowledge state:** record who knows what so characters cannot use information before learning it.
- **Emotional progression:** prevent repeated emotional endpoints unless cost, escalation, or motif is deliberate.
- **Dialogue power:** identify leverage, withholding, lies, tests, interruptions, and power shifts.
- **Subtext:** reduce direct motive/emotion statements when tension should carry hidden meaning.
- **Sensory texture:** store visual motifs, scent, sound, touch, dress, body language, stress behaviour, and overuse warnings.
- **Sentence rhythm:** track length variance, fragments, repeated openings, and monotony clusters.
- **Paragraphing as camera:** use paragraph breaks for emphasis, suspense, mobile readability, and attention control.
- **Exposition function:** exposition should attach to conflict, decision, discovery, danger, contradiction, object, ritual, or place.
- **Opening micro-promise:** the first chapter often works best when it seeds the macro premise through a small dramatic unit before leaning on future scale, category, rank ladders, or world history.
- **Functional repetition:** distinguish lexical, syntactic, imagery, emotional, and story-function repetition.
- **Therefore/because plotting:** prefer causal chains over additive `and then` scene lists.

## Webnovel/Progression Defaults

For webnovel and progression fantasy, add checks that general novel tools often miss:

- Early chapter hook.
- Micro-action before macro-roadmap.
- End-of-chapter forward desire or question.
- Recap bloat.
- Mobile paragraph length.
- Visible progression delta.
- Earned upgrades and explicit cost.
- Rival competence.
- Power limitations.
- Repeated training beats.
- Promise/payoff age.
- Repeated training scenes with no new cost, limitation, rival pressure, capability change, or strategic discovery.
- Kingdom-building progression signals such as authority, law, guard capacity, roads, water, rations, tax, trade, courts, boundaries, public trust, and logistics.
- Open-loop reminders that repeat without adding new evidence or changed stakes.

## Dynamic Story State

Novel Craft treats character sheets and scene records as live state, not static notes.

Character sheets should include:

- surface facts: age, appearance, clothing, injuries, possessions
- sensory signature: scent, sound, touch, visual motifs
- psychology: public mask, private self, wound, false belief, conscious want, unconscious need, fear, shame, contradiction
- behaviour: default body language, stress behaviour, speech style, taboo phrases, overuse list
- relationships: trust, debt, attraction, resentment, last change, hidden dynamic
- knowledge: facts known, facts not known, secrets, lies, false beliefs

Scene cards should include:

- POV, location, time, goal, conflict, stakes, turn, consequence
- entry and exit state
- open plot threads, promises opened, promises paid
- emotional beat, scene shape, progression delta
- do-not-repeat items
- causal link to previous and next scenes

Plot threads are state machines. Each thread should record introduced scene, current stage, appearances, last progression, expected payoff window, and risk notes such as `Do not repeat the ring clue again without new information`.

## Workflow

Use separate modes and passes:

1. Architect mode creates or updates the scene card.
2. Context mode builds the packet.
3. Draft mode writes from the packet.
4. Critic mode reviews scene function only.
5. Character mode reviews motivation, voice, and emotional causality.
6. Dialogue mode reviews power, subtext, exposition, and voice drift.
7. Prose mode reviews passive voice, filters, rhythm, imagery, paragraphing, and sensory load.
8. Continuity mode checks facts, timeline, objects, injuries, and knowledge state.
9. Revision mode applies one approved pass at a time.
10. Memory mode extracts and commits state changes.

## Author Control

Novel Craft can identify patterns. The author still decides whether the pattern is a problem.

High-risk areas:

- Subtext.
- Literary rhythm.
- Voice.
- Theme.
- Intentional summary.
- Genre expectation tradeoffs.
- Whole-plot restructuring.
