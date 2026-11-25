---
name: commit-message
description: Draft a commit message for a fiskus slice. Follows the Commit messages section of docs/contributing/workflow.md.
---

# Commit message

The rules live in `docs/contributing/workflow.md`, section "Commit messages". Follow that section. Do not restate its rules here.

Steps:

1. Read the "Commit messages" section of `docs/contributing/workflow.md`, the "Rules" section of `docs/contributing/code.md`, and the language rules in `docs/contributing/language.md` (the body is English).
2. Check the branch with `git branch --show-current`. Stop and report if it is `main`. Agents work on a branch, never on `main` (see `docs/contributing/workflow.md`).
3. Check the staged slice:
   - `git diff --cached --name-only` lists only the files of this slice. Stop if it lists unrelated files, or if it is empty.
   - `git status --short` shows nothing that belongs to the slice and is unstaged. Report any such file.
   - Nothing from `.agents/research/` or `.agents/plans/` is staged. Both are git-ignored, so a staged file there means the ignore rule was bypassed. Stop and report it.
4. Run `make gates` and record its exit code. If it fails, stop and report the failing gate. Do not draft a message for a failing slice.
5. Draft the title and body as the workflow section describes:
   - Title: imperative, verb first, 72 characters or fewer, no type prefix. Check the length with `printf '%s' "<title>" | wc -c`.
   - Body: wrapped at 72 characters. Check with `awk 'length > 72'` on the draft. Say what changed and why. Do not list the diff.
   - Name the decisions the reviewer should check, and anything not done in this slice.
6. Do not commit. Hand over: the staged file list, the draft, the branch name, and the `make gates` exit code.
