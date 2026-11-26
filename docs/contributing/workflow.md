# Workflow

Applies to humans and agents. fiskus uses trunk-based development: `main` is the trunk, every change is a short branch from it, and `main` stays releasable.

## Branches

Branch from `main`. Any name works, in lowercase with hyphens, as long as it is short-lived.

## Daily flow

| Step | Command | Why |
|---|---|---|
| 1. Update main | `git switch main && git pull --ff-only` | Start from the trunk |
| 2. Branch | `git switch -c <name>` | Branch from main, never from another branch |
| 3. Commit a slice | `git add <the slice's files>`, then `git commit` | One slice per commit. The owner commits |
| 4. Stay current | `git fetch origin && git rebase origin/main` | Rebase onto main. Repeat daily and before handover |
| 5. Gate | `make gates` | Same checks as CI |
| 6. Push | `git push --force-with-lease` after a rebase | Rewrites only your own branch, and refuses if someone else pushed |
| 7. Land | Open a pull request, merge by rebase | Keeps main linear, one commit per slice |
| 8. Clean up | Delete the branch | Branches stay short-lived |

Set this once, so a plain `git pull` rebases: `git config pull.rebase true`.

## Rules

| Do | Do not |
|---|---|
| `git rebase origin/main` to bring a branch up to date | `git merge main` into a branch. No merge commits on any branch |
| `git pull --ff-only` on main | `git pull` with merge, or `git pull --no-rebase` |
| `git push --force-with-lease` on your own branch after a rebase | `--force` on `main`, or on a branch someone else is using |
| Land by rebase, so each slice stays a commit on main | A merge commit, or a squash that hides the slices |
| Merge a slice as soon as its gates pass | A branch that drifts from main for weeks |

## Commit messages

- Title: imperative mood, verb first, 72 characters or fewer, no type prefix.
- Body: English. Say what changed and why. The diff already shows how. Wrap at 72 characters.
- Scope: one slice per commit. Stage only that slice's files.
- Gate: `make gates` passes before the commit.
- Changelog: a change that users can notice adds a line under `Unreleased` in `docs/changelog.md`, in the same slice. Internal-only slices say so in the handover.

Good title: `Round joint halves down before the zone check`. Bad title: `Update code`. A reviewer can tell the slice from the title.

## Agents

- Agents work on `main`, and stage their slice there. Before a slice, run `git pull --ff-only` on `main`, and check that the tree is clean. No branch is needed, because the owner reviews and commits each slice.
- Agents run `make gates`, and draft the commit message with the `commit-message` skill (`.agents/skills/commit-message.md`). That skill follows this page.
- Agents never commit, push, tag or merge. The owner does those steps.
- No planning references: a commit message never names its slice, phase or checkpoint numbers, or any planning status. Say what changed, why, and what is not done. The docs follow the same rule.
- Hand over: the staged files (one slice only), the commit message draft, and the `make gates` result.
- Humans may keep using branches and the daily flow above. The rebase rules still apply to any branch.
