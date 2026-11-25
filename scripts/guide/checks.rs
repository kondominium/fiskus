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

// The language rules of docs/contributing/language.md, checked on every prose page.

// Pages the rules apply to: the docs book, the agent skills and the root README. A page generated
// by `make charts` starts with a `<!-- Generated` marker and is checked by the drift check instead.
fn language_pages() -> Vec<PathBuf> {
    let mut found = Vec::new();
    files_under(&root().join("docs"), "md", &mut found);
    files_under(&root().join(".agents/skills"), "md", &mut found);
    found.push(root().join("README.md"));
    found.retain(|path| !read(path).starts_with("<!-- Generated"));
    found.sort();
    found
}

// The lines a reader reads: fenced code, inline code, HTML comments and URLs are left out.
fn prose_lines(text: &str) -> Vec<(usize, String)> {
    let mut in_fence = false;
    let mut lines = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence {
            lines.push((index + 1, prose_of(line)));
        }
    }
    lines
}

// Text in double quotes is an example of wrong usage, such as "We test every figure.", so it is left out too.
fn prose_of(line: &str) -> String {
    let outside_code: String = line
        .split('`')
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, part)| part)
        .collect::<String>()
        .split('"')
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, part)| part)
        .collect();
    let mut without_comments = String::new();
    let mut rest = outside_code.as_str();
    while let Some(start) = rest.find("<!--") {
        without_comments.push_str(rest.get(..start).unwrap_or_default());
        rest = rest.get(start..).unwrap_or_default();
        let Some(end) = rest.find("-->") else {
            rest = "";
            break;
        };
        rest = rest.get(end + "-->".len()..).unwrap_or_default();
    }
    without_comments.push_str(rest);
    without_comments
        .split_whitespace()
        .filter(|word| !word.contains("://"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn words(line: &str) -> impl Iterator<Item = &str> {
    line.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
}

// Fiskus is the subject of a sentence, so the docs never say we, our or us.
fn we_our_us(line: &str) -> Vec<&'static str> {
    let found = words(line).any(|word| matches!(word.to_lowercase().as_str(), "we" | "our" | "us"));
    if found {
        vec!["write fiskus as the subject, not we, our or us"]
    } else {
        Vec::new()
    }
}

fn lowercase_fiskus(line: &str) -> Vec<&'static str> {
    if words(line).any(|word| word == "Fiskus" || word == "FISKUS") {
        vec!["write fiskus in lowercase"]
    } else {
        Vec::new()
    }
}

fn one_section_per_citation(line: &str) -> Vec<&'static str> {
    if line.contains("§§") {
        vec!["cite one section per citation, not §§"]
    } else {
        Vec::new()
    }
}

// Euro amounts are written `12 348 €`: a space between thousands, a point for cents, a space before €.
fn euro_format(line: &str) -> Vec<&'static str> {
    let mut reasons = Vec::new();
    for (position, _) in line.match_indices('€') {
        let before = line.get(..position).unwrap_or_default();
        let after = line.get(position + '€'.len_utf8()..).unwrap_or_default();
        if before.ends_with(|c: char| c.is_ascii_digit()) {
            reasons.push("put a space before €");
        }
        if after.starts_with(|c: char| c.is_ascii_digit()) {
            reasons.push("put € after the amount");
        }
        let Some(amount) = before.strip_suffix(' ') else {
            continue;
        };
        let reversed_run: String = amount
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
            .collect();
        let run: String = reversed_run.chars().rev().collect();
        if run.contains(',') {
            reasons.push("use a space between thousands and a point for cents");
        }
        let whole = run.split('.').next().unwrap_or_default();
        if whole.len() >= 4 && !run.contains(',') {
            reasons.push("group thousands with a space, as in 12 348 €");
        }
    }
    reasons
}

// Every breach of one rule, as `page:line: reason`, so a failure lists them all.
fn breaches(rule: fn(&str) -> Vec<&'static str>) -> Vec<String> {
    let mut found = Vec::new();
    for page in language_pages() {
        let relative = page
            .strip_prefix(root())
            .unwrap_or(&page)
            .display()
            .to_string();
        for (number, line) in prose_lines(&read(&page)) {
            for reason in rule(&line) {
                found.push(format!("{relative}:{number}: {reason}: {line}"));
            }
        }
    }
    found
}

// Whether a rule reported anything for a line.
fn flags(reasons: &[&str]) -> bool {
    !reasons.is_empty()
}

// The rules must flag what they describe, or a zero count would prove nothing.
#[test]
fn language_rules_flag_their_breaches_and_pass_the_right_form() {
    assert_eq!(euro_format("costs 12348 €").len(), 1);
    assert!(!flags(&euro_format("costs 12 348 €")));
    assert!(flags(&euro_format("costs 12 348,99 €")));
    assert!(flags(&euro_format("costs 12 348.99€")));
    assert!(flags(&euro_format("costs €12 348")));
    assert!(flags(&we_our_us("We test every figure.")));
    assert!(!flags(&we_our_us("fiskus tests every figure.")));
    assert!(flags(&lowercase_fiskus("Fiskus computes the tax.")));
    assert!(!flags(&lowercase_fiskus("fiskus computes the tax.")));
    assert!(flags(&one_section_per_citation("§§ 32a, 26 EStG")));
}

// Code, quotes of wrong usage, comments and URLs are not prose.
#[test]
fn prose_leaves_out_code_quotes_comments_and_urls() {
    assert!(!flags(&we_our_us(&prose_of("Use `we` in code only."))));
    assert!(!flags(&we_our_us(&prose_of(
        "Bad: \"We test every figure.\""
    ))));
    assert!(!flags(&we_our_us(&prose_of(
        "<!-- we -->fiskus computes it."
    ))));
    assert!(!flags(&we_our_us(&prose_of(
        "See https://example.org/us/page for details."
    ))));
}

#[test]
fn pages_write_fiskus_as_the_subject_not_we_our_or_us() {
    let found = breaches(we_our_us);
    assert!(found.is_empty(), "{}", found.join("\n"));
}

#[test]
fn pages_write_fiskus_in_lowercase() {
    let found = breaches(lowercase_fiskus);
    assert!(found.is_empty(), "{}", found.join("\n"));
}

#[test]
fn pages_cite_one_section_per_citation() {
    let found = breaches(one_section_per_citation);
    assert!(found.is_empty(), "{}", found.join("\n"));
}

#[test]
fn pages_write_euro_amounts_in_the_language_format() {
    let found = breaches(euro_format);
    assert!(found.is_empty(), "{}", found.join("\n"));
}
