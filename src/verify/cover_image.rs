use std::path::Path;

use crate::parser::compile_page_phase_1;
use crate::verify::{Check, Severity, VerifyIssue};
use crate::GeneretoConfig;

/// Check that cover_image files referenced in frontmatter exist on disk.
pub fn check(config: &GeneretoConfig) -> Vec<VerifyIssue> {
    let mut issues = Vec::new();
    collect_from_dir(&config.content_path, &mut issues);
    issues
}

fn collect_from_dir(dir: &Path, issues: &mut Vec<VerifyIssue>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.is_dir() {
            collect_from_dir(&path, issues);
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            check_file(&path, issues);
        }
    }
}

fn check_file(filepath: &Path, issues: &mut Vec<VerifyIssue>) {
    let content = match std::fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(_) => return,
    };

    let (_, metadata_raw) = match compile_page_phase_1(&content) {
        Ok(r) => r,
        Err(_) => return,
    };

    let cover_image = match &metadata_raw.cover_image {
        Some(img) if !img.is_empty() => img,
        _ => return,
    };

    // Skip HTTP URLs
    if cover_image.starts_with("http://") || cover_image.starts_with("https://") {
        return;
    }

    let page_stem = filepath.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let content_dir = filepath.parent().unwrap_or(Path::new("."));

    // cover_image is resolved as <page_stem>/<cover_image>
    let resolved = content_dir.join(page_stem).join(cover_image);
    if !resolved.exists() {
        issues.push(VerifyIssue {
            check: Check::CoverImage,
            severity: Severity::Warning,
            file: filepath.to_path_buf(),
            line: None,
            message: format!(
                "Cover image not found: {}/{} (expected at {})",
                page_stem,
                cover_image,
                resolved.display()
            ),
        });
    }
}
