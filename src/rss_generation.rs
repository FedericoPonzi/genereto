use crate::page_metadata::BlogArticleMetadata;
use anyhow::Result;
use chrono::NaiveDate;
use rss::{ChannelBuilder, ItemBuilder};
use std::path::Path;

pub fn generate_rss(
    website_title: String,
    url: String,
    description: String,
    metadatas: Vec<BlogArticleMetadata>,
    output_dir: &Path,
) -> Result<()> {
    dbg!(&metadatas);

    let channel = ChannelBuilder::default()
        .title(website_title)
        .link(&url)
        .description(description)
        .language("en-us".to_string())
        .items(articles_to_items(url, metadatas))
        .build();

    let rss = channel.to_string();
    std::fs::write(output_dir.join("rss.xml"), rss)?;
    Ok(())
}
fn articles_to_items(url: String, metadatas: Vec<BlogArticleMetadata>) -> Vec<rss::Item> {
    metadatas
        .into_iter()
        .filter(|md| !(md.is_draft || md.file_name == "error.html"))
        .map(|md| {
            ItemBuilder::default()
                .title(md.title)
                .link(format!("{}/{}", url, md.file_name))
                .description(md.description)
                .pub_date(get_complaint_date(&md.publish_date))
                .build()
        })
        .collect()
}

/// Returns RFC-822 date format
fn get_complaint_date(date: &str) -> String {
    // takes date as "2024-02-01" and returns 01 Feb 24
    if let Ok(parsed_date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        parsed_date.format("%d %b %y").to_string()
    } else {
        "Invalid date format".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_complaint_date() {
        assert_eq!(get_complaint_date("2024-01-01"), "01 Jan 24");
        assert_eq!(get_complaint_date("2024-02-01"), "01 Feb 24");
        assert_eq!(get_complaint_date("2027-10-01"), "01 Oct 27");
    }
}
