use std::path::Path;

use regex::Regex;

use crate::verify::{Check, Severity, VerifyIssue};
use crate::GeneretoConfig;

/// Check for markdown links with empty display text: `[](url)`
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

    // Matches [](url) — we check manually that it's not preceded by !
    let empty_link_re = Regex::new(r"\[]\(([^)]+)\)").unwrap();

    let mut in_code = false;
    for (line_idx, line) in content.lines().enumerate() {
        if line.trim().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            continue;
        }

        for m in empty_link_re.find_iter(line) {
            // Skip if preceded by '!' (image syntax)
            let start = m.start();
            if start > 0 && line.as_bytes()[start - 1] == b'!' {
                continue;
            }
            let cap = empty_link_re.captures(m.as_str()).unwrap();
            let url = &cap[1];
            issues.push(VerifyIssue {
                check: Check::EmptyLinks,
                severity: Severity::Warning,
                file: filepath.to_path_buf(),
                line: Some(line_idx + 1),
                message: format!("Empty link text: []({})", url),
            });
        }
    }
}
