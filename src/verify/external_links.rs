use std::collections::HashMap;
use std::path::Path;

use regex::Regex;

use crate::verify::{Check, Severity, VerifyIssue};
use crate::GeneretoConfig;

const CACHE_FILENAME: &str = "link_cache.csv";
/// Re-check URLs older than this many days (6 months ≈ 180 days).
const RECHECK_DAYS: i64 = 180;

#[derive(Debug, Clone)]
struct CacheEntry {
    last_checked: String,
    status: String,
    ignore: bool,
}

fn load_cache(project_path: &Path) -> HashMap<String, CacheEntry> {
    let cache_path = project_path.join(CACHE_FILENAME);
    let mut map = HashMap::new();
    let content = match std::fs::read_to_string(&cache_path) {
        Ok(c) => c,
        Err(_) => return map,
    };

    for line in content.lines().skip(1) {
        // CSV format: url,last_checked,status,ignore
        let fields: Vec<&str> = line.splitn(4, ',').collect();
        if fields.len() < 4 {
            continue;
        }
        map.insert(
            fields[0].to_string(),
            CacheEntry {
                last_checked: fields[1].to_string(),
                status: fields[2].to_string(),
                ignore: fields[3].trim().eq_ignore_ascii_case("true"),
            },
        );
    }
    map
}

fn save_cache(project_path: &Path, cache: &HashMap<String, CacheEntry>) {
    let cache_path = project_path.join(CACHE_FILENAME);
    let mut lines = vec!["url,last_checked,status,ignore".to_string()];

    let mut entries: Vec<_> = cache.iter().collect();
    entries.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (url, entry) in entries {
        lines.push(format!(
            "{},{},{},{}",
            url, entry.last_checked, entry.status, entry.ignore
        ));
    }

    if let Err(e) = std::fs::write(&cache_path, lines.join("\n") + "\n") {
        warn!("Failed to write link cache: {}", e);
    }
}

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn is_stale(last_checked: &str) -> bool {
    let today = chrono::Local::now().date_naive();
    match chrono::NaiveDate::parse_from_str(last_checked, "%Y-%m-%d") {
        Ok(checked) => (today - checked).num_days() > RECHECK_DAYS,
        Err(_) => true,
    }
}

fn check_url_reachable(url: &str) -> (String, bool) {
    let agent = ureq::Agent::new_with_defaults();
    match agent
        .head(url)
        .header("User-Agent", "genereto-verify/1.0")
        .call()
    {
        Ok(response) => {
            let status = response.status().as_str().to_string();
            (status, true)
        }
        Err(ureq::Error::StatusCode(code)) => {
            let status = code.to_string();
            let code_num: u16 = code.into();
            let ok = code_num < 400;
            (status, ok)
        }
        Err(_) => ("unreachable".to_string(), false),
    }
}

/// Collect all external URLs from HTML files in the output directory.
fn collect_external_urls(output_dir: &Path) -> Vec<String> {
    let mut urls = std::collections::BTreeSet::new();
    collect_urls_from_dir(output_dir, &mut urls);
    urls.into_iter().collect()
}

fn collect_urls_from_dir(dir: &Path, urls: &mut std::collections::BTreeSet<String>) {
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
            collect_urls_from_dir(&path, urls);
        } else if path.extension().and_then(|e| e.to_str()) == Some("html") {
            if let Ok(content) = std::fs::read_to_string(&path) {
                let url_re = Regex::new(r#"(?:href|src)="(https?://[^"]+)""#).unwrap();
                for cap in url_re.captures_iter(&content) {
                    urls.insert(cap[1].to_string());
                }
            }
        }
    }
}

/// Check external URL reachability with CSV caching.
pub fn check(config: &GeneretoConfig, output_dir: &Path) -> Vec<VerifyIssue> {
    let mut issues = Vec::new();

    if !output_dir.exists() {
        issues.push(VerifyIssue {
            check: Check::ExternalLinks,
            severity: Severity::Error,
            file: output_dir.to_path_buf(),
            line: None,
            message: "Output directory does not exist — run a build first".to_string(),
        });
        return issues;
    }

    let urls = collect_external_urls(output_dir);
    if urls.is_empty() {
        return issues;
    }

    let mut cache = load_cache(&config.project_path);
    let today_str = today();
    let mut checked_count = 0;

    for url in &urls {
        // If cached and not stale, use cached result
        if let Some(entry) = cache.get(url) {
            if entry.ignore {
                continue;
            }
            if !is_stale(&entry.last_checked) {
                // Use cached status
                if entry.status == "unreachable"
                    || entry.status.starts_with('4')
                    || entry.status.starts_with('5')
                {
                    issues.push(VerifyIssue {
                        check: Check::ExternalLinks,
                        severity: Severity::Warning,
                        file: config.project_path.clone(),
                        line: None,
                        message: format!(
                            "External link unreachable (cached): {} (status: {})",
                            url, entry.status
                        ),
                    });
                }
                continue;
            }
        }

        // Need to check this URL
        let (status, reachable) = check_url_reachable(url);
        checked_count += 1;

        cache.insert(
            url.clone(),
            CacheEntry {
                last_checked: today_str.clone(),
                status: status.clone(),
                ignore: false,
            },
        );

        if !reachable {
            issues.push(VerifyIssue {
                check: Check::ExternalLinks,
                severity: Severity::Warning,
                file: config.project_path.clone(),
                line: None,
                message: format!("External link unreachable: {} (status: {})", url, status),
            });
        }
    }

    if checked_count > 0 {
        info!(
            "Checked {} external URL(s), {} total in cache",
            checked_count,
            cache.len()
        );
    }

    save_cache(&config.project_path, &cache);

    issues
}
