# Release

Prepare a fiskus release. Agents run `make release VERSION=…` and hand over the staged files, the gate result and the tag and push commands. Agents never commit, tag, push or publish.

## Steps

1. Check the pre-release checklist below.
2. Run `make release VERSION=x.y.z`.
3. Hand over the staged files (`Cargo.toml`, `Cargo.lock`, `docs/changelog.md`), the gate result, and the printed commit title and tag and push commands.

## Pre-release checklist

- Conformance all green, or the mismatch registered as a deviation.
- The guide and charts regenerated (`make charts`).
- `scripts/release/changelog.sh check` passes.
- `cargo semver-checks` run (skipped automatically until the crate is on crates.io).

## Rules

- Only the owner commits, tags, pushes and publishes.
- `make release` refuses to run unless the branch is `main`, the tree is clean, the branch is not behind `origin/main`, `VERSION` is greater than the current version, and `Unreleased` is not empty.
