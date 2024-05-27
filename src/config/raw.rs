use crate::config::CONFIG_FILENAME;
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
}

fn index_html() -> PathBuf {
    "index.html".into()
}
fn blog_destination() -> PathBuf {
    "".into()
}

impl Default for GeneretoConfigBlogRaw {
    fn default() -> Self {
        Self {
            base_template: index_html(),
            index_name: index_html(),
            destination: blog_destination(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub(crate) struct GeneretoConfigRaw {
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
    #[serde(default)]
    pub default_cover_image: String,
    #[serde(default)]
    // this is not an option because there is a default choice for each BlogConfig field
    // (in the Default trait impl)
    pub(crate) blog: GeneretoConfigBlogRaw,
}
impl GeneretoConfigRaw {
    pub fn load_from_path<P: AsRef<Path>>(project_path: P) -> anyhow::Result<Self> {
        let project_path = project_path.as_ref();
        let config_file: Self =
            serde_yaml::from_reader(&File::open(project_path.join(CONFIG_FILENAME))?)?;
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
            index_name: blog.html
            destination: some/directory/folder
        "#;
        let expected_full_config = GeneretoConfigRaw {
            template: "test_template".into(),
            title: "Test title".into(),
            url: "XXXXXXXXXXXXXXXX".into(),
            description: "Test description".into(),
            default_cover_image: "Something.jpg".into(),
            blog: GeneretoConfigBlogRaw {
                base_template: "blog-index.html".into(),
                index_name: "blog.html".into(),
                destination: "some/directory/folder".into(),
            },
        };

        let expected_no_blog = GeneretoConfigRaw {
            template: "test_template".into(),
            title: "Test title".into(),
            url: "XXXXXXXXXXXXXXXX".into(),
            description: "Test description".into(),
            default_cover_image: "Something.jpg".into(),
            blog: GeneretoConfigBlogRaw {
                base_template: "index.html".into(),
                index_name: "index.html".into(),
                destination: "".into(),
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
            let received: GeneretoConfigRaw = serde_yaml::from_str(cfg).unwrap();
            assert_eq!(received, expected);
        }
    }
}
