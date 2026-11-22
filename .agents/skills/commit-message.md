---
name: commit-message
description: Draft a commit message for a fiskus slice. Follows the Commit messages section of docs/contributing/code.md.
---

# Commit message

The rules live in `docs/contributing/code.md`, section "Commit messages". Follow that section. Do not restate its rules here.

Steps:

1. Read the "Commit messages" section and the "Rules" section of `docs/contributing/code.md`.
2. Check the staged slice with `git diff --cached --stat`. Only that slice's files should be staged.
3. Run `make check`. If a Kani harness changed, also run `make verify`. Record the result.
4. Draft the title and body as the section describes.
5. Do not commit. Hand over the staged file list, the draft, and the gate result.
