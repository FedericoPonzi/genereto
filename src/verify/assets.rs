use std::path::Path;

use regex::Regex;

use crate::verify::{Check, Severity, VerifyIssue};
use crate::GeneretoConfig;

/// Scan markdown files for image/asset references and verify they exist on disk.
pub fn check(config: &GeneretoConfig) -> Vec<VerifyIssue> {
    let mut issues = Vec::new();
    collect_from_dir(&config.content_path, &config.content_path, &mut issues);
    issues
}

fn collect_from_dir(dir: &Path, content_root: &Path, issues: &mut Vec<VerifyIssue>) {
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
            collect_from_dir(&path, content_root, issues);
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

    let page_name = filepath.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    let content_dir = filepath.parent().unwrap_or(Path::new("."));

    let md_img_re = Regex::new(r"!\[[^\]]*\]\(([^)]+)\)").unwrap();
    let html_img_re = Regex::new(r#"<img\s[^>]*src="([^"]+)""#).unwrap();
    let genereto_re = Regex::new(r#"\$GENERETO\['page_name'\]/([^\s"')]+)"#).unwrap();

    let mut in_code = false;
    for (line_idx, line) in content.lines().enumerate() {
        if line.trim().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            continue;
        }

        let line_num = line_idx + 1;

        // Check $GENERETO['page_name']/file references
        for cap in genereto_re.captures_iter(line) {
            let asset_name = &cap[1];
            let resolved = content_dir.join(page_name).join(asset_name);
            if !resolved.exists() {
                issues.push(VerifyIssue {
                    check: Check::Assets,
                    severity: Severity::Warning,
                    file: filepath.to_path_buf(),
                    line: Some(line_num),
                    message: format!("Missing asset: {}/{}", page_name, asset_name),
                });
            }
        }

        // Check markdown image references ![alt](path)
        for cap in md_img_re.captures_iter(line) {
            let img = &cap[1];
            if img.starts_with("http://") || img.starts_with("https://") || img.starts_with("$") {
                continue;
            }
            let resolved = content_dir.join(img);
            if !resolved.exists() {
                issues.push(VerifyIssue {
                    check: Check::Assets,
                    severity: Severity::Warning,
                    file: filepath.to_path_buf(),
                    line: Some(line_num),
                    message: format!("Missing image: {}", img),
                });
            }
        }

        // Check HTML <img src="..."> references
        for cap in html_img_re.captures_iter(line) {
            let src = &cap[1];
            if src.starts_with("http://") || src.starts_with("https://") || src.starts_with("$") {
                continue;
            }
            let resolved = content_dir.join(src);
            if !resolved.exists() {
                issues.push(VerifyIssue {
                    check: Check::Assets,
                    severity: Severity::Warning,
                    file: filepath.to_path_buf(),
                    line: Some(line_num),
                    message: format!("Missing image: {}", src),
                });
            }
        }
    }
}
