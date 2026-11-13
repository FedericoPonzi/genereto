use crate::{parser, DraftsOptions, GeneretoConfig};
use anyhow::{ensure, Context};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

/// Decode filesystem components once while preserving the original public URI
/// in metadata. Reject anything that could become routing syntax after decoding.
fn decode_segment(segment: &str) -> anyhow::Result<String> {
    let bytes = segment.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let digits = bytes
                .get(index + 1..index + 3)
                .context("Incomplete percent escape")?;
            ensure!(
                digits.iter().all(u8::is_ascii_hexdigit),
                "Invalid percent escape"
            );
            let digits = std::str::from_utf8(digits).context("Invalid percent escape")?;
            decoded.push(u8::from_str_radix(digits, 16).context("Invalid percent escape")?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    let decoded = String::from_utf8(decoded).context("Permalink must decode to valid UTF-8")?;
    ensure!(
        !decoded.is_empty()
            && decoded != "."
            && decoded != ".."
            && !decoded.contains(['/', '\\', '%', '?', '#', ':'])
            && !decoded.chars().any(char::is_control),
        "Unsafe decoded permalink segment: {segment}"
    );
    Ok(decoded)
}

pub(crate) fn permalink_path(permalink: &str) -> anyhow::Result<PathBuf> {
    ensure!(
        permalink.starts_with('/')
            && !permalink.starts_with("//")
            && !permalink.contains(['\\', '?', '#', ':'])
            && !permalink
                .chars()
                .any(|c| c.is_control() || c.is_whitespace()),
        "Unsafe permalink: {permalink}"
    );
    let relative = &permalink[1..];
    let segments = relative.strip_suffix('/').unwrap_or(relative);
    let mut path = PathBuf::new();
    if !segments.is_empty() {
        for segment in segments.split('/') {
            path.push(
                decode_segment(segment)
                    .with_context(|| format!("Unsafe permalink: {permalink}"))?,
            );
        }
    }
    if permalink.ends_with('/') {
        path.push("index.html");
    } else {
        ensure!(
            path.extension().is_some_and(|ext| ext == "html"),
            "Permalink must end in '/' or '.html': {permalink}"
        );
    }
    Ok(path)
}

pub(crate) fn article_destination(
    config: &GeneretoConfig,
    source: &Path,
    bundle: bool,
    permalink: Option<&str>,
) -> anyhow::Result<PathBuf> {
    if let Some(permalink) = permalink {
        Ok(config.output_dir_path.join(permalink_path(permalink)?))
    } else if bundle {
        Ok(config
            .get_blog_dest_path(source.parent().context("Bundle has no parent")?)
            .join("index.html"))
    } else {
        Ok(config.get_blog_dest_path(source))
    }
}

pub(crate) fn bundle_permalink(
    config: &GeneretoConfig,
    destination: &Path,
) -> anyhow::Result<String> {
    let relative = destination.strip_prefix(&config.output_dir_path)?;
    let path = relative.to_str().context("Non-UTF-8 bundle route")?;
    let path = if relative
        .file_name()
        .is_some_and(|name| name == "index.html")
    {
        path.strip_suffix("index.html").unwrap()
    } else {
        path
    };
    Ok(format!("/{path}"))
}

pub(crate) fn page_destination(
    config: &GeneretoConfig,
    source: &Path,
    bundle: bool,
    permalink: Option<&str>,
) -> anyhow::Result<PathBuf> {
    if let Some(permalink) = permalink {
        Ok(config.output_dir_path.join(permalink_path(permalink)?))
    } else if bundle {
        Ok(config
            .get_dest_path(source.parent().context("Bundle has no parent")?)
            .join("index.html"))
    } else {
        Ok(config.get_dest_path(source))
    }
}

pub(crate) fn apply_bundle_metadata(
    config: &GeneretoConfig,
    destination: &Path,
    metadata: &mut crate::page_metadata::PageMetadataRaw,
) -> anyhow::Result<()> {
    if metadata.permalink.is_none() {
        metadata.permalink = Some(bundle_permalink(config, destination)?);
    }
    if let Some(cover) = &mut metadata.cover_image {
        if !cover.is_empty() && !cover.starts_with('/') && !cover.starts_with("http") {
            let directory = metadata
                .permalink
                .as_ref()
                .unwrap()
                .rsplit_once('/')
                .unwrap()
                .0;
            *cover = format!("{directory}/{cover}");
        }
    }
    Ok(())
}

pub(crate) fn bundle_assets(
    source: &Path,
    destination: &Path,
) -> anyhow::Result<Vec<(PathBuf, PathBuf)>> {
    let source_directory = source.parent().context("Bundle has no parent")?;
    let output_directory = destination.parent().context("Output has no parent")?;
    let mut assets = Vec::new();
    for entry in fs::read_dir(source_directory)? {
        let entry = entry?;
        if entry.file_name() != "index.md" {
            assets.push((entry.path(), output_directory.join(entry.file_name())));
        }
    }
    Ok(assets)
}

#[derive(Default)]
struct Outputs {
    files: HashMap<PathBuf, PathBuf>,
    directories: HashSet<PathBuf>,
}

impl Outputs {
    fn directory(&mut self, path: &Path) -> anyhow::Result<()> {
        ensure!(
            !self.files.contains_key(path),
            "Output file/directory collision: {}",
            path.display()
        );
        if self.directories.insert(path.to_path_buf()) {
            if let Some(parent) = path.parent() {
                self.directory(parent)?;
            }
        }
        Ok(())
    }

    fn file(&mut self, path: &Path, source: &Path) -> anyhow::Result<()> {
        ensure!(
            !self.directories.contains(path) && !self.files.contains_key(path),
            "Output collision at {} (source {})",
            path.display(),
            source.display()
        );
        if let Some(parent) = path.parent() {
            self.directory(parent)?;
        }
        self.files.insert(path.to_path_buf(), source.to_path_buf());
        Ok(())
    }

    fn copied_tree(&mut self, source: &Path, destination: &Path) -> anyhow::Result<()> {
        if source.is_dir() {
            if source.file_name().is_some_and(|name| name == ".git") {
                return Ok(());
            }
            self.directory(destination)?;
            for entry in fs::read_dir(source)? {
                let entry = entry?;
                self.copied_tree(&entry.path(), &destination.join(entry.file_name()))?;
            }
        } else {
            self.file(destination, source)?;
        }
        Ok(())
    }
}

pub(crate) fn reject_symlinks(path: &Path) -> anyhow::Result<()> {
    let metadata =
        fs::symlink_metadata(path).with_context(|| format!("Cannot inspect {}", path.display()))?;
    ensure!(
        !metadata.file_type().is_symlink(),
        "Symlink inputs/output are not supported: {}",
        path.display()
    );
    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            reject_symlinks(&entry?.path())?;
        }
    }
    Ok(())
}

fn safe_output_path(config: &GeneretoConfig, path: &Path) -> anyhow::Result<()> {
    let relative = path
        .strip_prefix(&config.output_dir_path)
        .context("Blog output must stay inside the project output directory")?;
    ensure!(
        relative
            .components()
            .all(|c| matches!(c, Component::Normal(_))),
        "Unsafe blog output path: {}",
        path.display()
    );
    Ok(())
}

/// Reserve every generated/copied path before run() removes the old output.
pub(crate) fn validate(config: &GeneretoConfig, drafts: &DraftsOptions) -> anyhow::Result<()> {
    if let Ok(metadata) = fs::symlink_metadata(&config.output_dir_path) {
        ensure!(
            !metadata.file_type().is_symlink(),
            "Output directory must not be a symlink"
        );
    }
    safe_output_path(config, &config.blog.destination)?;
    safe_output_path(
        config,
        &config.blog.destination.join(&config.blog.index_name),
    )?;
    reject_symlinks(&config.content_path)?;
    reject_symlinks(&config.template_dir_path)?;
    let mut outputs = Outputs::default();
    let blog_path = config.content_path.join("blog");
    let has_blog = blog_path.exists() || config.content_path.join("blog.yml").exists();
    let mut article_count = 0usize;
    let mut available = HashSet::new();
    if let Some(entries) =
        crate::blog::BlogEntries::load_from_path(&config.content_path.join("blog.yml"))?
    {
        for entry in entries.entries {
            if let Some(field) = &config.blog.sort_by {
                ensure!(
                    entry
                        .custom_metadata
                        .get(field)
                        .and_then(|v| v.as_str())
                        .is_some(),
                    "Blog entry must provide string sort metadata '{field}'"
                );
            }
            if !entry.is_effective_draft("") || drafts.is_dev() {
                article_count += 1;
                if let Some(url) = &entry.permalink {
                    available.insert(url.clone());
                }
            }
            if let Some(permalink) = entry.permalink {
                permalink_path(&permalink)?;
            }
        }
    }
    if has_blog {
        outputs.file(&config.output_dir_path.join("rss.xml"), &blog_path)?;
    }
    if blog_path.exists() {
        for entry in fs::read_dir(&blog_path)? {
            let source = entry?.path();
            let bundle = source.is_dir() && source.join("index.md").is_file();
            if bundle || (source.is_file() && source.extension().is_some_and(|e| e == "md")) {
                let article = if bundle {
                    source.join("index.md")
                } else {
                    source.clone()
                };
                let (body, metadata) =
                    parser::compile_page_phase_1(&fs::read_to_string(&article)?)?;
                let destination =
                    article_destination(config, &article, bundle, metadata.permalink.as_deref())?;
                safe_output_path(config, &destination)?;
                let is_draft = metadata.is_effective_draft(&body);
                if let Some(field) = &config.blog.sort_by {
                    ensure!(
                        metadata
                            .custom_metadata
                            .get(field)
                            .and_then(|v| v.as_str())
                            .is_some(),
                        "Article {} must provide string sort metadata '{field}'",
                        article.display()
                    );
                }
                if (!is_draft || drafts.is_dev())
                    && article.file_stem().is_some_and(|name| name != "error")
                {
                    article_count += 1;
                    if metadata.permalink.is_some() || bundle {
                        available.insert(
                            metadata
                                .permalink
                                .clone()
                                .unwrap_or(bundle_permalink(config, &destination)?),
                        );
                    }
                }
                if is_draft && drafts.is_hide() {
                    continue;
                }
                if config.blog.generate_single_pages {
                    outputs.file(&destination, &article)?;
                }
                if bundle {
                    for (source, destination) in bundle_assets(&article, &destination)? {
                        outputs.copied_tree(&source, &destination)?;
                    }
                }
            } else if source.is_dir() {
                outputs.copied_tree(&source, &config.get_blog_dest_path(&source))?;
            }
        }
    }
    if has_blog {
        for page in crate::collections::pages(config, article_count, None)? {
            outputs.file(&page.destination, &blog_path)?;
        }
    }
    for archive in crate::collections::load(config)? {
        for article in &archive.articles {
            ensure!(
                available.contains(article),
                "Archive {} references unavailable article {article}",
                archive.permalink
            );
        }
        for page in crate::collections::pages(config, archive.articles.len(), Some(&archive))? {
            outputs.file(&page.destination, &blog_path)?;
        }
    }
    for entry in fs::read_dir(&config.content_path)? {
        let source = entry?.path();
        if source.file_name().is_some_and(|n| n == "blog") {
            continue;
        }
        let bundle = source.is_dir() && source.join("index.md").is_file();
        if bundle || source.extension().is_some_and(|e| e == "md") {
            let article = if bundle {
                source.join("index.md")
            } else {
                source.clone()
            };
            let (body, metadata) = parser::compile_page_phase_1(&fs::read_to_string(&article)?)?;
            if metadata.is_effective_draft(&body) && drafts.is_hide() {
                continue;
            }
            let destination =
                page_destination(config, &article, bundle, metadata.permalink.as_deref())?;
            safe_output_path(config, &destination)?;
            outputs.file(&destination, &article)?;
            if bundle {
                for (source, destination) in bundle_assets(&article, &destination)? {
                    outputs.copied_tree(&source, &destination)?;
                }
            }
        } else if source.is_dir() {
            outputs.copied_tree(&source, &config.get_dest_path(&source))?;
        }
    }
    for entry in fs::read_dir(&config.template_dir_path)? {
        let source = entry?.path();
        if source.is_dir() {
            outputs.copied_tree(
                &source,
                &config.output_dir_path.join(source.file_name().unwrap()),
            )?;
        }
    }
    Ok(())
}
