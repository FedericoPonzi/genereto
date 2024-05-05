use crate::config::GeneretoConfig;
use crate::page_metadata::PageMetadata;

use crate::fs_util::copy_directory_recursively;
use crate::parser::{load_compile_write, END_PATTERN, START_PATTERN};
use crate::DraftsOptions;
use anyhow::Context;
use std::path::Path;
use std::{fs, io};

const BLOG_ENTRIES_FOLDER_RELATIVE_PATH: &str = "blog";
const BLOG_ENTRY_TEMPLATE_FILENAME: &str = "blog.html";

// TODO: builders should be separated into two functions, one which operates with
// objects in memory (e.g. template file as string, content as string and return a string for the output
// then a function that works with filepaths and call the above function.
pub fn generate_blog(
    genereto_config: &GeneretoConfig,
    drafts_options: DraftsOptions,
) -> anyhow::Result<Option<Vec<PageMetadata>>> {
    if !should_generate_blog(&genereto_config.content_path) {
        info!(
            "Skipping blog generation. No blog generation needed, as '{}/{}' doesn't exists",
            genereto_config.content_path.display(),
            BLOG_ENTRIES_FOLDER_RELATIVE_PATH
        );
        return Ok(None);
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
    Ok(Some(metadatas))
}
fn build_index_page(
    mut template_view: String,
    articles: &[PageMetadata],
    destination_path: &Path,
    drafts_options: &DraftsOptions,
) -> anyhow::Result<()> {
    let mut links = "<ul class=\"index-list\">\n".to_string();
    for md in articles
        .iter()
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

/// Builds the articles to the destination
/// Returns a list of GeneretoMetadata
fn build_articles(
    genereto_config: &GeneretoConfig,
    drafts_options: &DraftsOptions,
) -> anyhow::Result<Vec<PageMetadata>> {
    debug!("Loading articles metadata");
    let mut articles = vec![];
    let template_path = &genereto_config
        .template_dir_path
        .join(BLOG_ENTRY_TEMPLATE_FILENAME);
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
        let destination_path = genereto_config.get_dest_path(&entry_path);
        info!("Compiling {entry_path_display} to {destination_path:?}.");
        if entry_path.is_dir() {
            if is_res_folder(&entry_path)? {
                continue;
            }
            copy_directory_recursively(&entry_path, &destination_path)?;
        } else if entry_path.is_file() {
            let article_opt = load_compile_write(
                &default_cover_image,
                &entry_path,
                drafts_options,
                &destination_path,
                &template_raw,
            )
            .with_context(|| format!("Failed to build page {entry_path_display}"))?;
            if let Some(md) = article_opt {
                articles.push(md);
            }
        } else {
            warn!("Found entry which is not a file nor a directory: {entry_path:?}. Skipping.");
        }
    }
    Ok(articles)
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
