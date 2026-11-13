use crate::{routes, GeneretoConfig};
use anyhow::{ensure, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Archive {
    pub permalink: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub articles: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct ListingContext {
    pub title: String,
    pub description: String,
    pub permalink: String,
    pub is_archive: bool,
}

pub(crate) struct ListingPage {
    pub destination: PathBuf,
    pub navigation_url: String,
    pub permalink: String,
}

pub(crate) fn load(config: &GeneretoConfig) -> anyhow::Result<Vec<Archive>> {
    let Some(relative) = &config.blog.archives_file else {
        return Ok(Vec::new());
    };
    ensure!(config.enable_jinja, "archives_file requires enable_jinja");
    ensure!(
        !relative.as_os_str().is_empty()
            && relative
                .components()
                .all(|c| matches!(c, Component::Normal(_))),
        "archives_file must be a project-relative path"
    );
    let source = config.project_path.join(relative);
    let mut ancestor = config.project_path.clone();
    for component in relative.components() {
        ancestor.push(component);
        ensure!(
            !fs::symlink_metadata(&ancestor)?.file_type().is_symlink(),
            "Symlink archive input: {}",
            ancestor.display()
        );
    }
    let archives: Vec<Archive> = serde_yaml_ng::from_reader(fs::File::open(&source)?)
        .context("Cannot parse archive definitions")?;
    for archive in &archives {
        ensure!(
            archive.permalink.ends_with('/'),
            "Archive permalink must end in '/'"
        );
        routes::permalink_path(&archive.permalink)?;
        ensure!(
            !archive.title.trim().is_empty(),
            "Archive title must not be empty"
        );
        ensure!(
            archive.articles.iter().collect::<HashSet<_>>().len() == archive.articles.len(),
            "Duplicate article in archive {}",
            archive.permalink
        );
    }
    Ok(archives)
}

fn page(
    config: &GeneretoConfig,
    destination: &Path,
    index_name: &Path,
    first_url: Option<&str>,
    pattern: Option<&str>,
    number: usize,
) -> anyhow::Result<ListingPage> {
    if let Some(pattern) = pattern {
        ensure!(
            pattern.matches("{page}").count() == 1,
            "pagination_path must contain exactly one {{page}}"
        );
        routes::permalink_path(&pattern.replace("{page}", "2"))?;
    }

    let (path, navigation_url) = if number == 1 {
        (
            destination.join(index_name),
            first_url
                .map(str::to_owned)
                .unwrap_or_else(|| index_name.to_string_lossy().into_owned()),
        )
    } else if let Some(pattern) = pattern {
        let url = pattern.replace("{page}", &number.to_string());
        (
            config.output_dir_path.join(routes::permalink_path(&url)?),
            url,
        )
    } else {
        let filename = crate::blog::get_page_filename(index_name, number);
        (
            destination.join(&filename),
            filename.to_string_lossy().into_owned(),
        )
    };
    let permalink = if navigation_url.starts_with('/') {
        navigation_url.clone()
    } else {
        routes::bundle_permalink(config, &path)?
    };
    Ok(ListingPage {
        destination: path,
        navigation_url,
        permalink,
    })
}

pub(crate) fn pages(
    config: &GeneretoConfig,
    article_count: usize,
    archive: Option<&Archive>,
) -> anyhow::Result<Vec<ListingPage>> {
    let first = if let Some(archive) = archive {
        config
            .output_dir_path
            .join(routes::permalink_path(&archive.permalink)?)
    } else {
        config.blog.destination.join(&config.blog.index_name)
    };
    let first_url = routes::bundle_permalink(config, &first)?;
    let archive_pattern = archive.map(|a| format!("{}page/{{page}}/", a.permalink));
    let pattern = archive_pattern
        .as_deref()
        .or(config.blog.pagination_path.as_deref());
    let destination = if archive.is_some() {
        first.parent().unwrap()
    } else {
        &config.blog.destination
    };
    let index_name = if archive.is_some() {
        Path::new("index.html")
    } else {
        &config.blog.index_name
    };
    let first_url = archive
        .map(|a| a.permalink.as_str())
        .or_else(|| pattern.map(|_| first_url.as_str()));
    let max = config
        .blog
        .max_entries_per_page
        .unwrap_or(article_count.max(1));
    ensure!(max > 0, "max_entries_per_page must be greater than zero");
    (1..=article_count.div_ceil(max).max(1))
        .map(|number| page(config, destination, index_name, first_url, pattern, number))
        .collect()
}
