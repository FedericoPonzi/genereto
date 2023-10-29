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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
        let project_path = project_path.as_ref();
        let config_file: Self =
            serde_yaml::from_reader(&File::open(project_path.join(CONFIG_FILENAME))?)?;
        Ok(Self {
            template_dir_path: project_path.join(TEMPLATES).join(&config_file.template),
            output_dir_path: project_path.join(OUTPUT_DIR),
            content_path: project_path.join(CONTENT),
            project_path: project_path.to_path_buf(),
            ..config_file
        })
    }
}
