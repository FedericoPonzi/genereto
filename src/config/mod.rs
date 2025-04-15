mod raw;

use crate::config::raw::GeneretoConfigRaw;
use anyhow::bail;
use std::path::{Path, PathBuf};

const OUTPUT_DIR: &str = "output";

const CONFIG_FILENAME: &str = "config.yml";
// folder name for the markdown files with the content of the generated website
const CONTENT: &str = "content";
// Template folder name
const TEMPLATES: &str = "templates";

pub fn validate_project_folders(project_path: &Path) -> anyhow::Result<()> {
    if !project_path.exists() {
        bail!("Project path {} does not exist", project_path.display());
    }
    let paths: [PathBuf; 4] = [
        project_path.to_path_buf(),
        project_path.join(CONFIG_FILENAME),
        project_path.join(TEMPLATES),
        project_path.join(CONTENT),
    ];
    for p in paths {
        if !p.exists() {
            bail!("Path {} does not exist", p.display());
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneretoConfig {
    pub template_dir_path: PathBuf,
    pub output_dir_path: PathBuf,
    pub project_path: PathBuf,
    pub content_path: PathBuf,
    //todo: move to option and if None, use the first item in the templates folder.
    pub template: String,
    /// Optional path to look for templates. Can be relative or absolute.
    pub template_base_path: Option<PathBuf>,
    /// title of the website - used in rss
    pub title: String,
    /// url of the website - used in rss.
    pub url: String,
    /// description of the website - used in rss.
    pub description: String,
    pub default_cover_image: String,
    pub blog: GeneretoConfigBlog,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneretoConfigBlog {
    pub base_template: PathBuf,
    pub index_name: PathBuf,
    pub destination: PathBuf,
    pub generate_single_pages: bool,
    pub title: Option<String>,
}
impl GeneretoConfigBlog {
    fn new_from_raw(project_path: &Path, raw_config: &GeneretoConfigRaw) -> Self {
        let blog_raw = &raw_config.blog;
        let base_template = project_path
            .join(TEMPLATES)
            .join(&raw_config.template)
            .join(&blog_raw.base_template);
        let destination = project_path.join(OUTPUT_DIR).join(&blog_raw.destination);

        Self {
            base_template,
            index_name: raw_config.blog.index_name.clone(),
            destination,
            generate_single_pages: raw_config.blog.generate_single_pages,
            title: raw_config.blog.title.clone(),
        }
    }
}

impl GeneretoConfig {
    pub fn load_from_folder<P: AsRef<Path>>(project_path: P) -> anyhow::Result<Self> {
        let project_path = project_path.as_ref().to_path_buf();
        let raw_config = GeneretoConfigRaw::load_from_path(&project_path)?;
        let blog = GeneretoConfigBlog::new_from_raw(&project_path, &raw_config);

        // Determine template directory path based on template_path and template
        let template_dir_path = if let Some(template_base_path) = &raw_config.template_base_path {
            if template_base_path.is_absolute() {
                if raw_config.template.is_empty() {
                    template_base_path.clone()
                } else {
                    template_base_path.join(&raw_config.template)
                }
            } else {
                let relative_base = project_path.join(template_base_path);
                if raw_config.template.is_empty() {
                    relative_base
                } else {
                    relative_base.join(&raw_config.template)
                }
            }
        } else {
            project_path.join(TEMPLATES).join(&raw_config.template)
        };

        let output_dir_path = project_path.join(OUTPUT_DIR);
        let content_path = project_path.join(CONTENT);
        Ok(Self {
            template_dir_path,
            output_dir_path,
            content_path,
            project_path,
            template: raw_config.template,
            template_base_path: raw_config.template_base_path,
            title: raw_config.title,
            url: raw_config.url,
            description: raw_config.description,
            default_cover_image: raw_config.default_cover_image,
            blog,
        })
    }

    pub fn get_blog_dest_path(&self, entry_path: &Path) -> PathBuf {
        self.inner_get_dest_path(entry_path, &self.blog.destination)
    }

    /// Given a file or directory, returns the final destination path in output directory.
    pub fn get_dest_path(&self, entry_path: &Path) -> PathBuf {
        self.inner_get_dest_path(entry_path, &self.output_dir_path)
    }
    fn inner_get_dest_path(&self, entry_path: &Path, base_path: &Path) -> PathBuf {
        println!("entry path: {entry_path:?}");

        let name = entry_path.file_name().unwrap().to_str().unwrap();
        // unwraps needed because these returns optional
        base_path.join(if entry_path.is_dir() {
            name.to_string()
        } else {
            name.replace(".md", ".html")
        })
    }
}
