use chrono::NaiveDate;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::Path;

const DESCRIPTION_LENGTH: usize = 150;

/// Included from a page file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetadata {
    /// Title of the page
    pub title: String,
    /// Publish date as string
    pub publish_date: String,
    /// Defaults to false. If true this article will not be processed.
    #[serde(default = "bool::default")]
    pub is_draft: bool,
    /// Keywords for this article
    pub keywords: String,
    /// Defaults to false. If true it will add a Table Of Contents.
    #[serde(default = "bool::default")]
    pub show_table_of_contents: bool,
    /// If empty, the first 150 chars will be used as description.
    pub description: Option<String>,
}
impl Display for PageMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Title: {}", self.title)
    }
}

/// Derived from PageMetadata and few more fields
#[derive(Debug, Clone)]
pub struct GeneretoMetadata {
    pub page_metadata: PageMetadata,
    /// Reading time in minutes
    pub reading_time_mins: String,
    /// Description of the page (metadata headers, etc.)
    pub description: String,
    /// Filename of the page
    pub file_name: String,
    /// Table of contents generated from headings.
    pub table_of_contents: String,
    /// Derived from git.
    pub last_modified_date: String,
}

impl GeneretoMetadata {
    pub fn new(
        mut page_metadata: PageMetadata,
        page_content: &str,
        file_name: String,
        file_path: &Path,
    ) -> Self {
        let table_of_contents = page_metadata
            .show_table_of_contents
            .then(|| generate_table_of_contents(page_content))
            .unwrap_or_default();
        let has_todos = contains_todos(page_content);
        if has_todos && !page_metadata.is_draft {
            println!("File {} has todos - setting is_draft to true.", file_name);
        }
        page_metadata.is_draft = page_metadata.is_draft || has_todos;
        page_metadata.title = if page_metadata.is_draft {
            format!("[DRAFT] {}", page_metadata.title)
        } else {
            page_metadata.title
        };

        Self {
            last_modified_date: get_last_modified_date(&page_metadata.publish_date, file_path),
            reading_time_mins: estimate_reading_time(page_content).to_string(),
            description: get_description(page_content, DESCRIPTION_LENGTH),
            page_metadata,
            file_name,
            table_of_contents,
        }
    }
    pub fn get_variables(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("$GENERETO['title']", &self.page_metadata.title.trim()),
            (
                "$GENERETO['publish_date']",
                &self.page_metadata.publish_date,
            ),
            ("$GENERETO['last_modified_date']", &self.last_modified_date),
            ("$GENERETO['read_time_minutes']", &self.reading_time_mins),
            ("$GENERETO['keywords']", self.page_metadata.keywords.trim()),
            ("$GENERETO['description']", self.description.trim()),
            ("$GENERETO['file_name']", &self.file_name),
            ("$GENERETO['table_of_contents']", &self.table_of_contents),
        ]
    }
}

impl PartialOrd for GeneretoMetadata {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.page_metadata
                .publish_date
                .cmp(&other.page_metadata.publish_date)
                .reverse(),
        )
    }
}

impl PartialEq for GeneretoMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.page_metadata.publish_date == other.page_metadata.publish_date
    }
}

impl Eq for GeneretoMetadata {}

impl Display for GeneretoMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Title: {}", self.page_metadata.title)
    }
}

/// Get last modified date of the file as string
/// Uses git to get last modified date. It will return the most recent date between the last update and the publish date
fn get_last_modified_date(publish_date: &str, file_path: &Path) -> String {
    let last_modified_date = get_last_modified_date_of_file_from_git(file_path)
        .unwrap_or_else(|| publish_date.to_string());
    if last_modified_date.is_empty() {
        return publish_date.to_string();
    }
    let last_update_as_date = NaiveDate::parse_from_str(&last_modified_date, "%Y-%m-%d").unwrap();
    let publish_date_as_date = NaiveDate::parse_from_str(&publish_date, "%Y-%m-%d").unwrap();
    if last_update_as_date < publish_date_as_date {
        publish_date.to_string()
    } else {
        last_modified_date
    }
}

/// Search page_content for "$GENERETO{TO DO: string.
fn contains_todos(page_content: &str) -> bool {
    // search page_content for "$GENERETO{TO DO string.
    page_content.contains(r"$GENERETO{TODO")
}

pub(crate) fn estimate_reading_time(page_content: &str) -> u16 {
    // Define the average reading speed in words per minute
    // https://www.sciencedirect.com/science/article/abs/pii/S0749596X19300786
    const AVERAGE_READING_SPEED: usize = 238;

    // Count the number of words on the page
    let word_count = page_content
        .split_whitespace()
        .filter(|w| w.chars().any(|c| c.is_alphabetic())) // Filter words with only symbols
        .count();

    // Calculate the estimated reading time in minutes
    (word_count as f64 / AVERAGE_READING_SPEED as f64).ceil() as u16
}

fn get_description(article: &str, limit: usize) -> String {
    let mut buff = String::new();
    for line in article.lines() {
        if line.trim().starts_with('#') {
            // titles come with the id. Remove the id from the title.
            // Example: `Introduction {#introduction}` becomes `Introduction`.
            buff.push_str(
                remove_after_last_character(line.trim_start_matches('#').trim(), '{').trim(),
            );
        } else {
            buff.push_str(line);
        }
        buff.push('\n');
        if buff.len() >= limit {
            break;
        }
    }
    truncate_text(&buff, limit)
}

fn truncate_text(article: &str, limit: usize) -> String {
    let mut truncated = String::from(article);

    if truncated.chars().count() > limit {
        truncated.truncate(limit);

        if let Some(last_space) = truncated.rfind(' ') {
            truncated.truncate(last_space);
        }

        truncated.push_str("...");
    }

    truncated
}

// On error, if git is not available in the system, it will return None.
fn get_last_modified_date_of_file_from_git(file_path: &Path) -> Option<String> {
    let mut git_cmd = std::process::Command::new("git");
    git_cmd.arg("-C");
    git_cmd.arg(file_path.parent().unwrap());
    git_cmd.arg("log");
    git_cmd.arg("-1");
    git_cmd.arg("--format=%cd");
    git_cmd.arg("--date=short");
    git_cmd.arg(file_path);
    let output = git_cmd.output().ok()?;
    let date = String::from_utf8(output.stdout).unwrap().trim().to_string();
    Some(date)
}

// TODO: What happens with overlaps of sections with same name?
/// Generate the table of contents
/// out is in html. I could change it to output markdown instead.
fn generate_table_of_contents(markdown: &str) -> String {
    let mut toc = String::new();
    let mut in_code_block = false;
    // current_depth = 1 is for a single '#' or an <h1>. HTML semantics wants that an article should
    // only have one h1 - the title of the page - which is above the ToC.
    let mut current_depth = 2;
    for line in markdown.lines() {
        if line.trim().starts_with("```") {
            in_code_block = !in_code_block;
        }
        if line.trim().starts_with('#') && !in_code_block {
            let last_depth = line.chars().take_while(|ch| *ch == '#').count();
            if current_depth > last_depth {
                toc.push_str("\n</ul>\n");
            }
            if current_depth < last_depth {
                toc.push_str("\n<ul>\n");
            }
            if current_depth == last_depth {
                toc.push_str("</li>\n");
            }
            // titles come with the id. Remove the id from the title.
            // Example: `Introduction {#introduction}` becomes `Introduction`.
            let title = remove_after_last_character(line.trim_start_matches('#').trim(), '{');
            let title = title.trim();
            let class_name = format!("table_of_contents-indent-{}", last_depth);
            let anchor = get_anchor_id_from_title(title);

            toc.push_str(&format!(
                "<li><a href=\"#{}\" class=\"{}\">{}</a>",
                anchor, class_name, title
            ));

            current_depth = last_depth;
        }
    }
    // todo: add a test.
    // this is for adding missing </ul> when the toc finishes with something less than h2 (so h3 or h4).
    while current_depth > 2 {
        toc.push_str("</ul>\n");
        current_depth -= 1;
    }

    format!("<ul class=\"table_of_contents\">\n{}</ul>", &toc[6..])
}

pub fn get_anchor_id_from_title(title: &str) -> String {
    remove_special_characters(title)
        .replace(' ', "-")
        .to_lowercase()
}

fn remove_special_characters(input: &str) -> String {
    // Define a regular expression pattern to match special characters
    let pattern = Regex::new(r"[^a-zA-Z0-9\s]+").unwrap();

    // Replace matches with an empty string to remove special characters
    pattern.replace_all(input, "").to_string()
}

// remove everything after the last occurrence of `character`. Check the tests.
fn remove_after_last_character(input: &str, character: char) -> String {
    if let Some(index) = input.rfind(character) {
        input[..index].to_string()
    } else {
        input.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::page_metadata::{
        contains_todos, generate_table_of_contents, get_description, remove_after_last_character,
    };
    use std::assert_eq;

    #[test]
    fn test_table_of_contents() {
        let test_input = r#"
## Introduction
## Getting Started
### Installation
## Basic Usage
# Advanced Features!!!!
"#;
        let expected: &str = "
<ul class=\"table_of_contents\">
<li><a href=\"#introduction\" class=\"table_of_contents-indent-2\">Introduction</a></li>
<li><a href=\"#getting-started\" class=\"table_of_contents-indent-2\">Getting Started</a>
<ul>
<li><a href=\"#installation\" class=\"table_of_contents-indent-3\">Installation</a>
</ul>
<li><a href=\"#basic-usage\" class=\"table_of_contents-indent-2\">Basic Usage</a>
</ul>
<li><a href=\"#advanced-features\" class=\"table_of_contents-indent-1\">Advanced Features!!!!</a></ul>";

        let table_of_contents = generate_table_of_contents(test_input);
        assert_eq!(table_of_contents.trim(), expected.trim());
        let test_input2 = r#"
## Introduction
## Getting Started
### Installation
## Basic Usage
### Advanced Features!!!!
"#;

        let expected2 = "<ul class=\"table_of_contents\">
<li><a href=\"#introduction\" class=\"table_of_contents-indent-2\">Introduction</a></li>
<li><a href=\"#getting-started\" class=\"table_of_contents-indent-2\">Getting Started</a>
<ul>
<li><a href=\"#installation\" class=\"table_of_contents-indent-3\">Installation</a>
</ul>
<li><a href=\"#basic-usage\" class=\"table_of_contents-indent-2\">Basic Usage</a>
<ul>
<li><a href=\"#advanced-features\" class=\"table_of_contents-indent-3\">Advanced Features!!!!</a></ul>
</ul>";
        let table_of_contents = generate_table_of_contents(test_input2);
        assert_eq!(table_of_contents.trim(), expected2.trim());
    }

    #[test]
    fn test_remove_after_last_character() {
        assert_eq!(
            remove_after_last_character("QA {hello} {id}", '{').trim(),
            "QA {hello}"
        );
        assert_eq!(
            remove_after_last_character("Some text without braces", '{'),
            "Some text without braces"
        );
    }

    #[test]
    fn test_get_description() {
        const TEST_INPUT: &str = "## Introduction {#introduction}\
        \nThis is a test description.";
        // it adds an extra new line at the end, but I don't care
        const EXPECTED: &str = "Introduction\nThis is a test description.\n";
        assert_eq!(get_description(TEST_INPUT, 100), EXPECTED);
    }

    #[test]
    fn test_todos() {
        const TEST_INPUT: &str = "## Introduction {#introduction}\
        \nThis is a test description. $GENERETO{TODO: finish this page}";
        assert!(contains_todos(TEST_INPUT));
    }
}
