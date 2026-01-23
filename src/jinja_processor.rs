use chrono::Datelike;
use minijinja::{context, Environment};
use serde::Serialize;
use std::collections::HashMap;

/// Site-level context available in all templates
#[derive(Debug, Clone, Serialize)]
pub struct SiteContext {
    pub title: String,
    pub url: String,
    pub description: String,
    pub current_year: i32,
}

impl SiteContext {
    pub fn new(title: &str, url: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            url: url.to_string(),
            description: description.to_string(),
            current_year: chrono::Local::now().year(),
        }
    }
}

/// Page-level context for individual page templates
#[derive(Debug, Clone, Serialize)]
pub struct PageContext {
    pub title: String,
    pub publish_date: String,
    pub description: String,
    pub keywords: String,
    pub file_name: String,
    pub cover_image: String,
    pub table_of_contents: String,
    pub read_time_minutes: String,
    pub last_modified_date: String,
    pub url: String,
    #[serde(flatten)]
    pub custom_metadata: HashMap<String, String>,
}

impl PageContext {
    pub fn from_page_metadata(metadata: &crate::page_metadata::PageMetadata) -> Self {
        Self {
            title: metadata.title.clone(),
            publish_date: metadata.publish_date.clone(),
            description: metadata.description.clone(),
            keywords: metadata.keywords.clone(),
            file_name: metadata.file_name.clone(),
            cover_image: metadata.cover_image.clone(),
            table_of_contents: metadata.table_of_contents.clone(),
            read_time_minutes: metadata.reading_time_mins.clone(),
            last_modified_date: metadata.last_modified_date.clone(),
            url: metadata.article_url.clone().unwrap_or_default(),
            custom_metadata: metadata.custom_metadata.clone(),
        }
    }
}

/// Render a page template with Jinja2
pub fn render_page(
    template: &str,
    site: &SiteContext,
    page: &PageContext,
    content: &str,
) -> anyhow::Result<String> {
    let mut env = Environment::new();
    env.add_template("page", template)?;
    let tmpl = env.get_template("page")?;
    let result = tmpl.render(context! {
        site => site,
        page => page,
        content => content,
    })?;
    Ok(result)
}

/// Pagination context available in blog index templates
#[derive(Debug, Clone, Serialize)]
pub struct PaginationContext {
    pub current_page: usize,
    pub total_pages: usize,
    pub has_prev: bool,
    pub has_next: bool,
    pub prev_url: String,
    pub next_url: String,
}

/// Render blog index template with articles list
pub fn render_blog_index(
    template: &str,
    site: &SiteContext,
    articles: &[PageContext],
    pagination: Option<&PaginationContext>,
) -> anyhow::Result<String> {
    let mut env = Environment::new();
    env.add_template("index", template)?;
    let tmpl = env.get_template("index")?;
    let result = tmpl.render(context! {
        site => site,
        articles => articles,
        pagination => pagination,
    })?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_site_context() -> SiteContext {
        SiteContext::new("Test Site", "https://example.com", "A test site")
    }

    fn create_test_page_context() -> PageContext {
        PageContext {
            title: "Test Page".to_string(),
            publish_date: "2024-01-15".to_string(),
            description: "A test page description".to_string(),
            keywords: "test, page".to_string(),
            file_name: "test-page.html".to_string(),
            cover_image: "cover.jpg".to_string(),
            table_of_contents: "<ul><li>Section 1</li></ul>".to_string(),
            read_time_minutes: "5".to_string(),
            last_modified_date: "2024-01-20".to_string(),
            url: "".to_string(),
            custom_metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_render_page_basic() {
        let template = "<html><title>{{ page.title }}</title></html>";
        let site = create_test_site_context();
        let page = create_test_page_context();

        let result = render_page(template, &site, &page, "").unwrap();
        assert_eq!(result, "<html><title>Test Page</title></html>");
    }

    #[test]
    fn test_render_page_with_site_context() {
        let template = "<html><title>{{ page.title }} - {{ site.title }}</title></html>";
        let site = create_test_site_context();
        let page = create_test_page_context();

        let result = render_page(template, &site, &page, "").unwrap();
        assert_eq!(
            result,
            "<html><title>Test Page - Test Site</title></html>"
        );
    }

    #[test]
    fn test_render_page_with_content() {
        let template = "<html><body>{{ content }}</body></html>";
        let site = create_test_site_context();
        let page = create_test_page_context();

        let result = render_page(template, &site, &page, "<p>Hello World</p>").unwrap();
        assert_eq!(result, "<html><body><p>Hello World</p></body></html>");
    }

    #[test]
    fn test_render_page_all_variables() {
        let template = r#"<html>
<head>
  <title>{{ page.title }}</title>
  <meta name="description" content="{{ page.description }}">
  <meta name="keywords" content="{{ page.keywords }}">
</head>
<body>
  <h1>{{ page.title }}</h1>
  <p>Published: {{ page.publish_date }}</p>
  <p>{{ page.read_time_minutes }} min read</p>
  <img src="{{ page.cover_image }}">
  {{ page.table_of_contents }}
  <article>{{ content }}</article>
  <footer>&copy; {{ site.current_year }} {{ site.title }}</footer>
</body>
</html>"#;

        let site = create_test_site_context();
        let page = create_test_page_context();
        let content = "<p>Article content here</p>";

        let result = render_page(template, &site, &page, content).unwrap();

        assert!(result.contains("<title>Test Page</title>"));
        assert!(result.contains("content=\"A test page description\""));
        assert!(result.contains("content=\"test, page\""));
        assert!(result.contains("<h1>Test Page</h1>"));
        assert!(result.contains("Published: 2024-01-15"));
        assert!(result.contains("5 min read"));
        assert!(result.contains("src=\"cover.jpg\""));
        assert!(result.contains("<ul><li>Section 1</li></ul>"));
        assert!(result.contains("<p>Article content here</p>"));
        assert!(result.contains("Test Site"));
    }

    #[test]
    fn test_render_page_with_custom_metadata() {
        let template = "<html><p>Author: {{ page.author }}</p><p>Category: {{ page.category }}</p></html>";
        let site = create_test_site_context();
        let mut page = create_test_page_context();
        page.custom_metadata
            .insert("author".to_string(), "John Doe".to_string());
        page.custom_metadata
            .insert("category".to_string(), "Tech".to_string());

        let result = render_page(template, &site, &page, "").unwrap();
        assert!(result.contains("Author: John Doe"));
        assert!(result.contains("Category: Tech"));
    }

    #[test]
    fn test_render_blog_index() {
        let template = r#"<html>
<body>
  <h1>{{ site.title }}</h1>
  {% for article in articles %}
  <article>
    <h2><a href="{{ article.file_name }}">{{ article.title }}</a></h2>
    <p>{{ article.publish_date }} | {{ article.read_time_minutes }} min read</p>
    <p>{{ article.description }}</p>
  </article>
  {% endfor %}
</body>
</html>"#;

        let site = create_test_site_context();
        let articles = vec![
            PageContext {
                title: "First Post".to_string(),
                publish_date: "2024-01-15".to_string(),
                description: "First post description".to_string(),
                keywords: "".to_string(),
                file_name: "first-post.html".to_string(),
                cover_image: "".to_string(),
                table_of_contents: "".to_string(),
                read_time_minutes: "3".to_string(),
                last_modified_date: "".to_string(),
                url: "".to_string(),
                custom_metadata: HashMap::new(),
            },
            PageContext {
                title: "Second Post".to_string(),
                publish_date: "2024-01-20".to_string(),
                description: "Second post description".to_string(),
                keywords: "".to_string(),
                file_name: "second-post.html".to_string(),
                cover_image: "".to_string(),
                table_of_contents: "".to_string(),
                read_time_minutes: "5".to_string(),
                last_modified_date: "".to_string(),
                url: "".to_string(),
                custom_metadata: HashMap::new(),
            },
        ];

        let result = render_blog_index(template, &site, &articles, None).unwrap();

        assert!(result.contains("<h1>Test Site</h1>"));
        assert!(result.contains("<a href=\"first-post.html\">First Post</a>"));
        assert!(result.contains("2024-01-15 | 3 min read"));
        assert!(result.contains("First post description"));
        assert!(result.contains("<a href=\"second-post.html\">Second Post</a>"));
        assert!(result.contains("2024-01-20 | 5 min read"));
        assert!(result.contains("Second post description"));
    }

    #[test]
    fn test_render_blog_index_empty() {
        let template = r#"<html>
<body>
  {% for article in articles %}
  <article>{{ article.title }}</article>
  {% endfor %}
  {% if articles|length == 0 %}
  <p>No articles yet.</p>
  {% endif %}
</body>
</html>"#;

        let site = create_test_site_context();
        let articles: Vec<PageContext> = vec![];

        let result = render_blog_index(template, &site, &articles, None).unwrap();
        assert!(result.contains("No articles yet."));
    }

    #[test]
    fn test_site_context_current_year() {
        let site = SiteContext::new("Test", "https://test.com", "Test");
        let expected_year = chrono::Local::now().year();
        assert_eq!(site.current_year, expected_year);
    }

    #[test]
    fn test_render_page_conditional() {
        let template = r#"<html>
{% if page.cover_image %}
<img src="{{ page.cover_image }}">
{% endif %}
</html>"#;

        let site = create_test_site_context();
        let page = create_test_page_context();

        let result = render_page(template, &site, &page, "").unwrap();
        assert!(result.contains("src=\"cover.jpg\""));

        // Test with empty cover image
        let mut page_no_cover = create_test_page_context();
        page_no_cover.cover_image = "".to_string();
        let result = render_page(template, &site, &page_no_cover, "").unwrap();
        assert!(!result.contains("<img"));
    }
}
