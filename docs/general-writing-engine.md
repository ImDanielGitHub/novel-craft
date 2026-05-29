# General Writing Engine Direction

Novel Craft starts with novels. Its deeper goal is broader: help LLMs produce better, more human-sounding writing for many purposes.

It does that by giving the model explicit writing rules, examples, counterexamples, purpose profiles, reader-effect rubrics, and workflow skills.

The tool should not become a model wrapper. It should stay as a local, model-neutral CLI plus skills that produce packets any model can use.

## Product Thesis

Good writing is not one style. It is writing that fits a purpose, a reader, a context, and a desired effect.

For fiction, that means reader attention, emotional causality, voice, rhythm, scene turns, continuity, and promise/payoff.

For other writing, the same engine should guide different effects:

- copywriting: desire, trust, proof, urgency, objection handling, offer clarity.
- essays and articles: thesis, structure, argument, evidence, transitions, originality.
- reports: decision support, source traceability, risk language, skim structure, actionability.
- proposals: buyer pain, scope clarity, proof, pricing logic, next action.
- email and outreach: tone, relationship, ask clarity, concision, reply likelihood.
- product and UX writing: task clarity, labels, error states, empty states, accessibility.
- technical docs: sequence, prerequisites, correctness, examples, edge cases, maintenance.
- social and content writing: hook, pacing, specificity, shareability, voice, format fit.

The core promise is simple: before an agent drafts, reviews, compares, or revises, the CLI tells it what to look for.

## Why Fiction First

Long-form fiction is a strong first domain because it stresses many writing capabilities at once:

- voice and point of view.
- rhythm and word choice.
- reader curiosity and emotional movement.
- scene and chapter structure.
- continuity over long context windows.
- repeated pattern detection.
- character state and knowledge state.
- promise, payoff, escalation, and surprise.

If the engine can manage a long novel without flattening voice, repeating beats, breaking continuity, or losing reader momentum, it has useful primitives. Those primitives can then support other writing jobs.

## Generalized Primitives

Future work should keep the engine split into reusable primitives and domain packs.

Core primitives:

- rule cards: effect, failure mode, break conditions, examples, counterexamples, review questions.
- purpose profiles: audience, desired effect, success criteria, failure modes, tone, evidence needs.
- style profiles: diction, sentence rhythm, formality, warmth, directness, taboo phrases, examples.
- context packets: relevant facts, audience, goal, constraints, source notes, examples, review rubric.
- deterministic checks: rough measurable signals such as density, repetition, readability, structure, claims.
- LLM review prompts: guided questions that turn metrics into judgement.
- revision passes: one focused pass at a time, such as clarity, voice, evidence, persuasion, structure.
- comparison reports: pairwise A/B review so models can choose the draft that better serves the reader.
- feedback memory: human reactions stored as evidence, not lost inside chat history.

Domain packs:

- `novel`: fiction, webnovel, serial, progression, character, plot, continuity.
- `copy`: landing pages, ads, sales pages, product positioning, offers.
- `essay`: thesis, argument, evidence, transitions, reader contract.
- `report`: claims, source chain, decision support, risk and recommendation language.
- `email`: outreach, negotiation, follow-up, support, internal updates.
- `proposal`: scope, buyer pain, proof, pricing, milestones, next steps.
- `product`: UX copy, onboarding, empty states, error states, settings, notifications.
- `docs`: tutorials, API docs, runbooks, troubleshooting, release notes.

## Purpose Profile Shape

Future profiles can look like this:

```yaml
id: proposal_review
domain: proposal
audience: business buyer deciding whether to approve work
job_to_be_done: understand the value, scope, proof, cost, risk, and next step
desired_effects:
  - trust
  - clarity
  - decision confidence
  - low friction to reply
failure_modes:
  - vague scope
  - inflated claims
  - weak proof
  - unclear pricing logic
  - no concrete next action
rule_groups:
  - claim_evidence_fit
  - audience_pain
  - offer_clarity
  - tone_pressure
  - skim_structure
review_passes:
  - structure
  - evidence
  - language
  - risk
  - final_action
```

This uses the same pattern as fiction scene cards and rule cards, but points it at a different writing job.

## Rule Card Standard

Every rule should help the LLM judge intent, not blindly replace text.

Each rule should include:

- what reader effect it protects.
- what to check.
- when the pattern usually hurts the writing.
- when it is useful or intentional.
- an example.
- a counterexample.
- questions for the LLM to ask before revising.
- deterministic limitations, if the CLI can only approximate the signal.
- a targeted rewrite strategy.

The important distinction is:

- weak rule: "Never use passive voice."
- strong rule: "Passive voice can reduce agency. It may still be right when the actor is unknown, hidden, or irrelevant. It may also be right when the sentence should focus on helplessness, institutional coldness, or victim impact."

That standard should apply across every domain pack.

## General Workflow

The long-term workflow should be:

1. Choose or infer a purpose profile.
2. Build a context packet with audience, goal, constraints, sources, examples, and style.
3. Generate or improve a draft using the packet.
4. Run deterministic checks to surface likely issues.
5. Create an LLM review packet grounded in rule cards and examples.
6. Compare alternatives when possible.
7. Revise one dimension at a time.
8. Store human feedback, liked samples, disliked samples, and recurring style preferences.
9. Reuse those preferences in future packets.

For beginners, the CLI should choose strong defaults. For advanced writers, it should expose more dials without making the workflow heavier by default.

## Guardrails

General writing support should improve quality without turning into spam, deception, or source laundering.

The engine should:

- preserve source policy and copyright boundaries.
- keep claims tied to evidence in reports, proposals, essays, and technical docs.
- avoid impersonating a living writer or brand without permission.
- distinguish polish from truth.
- keep model calls outside the v1 CLI boundary.
- keep human approval gates for high-risk claims, voice changes, and publishing steps.

## Roadmap Implication

The current `.novel/` project state is acceptable for the first fiction pack. A future major version can add a generalized project layer, such as `.writing/`.

That should wait until the domain-pack abstraction is clear.

Until then, public docs should describe Novel Craft as:

> A model-neutral writing-quality CLI, starting with long-form fiction.

That keeps the product focused today without closing off the bigger ambition.
