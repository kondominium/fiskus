// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Keeps the guide in `docs/guide` in step with the code. See `docs/contributing/guide.md`.

#![expect(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a guide check fails by panicking with the file that broke it, so the test output names the file"
)]

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
struct Coverage {
    chapter: Vec<Chapter>,
}

#[derive(Deserialize)]
struct Chapter {
    file: String,
    sections: Vec<String>,
}

#[derive(Deserialize)]
struct RuleFile {
    basic_allowance: Parameter,
    source: Option<Reference>,
}

#[derive(Deserialize)]
struct Parameter {
    value_eur: i128,
    citation: String,
    reference: String,
}

#[derive(Deserialize)]
struct Reference {
    reference: String,
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn latest_rule_file() -> PathBuf {
    fs::read_dir(root().join("rules"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter_map(|path| {
            let year: u16 = path.file_stem()?.to_str()?.parse().ok()?;
            Some((year, path))
        })
        .max_by_key(|(year, _)| *year)
        .map(|(_, path)| path)
        .expect("rules/ has a year file")
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
                .take_while(char::is_ascii_alphanumeric)
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

// The guide quotes the basic allowance of the latest year in rules/, so a new year must update it.
#[test]
fn parameter_values_appear_on_the_page_that_cites_them() {
    let chapters = coverage().chapter;
    let path = latest_rule_file();
    let file: RuleFile = toml::from_str(&read(&path)).unwrap();
    let parameter = file.basic_allowance;
    let value = euros(parameter.value_eur);
    let explained = chapters.iter().any(|chapter| {
        let text = page(&chapter.file);
        text.contains(&parameter.citation) && text.contains(&value)
    });
    assert!(
        explained,
        "basic_allowance: no guide page cites `{}` together with {value}",
        parameter.citation
    );
}

#[test]
fn parameter_references_exist() {
    let base = "https://gitlab.com/kondominium/fiskus/-/raw/main/docs/references/";
    for entry in fs::read_dir(root().join("rules")).unwrap() {
        let path = entry.unwrap().path();
        let file: RuleFile = toml::from_str(&read(&path)).unwrap();
        let mut references = vec![("basic_allowance", file.basic_allowance.reference)];
        if let Some(source) = file.source {
            references.push(("source", source.reference));
        }
        for (name, reference) in references {
            assert!(
                reference.starts_with(base),
                "{}: [{name}] reference is not a GitLab main-branch link: {reference}",
                path.display(),
            );
            let basename = reference.trim_start_matches(base);
            assert!(
                root().join("docs/references").join(basename).is_file(),
                "{}: [{name}] reference {reference} has no local file",
                path.display(),
            );
        }
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
