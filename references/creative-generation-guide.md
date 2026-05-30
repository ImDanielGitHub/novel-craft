# Creative Generation Guide

Novel Craft should help an agent generate better story options before drafting. The deterministic analyzer can find friction, but the agent still has to choose and revise well.

## Principle

Do not let the first workable premise become the book. Generate a wide option pool, then choose.

The tool should aim high by default. "Breakout" is not a special mode; every fiction packet should ask for a strong first chapter, costly advantages, meaningful scene turns, chapter-end continuation, and a wider story engine.

The opening often works best when it shows the macro experience through micro-action. A first chapter does not need to announce the whole roadmap. It can show the smallest living unit of the story: one wound, one need, one rule, one cost, one choice, one room, one relationship, or one boundary. Treat examples as indicators and helpers, not hard limits.

A good story seed usually balances:

- familiar genre appeal
- fresh twist
- relatable desire
- clear pressure
- emotional wound
- concrete first scene
- long-form expansion engine
- readable language

## Commands

```bash
novel-craft creative methods
novel-craft creative atlas --json
novel-craft creative tropes --genre breakout-serial
novel-craft creative tropes --genre system-isekai
novel-craft creative tropes --genre nightmare-survival
novel-craft creative tropes --genre rational-magus
novel-craft creative tropes --genre beast-bond-progression
novel-craft creative tropes --genre vr-cultivation
novel-craft creative tropes --genre monster-evolution
novel-craft creative brief --idea "weak-to-strong system isekai" --avoid "accounting, spreadsheets, office comedy" --must-include "required fact" --must-avoid "forbidden claim"
novel-craft creative diagnose chapter.md
novel-craft creative novelty chapter.md
novel-craft creative trope-check chapter.md --genre system-isekai
novel-craft creative tournament --idea "weak-to-strong kingdom-building isekai" --count 8
novel-craft rules guide
novel-craft eval rubric --genre system-isekai
novel-craft eval sheet chapter.md
novel-craft eval story chapter.md --genre system-isekai --profile fast-webnovel
novel-craft eval gate chapter.md --must-include "required fact" --must-avoid "forbidden claim"
novel-craft eval compare old.md new.md
novel-craft eval reader-profiles
novel-craft eval reader-check chapter.md --profile breakout-serial
novel-craft eval voice-drift chapter01.md chapter02.md --character Mara
novel-craft eval feedback-add chapter01 --dimension hook_and_reader_grip --rating 4 --comment "Wanted chapter two."
novel-craft eval feedback-report
novel-craft eval calibrate-add sample.md --label liked --reason "simple hook with a sharp cost"
novel-craft eval calibrate-report
novel-craft eval reward-export old.md new.md --winner b --dimension hook_and_reader_grip
novel-craft matrix heatmap
```

## Creativity Methods

- Diverge then converge: make 12-20 ideas before choosing.
- Morphological matrix: mix protagonist, wound, world, system rule, power cost, social arena, and antagonist pressure.
- Trope plus twist: start familiar, then change one causal pressure point.
- Constraint remix: every power gets a cost, cooldown, social price, or blind spot.
- Analogical transfer: borrow structure from another domain without copying surface details.
- Inversion: flip the genre default.
- SCAMPER: substitute, combine, adapt, modify, put to another use, eliminate, reverse.
- TRIZ-style contradiction: make two necessary goals oppose each other.
- Self-refine: draft, critique, revise, and preserve why the revision is better.
- Tree-of-thought branching: explore multiple hooks, power systems, wounds, and first turns.
- Simulated critic debate: genre fan, craft editor, novelty critic, and readability editor attack the premise.
- Reading-level gate: make sentences easier without making ideas smaller.

## Story Atlas

Use `novel-craft creative atlas --json` when the prompt is thin, over-familiar, or too linear. It gives the agent:

- 50 genres
- 50 subgenres
- 50 tropes
- 50 sub-tropes
- a mixing protocol
- the always-on novel excellence standard

The agent should pick one genre, one subgenre, two tropes, and one sub-trope, then make the ingredients affect causality. A trope is useful only when it changes choices, world rules, social pressure, power cost, chapter structure, or the next expected payoff.

## Always-On Novel Excellence Standard

Every brief and tournament should check:

- banger first chapter
- visible reader draw
- familiar pleasure plus fresh causal twist
- power, bond, system, status, or secret with a cost
- scene goal, conflict, turn, consequence, and next pressure
- chapter change in power, knowledge, status, relationship, danger, open question, or territory
- wider story engine across mystery, relationship, status, power, threat, territory, and theme
- fair chapter-end continuation reason
- concrete human texture instead of abstract destiny
- revision loop before final judgement

## First-Chapter Micro-Scene

Research and craft guidance converge on a practical rule: story first, context second. The opening should set tone, character, conflict, curiosity, and reader expectation without stopping to explain the whole system or world.

For agents:

- Do not begin by naming the macro category inside the prose.
- For kingdom-building, show one door, meal, protected person, dispute, boundary, scarce resource, or room before naming kingdoms, domains, empires, rulers, citizens, laws, or upgrade ladders.
- For progression, show one painful use of the mechanic before listing ranks.
- For magic systems, show a rule biting someone before explaining theory.
- For politics, show one official, taboo, queue, punishment, bribe, border, permit, or public consequence before summarising history.
- Let the first chapter show the larger experience through a smaller scene.

Use `novel-craft eval story` to review the `opening_guidance` section after drafting. A warning does not prove the opening is bad; it means the agent should ask whether macro labels can move later or become action, dialogue, discovery, cost, or consequence.

## System-Isekai Trope Axes

Use tropes as mixable parts, not clichés to copy.

- Entry: wrong hero, weak local body, tutorial dungeon, sacrifice victim, doomed minor villain, prophecy extra.
- System: hidden-cost status screen, choice-grown skill tree, mislabelled class, unreliable quest log, consequence inventory.
- Weakness: zero combat stats, no direct violence, injured body, language barrier, low social rank, power only works for others.
- Growth: weak skill gains depth, support class becomes strategic core, crafting/logistics beat brute force, monster ecology as power.
- Arena: guild, frontier village, caravan, academy, dungeon town, shrine city, floating market, border fort.
- Pressure: rival reincarnator, monster wave timer, noble ownership, church heresy, hidden beginner trap, winter before food.
- Freshness: system rewards solved civic problems, quests come from needy people, levelling creates responsibility, stats are public, tutorial is a scam.

## Serial Grip Profiles

These profiles are derived from public ranking, synopsis, and review patterns. They are not instructions to imitate any title.

- `breakout-serial`: umbrella profile for hard opening wound, visible reader draw, costly power, serial-retention engine, and breakout-quality gates.
- `nightmare-survival`: dark progression with survival trials, mythic ruins, binding flaws, and trust under ownership pressure.
- `rational-magus`: western fantasy progression where magic has rules, power is studied, morality is complex, and side characters keep changing.
- `beast-bond-progression`: first-path beast taming, companion partnership, evolution mystery, and emotional stakes inside the progression loop.
- `vr-cultivation`: dual-world cultivation fantasy where game progress, real-world weakness, innocence, status, and secrecy trade consequences.
- `monster-evolution`: nonhuman body problem, resource grind, visible evolution loop, and data/prose balance.
- `high-drama-romance`: public rejection, secret identity, pregnancy or heir stakes, status repair, and accountability before reconciliation.

Use these profiles with `creative brief`, `creative tournament`, `creative trope-check`, and `eval rubric`. A strong contender should name its opening wound, familiar genre appeal, freshness twist, power cost, world-depth signal, and chapter-end continuation reason.

## Reading And Word Choice

The CLI should flag language that narrows audience too early:

- niche job mechanics before wonder appears
- long sentence tangles
- abstract labels without evidence
- too much system jargon before desire is clear
- fancy diction where a vivid common word would hit harder

Target for broad webnovel readability:

- mostly grade 6-8 surface grammar
- one main idea per paragraph beat
- short paragraphs around action, dialogue, and reveals
- system information revealed through pressured choices
- complexity spent on world consequences, not sentence knots

## Evaluation Workflow

Creative writing should not be judged by one total score. Use separate dimensions:

- hook and genre appeal
- readability and flow
- causal coherence
- character agency
- character consistency
- emotional depth
- conflict handling
- novelty and specificity
- voice and language
- progression and payoff
- costly power
- mystery and world depth
- serial retention

Use `novel-craft eval sheet` for one draft and `novel-craft eval compare` for revisions. Pairwise comparison is especially useful because it asks which version better serves the reader instead of pretending there is a single objective quality number.

## High-Value Gap Checks

Use these before revising:

- `novel-craft creative novelty`: finds generic phrase traps, concrete specificity signals, freshness signals, office/niche framing traps, and trope saturation risk. It is a lexical signal report, not a final originality verdict or quality score.
- `novel-craft eval story`: reviews an existing chapter or story file after drafting. It gives structured guidance without pass/fail gate language.
- `novel-craft eval gate`: combines constraints, lint, reader fit, opening guidance, and lexical novelty signals into a pass/warn/fail report when hard requirements need that status.
- `novel-craft creative trope-check`: shows which familiar trope axes are visible and whether the draft has cost, limitation, contradiction, or consequence attached.
- `novel-craft creative tournament`: creates a prompt pack for A/B premise generation, so the agent explores multiple hooks before drafting.
- `novel-craft eval reader-check`: checks reading-level fit for fast webnovel, adult progression, middle-grade clear, or literary dense profiles.
- `novel-craft eval voice-drift`: compares dialogue fingerprints across chapters and asks for a voice-matrix check.
- `novel-craft eval feedback-*`: stores beta-reader reactions as evidence instead of losing them in chat history.
- `novel-craft eval calibrate-*`: stores liked/disliked samples as local taste anchors for future comparisons.
- `novel-craft eval reward-*`: exports pairwise preference records for future reward/evaluator adapters.
- `novel-craft matrix heatmap`: shows which open loops and threads are getting hot, stale, or ready for payoff.

The key idea is not that every check is deterministic. The CLI supplies evidence and review questions; the agent still owns the creative judgement.

## High-Aim But Not Popularity-Guaranteeing

Novel Craft can help an agent aim at known reader-grip factors: clear English, memorable characters, world depth, theme fit, update-friendly chapter hooks, community-discussion hooks, clear reader draw, and adaptation-friendly images. It cannot guarantee popularity, reader votes, platform support, rights status, editor taste, or prize outcomes.
