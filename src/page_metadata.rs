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
}

impl GeneretoMetadata {
    pub fn new(page_metadata: PageMetadata, page_content: &str, file_name: String) -> Self {
        let reading_time_mins = estimate_reading_time(page_content);
        let description = truncate_text(page_content, 150);
        Self {
            page_metadata,
            reading_time_mins: reading_time_mins.to_string(),
            description,
            file_name,
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
