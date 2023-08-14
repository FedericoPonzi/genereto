use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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
    /// Table of contents generated from headings
    pub table_of_contents: String,
}

impl GeneretoMetadata {
    pub fn new(page_metadata: PageMetadata, page_content: &str, file_name: String) -> Self {
        let reading_time_mins = estimate_reading_time(page_content);
        let description = truncate_text(page_content, 150);

        let table_of_contents = if page_metadata.show_table_of_contents {
            generate_table_of_contents(page_content)
        } else {
            String::new()
        };

        Self {
            page_metadata,
            reading_time_mins: reading_time_mins.to_string(),
            description,
            file_name,
            table_of_contents,
        }
    }
    pub fn get_variables(&self) -> Vec<(&'static str, &str)> {
        vec![
            ("$GENERETO['title']", &self.page_metadata.title),
            (
                "$GENERETO['publish_date']",
                &self.page_metadata.publish_date,
            ),
            ("$GENERETO['read_time_minutes']", &self.reading_time_mins),
            ("$GENERETO['keywords']", &self.page_metadata.keywords),
            ("$GENERETO['description']", &self.description),
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

pub(crate) fn estimate_reading_time(page_content: &str) -> u16 {
    // Define the average reading speed in words per minute
    const AVERAGE_READING_SPEED: usize = 200;

    // Count the number of words on the page
    let word_count = page_content.split_whitespace().count();

    // Calculate the estimated reading time in minutes
    let reading_time = (word_count as f64 / AVERAGE_READING_SPEED as f64).ceil() as u16;

    reading_time
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

// TODO: What happens with overlaps of sections with same name?
// todo for later: renders well but sublisting is not correct, it should be `<li>h2 title <ul><li>h3</li></ul></li>` right now is `<li>h2 title</li><ul><li>h3</li></ul>`.
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
            let depth = line.chars().take_while(|ch| *ch == '#').count();
            if current_depth > depth {
                toc.push_str("</ul>\n");
            }
            if current_depth < depth {
                toc.push_str("<ul>\n")
            }

            let title = remove_after_last_character(line.trim_start_matches('#').trim(), '{');
            let title = title.trim();
            let class_name = format!("table_of_contents-indent-{}", depth);
            let anchor = get_anchor_id_from_title(title);

            toc.push_str(&format!(
                "<li><a href=\"#{}\" class=\"{}\">{}</a></li>\n",
                anchor, class_name, title
            ));

            current_depth = depth;
        }
    }
    format!("<ul class=\"table_of_contents\">\n{}</ul>", toc)
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
    use crate::page_metadata::{generate_table_of_contents, remove_after_last_character};
    use std::{assert_eq, println};

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
<li><a href=\"#getting-started\" class=\"table_of_contents-indent-2\">Getting Started</a></li>
<ul>
<li><a href=\"#installation\" class=\"table_of_contents-indent-3\">Installation</a></li>
</ul>
<li><a href=\"#basic-usage\" class=\"table_of_contents-indent-2\">Basic Usage</a></li>
</ul>
<li><a href=\"#advanced-features\" class=\"table_of_contents-indent-1\">Advanced Features!!!!</a></li>
</ul>";

        let table_of_contents = generate_table_of_contents(test_input);
        println!("{table_of_contents}");
        assert_eq!(table_of_contents.trim(), expected.trim());
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
}
