use crate::config::GeneretoConfig;
use crate::page_metadata::{PageMetadata, PageMetadataRaw};

use crate::fs_util::copy_directory_recursively;
use crate::parser::{load_compile_write, END_PATTERN, START_PATTERN};
use crate::DraftsOptions;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const BLOG_ENTRIES_FOLDER_RELATIVE_PATH: &str = "blog";
const BLOG_ENTRIES_FILE_NAME: &str = "blog.yml";
const BLOG_ENTRY_TEMPLATE_FILENAME: &str = "blog.html";

#[derive(Debug, Serialize, Deserialize)]
struct BlogEntries {
    entries: Vec<PageMetadataRaw>,
}

impl BlogEntries {
    fn load_from_path(path: &Path) -> anyhow::Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        let entries: BlogEntries = serde_yaml::from_reader(&fs::File::open(path)?)?;
        Ok(Some(entries))
    }
}

pub fn generate_blog(
    genereto_config: &GeneretoConfig,
    drafts_options: &DraftsOptions,
) -> anyhow::Result<Option<Vec<PageMetadata>>> {
    if !should_generate_blog(&genereto_config.content_path) {
        info!(
            "Skipping blog generation. No blog generation needed, as '{}/{}' doesn't exists",
            genereto_config.content_path.display(),
            BLOG_ENTRIES_FOLDER_RELATIVE_PATH
        );
        return Ok(None);
    }
    debug!("Generating blog");
    fs::create_dir_all(&genereto_config.blog.destination)?;

    let mut metadatas = build_articles(genereto_config, drafts_options)?;

    // sort by published date
    metadatas.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let template_index_page =
        fs::read_to_string(&genereto_config.blog.base_template).context(format!(
            "Failed to read the base template at path: {:?}",
            genereto_config.blog.base_template
        ))?;
    // todo: if there is already an index.html, replace it with blog.html.
    let destination_path = &genereto_config
        .blog
        .destination
        .join(&genereto_config.blog.index_name);

    // Create an index.html
    build_index_page(
        template_index_page,
        &metadatas,
        destination_path,
        drafts_options,
        genereto_config,
    )
    .context("Failed to build index page.")?;
    Ok(Some(metadatas))
}
fn build_index_page(
    mut template_view: String,
    articles: &[PageMetadata],
    destination_path: &Path,
    drafts_options: &DraftsOptions,
    genereto_config: &GeneretoConfig,
) -> anyhow::Result<()> {
    // Extract the template content between start_content and end_content
    let start = template_view.find(START_PATTERN).unwrap();
    let end = template_view.find(END_PATTERN).unwrap();
    let template_content = &template_view[start + START_PATTERN.len()..end].trim();

    // If template is empty or just whitespace, fallback to a default template
    let template_to_use = if template_content.trim().is_empty() {
        "<div class=\"post\">\n<h2><a href=\"$GENERETO['file_name']\">$GENERETO['title']</a></h2>\n<div class=\"post-date\">$GENERETO['publish_date']</div>\n<p class=\"post-description\">$GENERETO['description']</p>\n</div>\n"
    } else {
        template_content
    };

    let mut html_content = String::new();
    for md in articles
        .iter()
        // error page doesn't need to be shown on the homepage.
        .filter(|el| el.file_name != "error.html")
        // if the page is a draft and we are not in dev mode, we need to skip it.
        .filter(|el| !el.is_draft || drafts_options.is_dev())
    {
        // Apply the template for each article
        let entry_content = md.apply(template_to_use.to_string());
        html_content.push_str(&entry_content);
        html_content.push('\n');
    }

    template_view.replace_range(start..end + END_PATTERN.len(), &html_content);

    // Replace the blog title if configured
    if let Some(blog_title) = &genereto_config.blog.title {
        template_view = template_view.replace("$GENERETO['title']", blog_title);
    } else {
        template_view = template_view.replace("$GENERETO['title']", &genereto_config.title);
    }

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
    let template_raw = fs::read_to_string(template_path)
        .context(format!("reading template path {:?}", template_path))?;
    let default_cover_image = &genereto_config.default_cover_image;

    // First try to load from blog.yml if it exists
    let yaml_path = genereto_config.content_path.join(BLOG_ENTRIES_FILE_NAME);
    if let Some(blog_entries) = BlogEntries::load_from_path(&yaml_path)? {
        for entry in blog_entries.entries {
            let metadata = PageMetadata::new(
                entry,
                "", // No content for YAML entries
                &yaml_path,
                default_cover_image,
            );
            debug!("Cover image: {}", metadata.cover_image);
            articles.push(metadata);
        }
    }

    // Then load from blog folder if it exists
    let blog_folder = genereto_config
        .content_path
        .join(BLOG_ENTRIES_FOLDER_RELATIVE_PATH);
    if blog_folder.exists() {
        for entry in fs::read_dir(&blog_folder)? {
            let entry_path = entry?.path();
            let entry_path_display = entry_path.display().to_string();
            let destination_path = genereto_config.get_blog_dest_path(&entry_path);
            info!("Compiling {entry_path_display} to {destination_path:?}.");
            if entry_path.is_dir() {
                copy_directory_recursively(&entry_path, &destination_path)?;
            } else if entry_path.is_file() && entry_path.extension().unwrap_or_default() == "md" {
                let article_opt = if genereto_config.blog.generate_single_pages {
                    load_compile_write(
                        default_cover_image,
                        &entry_path,
                        drafts_options,
                        &destination_path,
                        &template_raw,
                    )
                    .with_context(|| format!("Failed to build page {entry_path_display}"))?
                } else {
                    // When single pages are disabled, still parse metadata but skip file generation
                    let (_, md) = crate::parser::load_compile(
                        default_cover_image,
                        &entry_path,
                        &template_raw,
                    )
                    .with_context(|| {
                        format!("Failed to parse metadata for {entry_path_display}")
                    })?;
                    Some(md)
                };
                if let Some(md) = article_opt {
                    articles.push(md);
                }
            } else {
                warn!("Found entry which is not a file nor a directory: {entry_path:?}. Skipping.");
            }
        }
    }
    Ok(articles)
}

fn should_generate_blog(content_path: &Path) -> bool {
    // check if project_path/content/blog exists or if blog.yml exists
    content_path
        .join(BLOG_ENTRIES_FOLDER_RELATIVE_PATH)
        .exists()
        || content_path.join(BLOG_ENTRIES_FILE_NAME).exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GeneretoConfigBlog;
    use std::io;
    use tempfile::TempDir;

    #[test]
    fn test_should_generate_blog() -> io::Result<()> {
        let tmp_dir = TempDir::with_prefix("example")?;
        let project_path = tmp_dir.path();
        assert!(!should_generate_blog(project_path));

        // Test with blog folder
        fs::create_dir_all(project_path.join(BLOG_ENTRIES_FOLDER_RELATIVE_PATH))?;
        assert!(should_generate_blog(project_path));

        // Test with blog.yml
        fs::remove_dir_all(project_path.join(BLOG_ENTRIES_FOLDER_RELATIVE_PATH))?;
        assert!(!should_generate_blog(project_path));
        fs::write(
            project_path.join(BLOG_ENTRIES_FILE_NAME),
            "entries:\n  - title: Test\n    publish_date: 2024-01-01\n",
        )?;
        assert!(should_generate_blog(project_path));

        Ok(())
    }

    #[test]
    fn test_blog_entries_yaml() -> anyhow::Result<()> {
        let tmp_dir = TempDir::with_prefix("example")?;
        let yaml_content = r#"
entries:
  - title: Test Article 1
    publish_date: 2024-01-01
    keywords: test
    description: Test description 1
    cover_image: cover1.jpg
  - title: Test Article 2
    publish_date: 2024-01-02
    keywords: test2
    description: Test description 2
    cover_image: cover2.jpg
"#;
        let yaml_path = tmp_dir.path().join("blog.yml");
        fs::write(&yaml_path, yaml_content)?;

        let blog_entries = BlogEntries::load_from_path(&yaml_path)?.unwrap();
        assert_eq!(blog_entries.entries.len(), 2);
        assert_eq!(blog_entries.entries[0].title, "Test Article 1");
        assert_eq!(blog_entries.entries[1].title, "Test Article 2");

        Ok(())
    }

    #[test]
    fn test_build_index_page() -> anyhow::Result<()> {
        let tmp_dir = TempDir::with_prefix("example")?;
        let output_path = tmp_dir.path().join("index.html");

        // Create test articles
        let articles = vec![
            PageMetadata {
                title: "Test Article 1".to_string(),
                publish_date: "2024-01-01".to_string(),
                keywords: "test".to_string(),
                reading_time_mins: "5".to_string(),
                description: "Test description 1".to_string(),
                file_name: "article1.html".to_string(),
                table_of_contents: "".to_string(),
                last_modified_date: "2024-01-01".to_string(),
                cover_image: "cover1.jpg".to_string(),
                is_draft: false,
                add_title: false,
                url: None,
                custom_metadata: Default::default(),
            },
            PageMetadata {
                title: "Test Article 2".to_string(),
                publish_date: "2024-01-02".to_string(),
                keywords: "test2".to_string(),
                reading_time_mins: "3".to_string(),
                description: "Test description 2".to_string(),
                file_name: "article2.html".to_string(),
                table_of_contents: "".to_string(),
                last_modified_date: "2024-01-02".to_string(),
                cover_image: "cover2.jpg".to_string(),
                is_draft: false,
                add_title: false,
                url: None,
                custom_metadata: Default::default(),
            },
        ];

        // Test with custom template
        let template = format!(
            "<!DOCTYPE html><html><body><title>$GENERETO['title']</title>{}\n<div class=\"post\">\n<h2>$GENERETO['title']</h2>\n<p>$GENERETO['description']</p>\n</div>\n{}\n</body></html>",
            START_PATTERN, END_PATTERN
        );

        let drafts_options = DraftsOptions::Build;

        // Test with blog title
        let config = GeneretoConfig {
            template_dir_path: "template".into(),
            output_dir_path: "output".into(),
            project_path: "".into(),
            content_path: "content".into(),
            template: "test_template".into(),
            template_base_path: None,
            title: "Main Title".into(),
            url: "test.com".into(),
            description: "Test description".into(),
            default_cover_image: "cover.jpg".into(),
            blog: GeneretoConfigBlog {
                base_template: "blog-index.html".into(),
                index_name: "blog.html".into(),
                destination: "".into(),
                generate_single_pages: true,
                title: Some("Custom Blog Title".into()),
            },
        };

        build_index_page(
            template.clone(),
            &articles,
            &output_path,
            &drafts_options,
            &config,
        )?;

        let output_content = fs::read_to_string(&output_path)?;
        assert!(output_content.contains("<title>Custom Blog Title</title>"));
        assert!(output_content.contains("<h2>Test Article 1</h2>"));
        assert!(output_content.contains("<p>Test description 1</p>"));
        assert!(output_content.contains("<h2>Test Article 2</h2>"));
        assert!(output_content.contains("<p>Test description 2</p>"));

        // Test without blog title (should use main title)
        let mut config = config;
        config.blog.title = None;
        build_index_page(
            template.clone(),
            &articles,
            &output_path,
            &drafts_options,
            &config,
        )?;

        let output_content = fs::read_to_string(&output_path)?;
        assert!(output_content.contains("<title>Main Title</title>"));

        Ok(())
    }
}
