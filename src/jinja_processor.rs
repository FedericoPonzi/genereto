use crate::config::GeneretoConfig;
use crate::page_metadata::PageMetadata;
use anyhow::{Context, Result};
use minijinja::{Environment, Value};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct JinjaProcessor {
    env: Environment<'static>,
}

impl JinjaProcessor {
    pub fn new() -> Self {
        let env = Environment::new();
        Self { env }
    }

    pub fn process_template(
        &mut self,
        template_path: &Path,
        config: &GeneretoConfig,
        metadata: Option<&PageMetadata>,
        content: Option<&str>,
    ) -> Result<String> {
        let jinja_template_path = template_path.with_extension("html.jinja");
        
        if !jinja_template_path.exists() {
            return fs::read_to_string(template_path)
                .with_context(|| format!("Failed to read template file {template_path:?}"));
        }

        let template_content = fs::read_to_string(&jinja_template_path)
            .with_context(|| format!("Failed to read Jinja template file {jinja_template_path:?}"))?;

        let context = self.build_context(config, metadata, content);
        
        let template = self.env.template_from_str(&template_content)
            .context("Failed to parse Jinja template")?;
        
        template.render(&context)
            .context("Failed to render Jinja template")
    }

    fn build_context(
        &self,
        config: &GeneretoConfig,
        metadata: Option<&PageMetadata>,
        content: Option<&str>,
    ) -> Value {
        #[derive(Serialize)]
        struct SiteContext<'a> {
            title: &'a str,
            url: &'a str,
            description: &'a str,
            template: &'a str,
        }
        
        #[derive(Serialize)]
        struct BlogContext<'a> {
            title: &'a str,
        }
        
        #[derive(Serialize)]
        struct PageContext<'a> {
            title: &'a str,
            description: &'a str,
            keywords: &'a str,
            publish_date: &'a str,
            reading_time_mins: &'a str,
            file_name: &'a str,
            cover_image: &'a str,
            is_draft: bool,
            last_modified_date: &'a str,
            table_of_contents: &'a str,
            article_url: &'a Option<String>,
            website_url: &'a str,
            custom_metadata: &'a HashMap<String, String>,
        }
        
        #[derive(Serialize)]
        struct Context<'a> {
            site: SiteContext<'a>,
            #[serde(skip_serializing_if = "Option::is_none")]
            blog: Option<BlogContext<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<PageContext<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<&'a str>,
        }
        
        let site = SiteContext {
            title: &config.title,
            url: &config.url,
            description: &config.description,
            template: &config.template,
        };
        
        let blog = config.blog.title.as_ref().map(|title| BlogContext { title });
        
        let page = metadata.map(|meta| PageContext {
            title: &meta.title,
            description: &meta.description,
            keywords: &meta.keywords,
            publish_date: &meta.publish_date,
            reading_time_mins: &meta.reading_time_mins,
            file_name: &meta.file_name,
            cover_image: &meta.cover_image,
            is_draft: meta.is_draft,
            last_modified_date: &meta.last_modified_date,
            table_of_contents: &meta.table_of_contents,
            article_url: &meta.article_url,
            website_url: &meta.website_url,
            custom_metadata: &meta.custom_metadata,
        });
        
        let context = Context {
            site,
            blog,
            page,
            content,
        };
        
        Value::from_serialize(&context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GeneretoConfigBlog;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn test_jinja_processor_fallback() -> Result<()> {
        let tmp_dir = TempDir::with_prefix("jinja_test")?;
        let template_path = tmp_dir.path().join("template.html");
        
        fs::write(&template_path, "<!DOCTYPE html><html><body>{{site.title}}</body></html>")?;

        let config = GeneretoConfig {
            template_dir_path: tmp_dir.path().to_path_buf(),
            output_dir_path: tmp_dir.path().join("output"),
            project_path: tmp_dir.path().to_path_buf(),
            content_path: tmp_dir.path().join("content"),
            template: "main".to_string(),
            template_base_path: None,
            title: "Test Site".to_string(),
            url: "https://example.com".to_string(),
            description: "Test description".to_string(),
            enable_jinja: true,
            blog: GeneretoConfigBlog {
                base_template: tmp_dir.path().join("blog.html"),
                index_name: "index.html".into(),
                destination: tmp_dir.path().join("blog"),
                generate_single_pages: true,
                title: None,
                default_cover_image: None,
            },
        };

        let mut processor = JinjaProcessor::new();
        
        let result = processor.process_template(&template_path, &config, None, None)?;
        
        assert_eq!(result, "<!DOCTYPE html><html><body>{{site.title}}</body></html>");

        Ok(())
    }

    #[test]
    fn test_jinja_processor_with_jinja_template() -> Result<()> {
        let tmp_dir = TempDir::with_prefix("jinja_test")?;
        let template_path = tmp_dir.path().join("template.html");
        let jinja_template_path = tmp_dir.path().join("template.html.jinja");
        
        fs::write(&template_path, "fallback content")?;
        fs::write(&jinja_template_path, "<!DOCTYPE html><html><body>{{site.title}}</body></html>")?;

        let config = GeneretoConfig {
            template_dir_path: tmp_dir.path().to_path_buf(),
            output_dir_path: tmp_dir.path().join("output"),
            project_path: tmp_dir.path().to_path_buf(),
            content_path: tmp_dir.path().join("content"),
            template: "main".to_string(),
            template_base_path: None,
            title: "Test Site".to_string(),
            url: "https://example.com".to_string(),
            description: "Test description".to_string(),
            enable_jinja: true,
            blog: GeneretoConfigBlog {
                base_template: tmp_dir.path().join("blog.html"),
                index_name: "index.html".into(),
                destination: tmp_dir.path().join("blog"),
                generate_single_pages: true,
                title: None,
                default_cover_image: None,
            },
        };

        let mut processor = JinjaProcessor::new();
        
        let result = processor.process_template(&template_path, &config, None, None)?;
        
        assert_eq!(result, "<!DOCTYPE html><html><body>Test Site</body></html>");

        Ok(())
    }

    #[test]
    fn test_jinja_processor_with_page_metadata() -> Result<()> {
        let tmp_dir = TempDir::with_prefix("jinja_test")?;
        let jinja_template_path = tmp_dir.path().join("template.html.jinja");
        
        fs::write(&jinja_template_path, 
            "<!DOCTYPE html><html><body><h1>{{page.title}}</h1><p>{{page.description}}</p></body></html>")?;

        let config = GeneretoConfig {
            template_dir_path: tmp_dir.path().to_path_buf(),
            output_dir_path: tmp_dir.path().join("output"),
            project_path: tmp_dir.path().to_path_buf(),
            content_path: tmp_dir.path().join("content"),
            template: "main".to_string(),
            template_base_path: None,
            title: "Test Site".to_string(),
            url: "https://example.com".to_string(),
            description: "Test description".to_string(),
            enable_jinja: true,
            blog: GeneretoConfigBlog {
                base_template: tmp_dir.path().join("blog.html"),
                index_name: "index.html".into(),
                destination: tmp_dir.path().join("blog"),
                generate_single_pages: true,
                title: None,
                default_cover_image: None,
            },
        };

        let metadata = PageMetadata {
            title: "Test Page".to_string(),
            description: "Test description".to_string(),
            keywords: "test".to_string(),
            publish_date: "2024-01-01".to_string(),
            reading_time_mins: "5".to_string(),
            file_name: "test.html".to_string(),
            table_of_contents: "".to_string(),
            last_modified_date: "2024-01-01".to_string(),
            cover_image: "cover.jpg".to_string(),
            is_draft: false,
            add_title: false,
            article_url: None,
            website_url: "https://example.com".to_string(),
            custom_metadata: HashMap::new(),
        };

        let mut processor = JinjaProcessor::new();
        let template_path = tmp_dir.path().join("template.html");
        
        let result = processor.process_template(&template_path, &config, Some(&metadata), None)?;
        
        assert_eq!(result, "<!DOCTYPE html><html><body><h1>Test Page</h1><p>Test description</p></body></html>");

        Ok(())
    }
}