// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Keeps the guide in `docs/guide` and the internals pages in `docs/internals` in step with the code.
//! See `docs/contributing/guide.md`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
struct Coverage {
    guide_year: u16,
    chapter: Vec<Chapter>,
}

#[derive(Deserialize)]
struct Chapter {
    file: String,
    sections: Vec<String>,
}

#[derive(Deserialize)]
struct Parameter {
    value_eur: i128,
    citation: String,
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn coverage() -> Coverage {
    let path = root().join("docs/guide/coverage.toml");
    toml::from_str(&read(&path)).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn page(file: &str) -> String {
    read(&root().join("docs").join(file))
}

// Article numbers cited as `§ 32a` or `§ 26 EStG`, without the `§`.
fn articles(text: &str) -> BTreeSet<String> {
    text.match_indices('§')
        .map(|(index, _)| text[index + '§'.len_utf8()..].trim_start_matches(' '))
        .map(|rest| {
            rest.chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect::<String>()
        })
        .filter(|article| article.starts_with(|c: char| c.is_ascii_digit()))
        .collect()
}

fn files_under(dir: &Path, extension: &str, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap_or_else(|error| panic!("{}: {error}", dir.display())) {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files_under(&path, extension, found);
        } else if path.extension().and_then(|e| e.to_str()) == Some(extension) {
            found.push(path);
        }
    }
}

// Sections the code cites. A listed section must appear here, so the guide cannot describe
// a section that src/ and rules/ do not refer to.
fn implemented_articles() -> BTreeSet<String> {
    let mut sources = Vec::new();
    files_under(&root().join("src"), "rs", &mut sources);
    files_under(&root().join("rules"), "toml", &mut sources);
    sources
        .iter()
        .flat_map(|path| articles(&read(path)))
        .collect()
}

// Same format as the guide's own euro amounts: space between thousands, no decimals.
fn euros(value: i128) -> String {
    let digits = value.to_string();
    let mut out = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(digit);
    }
    format!("{out} €")
}

#[test]
fn every_guide_page_is_listed_and_every_listed_page_exists() {
    let on_disk: BTreeSet<String> = fs::read_dir(root().join("docs/guide"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("md"))
        .map(|path| format!("guide/{}", path.file_name().unwrap().to_string_lossy()))
        .collect();
    let listed: BTreeSet<String> = coverage().chapter.into_iter().map(|c| c.file).collect();
    assert_eq!(on_disk, listed);
}

#[test]
fn chapters_cite_only_the_sections_listed_for_them() {
    for chapter in coverage().chapter {
        let text = page(&chapter.file);
        assert!(
            !text.contains("§§"),
            "{}: write one § per citation",
            chapter.file
        );
        let listed: BTreeSet<String> = chapter.sections.into_iter().collect();
        let unlisted: Vec<String> = articles(&text).difference(&listed).cloned().collect();
        assert!(
            unlisted.is_empty(),
            "{} cites {unlisted:?}, which coverage.toml does not list for it",
            chapter.file
        );
    }
}

#[test]
fn listed_sections_are_cited_by_the_code() {
    let implemented = implemented_articles();
    for chapter in coverage().chapter {
        for section in chapter.sections {
            assert!(
                implemented.contains(&section),
                "§ {section} is listed for {} but src/ and rules/ do not cite it",
                chapter.file
            );
        }
    }
}

#[test]
fn parameter_values_appear_on_the_page_that_cites_them() {
    let Coverage {
        guide_year,
        chapter: chapters,
    } = coverage();
    let path = root()
        .join("rules")
        .join(guide_year.to_string())
        .join("parameters.toml");
    let parameters: BTreeMap<String, Parameter> = toml::from_str(&read(&path)).unwrap();
    for (name, parameter) in parameters {
        let value = euros(parameter.value_eur);
        let explained = chapters.iter().any(|chapter| {
            let text = page(&chapter.file);
            text.contains(&parameter.citation) && text.contains(&value)
        });
        assert!(
            explained,
            "{name}: no guide page cites `{}` together with {value}",
            parameter.citation
        );
    }
}

#[test]
fn every_guide_page_has_exactly_one_icon() {
    for chapter in coverage().chapter {
        let text = page(&chapter.file);
        let icons: Vec<&str> = text
            .lines()
            .filter_map(|line| {
                line.strip_prefix("{{#include assets/icons/")?
                    .strip_suffix("}}")
            })
            .collect();
        assert_eq!(
            icons.len(),
            1,
            "{}: needs exactly one icon, found {}",
            chapter.file,
            icons.len()
        );
        let lines: Vec<&str> = text.lines().collect();
        let icon_at = lines
            .iter()
            .position(|line| line.starts_with("{{#include assets/icons/"))
            .unwrap_or(usize::MAX);
        let heading_at = lines
            .iter()
            .position(|line| line.starts_with("# "))
            .unwrap_or(usize::MAX);
        assert!(
            icon_at < heading_at,
            "{}: the icon must sit above the H1 so it floats left of the title",
            chapter.file
        );
        let icon = root().join("docs/guide/assets/icons").join(icons[0]);
        assert!(
            icon.exists(),
            "{}: missing {}",
            chapter.file,
            icon.display()
        );
        let svg = read(&icon);
        let opening = svg
            .lines()
            .find(|line| line.starts_with("<svg"))
            .unwrap_or("");
        assert!(
            opening.ends_with('>'),
            "{}: the <svg> tag must sit on one line, or mdBook renders it as text",
            icon.display()
        );
    }
}

#[test]
fn every_illustration_is_labelled_as_one() {
    for chapter in coverage().chapter {
        for line in page(&chapter.file).lines() {
            let Some(name) = line
                .strip_prefix("{{#include assets/illustrations/")
                .and_then(|rest| rest.strip_suffix("}}"))
            else {
                continue;
            };
            let svg = read(&root().join("docs/guide/assets/illustrations").join(name));
            assert!(
                svg.contains("<title>Illustration"),
                "{}: {name} must be titled as an illustration",
                chapter.file
            );
        }
    }
}

#[test]
fn guide_writes_fiskus_in_lowercase() {
    for chapter in coverage().chapter {
        assert!(
            !page(&chapter.file).contains("Fiskus"),
            "{}: write fiskus in lowercase",
            chapter.file
        );
    }
}

fn internals_pages() -> Vec<PathBuf> {
    let mut pages = Vec::new();
    files_under(&root().join("docs/internals"), "md", &mut pages);
    pages
}

// Paths of the `{{#include ...}}` lines in a page, relative to the page.
fn included(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(|line| line.strip_prefix("{{#include ")?.strip_suffix("}}"))
        .collect()
}

fn is_icon(line: &str) -> bool {
    line.starts_with("{{#include") && line.contains("icons/")
}

// Targets named in backticks as `make <target>`. A `<name>` placeholder is skipped.
fn named_make_targets(text: &str) -> Vec<String> {
    text.match_indices("`make ")
        .map(|(index, pattern)| {
            text[index + pattern.len()..]
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                .collect::<String>()
        })
        .filter(|name| !name.is_empty() && !name.ends_with('-'))
        .collect()
}

#[test]
fn internals_pages_have_one_icon_above_the_title() {
    for page in internals_pages() {
        let text = read(&page);
        let lines: Vec<&str> = text.lines().collect();
        let icons = lines.iter().filter(|line| is_icon(line)).count();
        assert_eq!(
            icons,
            1,
            "{}: needs exactly one icon, found {icons}",
            page.display()
        );
        let icon = included(&text)
            .into_iter()
            .find(|path| path.contains("icons/"))
            .map(|path| page.parent().unwrap().join(path))
            .unwrap_or_default();
        assert!(
            icon.exists(),
            "{}: missing {}",
            page.display(),
            icon.display()
        );
        let icon_at = lines
            .iter()
            .position(|line| is_icon(line))
            .unwrap_or(usize::MAX);
        let heading_at = lines
            .iter()
            .position(|line| line.starts_with("# "))
            .unwrap_or(usize::MAX);
        assert!(
            icon_at < heading_at,
            "{}: the icon must sit above the H1",
            page.display()
        );
    }
}

#[test]
fn internals_illustrations_are_labelled_as_illustrations() {
    for page in internals_pages() {
        for path in included(&read(&page))
            .into_iter()
            .filter(|path| path.contains("illustrations/"))
        {
            let file = page.parent().unwrap().join(path);
            assert!(
                read(&file).contains("<title>Illustration"),
                "{}: {} must be titled as an illustration",
                page.display(),
                file.display()
            );
        }
    }
}

#[test]
fn internals_test_page_names_every_kani_proof() {
    let mut sources = Vec::new();
    files_under(&root().join("src"), "rs", &mut sources);
    let mut proofs = Vec::new();
    for source in sources {
        let text = read(&source);
        let lines: Vec<&str> = text.lines().collect();
        proofs.extend(
            lines
                .windows(2)
                .filter(|pair| pair[0].trim() == "#[kani::proof]")
                .filter_map(|pair| pair[1].trim().strip_prefix("fn ")?.split('(').next())
                .map(str::to_string),
        );
    }
    assert!(!proofs.is_empty(), "no Kani proofs found in src/");
    let page = read(&root().join("docs/internals/test/README.md"));
    for proof in proofs {
        assert!(
            page.contains(&proof),
            "docs/internals/test/README.md does not name the Kani proof {proof}"
        );
    }
}

#[test]
fn internals_pages_only_name_make_targets_that_exist() {
    let makefile = read(&root().join("Makefile"));
    let targets: BTreeSet<&str> = makefile
        .lines()
        .filter_map(|line| line.split_once(':').map(|(name, _)| name))
        .filter(|name| {
            name.starts_with(|c: char| c.is_ascii_lowercase())
                && name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        })
        .collect();
    for page in internals_pages() {
        for target in named_make_targets(&read(&page)) {
            assert!(
                targets.contains(target.as_str()),
                "{}: `make {target}` is not a Makefile target",
                page.display()
            );
        }
    }
}
