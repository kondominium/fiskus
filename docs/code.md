# Code gates and rules

Applies to humans and agents.

## Gates

- Scratch: research, drafts and plans go in `.agents/research/` and `.agents/plans/`. Both are git-ignored and never committed.
- Commit: agents never commit. Hand over a staged diff (only the slice's files), a commit message draft, and the `make check` result.
- Check: `make check` must pass before handover.

## Rules

- No `f64` for money.
- No `unwrap()` or `expect()` in library code reachable from user input. Return `Result`.
- No runtime filesystem reads. Tax parameters are compiled in.
- Every number in an assessment carries its § citation and parameter version.
- Verify tax figures against the statute before relying on them.
