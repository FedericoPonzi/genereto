use std::path::Path;

use regex::Regex;

use crate::parser::compile_page_phase_1;
use crate::verify::{Check, Severity, VerifyIssue};
use crate::GeneretoConfig;

/// Check that filename date prefix matches publish_date in frontmatter.
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
    let filename = match filepath.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return,
    };

    let date_re = Regex::new(r"^(\d{4}-\d{2}-\d{2})-").unwrap();
    let file_date = match date_re.captures(filename) {
        Some(cap) => cap[1].to_string(),
        None => return, // Filename doesn't have a date prefix — nothing to check
    };

    let content = match std::fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(_) => return,
    };

    let (_, metadata_raw) = match compile_page_phase_1(&content) {
        Ok(r) => r,
        Err(_) => return,
    };

    let publish_date = &metadata_raw.publish_date;
    if publish_date.is_empty() {
        issues.push(VerifyIssue {
            check: Check::DateMismatch,
            severity: Severity::Warning,
            file: filepath.to_path_buf(),
            line: None,
            message: "Filename has date prefix but publish_date is empty".to_string(),
        });
        return;
    }

    if publish_date != &file_date {
        issues.push(VerifyIssue {
            check: Check::DateMismatch,
            severity: Severity::Warning,
            file: filepath.to_path_buf(),
            line: None,
            message: format!(
                "Filename date ({}) != publish_date ({})",
                file_date, publish_date
            ),
        });
    }
}
