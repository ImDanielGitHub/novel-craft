---
name: novel-evaluation-review
description: Use Novel Craft to evaluate creative-writing drafts with multi-dimensional rubrics, evidence-based score sheets, and pairwise revision comparisons.
---

# Novel Evaluation Review

Use this skill when judging whether a chapter, opening, revision, or model output is actually better.

## Workflow

Create a rubric:

```bash
novel-craft eval rubric --genre system-isekai
```

Create a score sheet for one draft:

```bash
novel-craft eval sheet <draft.md> --genre system-isekai
```

Compare two versions:

```bash
novel-craft eval compare <old.md> <new.md> --genre system-isekai
```

Check reader fit:

```bash
novel-craft eval reader-check <draft.md> --profile fast-webnovel
```

Check novelty and trope freshness before revising:

```bash
novel-craft creative novelty <draft.md> --genre system-isekai
novel-craft creative trope-check <draft.md> --genre system-isekai
```

Compare character voice across files:

```bash
novel-craft eval voice-drift <chapter01.md> <chapter02.md> --character <Name>
```

Preserve real reader evidence:

```bash
novel-craft eval feedback-add <target> --dimension hook_and_promise --rating 4 --comment "<reader reaction>"
novel-craft eval feedback-report
```

Calibrate taste and export pairwise preferences:

```bash
novel-craft eval calibrate-add <sample.md> --label liked --reason "<why it worked>"
novel-craft eval calibrate-report
novel-craft eval reward-export <old.md> <new.md> --winner b --dimension novelty_and_specificity
```

## Review Rules

- Do not collapse creative writing into one score.
- Score dimensions independently: hook, readability, causality, agency, consistency, emotion, conflict, novelty, voice, and progression/payoff.
- Cite text evidence for every judgement.
- Prefer pairwise comparison for revisions: which version better serves the reader effect and why?
- Use deterministic metrics as leads, not final truth.
- Treat novelty, voice drift, and reader-level checks as approximation packets. The LLM still needs to judge whether the detected pattern serves the intended effect.
- Use stored beta feedback and taste calibration as evidence, not as automatic orders.
