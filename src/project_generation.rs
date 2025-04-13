use anyhow::bail;
use std::path::Path;

const SAMPLE_BLOG_PAGE: &str = include_str!("../sample-genereto-project/templates/main/blog.html");
const SAMPLE_INDEX_PAGE: &str =
    include_str!("../sample-genereto-project/templates/main/index.html");
const SAMPLE_CONFIG_FILE: &str = include_str!("../sample-genereto-project/config.yml");
const SAMPLE_CONTENT_PAGE: &str =
    include_str!("../sample-genereto-project/content/blog/2024-05-04-hello-world.md");
const SAMPLE_IMAGE: &[u8] =
    include_bytes!("../sample-genereto-project/content/blog/2024-05-04-hello-world/rustacean.png");

pub fn generate_project(project_path: &Path, override_git: bool) -> anyhow::Result<()> {
    // check if project_path has a .git directory.
    // if it doesn't and override_git is false, this script could cause irreversible overwrites.
    if !project_path.join(".git").exists() && !override_git {
        bail!(".git directory not found. This script could cause irreversible overwrites. Please initialize a git repo using `git init` or use --override-git flag. Exiting.")
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
    // create a directory for today's blog post
    std::fs::create_dir_all(
        project_path.join(format!("genereto-project/content/{today}-hello-world")),
    )?;

    let sample_content = SAMPLE_CONTENT_PAGE.replace("2024-05-04", &today.to_string());
    std::fs::write(
        project_path.join(format!("genereto-project/content/{today}-hello-world.md")),
        sample_content,
    )?;
    std::fs::write(
        project_path.join(format!(
            "genereto-project/content/{today}-hello-world/rustacean.png"
        )),
        SAMPLE_IMAGE,
    )?;

    Ok(())
}

