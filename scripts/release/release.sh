#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
    echo "VERSION is required: make release VERSION=x.y.z" >&2
    exit 1
fi
if [[ ! $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "VERSION must be x.y.z" >&2
    exit 1
fi

branch="$(git branch --show-current)"
if [ "$branch" != "main" ]; then
    echo "release must run on main (current: $branch)" >&2
    exit 1
fi

if [ -n "$(git status --porcelain)" ]; then
    echo "working tree must be clean" >&2
    exit 1
fi

git fetch origin main --quiet 2>/dev/null || true
if ! git merge-base --is-ancestor origin/main HEAD 2>/dev/null; then
    echo "branch is behind origin/main" >&2
    exit 1
fi

current_version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"

# VERSION must be greater than current_version
if [ "$VERSION" = "$current_version" ]; then
    echo "VERSION must be greater than the current version" >&2
    exit 1
fi
current_base="${current_version%%-*}"
version_base="${VERSION%%-*}"
if [ "$current_base" = "$version_base" ]; then
    if [[ "$current_version" == *-* && ! "$VERSION" =~ - ]]; then
        : # release after its own dev version
    else
        echo "VERSION must be greater than the current version" >&2
        exit 1
    fi
else
    newest_base="$(printf '%s\n%s\n' "$current_base" "$version_base" | sort -V | tail -1)"
    if [ "$newest_base" != "$version_base" ]; then
        echo "VERSION must be greater than the current version" >&2
        exit 1
    fi
fi

# Unreleased must not be empty
unreleased_body="$(awk '
    $0 == "## [Unreleased]" { insec = 1; next }
    insec && /^## \[/ { exit }
    insec && /^\[/ { exit }
    insec { print }
' docs/changelog.md)"
if [ -z "$(printf '%s' "$unreleased_body" | tr -d '[:space:]')" ]; then
    echo "Unreleased section is empty" >&2
    exit 1
fi

# Set version in Cargo.toml
sed -i '' "s/^version = .*/version = \"$VERSION\"/" Cargo.toml

# Refresh Cargo.lock
mise exec -- cargo build >/dev/null

# Stamp changelog
scripts/release/changelog.sh stamp "$VERSION"

# Gates
make gates
make package-check

# Semver checks, skipped until the crate is on crates.io
if command -v cargo-semver-checks >/dev/null 2>&1; then
    if cargo search fiskus 2>/dev/null | grep -q '^fiskus = '; then
        mise exec -- cargo semver-checks
    else
        echo "skipping cargo semver-checks: fiskus is not on crates.io yet"
    fi
else
    echo "skipping cargo semver-checks: cargo-semver-checks is not installed"
fi

# Stage
git add Cargo.toml Cargo.lock docs/changelog.md

echo
echo "Commit title: Release $VERSION"
echo
echo "Owner runs:"
echo "  git commit -m \"Release $VERSION\""
echo "  git tag -a v$VERSION -m \"fiskus $VERSION\""
echo "  git push origin main v$VERSION"
