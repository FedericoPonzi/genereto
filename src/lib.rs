#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};
use std::{fs, io};

use anyhow::Context;

pub use config::GeneretoConfig;
pub use config::GeneretoConfigBlog;
pub use project_generation::generate_project;
pub mod blog;

use crate::fs_util::copy_directory_recursively;
use crate::jinja_processor::SiteContext;
use crate::rss_generation::generate_rss;

mod config;
mod fs_util;
pub mod jinja_processor;
mod page_metadata;
mod parser;
mod project_generation;
mod rss_generation;

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
    pub fn new(is_dev: bool) -> Self {
        if is_dev {
            DraftsOptions::Dev
        } else {
            DraftsOptions::Build
        }
    }

    fn is_dev(&self) -> bool {
        matches!(self, DraftsOptions::Dev)
    }
    fn is_hide(&self) -> bool {
        matches!(self, DraftsOptions::Hide)
    }
}

/// project: path to the project.
pub fn run(project_path: PathBuf, drafts_options: DraftsOptions) -> anyhow::Result<PathBuf> {
    let genereto_config = GeneretoConfig::load_from_folder(project_path)?;
    debug!("GeneretoConfig: {genereto_config:?}");
    if genereto_config.output_dir_path.exists() {
        fs::remove_dir_all(&genereto_config.output_dir_path)?;
    }
    fs::create_dir_all(&genereto_config.output_dir_path)?;

    let metadata = blog::generate_blog(&genereto_config, &drafts_options)?;
    let has_blog = metadata.is_some();
    if has_blog {
        generate_rss(
            &genereto_config.title,
            &genereto_config.url,
            &genereto_config.description,
            metadata.unwrap(),
            &genereto_config.output_dir_path,
        )?;
    }
    compile_pages(&genereto_config, &drafts_options)?;

    copy_folders_from_template(
        &genereto_config.template_dir_path,
        &genereto_config.output_dir_path,
    )?;
    Ok(genereto_config.output_dir_path)
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
    // Create site context for Jinja rendering if enabled
    let site_context = if genereto_config.enable_jinja {
        Some(SiteContext::new(
            &genereto_config.title,
            &genereto_config.url,
            &genereto_config.description,
        ))
    } else {
        None
    };

    // Load the default template once before the loop
    let default_template = parser::load_template(
        &genereto_config.template_dir_path,
        PAGE_TEMPLATE_FILENAME,
    )?;

    for entry in fs::read_dir(&genereto_config.content_path)? {
        let entry_path = entry?.path();
        let entry_path_name = entry_path.file_name().unwrap().to_str().unwrap();
        let destination_path = genereto_config.get_dest_path(&entry_path);

        if entry_path.is_dir() {
            if entry_path_name == "blog" {
                debug!("Skipping blog directory from compile pages.");
                continue;
            }
            copy_directory_recursively(&entry_path, &destination_path)?;
        } else if entry_path.is_file() && entry_path.extension().unwrap_or_default() == "md" {
            // Read source content and parse metadata first to check for custom template
            let source_content = fs::read_to_string(&entry_path)
                .with_context(|| format!("Failed to read page {entry_path:?}"))?;
            let (intermediate_content, metadata_raw) = parser::compile_page_phase_1(&source_content)
                .with_context(|| format!("Failed to parse page {entry_path:?}"))?;

            // Use custom template if specified, otherwise use default
            let template_raw = if let Some(ref template_file) = metadata_raw.template_file {
                parser::load_template(&genereto_config.template_dir_path, template_file)
                    .with_context(|| {
                        format!(
                            "Page '{}' specifies template '{}' which could not be loaded",
                            entry_path.display(),
                            template_file
                        )
                    })?
            } else {
                default_template.clone()
            };

            // Compile phase 2 with the selected template
            let (content, metadata) = parser::compile_page_phase_2(
                intermediate_content,
                &template_raw,
                metadata_raw,
                "",
                &entry_path,
                &genereto_config.url,
                site_context.as_ref(),
            )
            .with_context(|| format!("Failed to compile page {entry_path:?}"))?;

            // Handle drafts and write output
            if metadata.is_draft && drafts_options.is_hide() {
                continue;
            }
            fs::write(&destination_path, content)
                .with_context(|| format!("Failed to write page to {destination_path:?}"))?;
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
