use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub(crate) fn estimate_reading_time(page_content: &str) -> u16 {
    // Define the average reading speed in words per minute
    const AVERAGE_READING_SPEED: usize = 200;

    // Count the number of words on the page
    let word_count = page_content.split_whitespace().count();

    // Calculate the estimated reading time in minutes
    let reading_time = (word_count as f64 / AVERAGE_READING_SPEED as f64).ceil() as u16;

    reading_time
}

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
