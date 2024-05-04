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
    // takes date as "2018-08-06" and returns "Mon, 06 Aug 2018 00:00:00 UTC"
    if let Ok(parsed_date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        error!("Parsed date: {}", parsed_date);
        parsed_date.format("%a, %d %b %Y 00:00:00 UTC").to_string()
    } else {
        error!("Error parsing date: {}, will use it as date string.", date);
        date.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_complaint_date() {
        assert_eq!(
            get_complaint_date("2018-08-06"),
            "Mon, 06 Aug 2018 00:00:00 UTC"
        );
    }
}
