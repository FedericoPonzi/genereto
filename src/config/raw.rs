use crate::config::CONFIG_FILENAME;
use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct GeneretoConfigBlogRaw {
    #[serde(default = "index_html")]
    pub(crate) base_template: PathBuf,
    #[serde(default = "index_html")]
    pub(crate) index_name: PathBuf,
    #[serde(default = "blog_destination")]
    pub(crate) destination: PathBuf,
    #[serde(default = "default_single_pages")]
    pub(crate) generate_single_pages: bool,
    #[serde(default)]
    pub(crate) title: Option<String>,
    #[serde(default)]
    pub(crate) default_cover_image: String,
}

fn index_html() -> PathBuf {
    "index.html".into()
}
fn blog_destination() -> PathBuf {
    "".into()
}

fn default_single_pages() -> bool {
    true
}

impl Default for GeneretoConfigBlogRaw {
    fn default() -> Self {
        Self {
            base_template: index_html(),
            index_name: index_html(),
            destination: blog_destination(),
            generate_single_pages: default_single_pages(),
            title: None,
            default_cover_image: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub(crate) struct GeneretoConfigRaw {
    #[serde(default)]
    pub template: String,
    /// Optional path to look for templates. Can be relative or absolute.
    #[serde(default)]
    pub template_base_path: Option<PathBuf>,
    /// title of the website - used in rss
    #[serde(default)]
    pub title: String,
    /// url of the website - used in rss.
    #[serde(default)]
    pub url: String,
    /// description of the website - used in rss.
    #[serde(default)]
    pub description: String,

    #[serde(default)]
    // this is not an option because there is a default choice for each BlogConfig field
    // (in the Default trait impl)
    pub(crate) blog: GeneretoConfigBlogRaw,
}
impl GeneretoConfigRaw {
    pub fn load_from_path(project_path: &Path) -> anyhow::Result<Self> {
        if !project_path.exists() {
            bail!("Project path '{}' doesn't exists.", project_path.display());
        }
        if !project_path.is_dir() {
            bail!(
                "Project path '{}' is not a folder. It should be a folder with a '{}' file inside.",
                project_path.display(),
                CONFIG_FILENAME
            );
        }
        let config_file: Self =
            serde_yaml_ng::from_reader(&File::open(project_path.join(CONFIG_FILENAME))?)?;
        Ok(config_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::raw::GeneretoConfigBlogRaw;
    #[test]
    fn test_load_config_without_blog() {
        let sample_full_config = r#"
        template_base_path: a
        template: test_template
        title: full_config
        url: XXXXXXXXXXXXXXXX
        description: Test description
        blog:
            base_template: blog-index.html
            index_name: blog.html
            destination: some/directory/folder
            default_cover_image: Something.jpg
        "#;
        let expected_full_config = GeneretoConfigRaw {
            template: "test_template".into(),
            template_base_path: Some("a".into()),
            title: "full_config".into(),
            url: "XXXXXXXXXXXXXXXX".into(),
            description: "Test description".into(),
            blog: GeneretoConfigBlogRaw {
                base_template: "blog-index.html".into(),
                index_name: "blog.html".into(),
                destination: "some/directory/folder".into(),
                generate_single_pages: true,
                title: None,
                default_cover_image: "Something.jpg".into(),
            },
        };

        let expected_no_blog = GeneretoConfigRaw {
            template: "test_template".into(),
            template_base_path: None,
            title: "no_blog".into(),
            url: "XXXXXXXXXXXXXXXX".into(),
            description: "Test description".into(),
            blog: GeneretoConfigBlogRaw {
                base_template: "index.html".into(),
                index_name: "index.html".into(),
                destination: "".into(),
                generate_single_pages: true,
                title: None,
                default_cover_image: String::new(),
            },
        };

        let no_blog = r#"
        template: test_template
        title: no_blog
        url: XXXXXXXXXXXXXXXX
        description: Test description
        "#;
        for (expected, cfg) in [
            (expected_full_config, sample_full_config),
            (expected_no_blog, no_blog),
        ] {
            let received: GeneretoConfigRaw = serde_yaml_ng::from_str(cfg).unwrap();
            assert_eq!(received, expected);
        }
    }
}
