use crate::page_metadata::GeneretoMetadata;
use anyhow::Result;
use rss::{ChannelBuilder, ItemBuilder};
use std::path::Path;

pub fn generate_rss(
    website_title: String,
    url: String,
    description: String,
    metadatas: Vec<(String, GeneretoMetadata)>,
    output_dir: &Path,
) -> Result<()> {
    dbg!(&metadatas);

    let channel = ChannelBuilder::default()
        .title(website_title)
        .link(url)
        .description(description)
        .language("en-us".to_string())
        .items(articles_to_items(metadatas))
        .build();

    let rss = channel.to_string();
    std::fs::write(output_dir.join("rss.xml"), rss)?;
    Ok(())
}
fn articles_to_items(metadatas: Vec<(String, GeneretoMetadata)>) -> Vec<rss::Item> {
    let mut ret = vec![];
    for (dest_filepath, md) in metadatas {
        if md.is_draft || md.file_name == "error.html" {
            continue;
        }
        let item = ItemBuilder::default()
            .title(md.title)
            .link(dest_filepath)
            .description(md.description)
            .pub_date(md.publish_date)
            .build();
        ret.push(item);
    }
    ret
}
