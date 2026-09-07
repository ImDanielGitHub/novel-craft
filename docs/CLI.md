# Novel Craft 0.2 CLI

The executable names `novel-craft` and `novel` are aliases. Node 22.14+ is required. All operations are local except an explicitly requested model invocation. The CLI stores no model credentials. Run `schema --json` to discover commands, flags, response envelopes and model-output schemas from the installed version.

## Machine contract

Use `--json` on any command. Standard output contains exactly one JSON object with `schema_version`, `version`, `ok`, `command`, and either `data` or `error`. Errors have `code`, `message` and `details`. `ok` describes successful execution: a `check` can execute successfully but report `data.passed=false` and exit 5. Non-JSON errors go to standard error. Model logs are not mixed into command output.

Exit codes: 0 success; 2 invalid input; 3 workspace conflict, stale revision or corrupt state; 4 model/protocol/timeout failure; 5 failed explicit checks or unresolved review when delegated acceptance was requested.

Global flags are `--json`, `--project <directory>`, `--out <new-file>`, `--help` and `--version`. `--out` writes a result exclusively and returns a JSON receipt rather than mixing prose into stdout. Existing outputs are never replaced. Unknown command-specific flags and unexpected positional arguments fail. There are no interactive prompts in machine mode.

## Commands

| Command | Purpose and important flags |
| --- | --- |
| `init` | Create a workspace. `--title`, `--idea`, repeated `--genre`, `--genre-file`, `--language`, `--audience`, `--voice`. |
| `create <directory>` | Initialise and actually generate prose. Requires `--idea`; accepts project and generation flags. |
| `project show` | Read current intention, profiles, voice and revision. |
| `project update` | Merge supplied project fields; requires `--expect <revision>`. Reconsider earlier chapters after intent/voice/genre changes. |
| `generate` | Append new chapters through plan, draft, review and bounded revision. |
| `revise` | Revise an existing `--chapter` for a required `--instruction`. A chapter revision returns a full chapter proposal, not a silent write. |
| `import` | Propose a copied manuscript: `--file`, optional `--chapter`, `--title`, `--summary`, `--facts <JSON-file>`. `--adopt` explicitly reads an externally edited tracked chapter. Source files are never modified by import. |
| `context` | `--chapter`, `--pov`, `--query`, `--budget`, `--examples`. Returns relevant state, source references, text and omission warnings. |
| `review` | Review `--chapter` through the runner. `--packet` returns the input and schema without running a model. |
| `check <file>` | Explicit `--min-words`, `--max-words`, repeated `--must-include` and `--must-avoid`. Literals are case-sensitive, not semantic facts. |
| `audit` | Inspect source integrity, fact provenance, possible competing structured facts and downstream review flags. Not an automated proof of plot consistency. |
| `diff <proposal-id>` | Show the exact proposal-base text and proposed full text, facts, plan and reviews. Flags stale revision state. |
| `commit <proposal-id>` | Apply an inspected proposal with `--expect <base-revision>`. `--accept-facts` separately approves its candidate facts. |
| `canon list` | Show accepted, candidate, rejected and superseded facts with sources. Bare `canon` also lists them. |
| `canon accept <fact-id>` | Approve one candidate, requiring current `--expect`. |
| `canon reject <fact-id>` | Reject one candidate, requiring current `--expect`; retains the evidence and decision. |
| `preferences add` | Save a writer choice using `--instruction` and `--expect`. Bare `preferences` lists them. |
| `status` | Show chapter states, modified files, fact counts, pending recovery and current revision. |
| `history` | Read receipts from current history. |
| `runs` | List generation checkpoints, stages and model-call counts. |
| `undo` | Restore the immediate parent as a new revision. Requires `--expect` and `--yes`. Repeating undo reverses the last restoration; it is not a multi-step historical selector. |
| `recover` | Complete an interrupted authorised commit with `--yes`; conflicting manual edits block recovery. |
| `unlock` | Release a stale process lock with `--yes`; refuses a live PID. |
| `export` | `--format md|html|epub --out <new-file>`. Exports committed text only. |
| `genres [category]` | List profiles, or show a profile with sources and optional `--examples`. |
| `genres coverage` | Show dated observed taxonomy, mappings and exact-source limitations. |
| `guide` | Craft principles, original showing/telling examples and references. |
| `schema [plan|chapter|review]` | Discover the CLI contract or one model-output schema. |
| `setup` | Copy the single agent entry skill into `--target <skills-directory>`. Defaults to `.agents/skills` inside the selected directory. Never overwrites an edited skill. |
| `doctor` | Check Node support, workspace state and Codex executable availability. It does not prove model authentication. |

## Generation flags

`--chapters` defaults to 1, maximum 50 per run. `--words` defaults to 1800 and is a model target, not a guarantee. Use explicit `--min-words` and `--max-words` to enforce a range. `--passes` defaults to 1 and allows 0 to 3 revision rounds per chapter. `--max-calls` is a total cap, defaulting to `1 + chapters * 4`; reaching it preserves a resumable checkpoint. `--budget` defaults to an estimated 12000 input-context tokens; it uses UTF-8 byte count divided by four, not a provider tokenizer. The prompt and output also consume model context. `--timeout-ms` defaults to 300000 per model call.

`--accept` explicitly delegates manuscript approval only after the configured workflow finishes without unresolved major findings or explicit constraints. It never guarantees literary quality. Facts remain candidates unless `--accept-facts` is also supplied. Model-generated acceptance is recorded as delegated, not as a human read-through.

Resume with `generate --resume <run-id>` and the original runner configuration. Supply a larger `--max-calls` only when deliberately increasing the budget. Resume never rereads an executable command from a project checkpoint. Drafts from a failed run remain under `.novelcraft/runs/`; if the manuscript changed, preserve those outputs and start a new proposal against current state rather than forcing a stale resume.

## Model runners

The default `--runner codex` invokes the separately installed, authenticated Codex CLI. It uses a disposable directory, read-only sandbox, ephemeral session, output schema and final-message file. It does not obtain keys, make account changes, or claim to override the user's provider billing. `--model <name>` is passed through to Codex. The actual subprocess is similar to:

```text
codex exec --ephemeral --skip-git-repo-check --sandbox read-only --output-schema <schema.json> --output-last-message <result.json> -
```

For another agent, use an explicitly trusted executable:

```bash
novel-craft generate --runner-command node \
  --runner-arg=/absolute/path/to/my-writing-runner.mjs --json
```

Arguments are passed as an array with **no shell**. Each invocation receives one JSON value on standard input:

```json
{
  "protocol": "novel-craft/1",
  "stage": "draft",
  "prompt": "Complete instructions and task data",
  "schema": {"type": "object"},
  "input": {"context": {}, "chapter": {"number": 1}}
}
```

The runner returns exactly one JSON object matching the supplied schema, not Markdown fences and not the CLI's result envelope. Supported stages are `plan`, `draft`, `review` and `revise`. Use `schema plan`, `schema chapter` and `schema review` for complete shapes. Errors use a non-zero process exit. Runner stderr is drained but not persisted because providers may emit sensitive information. Timeouts stop the process group on POSIX; platform process behaviour should be validated for custom Windows runners. Output is capped at 8 MiB. A custom runner is trusted local code and inherits the caller's environment; select it intentionally.

The repository's fixture implements this transport solely for regression tests. Its fixed prose is not used by the production CLI and does not establish model performance.

## Canon and viewpoint

An import facts file is a JSON array such as:

```json
[
  {
    "subject": "Mara",
    "predicate": "holds",
    "value": "brass key",
    "kind": "world",
    "known_by": ["Mara", "Ivo"],
    "quote": "Mara held the brass key."
  }
]
```

The quote must exist literally in the imported prose. Kinds are `world`, `belief`, `reader` and `plan`. Generated facts use the same shape. Source chapter, content hash and chapter-based validity are assigned by the workspace. Exact quotation supports provenance, not truth: a character's dialogue can be mistaken. Inspect the proposed interpretation before approval.

`context --pov <name>` filters structured facts by character knowledge and excludes author plans; it does not promise that raw chapter text contains no information outside that viewpoint. Context reports this boundary. Revising an earlier chapter supersedes facts derived from that chapter and flags later chapters for review; it does not automatically rewrite the rest of the book.

## Custom profiles

`init --genre-file <file.json>` or `project update --genre-file <file.json> --expect <revision>` accepts a profile with `id`, `name`, `reader_appeal`, `serial_engine`, `review_questions` (array), `references` (array of objects with `title` and HTTPS `url` fields), and `example` (`prose`, `lesson`). An optional `kind` distinguishes genre from setting, audience or content. Custom examples must be original or permissioned. Custom profiles are project data, not executable plugins. Do not use them to store secrets or to override approval boundaries.

## Files and recovery

`.novelcraft/HEAD.json` names the current immutable revision; revisions contain project settings, approved chapter hashes, facts and receipts. `.novelcraft/objects/` contains immutable chapter text. `manuscript/` is the ordinary Markdown view. `.novelcraft/proposals/` and `runs/` preserve work that is not yet accepted. Keep the entire workspace in backups; manuscript files alone cannot restore approval history.

A commit validates every source, writes immutable objects and a journal, materialises text, then advances HEAD. A crash can leave a journal. `status` reports it and `recover --yes` completes it after validating all affected files. External edits are never silently replaced. A stale lock after a stopped process can be removed with `unlock --yes`. Do not manually delete journals or recovery evidence to make the error disappear.

The journal protects ordinary process interruption and guarded recovery; it is not a substitute for backups, reliable storage or testing on unusual network filesystems. Concurrent agents can prepare proposals; only one matching current revision can commit. No background daemon or watcher is required.
