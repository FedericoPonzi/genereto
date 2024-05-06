#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};

use anyhow::Context;
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
use crate::parser::load_compile_write;
use crate::rss_generation::generate_rss;

const PAGE_TEMPLATE_FILENAME: &str = "index.html";

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
    debug!("GeneretoConfig: {genereto_config:?}");
    if genereto_config.output_dir_path.exists() {
        fs::remove_dir_all(&genereto_config.output_dir_path)?;
    }
    fs::create_dir_all(&genereto_config.output_dir_path)?;

    let metadatas = generate_blog(&genereto_config, &drafts_options)?;
    let has_blog = metadatas.is_some();
    if has_blog {
        generate_rss(
            &genereto_config.title,
            &genereto_config.url,
            &genereto_config.description,
            metadatas.unwrap(),
            &genereto_config.output_dir_path,
        )?;
    }
    compile_pages(&genereto_config, &drafts_options)?;

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

fn compile_pages(
    genereto_config: &GeneretoConfig,
    drafts_options: &DraftsOptions,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(&genereto_config.content_path)? {
        let entry_path = entry?.path();
        let template_path = &genereto_config
            .template_dir_path
            .join(PAGE_TEMPLATE_FILENAME);
        let template_raw = fs::read_to_string(template_path)?;

        let entry_path_name = entry_path.file_name().unwrap().to_str().unwrap();
        let destination_path = genereto_config.get_dest_path(&entry_path);
        if entry_path.is_dir() {
            if entry_path_name == "blog" {
                debug!("Skipping blog directory from compile pages.");
                continue;
            }
            copy_directory_recursively(&entry_path, &destination_path)?;
        } else if entry_path.is_file() && entry_path.extension().unwrap_or_default() == "md" {
            // TODO: test. any other non-md file is copied over to the output folder.
            let _page_opt = load_compile_write(
                &genereto_config.default_cover_image,
                &entry_path,
                drafts_options,
                &destination_path,
                &template_raw,
            )
            .with_context(|| format!("Failed to build page {entry_path:?}"))?;
        } else {
            warn!("Found entry which is not a file nor a directory: {entry_path:?}. Skipping.");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compile_page() {}
}
