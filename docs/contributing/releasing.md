# Releasing

This page covers cutting a fiskus release. For branches and commits, see [Workflow](workflow.md). For gates, see [Code](code.md).

## Version policy

fiskus is pre-1.0. Versions follow semantic versioning with these rules:

- A **minor** release covers any change to a computed result (a corrected figure, a new year or rule) and any API change. Breaking API changes are allowed in a minor before 1.0. `#[non_exhaustive]` keeps new years and errors non-breaking.
- A **patch** release covers documentation, performance and internal changes with identical computed results.
- A changelog line that changes results says so explicitly, starting with `Changes results:`. Users who pin a version can see it at a glance.

## Steps

1. Run `make release VERSION=x.y.z`. The target refuses to run unless the branch is `main`, the tree is clean, the branch is not behind `origin/main`, `VERSION` is greater than the current version, and `Unreleased` is not empty.
2. Review the staged diff.
3. Commit with the title `Release x.y.z`.
4. Tag the commit: `git tag -a vX.Y.Z -m "fiskus X.Y.Z"`.
5. Push: `git push origin main vX.Y.Z`.
6. Watch `release.yml`.

## What is automated, and what only the owner does

`make release` prepares the files, runs the gates and stages them. It never commits or tags.

Only the owner does:

- the commit
- the tag
- the push
- the first-publish token
- enabling trusted publishing on crates.io
- the `release` environment approval, when it is required

## Recovery

- If a job fails before `publish`: delete the tag locally and on the remote, fix the problem on `main`, then tag again.
- After `publish`: a version cannot be republished. Use `cargo yank --version x.y.z`, then release a patch and record it in the changelog.

## Changelog rules

One line per slice that users can notice, in the right group, following the [language rules](language.md). A line that changes a result starts with `Changes results:`.
