#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};

use anyhow::Context;
use config::GeneretoConfig;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

mod config;

const START_PATTERN: &str = "<!-- start_content -->";
const END_PATTERN: &str = "<!-- end_content -->";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PageMetadata {
    /// Title of the page
    title: String,
    /// Publish date as string
    publish_date: Option<String>,
    /// Read time in minutes, will eventually be automated.
    read_time_minutes: Option<u16>,
    /// Defaults to false. If true this article will not be processed.
    #[serde(default = "bool::default")]
    is_draft: bool,
}

/// project: path to the project.
pub fn run(project_path: PathBuf) -> anyhow::Result<()> {
    // todo: move to config.

    GeneretoConfig::validate_project_folders(&project_path)?;
    let genereto_config = GeneretoConfig::load_from_path(project_path)?;

    if genereto_config.output_dir_path.exists() {
        fs::remove_dir_all(&genereto_config.output_dir_path)?;
    }
    fs::create_dir_all(&genereto_config.output_dir_path)?;

    build(
        genereto_config.content_path,
        genereto_config.template_dir_path,
        genereto_config.output_dir_path,
    )?;
    Ok(())
}

fn build(content_dir: PathBuf, template: PathBuf, output_dir: PathBuf) -> anyhow::Result<()> {
    debug!(
        "Gonna build for {} with template {} and out_page {}",
        content_dir.display(),
        template.display(),
        output_dir.display()
    );
    // iterate on all files in content folder, and call build_page
    let mut file_list = vec![];
    for entry in fs::read_dir(content_dir)? {
        debug!("Entry: {:?}", entry);
        let entry_path = entry?.path();
        let filename = entry_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".md", ".html");

        // assume entry is a file (for now?)
        build_page(
            &template.join("blog.html"),
            entry_path,
            output_dir.join(&filename),
        )
        .context("Failed to build page.")?;
        file_list.push(filename);
    }

    // Create an index.html
    build_index_page(
        &template.join("index.html"),
        file_list,
        output_dir.join("index.html"),
    )
    .context("Failed to build index page.")?;

    Ok(())
}

fn build_index_page(
    template: &Path,
    file_list: Vec<String>,
    out_page: PathBuf,
) -> anyhow::Result<()> {
    debug!(
        "Gonna build index page for {} with template {} and out_page {}",
        file_list.join(", "),
        template.display(),
        out_page.display()
    );
    let mut links = "".to_string();
    for i in file_list.into_iter().filter(|el| el != "error.html") {
        links.push_str(&format!("<li><a href=\"{}\">{}</a></li>", i, i));
    }
    let mut template_view = fs::read_to_string(template)?;
    let html_content = links;
    let start = template_view.find(START_PATTERN).unwrap();
    let end = template_view.find(END_PATTERN).unwrap();
    template_view.replace_range(start..end + END_PATTERN.len(), &html_content);

    //println!("Result:\n {}", template_view);
    fs::write(out_page, template_view).context("Failed writing to output page")?;
    Ok(())
}

fn build_page(template: &Path, in_page: PathBuf, out_page: PathBuf) -> anyhow::Result<()> {
    debug!(
        "Gonna build page for {} with template {} and out_page {}",
        in_page.display(),
        template.display(),
        out_page.display()
    );
    // 1 read in_page.
    let page = fs::read_to_string(in_page)?;
    let pattern = Regex::new(r"---+\n").unwrap();
    let mut fields: Vec<&str> = pattern.splitn(&page, 2).collect();
    let (metadata, content) = (fields.remove(0), fields.remove(0));
    debug!("Metadata: {:?}", metadata);
    debug!("Content: {}", content);

    let metadata: PageMetadata =
        serde_yaml::from_str(metadata).expect("Failed to deserialize metadata");
    let mut final_page = fs::read_to_string(template)?;
    let html_content = load_markdown(content);
    let start = final_page.find(START_PATTERN).unwrap();
    let end = final_page.find(END_PATTERN).unwrap();

    final_page.replace_range(start..end + END_PATTERN.len(), &html_content);
    let final_page = apply_variables(metadata, final_page);

    fs::write(out_page, final_page).context("Failed writing to output page")?;
    Ok(())
}

// Apply variables to the final page.
fn apply_variables(metadata: PageMetadata, mut final_page: String) -> String {
    for i in [
        ("$GENERETO['title']", metadata.title),
        (
            "$GENERETO['publish_date']",
            metadata.publish_date.unwrap_or_else(|| "".to_string()),
        ),
        (
            "$GENERETO['read_time_minutes']",
            metadata.read_time_minutes.unwrap_or(0).to_string(),
        ),
    ] {
        final_page = final_page.replace(i.0, &i.1);
    }
    final_page
}

fn load_markdown(markdown_input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
