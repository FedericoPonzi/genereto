use crate::config::GeneretoConfig;
use crate::jinja_processor::{PageContext, PaginationContext, SiteContext};
use crate::page_metadata::{PageMetadata, PageMetadataRaw};

use crate::fs_util::copy_directory_recursively;
use crate::parser::{END_PATTERN, START_PATTERN};
use crate::DraftsOptions;
use anyhow::Context;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const BLOG_ENTRIES_FOLDER_RELATIVE_PATH: &str = "blog";
const BLOG_ENTRIES_FILE_NAME: &str = "blog.yml";
const BLOG_ENTRY_TEMPLATE_FILENAME: &str = "blog.html";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BlogEntries {
    pub(crate) entries: Vec<PageMetadataRaw>,
}

impl BlogEntries {
    pub(crate) fn load_from_path(path: &Path) -> anyhow::Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        let entries: BlogEntries = serde_yaml_ng::from_reader(&fs::File::open(path)?)?;
        Ok(Some(entries))
    }
}

/// Given an index filename (e.g. "index.html" or "blog.html") and a page number,
/// returns the filename for that page.
/// Page 1 returns the original name; page N >= 2 returns "name-page-N.ext".
pub(crate) fn get_page_filename(index_name: &Path, page_number: usize) -> PathBuf {
    if page_number == 1 {
        return index_name.to_path_buf();
    }
    let stem = index_name
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("index");
    let ext = index_name
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("html");
    PathBuf::from(format!("{}-page-{}.{}", stem, page_number, ext))
}

/// Generates the pagination navigation HTML for marker-based templates.
fn build_pagination_html(pagination: &PaginationContext) -> String {
    let mut html = String::from("<nav class=\"pagination\">\n");
    if pagination.has_prev {
        html.push_str(&format!(
            "  <a href=\"{}\" class=\"pagination-prev\">Previous</a>\n",
            pagination.prev_url
        ));
    }
    html.push_str(&format!(
        "  <span class=\"pagination-info\">Page {} of {}</span>\n",
        pagination.current_page, pagination.total_pages
    ));
    if pagination.has_next {
        html.push_str(&format!(
            "  <a href=\"{}\" class=\"pagination-next\">Next</a>\n",
            pagination.next_url
        ));
    }
    html.push_str("</nav>");
    html
}

/// Builds legacy flat Markdown posts and `blog/<bundle>/index.md` articles.
///
/// Bundle assets are copied beside the article's canonical output, excluding
/// the source index.md. Relative bundle covers use this same public directory.
/// `permalink: /nested/slug/` selects a site-root route;
/// `.html` routes are also supported. Percent-encoded UTF-8 is preserved in
/// public URLs and decoded once for filesystem paths. HTML bodies require
/// `content_format: html` and must already be sanitized by the caller/importer.
/// Structured frontmatter is available in Jinja; marker templates can substitute
/// scalar fields only.
pub fn generate_blog(
    genereto_config: &GeneretoConfig,
    drafts_options: &DraftsOptions,
) -> anyhow::Result<Option<Vec<PageMetadata>>> {
    if should_generate_blog(&genereto_config.content_path) {
        crate::routes::validate(genereto_config, drafts_options)?;
        // Unlike run(), this entry point does not clear an existing output tree.
        if genereto_config.output_dir_path.exists() {
            crate::routes::reject_symlinks(&genereto_config.output_dir_path)?;
        }
    }
    generate_blog_validated(genereto_config, drafts_options)
}

pub(crate) fn generate_blog_validated(
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

    // Create site context for Jinja rendering if enabled
    let site_context = if genereto_config.enable_jinja {
        Some(SiteContext::new(
            &genereto_config.title,
            &genereto_config.url,
            &genereto_config.description,
        ))
    } else {
        None
    };

    let mut metadatas = build_articles(genereto_config, drafts_options, site_context.as_ref())?;

    if let Some(field) = &genereto_config.blog.sort_by {
        metadatas.sort_by(|a, b| {
            b.custom_metadata[field]
                .as_str()
                .cmp(&a.custom_metadata[field].as_str())
                .then_with(|| a.permalink.cmp(&b.permalink))
                .then_with(|| a.file_name.cmp(&b.file_name))
        });
    } else {
        metadatas.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let template_index_page =
        fs::read_to_string(&genereto_config.blog.base_template).context(format!(
            "Failed to read the base template at path: {:?}",
            genereto_config.blog.base_template
        ))?;
    let template_index_page =
        crate::parser::process_includes(&template_index_page, &genereto_config.template_dir_path)?;

    // Filter articles for display (move filtering before pagination)
    let filtered_articles: Vec<&PageMetadata> = metadatas
        .iter()
        .filter(|el| el.file_name != "error.html")
        .filter(|el| !el.is_draft || drafts_options.is_dev())
        .collect();

    build_listing(
        &template_index_page,
        &filtered_articles,
        genereto_config,
        site_context.as_ref(),
        None,
    )?;
    let indexed: std::collections::HashMap<_, _> = filtered_articles
        .iter()
        .enumerate()
        .filter_map(|(rank, md)| md.permalink.as_ref().map(|url| (url.as_str(), (rank, *md))))
        .collect();
    for archive in crate::collections::load(genereto_config)? {
        let mut members = archive
            .articles
            .iter()
            .map(|url| {
                indexed.get(url.as_str()).copied().with_context(|| {
                    format!(
                        "Archive {} references unavailable article {url}",
                        archive.permalink
                    )
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        members.sort_by_key(|(rank, _)| *rank);
        let members: Vec<_> = members.into_iter().map(|(_, md)| md).collect();
        build_listing(
            &template_index_page,
            &members,
            genereto_config,
            site_context.as_ref(),
            Some(&archive),
        )?;
    }

    Ok(Some(metadatas))
}

fn build_listing(
    template: &str,
    articles: &[&PageMetadata],
    config: &GeneretoConfig,
    site: Option<&SiteContext>,
    archive: Option<&crate::collections::Archive>,
) -> anyhow::Result<()> {
    let routes = crate::collections::pages(config, articles.len(), archive)?;
    let max = config
        .blog
        .max_entries_per_page
        .unwrap_or(articles.len().max(1));
    let pages = routes.len();
    for (index, route) in routes.iter().enumerate() {
        let start = (index * max).min(articles.len());
        let end = (start + max).min(articles.len());
        let pagination = config.blog.max_entries_per_page.map(|_| PaginationContext {
            current_page: index + 1,
            total_pages: pages,
            has_prev: index > 0,
            has_next: index + 1 < pages,
            prev_url: if index > 0 {
                routes[index - 1].navigation_url.clone()
            } else {
                String::new()
            },
            next_url: routes
                .get(index + 1)
                .map(|r| r.navigation_url.clone())
                .unwrap_or_default(),
        });
        let listing = crate::collections::ListingContext {
            title: archive
                .map(|a| &a.title)
                .or(config.blog.title.as_ref())
                .unwrap_or(&config.title)
                .clone(),
            description: archive
                .map(|a| &a.description)
                .unwrap_or(&config.description)
                .clone(),
            permalink: route.permalink.clone(),
            is_archive: archive.is_some(),
        };
        build_index_page(
            template.to_owned(),
            &articles[start..end],
            &route.destination,
            config,
            site,
            pagination.as_ref(),
            Some(&listing),
        )
        .with_context(|| format!("Failed to build listing {}", route.permalink))?;
    }
    Ok(())
}

fn build_index_page(
    template_view: String,
    articles: &[&PageMetadata],
    destination_path: &Path,
    genereto_config: &GeneretoConfig,
    site_context: Option<&SiteContext>,
    pagination: Option<&PaginationContext>,
    listing: Option<&crate::collections::ListingContext>,
) -> anyhow::Result<()> {
    let final_content = if let Some(site) = site_context {
        // Use Jinja2 template rendering
        let page_contexts: Vec<PageContext> = articles
            .iter()
            .map(|md| PageContext::from_page_metadata(md))
            .collect();
        crate::jinja_processor::render_listing(
            &template_view,
            site,
            &page_contexts,
            pagination,
            listing,
        )?
    } else {
        // Use traditional marker-based rendering
        let mut template_view = template_view;

        // Extract the template content between start_content and end_content
        let start = template_view.find(START_PATTERN).unwrap();
        let end = template_view.find(END_PATTERN).unwrap();
        let template_content = &template_view[start + START_PATTERN.len()..end].trim();

        // If template is empty or just whitespace, fallback to a default template
        let template_to_use = if template_content.trim().is_empty() {
            "<div class=\"post\">\n<h2><a href=\"$GENERETO['permalink']\">$GENERETO['title']</a></h2>\n<div class=\"post-date\">$GENERETO['publish_date']</div>\n<p class=\"post-description\">$GENERETO['description']</p>\n</div>\n"
        } else {
            template_content
        };

        let mut html_content = String::new();
        for md in articles {
            // Apply the template for each article
            let entry_content = md.apply(template_to_use.to_string())?;
            html_content.push_str(&entry_content);
            html_content.push('\n');
        }

        template_view.replace_range(start..end + END_PATTERN.len(), &html_content);

        // Replace pagination placeholder
        let pagination_html = if let Some(p) = pagination {
            build_pagination_html(p)
        } else {
            String::new()
        };
        template_view = template_view.replace("$GENERETO['pagination']", &pagination_html);

        // Replace the blog title if configured
        if let Some(blog_title) = &genereto_config.blog.title {
            template_view = template_view.replace("$GENERETO['title']", blog_title);
        } else {
            template_view = template_view.replace("$GENERETO['title']", &genereto_config.title);
        }

        // Replace remaining site-level placeholders in the outer template
        template_view =
            template_view.replace("$GENERETO['description']", &genereto_config.description);
        template_view = template_view.replace("$GENERETO['keywords']", "");
        template_view = template_view.replace("$GENERETO['url']", &genereto_config.url);
        template_view = template_view.replace(
            "$GENERETO['current_year']",
            &chrono::Local::now().year().to_string(),
        );

        template_view
    };

    fs::create_dir_all(destination_path.parent().context("Listing has no parent")?)?;
    fs::write(destination_path, final_content).context("Failed writing to output page")?;
    Ok(())
}

/// Builds the articles to the destination
/// Returns a list of GeneretoMetadata
fn build_articles(
    genereto_config: &GeneretoConfig,
    drafts_options: &DraftsOptions,
    site_context: Option<&SiteContext>,
) -> anyhow::Result<Vec<PageMetadata>> {
    debug!("Loading articles metadata");
    let mut articles = vec![];

    // Load the default blog template once before the loop
    let default_template = crate::parser::load_template(
        &genereto_config.template_dir_path,
        BLOG_ENTRY_TEMPLATE_FILENAME,
    )?;

    let default_cover_image = &genereto_config
        .blog
        .default_cover_image
        .clone()
        .unwrap_or_default();

    // First try to load from blog.yml if it exists
    let yaml_path = genereto_config.content_path.join(BLOG_ENTRIES_FILE_NAME);
    if let Some(blog_entries) = BlogEntries::load_from_path(&yaml_path)? {
        for entry in blog_entries.entries {
            let metadata = PageMetadata::new(
                entry,
                "", // No content for YAML entries
                &yaml_path,
                default_cover_image,
                &genereto_config.url,
            );
            articles.push(metadata);
        }
    }

    // Then load from blog folder if it exists
    let blog_folder = genereto_config
        .content_path
        .join(BLOG_ENTRIES_FOLDER_RELATIVE_PATH);
    if !blog_folder.exists() {
        return Ok(articles);
    }

    for entry in fs::read_dir(&blog_folder)? {
        let mut entry_path = entry?.path();
        let bundle = entry_path.is_dir() && entry_path.join("index.md").is_file();
        if bundle {
            entry_path = entry_path.join("index.md");
        }
        let entry_path_display = entry_path.display().to_string();
        let destination_path = genereto_config.get_blog_dest_path(&entry_path);
        info!("Compiling {entry_path_display} to {destination_path:?}.");

        if entry_path.is_dir() {
            copy_directory_recursively(&entry_path, &destination_path)?;
        } else if entry_path.is_file() && entry_path.extension().unwrap_or_default() == "md" {
            // Read source content and parse metadata first to check for custom template
            let source_content = fs::read_to_string(&entry_path)
                .with_context(|| format!("Failed to read blog post {entry_path_display}"))?;
            let (intermediate_content, mut metadata_raw) =
                crate::parser::compile_page_phase_1(&source_content)
                    .with_context(|| format!("Failed to parse blog post {entry_path_display}"))?;
            let destination_path = crate::routes::article_destination(
                genereto_config,
                &entry_path,
                bundle,
                metadata_raw.permalink.as_deref(),
            )?;
            if bundle {
                crate::routes::apply_bundle_metadata(
                    genereto_config,
                    &destination_path,
                    &mut metadata_raw,
                )?;
            }

            // Use custom template if specified, otherwise use default
            let template_raw = if let Some(ref template_file) = metadata_raw.template_file {
                crate::parser::load_template(&genereto_config.template_dir_path, template_file)
                    .with_context(|| {
                        format!(
                            "Blog post '{}' specifies template '{}' which could not be loaded",
                            entry_path_display, template_file
                        )
                    })?
            } else {
                default_template.clone()
            };

            // Compile phase 2 with the selected template
            let (content, metadata) = crate::parser::compile_page_phase_2(
                intermediate_content,
                &template_raw,
                metadata_raw,
                default_cover_image,
                &entry_path,
                &genereto_config.url,
                site_context,
            )
            .with_context(|| format!("Failed to compile blog post {entry_path_display}"))?;

            // Handle drafts and write output if generating single pages
            if metadata.is_draft && drafts_options.is_hide() {
                continue;
            }

            if genereto_config.blog.generate_single_pages {
                fs::create_dir_all(destination_path.parent().unwrap())?;
                fs::write(&destination_path, content).with_context(|| {
                    format!("Failed to write blog post to {destination_path:?}")
                })?;
            }
            if bundle {
                let parent = destination_path.parent().unwrap();
                fs::create_dir_all(parent)?;
                for (source, destination) in
                    crate::routes::bundle_assets(&entry_path, &destination_path)?
                {
                    copy_directory_recursively(&source, &destination)?;
                }
            }

            articles.push(metadata);
        } else {
            warn!("Found entry which is not a file nor a directory: {entry_path:?}. Skipping.");
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
        let article1 = PageMetadata {
            title: "Test Article 1".to_string(),
            publish_date: "2024-01-01".to_string(),
            keywords: "test".to_string(),
            reading_time_mins: "5".to_string(),
            description: "Test description 1".to_string(),
            file_name: "article1.html".to_string(),
            page_name: "article1".to_string(),
            table_of_contents: "".to_string(),
            last_modified_date: "2024-01-01".to_string(),
            cover_image: "cover1.jpg".to_string(),
            is_draft: false,
            add_title: false,
            article_url: None,
            website_url: "test.com".to_string(),
            template_file: None,
            permalink: None,
            custom_metadata: Default::default(),
        };
        let article2 = PageMetadata {
            title: "Test Article 2".to_string(),
            publish_date: "2024-01-02".to_string(),
            keywords: "test2".to_string(),
            reading_time_mins: "3".to_string(),
            description: "Test description 2".to_string(),
            file_name: "article2.html".to_string(),
            page_name: "article2".to_string(),
            table_of_contents: "".to_string(),
            last_modified_date: "2024-01-02".to_string(),
            cover_image: "cover2.jpg".to_string(),
            is_draft: false,
            add_title: false,
            article_url: None,
            website_url: "test.com".to_string(),
            template_file: None,
            permalink: None,
            custom_metadata: Default::default(),
        };
        let articles: Vec<&PageMetadata> = vec![&article1, &article2];

        // Test with custom template
        let template = format!(
            "<!DOCTYPE html><html><body><title>$GENERETO['title']</title>{}\n<div class=\"post\">\n<h2>$GENERETO['title']</h2>\n<p>$GENERETO['description']</p>\n</div>\n{}\n$GENERETO['pagination']\n</body></html>",
            START_PATTERN, END_PATTERN
        );

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
            enable_jinja: false,
            root_files: Vec::new(),

            blog: GeneretoConfigBlog {
                base_template: "blog-index.html".into(),
                index_name: "blog.html".into(),
                destination: "".into(),
                generate_single_pages: true,
                title: Some("Custom Blog Title".into()),
                default_cover_image: Some("cover.jpg".into()),
                max_entries_per_page: None,
                pagination_path: None,
                archives_file: None,
                sort_by: None,
            },
        };

        build_index_page(
            template.clone(),
            &articles,
            &output_path,
            &config,
            None, // No jinja mode
            None, // No pagination
            None,
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
            &config,
            None, // No jinja mode
            None, // No pagination
            None,
        )?;

        let output_content = fs::read_to_string(&output_path)?;
        assert!(output_content.contains("<title>Main Title</title>"));

        Ok(())
    }

    #[test]
    fn test_get_page_filename() {
        let index = PathBuf::from("index.html");
        assert_eq!(get_page_filename(&index, 1), PathBuf::from("index.html"));
        assert_eq!(
            get_page_filename(&index, 2),
            PathBuf::from("index-page-2.html")
        );
        assert_eq!(
            get_page_filename(&index, 10),
            PathBuf::from("index-page-10.html")
        );

        let blog = PathBuf::from("blog.html");
        assert_eq!(get_page_filename(&blog, 1), PathBuf::from("blog.html"));
        assert_eq!(
            get_page_filename(&blog, 3),
            PathBuf::from("blog-page-3.html")
        );
    }

    #[test]
    fn test_build_pagination_html() {
        // Test middle page (has both prev and next)
        let ctx = PaginationContext {
            current_page: 2,
            total_pages: 3,
            has_prev: true,
            has_next: true,
            prev_url: "index.html".to_string(),
            next_url: "index-page-3.html".to_string(),
        };
        let html = build_pagination_html(&ctx);
        assert!(html.contains("pagination-prev"));
        assert!(html.contains("pagination-next"));
        assert!(html.contains("index.html"));
        assert!(html.contains("index-page-3.html"));
        assert!(html.contains("Page 2 of 3"));

        // Test first page (no prev)
        let ctx = PaginationContext {
            current_page: 1,
            total_pages: 3,
            has_prev: false,
            has_next: true,
            prev_url: String::new(),
            next_url: "index-page-2.html".to_string(),
        };
        let html = build_pagination_html(&ctx);
        assert!(!html.contains("pagination-prev"));
        assert!(html.contains("pagination-next"));
        assert!(html.contains("Page 1 of 3"));

        // Test last page (no next)
        let ctx = PaginationContext {
            current_page: 3,
            total_pages: 3,
            has_prev: true,
            has_next: false,
            prev_url: "index-page-2.html".to_string(),
            next_url: String::new(),
        };
        let html = build_pagination_html(&ctx);
        assert!(html.contains("pagination-prev"));
        assert!(!html.contains("pagination-next"));
        assert!(html.contains("Page 3 of 3"));
    }
}
