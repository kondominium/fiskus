# AGENTS.md

Read these before you change anything:

- `docs/contributing/index.md`: what each contributing page covers.
- `docs/contributing/workflow.md`: branches, rebasing, commits, handover.
- `docs/contributing/code.md`: gates and code rules.
- `docs/contributing/tests.md`: which test layer proves a claim.
- `docs/contributing/language.md` and `docs/contributing/guide.md`: before you touch any doc or the guide.

Hard rules:

- Run `make gates` before handover, and report its result. A failing gate is not handed over.
- Never commit, push or merge. Hand over the staged files (one slice), a commit message draft and the gate result.
- Branch from `main`. Keep your branch current with `git rebase origin/main`. Never merge `main` into a branch.
- Run tests only through `make test-*` targets.
- Scratch work goes in `.agents/research/` and `.agents/plans/`. Both are git-ignored.
- To draft a commit message, use the `commit-message` skill (`.agents/skills/commit-message.md`).
