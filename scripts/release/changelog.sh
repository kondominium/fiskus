#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

FILE="docs/changelog.md"
BASE="https://github.com/kondominium/fiskus"

cargo_version() {
    sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1
}

valid_date() {
    [ "$(date -j -f '%Y-%m-%d' "$1" '+%Y-%m-%d' 2>/dev/null)" = "$1" ]
}

trim_trailing_blanks() {
    awk 'NF { p = NR } { lines[NR] = $0 } END { for (i = 1; i <= p; i++) print lines[i] }'
}

check() {
    local errors=0
    local first
    first="$(grep -m 1 '^## \[' "$FILE" || true)"
    if [ "$first" != "## [Unreleased]" ]; then
        echo "changelog check: [Unreleased] must be the first version section" >&2
        errors=1
    fi

    local headings
    headings="$(grep '^## \[' "$FILE" || true)"
    local versions=()
    local line ver date
    while IFS= read -r line; do
        [ -z "$line" ] && continue
        if [ "$line" = "## [Unreleased]" ]; then
            continue
        fi
        if [[ ! $line =~ ^##\ \[([0-9]+\.[0-9]+\.[0-9]+)\]\ -\ ([0-9]{4}-[0-9]{2}-[0-9]{2})$ ]]; then
            echo "changelog check: bad released section heading: $line" >&2
            errors=1
            continue
        fi
        ver="${BASH_REMATCH[1]}"
        date="${BASH_REMATCH[2]}"
        if ! valid_date "$date"; then
            echo "changelog check: invalid ISO date in heading: $line" >&2
            errors=1
        fi
        versions+=("$ver")
    done <<< "$headings"

    # duplicates
    local sorted_unique
    sorted_unique="$(printf '%s\n' "${versions[@]}" 2>/dev/null | sort -u | wc -l | tr -d ' ')"
    if [ "${#versions[@]}" -ne "$sorted_unique" ] && [ "${#versions[@]}" -gt 0 ]; then
        echo "changelog check: duplicate released version" >&2
        errors=1
    fi

    # descending semver order
    if [ "${#versions[@]}" -gt 0 ]; then
        local sorted
        sorted="$(printf '%s\n' "${versions[@]}" | sort -V -r)"
        local joined sorted_joined
        joined="$(printf '%s\n' "${versions[@]}")"
        sorted_joined="$sorted"
        if [ "$joined" != "$sorted_joined" ]; then
            echo "changelog check: released versions are not in descending semver order" >&2
            errors=1
        fi
    fi

    # compare links
    local first_link
    first_link="$(grep -n -E '^\[[^]]+\]: ' "$FILE" | head -1 | cut -d: -f1 || true)"
    if [ -n "$first_link" ]; then
        # no heading after first link
        local after_bad
        after_bad="$(sed -n "${first_link},\$p" "$FILE" | grep -c '^## \[' || true)"
        if [ "$after_bad" -gt 0 ]; then
            echo "changelog check: version section appears after the compare links" >&2
            errors=1
        fi
        # every line after first link is blank or a link
        local bad_tail
        bad_tail="$(sed -n "${first_link},\$p" "$FILE" | grep -v -E '^(\[[^]]+\]: |[[:space:]]*$)' | head -1 || true)"
        if [ -n "$bad_tail" ]; then
            echo "changelog check: compare links must be at the bottom of the file" >&2
            errors=1
        fi
    fi

    local link_versions
    link_versions="$(grep -E '^\[[^]]+\]: ' "$FILE" | sed -E 's/^\[([^]]+)\]: .*/\1/' | sort || true)"
    if [ "${#versions[@]}" -gt 0 ]; then
        local expected_links
        expected_links="$(printf 'Unreleased\n%s\n' "$(printf '%s\n' "${versions[@]}")" | sort)"
        if [ "$link_versions" != "$expected_links" ]; then
            echo "changelog check: compare links do not match the sections" >&2
            errors=1
        fi
    else
        if [ -n "$link_versions" ] && [ "$link_versions" != "Unreleased" ]; then
            echo "changelog check: compare links do not match the sections" >&2
            errors=1
        fi
    fi

    local current
    current="$(cargo_version)"
    if [ "${#versions[@]}" -gt 0 ]; then
        if [ "${versions[0]}" != "$current" ]; then
            echo "changelog check: newest released version ${versions[0]} does not equal Cargo.toml version $current" >&2
            errors=1
        fi
    else
        if [ "$current" != "0.1.0-dev" ]; then
            echo "changelog check: no released section is only valid before the first release, with Cargo.toml version 0.1.0-dev" >&2
            errors=1
        fi
    fi

    if [ "$errors" -ne 0 ]; then
        exit 1
    fi
    echo "changelog check: ok"
}

stamp() {
    local version="$1"
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "version must be x.y.z" >&2
        exit 1
    fi

    local today
    today="$(date +%F)"

    # Insert a fresh Unreleased heading and rename the old one.
    awk -v ver="$version" -v today="$today" '
        $0 == "## [Unreleased]" && !done {
            print "## [Unreleased]"
            print ""
            print "## [" ver "] - " today
            done = 1
            next
        }
        { print }
    ' "$FILE" > "$FILE.tmp"
    mv "$FILE.tmp" "$FILE"

    # Collect released versions in document order.
    local versions
    versions="$(grep -E '^## \[[0-9]+\.[0-9]+\.[0-9]+\] - ' "$FILE" | sed -E 's/^## \[([^]]+)\].*/\1/')"

    # Remove all compare links.
    grep -v -E '^\[[^]]+\]: ' "$FILE" | trim_trailing_blanks > "$FILE.body"
    {
        cat "$FILE.body"
        echo ""
        if [ -n "$versions" ]; then
            local newest
            newest="$(printf '%s\n' "$versions" | head -1)"
            echo "[Unreleased]: $BASE/compare/v$newest...HEAD"
        fi
        printf '%s\n' "$versions" | while IFS= read -r v; do
            [ -z "$v" ] && continue
            echo "[$v]: $BASE/releases/tag/v$v"
        done
        echo ""
    } > "$FILE"
    rm -f "$FILE.body"
}

notes() {
    local version="$1"
    awk -v ver="$version" '
        $0 ~ "^## \\[" ver "\\]" { insec = 1 }
        insec && /^## \[/ && $0 !~ "^## \\[" ver "\\]" { exit }
        insec && /^\[/ { exit }
        insec { print }
    ' "$FILE"
}

case "${1:-}" in
    check)
        check
        ;;
    stamp)
        if [ "$#" -ne 2 ]; then
            echo "usage: changelog.sh stamp <version>" >&2
            exit 2
        fi
        stamp "$2"
        ;;
    notes)
        if [ "$#" -ne 2 ]; then
            echo "usage: changelog.sh notes <version>" >&2
            exit 2
        fi
        notes "$2"
        ;;
    *)
        echo "usage: changelog.sh {check|stamp <version>|notes <version>}" >&2
        exit 2
        ;;
esac
