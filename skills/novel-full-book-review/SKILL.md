---
name: novel-full-book-review
description: Use Novel Craft for whole-manuscript or long-arc audits covering continuity, repeated beats, open promises, pacing, progression cadence, and craft-rule aggregate reports.
---

# Novel Full Book Review

Use this skill when the user asks for a whole-book, full-manuscript, arc, continuity, pacing, repeated-beat, or long-context review.

## Workflow

1. Start in the story folder.
2. Ensure `.novel/` exists. If not, run `novel-craft init`.
3. Run the audit:

```bash
novel-craft full-book <manuscript-folder-or-file>
```

4. Build and audit the story matrix:

```bash
novel-craft matrix build
novel-craft matrix audit
```

5. Run focused long-form checks:

```bash
novel-craft audit repetition <manuscript-folder-or-file> --recent 20
novel-craft audit causality
```

6. Read `.novel/reports/full-book-audit.md` plus matrix/audit output.
7. Summarise findings by severity: canon contradictions, knowledge-state leaks, promise/payoff drift, repeated scene functions, therefore/because causality gaps, progression gaps, then line/scene polish.

## Review Posture

- Treat metrics as triage, not truth.
- Do not recommend global rewrites until plot/canon issues are understood.
- For high-risk voice, subtext, literary rhythm, or intentional summary, give author-choice notes rather than hard commands.
- Repetition is not only repeated words. Check repeated emotional endpoints, repeated scene shapes, repeated power dynamics, repeated locations, repeated endings, and repeated promise reminders with no new information.
- For long-context review, prefer scene cards, matrix rows, plot threads, and memory diffs over stuffing the whole manuscript into a single prompt.
