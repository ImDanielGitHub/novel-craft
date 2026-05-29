---
name: novel-rulebook-review
description: Use Novel Craft to inspect, explain, extend, or tune machine-readable fiction craft rule cards and evaluation fixtures for the CLI.
---

# Novel Rulebook Review

Use this skill when the user wants to add writing rules, tune thresholds, review the rulebook, add fixtures, or change how the CLI judges prose.

## Source Files

- `/Users/da393/plugins/novel-craft/rules/default.yml`
- `/Users/da393/plugins/novel-craft/references/craft-rulebook.md`
- `/Users/da393/plugins/novel-craft/evals/fixtures/`

## Workflow

1. Read the relevant rule card first.
2. Add or edit the rule card with:
   - `id`
   - `name`
   - `level`
   - `description`
   - `effect`
   - `priority`
   - `applies_when`
   - `detect`
   - `usual_problem`
   - `good_when`
   - `bad_when`
   - `break_when`
   - `severity`
   - `automation_risk`
   - `rewrite_strategy`
   - `rewrite_strategies`
   - `llm_guidance`
   - `llm_questions`
   - `deterministic_limitations`
   - `examples`
   - `counterexamples`
3. Add at least one fixture that should trigger the rule and, when possible, one fixture that should not.
4. Run:

```bash
novel-craft rules list
novel-craft rules guide
novel-craft lint line <fixture.md>
novel-craft lint scene <fixture.md>
novel-craft lint plot <fixture.md>
novel-craft analyse <fixture.md>
```

For existing projects that copied older rules:

```bash
novel-craft rules refresh
```

## Rule Philosophy

Rules need exceptions. Good fiction often breaks advice deliberately. The CLI should flag review-worthy patterns, not pretend every detection is a mistake.

Every rule should be framed as a reader or story effect. Avoid absolute rules such as "never use passive voice" or "always show." Instead encode:

- the usual problem
- when it is good
- when it is bad
- when to break it
- an example
- a counterexample or keep-case
- LLM questions that force context-aware judgement
- how to revise without flattening voice
- whether the finding should classify as likely mistake, possibly intentional, or clearly functional
