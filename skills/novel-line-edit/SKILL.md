---
name: novel-line-edit
description: Use Novel Craft to run line, sentence, scene, and dialogue craft lint passes on fiction drafts, including passive voice, filter words, embodied emotion, concrete detail, rhythm, tags, and exposition.
---

# Novel Line Edit

Use this skill when the user asks to improve prose, line edit, polish a scene, check passive voice, show-not-tell, dialogue, rhythm, or bad writing patterns.

## Workflow

Run the narrowest useful pass first:

```bash
novel-craft lint line <draft.md>
novel-craft lint scene <draft.md>
```

For the deeper effect-aware report:

```bash
novel-craft rules guide --level line
novel-craft analyse <draft.md>
novel-craft review <draft.md> --rubric prose
```

Create a targeted report:

```bash
novel-craft revise <draft.md> --pass line
```

For conservative deterministic fixes only:

```bash
novel-craft revise <draft.md> --pass line --apply-safe --out <draft.revised.md>
```

## Editing Doctrine

- Show/tell is situational.
- Active voice is usually stronger in action, but passive can be correct for mystery, withheld actor, victim focus, helplessness, institutional coldness, rhythm, or delayed revelation.
- Filter words are not automatically bad. Keep them when perception itself matters.
- Sentence fragments are not automatically bad. Keep them for shock, rhythm, punchline, or broken thought.
- Preserve voice. Do not flatten every sentence into the same crisp style.
- Explain the mental model briefly when teaching: rule, exception, and exact local example.
- Classify every finding as likely mistake, possibly intentional, or clearly functional before revising.
- Use examples and counterexamples from `novel-craft rules guide`; deterministic findings are leads, not verdicts.
