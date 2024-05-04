use crate::config::GeneretoConfig;
use crate::page_metadata::{BlogArticleMetadata, BlogArticleMetadataRaw};

use crate::fs_util::copy_directory_recursively;
use crate::parser::{add_ids_to_headings, compile_markdown_to_html, filter_out_comments};
use crate::{DraftsOptions, END_PATTERN, START_PATTERN};
use anyhow::Context;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::{fs, io};

const BLOG_ENTRIES_FOLDER_RELATIVE_PATH: &str = "blog";
const BLOG_ENTRY_FILENAME: &str = "blog.html";

// TODO: builders should be separated into two functions, one which operates with
// objects in memory (e.g. template file as string, content as string and return a string for the output
// then a function that works with filepaths and call the above function.
pub fn generate_blog(
    genereto_config: &GeneretoConfig,
    drafts_options: DraftsOptions,
) -> anyhow::Result<Vec<BlogArticleMetadata>> {
    if !should_generate_blog(&genereto_config.content_path) {
        info!(
            "Skipping blog generation. No blog generation needed, as '{}/{}' doesn't exists",
            genereto_config.content_path.display(),
            BLOG_ENTRIES_FOLDER_RELATIVE_PATH
        );
        return Ok(vec![]);
    }
    debug!("Genereting blog");

    let mut metadatas = build_articles(genereto_config, &drafts_options)?;

    // sort by published date
    metadatas.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let template_index_page = fs::read_to_string(&genereto_config.blog.base_template)?;
    // todo: if there is already an index.html, replace it with blog.html.
    let template_destination_path = &genereto_config.blog.index_destination;

    // Create an index.html
    build_index_page(
        template_index_page,
        &metadatas,
        template_destination_path,
        &drafts_options,
    )
    .context("Failed to build index page.")?;
    Ok(metadatas)
}
fn build_index_page(
    mut template_view: String,
    articles: &[BlogArticleMetadata],
    destination_path: &Path,
    drafts_options: &DraftsOptions,
) -> anyhow::Result<()> {
    let mut links = "<ul class=\"index-list\">\n".to_string();
    for md in articles
        .into_iter()
        // error page doesn't need to be shown on the homepage.
        .filter(|el| el.file_name != "error.html")
        // if the page is a draft and we are not in dev mode, we need to skip it.
        .filter(|el| !el.is_draft || drafts_options.is_dev())
    {
        let li_entry = format!(
            "<li><a href=\"{}\">{}</a> - {} ({})</li>",
            md.file_name, md.title, md.publish_date, md.keywords,
        );
        links.push_str(&li_entry);
    }
    links.push_str("</ul>\n");
    let html_content = links;
    let start = template_view.find(START_PATTERN).unwrap();
    let end = template_view.find(END_PATTERN).unwrap();
    template_view.replace_range(start..end + END_PATTERN.len(), &html_content);

    fs::write(destination_path, template_view).context("Failed writing to output page")?;
    Ok(())
}

/// Returns a list of dest_filename, GeneretoMetadata for each file.
fn build_articles(
    genereto_config: &GeneretoConfig,
    drafts_options: &DraftsOptions,
) -> anyhow::Result<Vec<BlogArticleMetadata>> {
    debug!("Loading articles metadata");
    let mut articles = vec![];
    let template_path = &genereto_config.template_dir_path.join(BLOG_ENTRY_FILENAME);
    let template_raw = fs::read_to_string(template_path)?;
    let default_cover_image = genereto_config
        .default_cover_image
        .clone()
        .unwrap_or_default();
    for entry in fs::read_dir(
        genereto_config
            .content_path
            .join(BLOG_ENTRIES_FOLDER_RELATIVE_PATH), // todo: this can be moved to the config.
    )? {
        let entry_path = entry?.path();
        let entry_path_display = entry_path.display().to_string();
        info!("Blog entry path: {}", entry_path_display);
        if entry_path.is_dir() {
            if is_res_folder(&entry_path)? {
                continue;
            }
            let file_name = entry_path.file_name().unwrap();

            copy_directory_recursively(
                &entry_path,
                genereto_config.output_dir_path.join(file_name),
            )?;
        } else if entry_path.is_file() {
            let dest_filename = generate_file_name(&entry_path);
            let article_opt = build_blog_entry(
                &template_raw,
                entry_path,
                genereto_config.output_dir_path.join(&dest_filename),
                &default_cover_image,
                drafts_options,
            )
            .with_context(|| format!("Failed to build page {entry_path_display}"))?;
            if let Some(article) = article_opt {
                articles.push(article);
            }
        }
    }
    Ok(articles)
}

fn build_blog_entry(
    template: &str,
    entry_path: PathBuf,
    destination_path: PathBuf,
    default_cover_image: &str,
    drafts_options: &DraftsOptions,
) -> anyhow::Result<Option<BlogArticleMetadata>> {
    // to ease log printing
    let entry_path_str = entry_path.display().to_string();

    // 1 read in_page.
    let page = fs::read_to_string(&entry_path)?;
    let pattern = Regex::new(r"---+\n").unwrap();
    let mut fields: Vec<&str> = pattern.splitn(&page, 2).collect();
    if fields.len() < 2 {
        return Err(anyhow::anyhow!(
            "Failed to find metadata in page {}",
            entry_path_str
        ));
    }
    let (metadata, content) = (fields.remove(0), fields.remove(0));
    let content = add_ids_to_headings(content);
    debug!("Metadata: {:?}", metadata);

    let metadata: BlogArticleMetadataRaw = serde_yaml::from_str(metadata)
        .context("Failed to deserialize metadata, did you remember to put the metadata section?")?;

    println!(
        "is draft: {}, drafts_options: {:?}",
        metadata.is_draft, drafts_options
    );

    if metadata.is_draft && drafts_options.is_hide() {
        return Ok(None);
    }
    let final_page = template;
    let mut final_page = filter_out_comments(final_page);

    let html_content = compile_markdown_to_html(&content);
    let start = final_page.find(START_PATTERN).unwrap();
    let end = final_page.find(END_PATTERN).unwrap();

    final_page.replace_range(start..end + END_PATTERN.len(), &html_content);

    let file_name = destination_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    info!("Writing blog entry to {}", file_name);
    let genereto_metadata = BlogArticleMetadata::new(
        metadata,
        &content,
        file_name,
        &entry_path,
        default_cover_image,
    );
    let final_page = apply_variables(&genereto_metadata, final_page);

    fs::write(destination_path, final_page).context("Failed writing to output page")?;
    Ok(Some(genereto_metadata))
}

// Apply variables to the final page.
fn apply_variables(metadata: &BlogArticleMetadata, mut final_page: String) -> String {
    for i in metadata.get_variables() {
        final_page = final_page.replace(i.0, &i.1);
    }
    final_page
}

fn generate_file_name(p: &Path) -> String {
    // unwraps needed because these returns optional
    p.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".md", ".html")
}

fn is_res_folder(entry_path: &Path) -> io::Result<bool> {
    let ret = entry_path.file_name().unwrap().to_str().unwrap() == "res";
    if ret {
        warn!("Please put the 'res' folder in the templating folder. Skipping copy.");
    }
    Ok(ret)
}

fn should_generate_blog(content_path: &Path) -> bool {
    // check if project_path/content/blog exists.
    content_path
        .join(BLOG_ENTRIES_FOLDER_RELATIVE_PATH)
        .exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use tempdir::TempDir;

    #[test]
    fn test_should_generate_blog() -> io::Result<()> {
        let tmp_dir = TempDir::new("example")?;
        let project_path = tmp_dir.path();
        assert!(!should_generate_blog(project_path));

        fs::create_dir_all(project_path.join(BLOG_ENTRIES_FOLDER_RELATIVE_PATH)).unwrap();
        assert!(should_generate_blog(project_path));

        Ok(())
    }
}
