use std::path::Path;

use regex::Regex;

use crate::verify::{Check, Severity, VerifyIssue};

/// Post-build check: scan output HTML files for broken local href/src references.
pub fn check(output_dir: &Path) -> Vec<VerifyIssue> {
    let mut issues = Vec::new();
    if !output_dir.exists() {
        issues.push(VerifyIssue {
            check: Check::InternalLinks,
            severity: Severity::Error,
            file: output_dir.to_path_buf(),
            line: None,
            message: "Output directory does not exist — run a build first".to_string(),
        });
        return issues;
    }
    collect_from_dir(output_dir, output_dir, &mut issues);
    issues
}

fn collect_from_dir(dir: &Path, output_root: &Path, issues: &mut Vec<VerifyIssue>) {
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
            collect_from_dir(&path, output_root, issues);
        } else if path.extension().and_then(|e| e.to_str()) == Some("html") {
            check_html_file(&path, output_root, issues);
        }
    }
}

fn check_html_file(filepath: &Path, output_root: &Path, issues: &mut Vec<VerifyIssue>) {
    let content = match std::fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(_) => return,
    };

    let file_dir = filepath.parent().unwrap_or(output_root);

    // Match href="..." and src="..." attributes
    let attr_re = Regex::new(r#"(?:href|src)="([^"]+)""#).unwrap();

    for (line_idx, line) in content.lines().enumerate() {
        for cap in attr_re.captures_iter(line) {
            let reference = &cap[1];
            if should_skip(reference) {
                continue;
            }

            // Strip fragment (#anchor)
            let path_part = reference.split('#').next().unwrap_or(reference);
            // Strip query string
            let path_part = path_part.split('?').next().unwrap_or(path_part);

            if path_part.is_empty() {
                continue;
            }

            let resolved = if path_part.starts_with('/') {
                // Absolute path relative to output root
                output_root.join(&path_part[1..])
            } else {
                // Relative path from file's directory
                file_dir.join(path_part)
            };

            // Check if it exists as a file or directory (directory = has index.html)
            if !resolved.exists() && !resolved.join("index.html").exists() {
                issues.push(VerifyIssue {
                    check: Check::InternalLinks,
                    severity: Severity::Error,
                    file: filepath.to_path_buf(),
                    line: Some(line_idx + 1),
                    message: format!("Broken internal link: {}", reference),
                });
            }
        }
    }
}

fn should_skip(reference: &str) -> bool {
    reference.starts_with("http://")
        || reference.starts_with("https://")
        || reference.starts_with("mailto:")
        || reference.starts_with("data:")
        || reference.starts_with("javascript:")
        || reference.starts_with('#')
}
