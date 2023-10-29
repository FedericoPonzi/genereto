use anyhow::bail;
use std::path::{Path, PathBuf};

const SAMPLE_BLOG_PAGE: &str = include_str!("../sample-genereto-project/templates/main/blog.html");
const SAMPLE_INDEX_PAGE: &str =
    include_str!("../sample-genereto-project/templates/main/index.html");
const SAMPLE_CONFIG_FILE: &str = include_str!("../sample-genereto-project/config.yml");
const SAMPLE_CONTENT_PAGE: &str =
    include_str!("../sample-genereto-project/content/2023-05-15-hello-world.md");
const SAMPLE_IMAGE: &[u8] =
    include_bytes!("../sample-genereto-project/content/2023-05-15-hello-world/rustacean.png");

pub fn generate_project(project_path: &Path) -> anyhow::Result<()> {
    // check if project_path has a .git directory.
    // if it doesn't, this script could cause irreversible overwrites. Let's bail and ask user to use git.
    if !project_path.join(".git").exists() {
        bail!(".git directory not found. This script could cause irreversible overwrites. Please use initialize a git repo using `git init`. Exiting.")
    }
    // create directories:
    std::fs::create_dir_all(project_path.join("genereto-project/templates/main/res"))?;
    //start writing files to where they belong:
    std::fs::write(
        project_path.join("genereto-project/templates/main/blog.html"),
        SAMPLE_BLOG_PAGE,
    )?;
    std::fs::write(
        project_path.join("genereto-project/templates/main/index.html"),
        SAMPLE_INDEX_PAGE,
    )?;
    std::fs::write(
        project_path.join("genereto-project/config.yml"),
        SAMPLE_CONFIG_FILE,
    )?;
    // get todays date in the format YYYY-MM-DD
    let today = chrono::offset::Local::now().format("%Y-%m-%d").to_string();
    // create a directory for today's blog po
    std::fs::create_dir_all(
        project_path.join(format!("genereto-project/content/{today}-hello-world")),
    )?;

    std::fs::write(
        project_path.join(format!("genereto-project/content/{today}-hello-world.md")),
        SAMPLE_CONTENT_PAGE,
    )?;
    std::fs::write(
        project_path.join(format!(
            "genereto-project/content/{today}-hello-world/rustacean.png"
        )),
        SAMPLE_IMAGE,
    )?;

    Ok(())
}
