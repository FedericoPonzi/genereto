#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};

use config::GeneretoConfig;
use std::{fs, io};

mod blog;
mod config;
mod fs_util;
mod page_metadata;
mod parser;
mod project_generation;
mod rss_generation;

pub use project_generation::generate_project;

use crate::blog::generate_blog;
use crate::fs_util::copy_directory_recursively;
use crate::rss_generation::generate_rss;

const START_PATTERN: &str = "<!-- start_content -->";
const END_PATTERN: &str = "<!-- end_content -->";

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum DraftsOptions {
    /// Default. Builds the draft page, but is not linked anywhere. Useful to share the draft.
    #[default]
    Build,
    /// Considers the draft page as a normal page. Useful during development to preview drafts
    Dev,
    /// Hides draft pages. They will not be built and will not be linked anywhere.
    Hide,
}
impl DraftsOptions {
    fn is_dev(&self) -> bool {
        matches!(self, DraftsOptions::Dev)
    }
    fn is_hide(&self) -> bool {
        matches!(self, DraftsOptions::Hide)
    }
}

/// project: path to the project.
pub fn run(project_path: PathBuf, drafts_options: DraftsOptions) -> anyhow::Result<()> {
    let genereto_config = GeneretoConfig::load_from_path(project_path)?;

    if genereto_config.output_dir_path.exists() {
        fs::remove_dir_all(&genereto_config.output_dir_path)?;
    }
    fs::create_dir_all(&genereto_config.output_dir_path)?;

    let metadatas = generate_blog(&genereto_config, drafts_options)?;
    let has_blog = metadatas.is_some();
    if has_blog {
        generate_rss(
            genereto_config.title,
            genereto_config.url,
            genereto_config.description,
            metadatas.unwrap(),
            &genereto_config.output_dir_path,
        )?;
    }

    copy_folders_from_template(
        &genereto_config.template_dir_path,
        &genereto_config.output_dir_path,
    )?;

    Ok(())
}

/// Used to copy resources and assets from the template folder
fn copy_folders_from_template(template_dir_path: &Path, output_dir_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(template_dir_path)? {
        let entry_path = entry?.path();
        let entry_path_name = entry_path.file_name().unwrap().to_str().unwrap();
        if entry_path.is_dir() {
            copy_directory_recursively(
                entry_path.as_ref(),
                output_dir_path.join(entry_path_name).as_ref(),
            )?;
        }
    }
    Ok(())
}
