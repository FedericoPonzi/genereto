use std::path::Path;

use regex::Regex;

use crate::verify::{Check, Severity, VerifyIssue};

/// Post-build check: scan output HTML files for unresolved $GENERETO['...'] placeholders
/// in href and src attribute values.
pub fn check(output_dir: &Path) -> Vec<VerifyIssue> {
    let mut issues = Vec::new();
    if !output_dir.exists() {
        return issues;
    }
    collect_from_dir(output_dir, &mut issues);
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
        } else if path.extension().and_then(|e| e.to_str()) == Some("html") {
            check_html_file(&path, issues);
        }
    }
}

fn check_html_file(filepath: &Path, issues: &mut Vec<VerifyIssue>) {
    let content = match std::fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(_) => return,
    };

    // Match $GENERETO['...'] in href="..." and src="..." attributes only
    let attr_re = Regex::new(r#"(?:href|src)="([^"]*\$GENERETO\[[^\]]*\][^"]*)""#).unwrap();

    for (line_idx, line) in content.lines().enumerate() {
        for cap in attr_re.captures_iter(line) {
            let reference = &cap[1];
            issues.push(VerifyIssue {
                check: Check::UnresolvedPlaceholders,
                severity: Severity::Error,
                file: filepath.to_path_buf(),
                line: Some(line_idx + 1),
                message: format!("Unresolved placeholder in attribute: {}", reference),
            });
        }
    }
}
