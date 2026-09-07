# Novel Craft 0.2: architecture, scope and migration

## Product boundary

The writer controls intent and taste. The writing agent reads, invents and judges. Novel Craft makes state, context, bounded operations and recovery dependable. A useful output is an improved chapter and inspectable history, not a better-looking metric.

Starting from scratch and revising existing work share the same state and approval path. Outlining is available through the generation pipeline; an existing editor agent can write directly without a mandatory tournament or top-down outline. Every automatic generation run creates a proposal. It becomes accepted only through an explicit commit or explicit delegated acceptance.

## Components

| Module | Responsibility |
| --- | --- |
| `cli/index.mjs` | Strict command routing, help, versioned JSON, exit codes and explicit approval flags. |
| `cli/store.mjs` | Immutable revisions and text, current state, source checks, proposals, fact decisions, merge updates, commits, recovery and undo. |
| `cli/context.mjs` | Relevant source-backed context, preceding prose, selective summaries, knowledge scopes and budget/coverage reporting. |
| `cli/generate.mjs` | Checkpointed plan/draft/review/revise orchestration, model-call budgets and proposal creation. |
| `cli/runner.mjs` | Authenticated external Codex CLI or explicit JSON executable protocol; timeouts and bounded output. |
| `cli/schemas.mjs` | Model-output contracts and local validation. |
| `cli/checks.mjs` | Explicit measurable checks, fact/review excerpt validation and integrity audit. |
| `cli/catalogue.mjs` | Dated category mappings, original examples, craft principles and attributed reading pointers. |
| `cli/export.mjs` | Committed manuscript assembly and escaped reading exports. |
| `cli/util.mjs` | Paths, safe writes, identifiers, input validation, hashes and Unicode word counts. |
| `cli/verify-package.mjs` | Pack allowlist, isolated offline installation and installed-runtime smoke workflow. |

No model credentials, hosted database, binary installer or runtime npm dependency is bundled. A provider can be exchanged at the runner boundary without changing story storage. The CLI is not an omniscient editor: semantic judgements remain fallible and require actual textual evidence.

## What changed from 0.1

The npm executables now use the Node implementation. Old Rust source, rule files and launcher tests remain available for historical compatibility and maintenance, but are not shipped in the 0.2 package. Old documentation outside `README.md`, `docs/CLI.md` and this page describes the legacy surface unless explicitly updated.

The former overlapping `eval`, `lint`, `matrix`, `memory` and creative-tournament commands are not silently aliased to unrelated capabilities. Use `schema` to discover the current contract. A legacy command produces an error rather than a fabricated result. The old line-level heuristics do not act as literary quality gates in 0.2.

A language/runtime change was chosen because the distributed npm package already required Node, while the platform binary layer complicated installation. Preserving the old code avoids destructive repository history changes. The new domain modules can be adopted by a different interface without copying a second agent harness into every client.

## Migration without data loss

1. Back up the complete 0.1 project, including `.novel/` and manuscript files.
2. Create a separate empty directory and run `novel-craft init` there with the real title, premise and relevant profiles. Initialisation refuses to overwrite either an existing 0.2 workspace or legacy `.novel/` state in place.
3. Import manuscript copies one chapter at a time. Inspect each proposal and commit it using its returned base revision. The source files are not changed.
4. Convert only reviewed legacy notes into candidate fact arrays with exact supporting excerpts. Imported notes are not automatically true just because an earlier tool stored them.
5. Accept or reject candidates explicitly. Read back `canon`, `context` and `audit`, then export and inspect the assembled book.

Automatic semantic migration of every legacy YAML object is not implemented. It would be unsafe to turn inconsistent legacy notes into accepted canon without inspection. The original workspace remains intact.

## Research and reading policy

The following sources inform design decisions, not claims that this implementation has reproduced their experimental results:

- [DOC, ACL 2023](https://aclanthology.org/2023.acl-long.190/): detailed outline control informs optional hierarchical plans, not mandatory outlining for every writer.
- [CoAuthor, CHI 2022](https://coauthor.stanford.edu/): writer acceptance, rejection and editing motivate local intervention and retained human authority.
- [ConStory-Bench, ACL Findings 2026](https://aclanthology.org/2026.findings-acl.410/): evidence-backed consistency evaluation motivates source excerpts and explicit limitations, not a universal consistency score.
- [Writing Excuses: Tell, Don't Show](https://writingexcuses.com/16-33-tell-dont-show/): showing and telling serve pacing and distance; neither is a global lint defect.
- [Sanderson's First Law](https://www.brandonsanderson.com/blogs/blog/sandersons-first-law): reader understanding of magic can matter to conflict resolution; this is a craft lens, not an obligation to explain every mystery.
- [Agent Skills](https://agentskills.io/specification): one concise entry skill, with detailed contracts discoverable on demand.
- [Codex non-interactive mode](https://developers.openai.com/codex/noninteractive/): the default adapter uses an external authenticated execution surface rather than owning credentials.
- [npm trusted publishers](https://docs.npmjs.com/trusted-publishers/): releases use the existing GitHub workflow identity, without long-lived publication tokens.

The installed catalogue includes individual reference titles, source types, URLs, use notes and original vignettes. Public-domain classics supply transferable craft examples; authorised publisher pages supply discovery pointers for contemporary genres. A transfer reference does not claim a book belongs to every profile that cites it. No full hosted novel or copyrighted chapter is copied into this repository, downloaded by the CLI, or used as a hidden training corpus. Fan-fiction guidance emphasises original or permissioned worlds and does not make legal promises.

The observed 57-label taxonomy comes from NovelFull on 7 September 2026. NovelFullbook was inaccessible. The machine-readable coverage report preserves the distinction. Categories can be expanded through reviewed profiles and explicit project-level custom profiles.

## Proof and remaining limits

Regression tests run real local processes, state transitions, interruptions and packed installations. They use an explicitly named deterministic model-protocol fixture so transport and workflow failures are reproducible without paid model access. This does not establish that a particular live model follows instructions or writes compelling prose.

A serious writing evaluation should compare the same model with ordinary manuscript access, with the skill alone, and with the full workspace. Use matched tasks and resource limits, blind/randomised reader comparisons, separate continuity and harmful-revision judgements, and genre-diverse permissioned manuscripts. Publish failed examples, not only flattering results. Do not collapse these observations into an unvalidated overall score.

Known boundaries: relevance is lexical/recency-based rather than a semantic retrieval service; summaries are model or author interpretations; POV filtering applies to structured facts, not perfect information-flow control over raw prose; competing structured values are review leads; whole-run proposals require enough context for the essential chapter; a large edit can require subsequent chapters to be reviewed manually. No platform distribution, sales, awards, rankings or reader-retention result is guaranteed.

## Release discipline

CI retains actual legacy Rust checks under their required names and adds new runtime tests plus clean package installation. The release workflow runs only after successful main-branch CI (or an explicit tag), re-verifies the package, uses the existing `release.yml`/`npm` trusted publisher identity, checks whether the exact version already exists, and creates a release artefact. Environment approvals and npm publisher configuration are not bypassed. A built tarball, merged source, completed publication and verified registry installation are separate states.
