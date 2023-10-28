#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use config::GeneretoConfig;
use regex::Regex;
use std::fs;

mod config;
mod page_metadata;

use crate::page_metadata::{get_anchor_id_from_title, GeneretoMetadata};
use page_metadata::PageMetadata;

const START_PATTERN: &str = "<!-- start_content -->";
const END_PATTERN: &str = "<!-- end_content -->";

/// project: path to the project.
pub fn run(project_path: PathBuf, skip_drafts: bool) -> anyhow::Result<()> {
    // todo: move to config.

    let genereto_config = GeneretoConfig::load_from_path(project_path)?;

    if genereto_config.output_dir_path.exists() {
        fs::remove_dir_all(&genereto_config.output_dir_path)?;
    }
    fs::create_dir_all(&genereto_config.output_dir_path)?;

    build(
        genereto_config.content_path,
        genereto_config.template_dir_path.clone(),
        genereto_config.output_dir_path.clone(),
        skip_drafts,
    )?;
    copy_resources(
        genereto_config.template_dir_path,
        genereto_config.output_dir_path,
    )?;
    Ok(())
}
fn copy_resources(template_dir_path: PathBuf, output_dir_path: PathBuf) -> anyhow::Result<()> {
    for entry in fs::read_dir(template_dir_path)? {
        let entry_path = entry?.path();
        if entry_path.is_dir() {
            copy_directory_recursively(
                entry_path.clone(),
                output_dir_path.join(entry_path.file_name().unwrap().to_str().unwrap()),
            )?;
        }
    }
    Ok(())
}

fn build(
    content_dir: PathBuf,
    template: PathBuf,
    output_dir: PathBuf,
    skip_drafts: bool,
) -> anyhow::Result<()> {
    debug!(
        "Gonna build for {} with template {} and out_page {}",
        content_dir.display(),
        template.display(),
        output_dir.display()
    );
    // iterate on all files in content folder, and call build_page
    let mut file_list = vec![];
    let gen_dest_filename =
        |p: &Path| -> Option<String> { Some(p.file_name()?.to_str()?.replace(".md", ".html")) };

    for entry in fs::read_dir(content_dir)? {
        debug!("Entry: {:?}", entry);
        let entry_path = entry?.path();
        let dest_filename =
            gen_dest_filename(&entry_path).ok_or(anyhow!("failed to gen dest_filename"))?;
        if entry_path.is_file() {
            let metadata = build_page(
                &template.join("blog.html"),
                &entry_path,
                output_dir.join(&dest_filename),
                skip_drafts,
            )
            .with_context(|| format!("Failed to build page {entry_path:?}"))?;
            if let Some(metadata) = metadata {
                file_list.push((dest_filename, metadata));
            }
        } else if entry_path.is_dir() {
            if entry_path.file_name().unwrap().to_str().unwrap() == "res" {
                warn!("Please put the 'res' folder in the templating folder. Skipping copy.");
                continue;
            }
            copy_directory_recursively(entry_path, output_dir.join(&dest_filename))?;
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
    for (_, metadata) in file_list.into_iter().filter(|el| el.0 != "error.html") {
        let li_entry = format!(
            "<li><a href=\"{}\">{}</a> - {} ({})</li>",
            metadata.file_name,
            metadata.page_metadata.title,
            metadata.page_metadata.publish_date,
            metadata.page_metadata.keywords,
        );
        links.push_str(&li_entry);
    }
    let mut template_view = fs::read_to_string(template)?;
    let html_content = links;
    let start = template_view.find(START_PATTERN).unwrap();
    let end = template_view.find(END_PATTERN).unwrap();
    template_view.replace_range(start..end + END_PATTERN.len(), &html_content);

    fs::write(out_page, template_view).context("Failed writing to output page")?;
    Ok(())
}

/// Build a page from a template and a content file.
/// The content file is read in, the metadata is extracted,
/// the content is loaded and converted to html,
/// the html is inserted into the template,
/// the template is written to the output folder.
/// The metadata is returned to the caller.
/// If skip_drafts is true, and the page is a draft,
/// None is returned.
fn build_page(
    template: &Path,
    in_page: &Path,
    out_page: PathBuf,
    skip_drafts: bool,
) -> anyhow::Result<Option<GeneretoMetadata>> {
    let file_name = out_page.file_name().unwrap().to_str().unwrap().to_string();
    let in_page_display = in_page.display().to_string();
    debug!(
        "Gonna build page for {} with template {} and out_page {}",
        in_page_display,
        template.display(),
        out_page.display()
    );
    // 1 read in_page.
    let page = fs::read_to_string(in_page)?;
    let pattern = Regex::new(r"---+\n").unwrap();
    let mut fields: Vec<&str> = pattern.splitn(&page, 2).collect();
    if fields.len() < 2 {
        return Err(anyhow::anyhow!(
            "Failed to find metadata in page {}",
            in_page_display
        ));
    }
    let (metadata, content) = (fields.remove(0), fields.remove(0));
    let content = add_ids_to_headings(content);
    debug!("Metadata: {:?}", metadata);
    //debug!("Content: {}", content);

    let metadata: PageMetadata = serde_yaml::from_str(metadata)
        .context("Failed to deserialize metadata, did you remember to put the metadata section?")?;

    println!(
        "is draft: {}, skip_drafts: {}",
        metadata.is_draft, skip_drafts
    );

    if metadata.is_draft && skip_drafts {
        return Ok(None);
    }

    let mut final_page = fs::read_to_string(template)?;
    let html_content = load_markdown(&content);
    let start = final_page.find(START_PATTERN).unwrap();
    let end = final_page.find(END_PATTERN).unwrap();

    final_page.replace_range(start..end + END_PATTERN.len(), &html_content);
    let genereto_metadata = GeneretoMetadata::new(metadata, &content, file_name, in_page);
    let final_page = apply_variables(&genereto_metadata, final_page);

    fs::write(out_page, final_page).context("Failed writing to output page")?;
    Ok(Some(genereto_metadata))
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
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Parses the markdown to add ids to h1,h2,h3
/// to allow building the ToC.
fn add_ids_to_headings(content: &str) -> String {
    let mut content_new = String::new();
    let mut in_code_block = false;

    for line in content.lines() {
        let mut line_new = line.to_string();
        if line.trim().starts_with("```") {
            in_code_block = !in_code_block;
        }
        if line.trim().starts_with('#') && !in_code_block {
            let title = line.trim_start_matches('#').trim();
            let anchor = get_anchor_id_from_title(title);
            line_new = format!("{line}{{#{anchor}}}");
        }

        content_new.push_str(&format!("{line_new}\n"));
    }
    content_new
}
