# AGENTS.md

Read these before you change anything:

- `docs/contributing/index.md`: what each contributing page covers.
- `docs/contributing/workflow.md`: branches, rebasing, commits, handover.
- `docs/contributing/code.md`: gates and code rules.
- `docs/contributing/tests.md`: which test layer proves a claim.
- `docs/contributing/language.md` and `docs/contributing/guide.md`: before you touch any doc or the guide.

Hard rules:

- Run `make gates` before handover, and report its result. A failing gate is not handed over.
- Docs and commit messages never refer to planning notes: no slice, phase or checkpoint numbers, and no planning status. The `commit-message` skill checks drafts with a grep. No check runs on the docs for this yet.
- Never commit, push, tag or merge. Hand over the staged files (one slice), a commit message draft and the gate result.
- Work on `main` and stage there. Run `git pull --ff-only` first, and stop if the tree is not clean before you start. Never create a branch, unless the owner asks for one.
- If you do work on a branch, branch from `main`, keep it current with `git rebase origin/main`, and never merge `main` into it.
- Run tests only through `make test-*` targets.
- Scratch work goes in `.agents/research/` and `.agents/plans/`. Both are git-ignored.
- To draft a commit message, use the `commit-message` skill (`.agents/skills/commit-message.md`).
- To fetch BMF calculator results into the conformance fixtures, use the `add-conformance-cases` skill (`.agents/skills/add-conformance-cases.md`).
