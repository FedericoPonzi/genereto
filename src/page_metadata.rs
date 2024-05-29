use crate::parser::get_anchor_id_from_title;
use chrono::NaiveDate;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::Path;

const DESCRIPTION_LENGTH: usize = 150;

/// Included from the top of an article file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetadataRaw {
    /// Title of the page
    pub title: String,
    /// Publish date as string
    #[serde(default)]
    pub publish_date: String,
    /// Defaults to false. If true, this article will not be processed.
    #[serde(default = "bool::default")]
    pub is_draft: bool,
    /// Keywords for this article
    pub keywords: String,
    /// Defaults to false. If true, it will add a Table Of Contents.
    #[serde(default = "bool::default")]
    pub show_table_of_contents: bool,
    /// If empty, the first 150 chars will be used as description.
    pub description: Option<String>,
    pub cover_image: Option<String>,
}

impl Display for PageMetadataRaw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Title: {}", self.title)
    }
}

/// Derived from PageMetadata and few more fields
#[derive(Debug, Clone)]
pub struct PageMetadata {
    pub title: String,
    pub publish_date: String,
    pub keywords: String,
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
    pub cover_image: String,
    pub is_draft: bool,
}

impl PageMetadata {
    pub fn new(
        mut page_metadata: PageMetadataRaw,
        page_content: &str,
        file_path: &Path,
        default_cover_image: &str,
    ) -> Self {
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".md", ".html");
        let table_of_contents = page_metadata
            .show_table_of_contents
            .then(|| generate_table_of_contents(page_content))
            .unwrap_or_default();
        let has_todos = contains_todos(page_content);
        if has_todos && !page_metadata.is_draft {
            info!("File {} has todos - setting is_draft to true.", file_name);
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
            cover_image: Self::get_cover_image(
                default_cover_image,
                page_metadata.cover_image.as_ref(),
                &file_name,
            ),
            title: page_metadata.title,
            keywords: page_metadata.keywords,
            publish_date: page_metadata.publish_date,
            is_draft: page_metadata.is_draft,
            file_name,
            table_of_contents,
        }
    }
    fn get_cover_image(
        default_cover_image: &str,
        page_cover_image: Option<&String>,
        file_name: &str,
    ) -> String {
        match page_cover_image {
            Some(cover_image) => format!("{}/{}", &file_name.replace(".html", ""), cover_image),
            None => default_cover_image.to_string(),
        }
    }
    pub fn get_variables(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("$GENERETO['title']", self.title.trim()),
            ("$GENERETO['publish_date']", &self.publish_date),
            ("$GENERETO['last_modified_date']", &self.last_modified_date),
            ("$GENERETO['read_time_minutes']", &self.reading_time_mins),
            ("$GENERETO['keywords']", self.keywords.trim()),
            ("$GENERETO['description']", self.description.trim()),
            ("$GENERETO['file_name']", &self.file_name),
            ("$GENERETO['table_of_contents']", &self.table_of_contents),
            ("$GENERETO['cover_image']", &self.cover_image),
        ]
    }

    // Apply variables to the final page.
    pub(crate) fn apply(&self, mut final_page: String) -> String {
        for i in self.get_variables() {
            final_page = final_page.replace(i.0, i.1);
        }
        final_page
    }
}

impl PartialOrd for PageMetadata {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.publish_date.cmp(&other.publish_date).reverse())
    }
}

impl PartialEq for PageMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.publish_date == other.publish_date
    }
}

impl Eq for PageMetadata {}

impl Display for PageMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Title: {}", self.title)
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
    let publish_date = if publish_date.is_empty() {
        &last_modified_date
    } else {
        publish_date
    };
    let last_update_as_date = NaiveDate::parse_from_str(&last_modified_date, "%Y-%m-%d").unwrap();
    let publish_date_as_date = NaiveDate::parse_from_str(publish_date, "%Y-%m-%d").unwrap();
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

/// Some filtering the get_description should be doing:
/// 1. remove links if any. Otherwise the markdown will end up in the preview.
/// 2. remove any whitespace at the start and at the end.
/// 3. remove any markdown if it's a title.
fn get_description(article: &str, limit: usize) -> String {
    let mut buff = String::new();
    for line in article.lines() {
        if line.trim().starts_with('#') {
            // skip it, it's usually a title like introduction.
        } else {
            buff.push_str(line);
        }
        buff = remove_links(buff);
        buff.push('\n');
        if buff.len() >= limit {
            break;
        }
    }
    use pulldown_cmark::{Event, Parser};
    // Extract plaintext
    let parser = Parser::new(&buff);
    let mut plaintext = String::new();
    for event in parser {
        match event {
            Event::Text(text) => plaintext.push_str(&text),
            Event::Code(text) => plaintext.push_str(&text),
            Event::Html(text) => plaintext.push_str(&text),
            Event::SoftBreak | Event::HardBreak => plaintext.push('\n'),
            _ => (),
        }
    }

    truncate_text(&plaintext.trim(), limit)
}

fn remove_links(buff: String) -> String {
    let re = Regex::new(r"\[([^)]*)\]\([^)]*\)").unwrap();

    let result = re.replace_all(&buff, |caps: &regex::Captures| {
        if let Some(title) = caps.get(1) {
            title.as_str().to_string()
        } else {
            caps[0].to_string()
        }
    });

    result.into_owned()
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
    git_cmd.current_dir(file_path.parent().unwrap());
    git_cmd.arg("log");
    git_cmd.arg("-1");
    git_cmd.arg("--format=%cd");
    git_cmd.arg("--date=short");
    git_cmd.arg(file_path.file_name().unwrap());
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
        const EXPECTED: &str = "This is a test description.";
        assert_eq!(get_description(TEST_INPUT, 100), EXPECTED);

        const TEST_INPUT_LINK: &str = "hello world! [how](http://google.com) are you?";
        const EXPECTED_LINK: &str = "hello world! how are you?";
        assert_eq!(
            get_description(TEST_INPUT_LINK, TEST_INPUT_LINK.len()),
            EXPECTED_LINK
        );
    }

    #[test]
    fn test_todos() {
        const TEST_INPUT: &str = "## Introduction {#introduction}\
        \nThis is a test description. $GENERETO{TODO: finish this page}";
        assert!(contains_todos(TEST_INPUT));
    }
}
