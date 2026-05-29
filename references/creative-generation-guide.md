# Creative Generation Guide

Novel Craft should help the user's LLM generate better story options before drafting. The deterministic analyzer can find friction, but the LLM is the main creative engine.

## Principle

Do not let the first workable premise become the book. Generate a wide option pool, then choose.

A good story seed usually balances:

- familiar genre promise
- fresh twist
- relatable desire
- clear pressure
- emotional wound
- concrete first scene
- long-form expansion engine
- readable language

## Commands

```bash
novel creative methods
novel creative tropes --genre system-isekai
novel creative brief --idea "weak-to-strong system isekai" --avoid "accounting, spreadsheets, office comedy"
novel creative diagnose chapter.md
novel creative novelty chapter.md --genre system-isekai
novel creative trope-check chapter.md --genre system-isekai
novel creative tournament --idea "weak-to-strong kingdom-building isekai" --count 8
novel rules guide
novel eval rubric --genre system-isekai
novel eval sheet chapter.md
novel eval compare old.md new.md
novel eval reader-profiles
novel eval reader-check chapter.md --profile fast-webnovel
novel eval voice-drift chapter01.md chapter02.md --character Mara
novel eval feedback-add chapter01 --dimension hook_and_promise --rating 4 --comment "Wanted chapter two."
novel eval feedback-report
novel eval calibrate-add sample.md --label liked --reason "simple hook with a sharp cost"
novel eval calibrate-report
novel eval reward-export old.md new.md --winner b --dimension hook_and_promise
novel matrix heatmap
```

## LLM Creativity Methods

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

## System-Isekai Trope Axes

Use tropes as mixable parts, not clichés to copy.

- Entry: wrong hero, weak local body, tutorial dungeon, sacrifice victim, doomed minor villain, prophecy extra.
- System: hidden-cost status screen, choice-grown skill tree, mislabelled class, unreliable quest log, memory/debt inventory.
- Weakness: zero combat stats, no direct violence, injured body, language barrier, low social rank, power only works for others.
- Growth: weak skill gains depth, support class becomes strategic core, crafting/logistics beat brute force, monster ecology as power.
- Arena: guild, frontier village, caravan, academy, dungeon town, shrine city, floating market, border fort.
- Pressure: rival reincarnator, monster wave timer, noble ownership, church heresy, hidden beginner trap, winter before food.
- Freshness: system rewards promises, quests come from needy people, leveling creates responsibility, stats are public, tutorial is a scam.

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

- hook and genre promise
- readability and flow
- causal coherence
- character agency
- character consistency
- emotional depth
- conflict handling
- novelty and specificity
- voice and language
- progression and payoff

Use `novel-craft eval sheet` for one draft and `novel-craft eval compare` for revisions. Pairwise comparison is especially useful because it asks which version better serves the reader instead of pretending there is a single objective quality number.

## High-Value Gap Checks

Use these before asking the LLM to revise:

- `novel-craft creative novelty`: finds generic phrase traps, concrete specificity signals, freshness signals, office/niche framing traps, and trope saturation risk. It is a lexical proxy, not a final originality verdict.
- `novel-craft creative trope-check`: shows which familiar trope axes are visible and whether the draft has cost, limitation, contradiction, or consequence attached.
- `novel-craft creative tournament`: creates a prompt pack for A/B premise generation, so the agent explores multiple hooks before drafting.
- `novel-craft eval reader-check`: checks reading-level fit for fast webnovel, adult progression, middle-grade clear, or literary dense profiles.
- `novel-craft eval voice-drift`: compares dialogue fingerprints across chapters and asks the LLM to check voice matrix fit.
- `novel-craft eval feedback-*`: stores beta-reader reactions as evidence instead of losing them in chat history.
- `novel-craft eval calibrate-*`: stores liked/disliked samples as local taste anchors for future LLM judges.
- `novel-craft eval reward-*`: exports pairwise preference records for future reward/evaluator adapters.
- `novel-craft matrix heatmap`: shows which promises and threads are getting hot, stale, or ready for payoff.

The key idea is not that every check is deterministic. The CLI supplies evidence and review questions; the user's LLM remains the creative judge.
