#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};

use anyhow::Context;
use config::GeneretoConfig;
use regex::Regex;
use std::fs;

mod config;
mod page_metadata;

use crate::page_metadata::GeneretoMetadata;
use page_metadata::PageMetadata;

const START_PATTERN: &str = "<!-- start_content -->";
const END_PATTERN: &str = "<!-- end_content -->";

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

        if entry_path.is_file() {
            let metadata = build_page(
                &template.join("blog.html"),
                entry_path,
                output_dir.join(&filename),
            )
            .context("Failed to build page.")?;
            file_list.push((filename, metadata));
        } else if entry_path.is_dir() {
            copy_directory_recursively(entry_path, output_dir.join(&filename))?;
        }
    }

    file_list.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    // Create an index.html
    build_index_page(
        &template.join("index.html"),
        file_list,
        output_dir.join("index.html"),
    )
    .context("Failed to build index page.")?;

    Ok(())
}

fn copy_directory_recursively<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dest: Q,
) -> std::io::Result<()> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    if src.is_file() {
        fs::copy(src, dest)?;
    } else if src.is_dir() {
        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }

        let entries = fs::read_dir(src)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            let dest_path = dest.join(entry.file_name());

            copy_directory_recursively(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}

fn build_index_page(
    template: &Path,
    file_list: Vec<(String, GeneretoMetadata)>,
    out_page: PathBuf,
) -> anyhow::Result<()> {
    let mut links = "".to_string();
    for i in file_list.into_iter().filter(|el| el.0 != "error.html") {
        let li_entry = &format!(
            "<li><a href=\"https://blog.fponzi.me/{}\">{}</a> - {} ({})</li>",
            i.1.file_name,
            i.1.page_metadata.title,
            i.1.page_metadata.publish_date,
            i.1.page_metadata.keywords,
        );
        links.push_str(li_entry);
    }
    let mut template_view = fs::read_to_string(template)?;
    let html_content = links;
    let start = template_view.find(START_PATTERN).unwrap();
    let end = template_view.find(END_PATTERN).unwrap();
    template_view.replace_range(start..end + END_PATTERN.len(), &html_content);

    fs::write(out_page, template_view).context("Failed writing to output page")?;
    Ok(())
}

fn build_page(
    template: &Path,
    in_page: PathBuf,
    out_page: PathBuf,
) -> anyhow::Result<GeneretoMetadata> {
    let file_name = out_page.file_name().unwrap().to_str().unwrap().to_string();
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
    let genereto_metadata = GeneretoMetadata::new(metadata, content, file_name);
    let final_page = apply_variables(&genereto_metadata, final_page);

    fs::write(out_page, final_page).context("Failed writing to output page")?;
    Ok(genereto_metadata)
}

// Apply variables to the final page.
fn apply_variables(metadata: &GeneretoMetadata, mut final_page: String) -> String {
    for i in metadata.get_variables() {
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
