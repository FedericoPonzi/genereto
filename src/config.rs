use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};

const OUTPUT_DIR: &str = "output";

const CONFIG_FILENAME: &str = "config.yml";
// folder name for the markdown files with the content of the generated website
const CONTENT: &str = "content";
// Template folder name
const TEMPLATES: &str = "templates";

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GeneretoConfig {
    #[serde(default)]
    pub template_dir_path: PathBuf,
    #[serde(default)]
    pub output_dir_path: PathBuf,
    #[serde(default)]
    pub project_path: PathBuf,
    #[serde(default)]
    pub content_path: PathBuf,
    //todo: move to option and if None, use the first item in the templates folder.
    pub template: String,
    /// title of the website - used in rss
    #[serde(default)]
    pub title: String,
    /// url of the website - used in rss.
    #[serde(default)]
    pub url: String,
    /// description of the website - used in rss.
    #[serde(default)]
    pub description: String,
    pub default_cover_image: Option<String>,
    #[serde(default)]
    pub(crate) blog: BlogConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct BlogConfig {
    #[serde(default = "index_html")]
    pub(crate) base_template: PathBuf,
    #[serde(default = "index_html")]
    pub(crate) index_destination: PathBuf,
}
fn index_html() -> PathBuf {
    "index.html".into()
}
impl Default for BlogConfig {
    fn default() -> Self {
        Self {
            base_template: index_html(),
            index_destination: index_html(),
        }
    }
}

impl GeneretoConfig {
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

    pub fn load_from_path<P: AsRef<Path>>(project_path: P) -> anyhow::Result<Self> {
        Self::validate_project_folders(project_path.as_ref())?;
        // todo, validate url is an url.
        // todo, validate cover_image exists.
        let project_path = project_path.as_ref();
        let config_file: Self =
            serde_yaml::from_reader(&File::open(project_path.join(CONFIG_FILENAME))?)?;
        // todo: create RawConfig and ParsedConfig structs
        Ok(Self {
            template_dir_path: project_path.join(TEMPLATES).join(&config_file.template),
            output_dir_path: project_path.join(OUTPUT_DIR),
            content_path: project_path.join(CONTENT),
            project_path: project_path.to_path_buf(),
            blog: BlogConfig {
                base_template: project_path
                    .join(TEMPLATES)
                    .join(&config_file.template)
                    .join(config_file.blog.base_template),
                index_destination: project_path
                    .join(OUTPUT_DIR)
                    .join(config_file.blog.index_destination),
            },
            ..config_file
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_config_without_blog() {
        let sample_full_config = r#"
        template_dir_path: a
        output_dir_path: b
        project_path: c
        content_path: d
        template: test_template
        title: Test title
        url: XXXXXXXXXXXXXXXX
        description: Test description
        default_cover_image: Something.jpg
        blog:
            base_template: blog-index.html
            index_destination: blog.html
        "#;
        let expected_full_config = GeneretoConfig {
            template_dir_path: "a".into(),
            output_dir_path: "b".into(),
            project_path: "c".into(),
            content_path: "d".into(),
            template: "test_template".into(),
            title: "Test title".into(),
            url: "XXXXXXXXXXXXXXXX".into(),
            description: "Test description".into(),
            default_cover_image: Some("Something.jpg".into()),
            blog: BlogConfig {
                base_template: "blog-index.html".into(),
                index_destination: "blog.html".into(),
            },
        };

        let expected_no_blog = GeneretoConfig {
            template_dir_path: "a".into(),
            output_dir_path: "b".into(),
            project_path: "c".into(),
            content_path: "d".into(),
            template: "test_template".into(),
            title: "Test title".into(),
            url: "XXXXXXXXXXXXXXXX".into(),
            description: "Test description".into(),
            default_cover_image: Some("Something.jpg".into()),
            blog: BlogConfig {
                base_template: "index.html".into(),
                index_destination: "index.html".into(),
            },
        };

        let no_blog = r#"
        template_dir_path: a
        output_dir_path: b
        project_path: c
        content_path: d
        template: test_template
        title: Test title
        url: XXXXXXXXXXXXXXXX
        description: Test description
        default_cover_image: Something.jpg
        "#;
        for (expected, cfg) in [
            (expected_full_config, sample_full_config),
            (expected_no_blog, no_blog),
        ] {
            let received: GeneretoConfig = serde_yaml::from_str(cfg).unwrap();
            assert_eq!(received, expected);
        }
    }
}
