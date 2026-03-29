use std::fmt;
use std::path::{Path, PathBuf};

use crate::GeneretoConfig;

pub mod assets;
pub mod cover_image;
pub mod date_mismatch;
pub mod empty_links;
pub mod external_links;
pub mod internal_links;

/// The set of available verification checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, clap::ValueEnum)]
pub enum Check {
    /// Missing image/asset files referenced in markdown content
    Assets,
    /// cover_image frontmatter points to a nonexistent file
    CoverImage,
    /// Filename date prefix doesn't match publish_date
    DateMismatch,
    /// Markdown links with empty display text
    EmptyLinks,
    /// Broken local href/src references in generated HTML
    InternalLinks,
    /// External URL reachability (cached via link_cache.csv)
    ExternalLinks,
}

impl Check {
    pub fn all() -> Vec<Check> {
        vec![
            Check::Assets,
            Check::CoverImage,
            Check::DateMismatch,
            Check::EmptyLinks,
            Check::InternalLinks,
            Check::ExternalLinks,
        ]
    }
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Check::Assets => write!(f, "assets"),
            Check::CoverImage => write!(f, "cover-image"),
            Check::DateMismatch => write!(f, "date-mismatch"),
            Check::EmptyLinks => write!(f, "empty-links"),
            Check::InternalLinks => write!(f, "internal-links"),
            Check::ExternalLinks => write!(f, "external-links"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Warning => write!(f, "warning"),
            Severity::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VerifyIssue {
    pub check: Check,
    pub severity: Severity,
    pub file: PathBuf,
    pub line: Option<usize>,
    pub message: String,
}

impl fmt::Display for VerifyIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let loc = if let Some(line) = self.line {
            format!("{}:{}", self.file.display(), line)
        } else {
            self.file.display().to_string()
        };
        write!(
            f,
            "[{}] {}: {} ({})",
            self.severity, self.check, self.message, loc
        )
    }
}

/// Run the specified checks against a genereto project.
///
/// `checks` — which checks to run. If empty, all checks run.
/// `output_dir` — the built output directory (needed for internal/external link checks).
///
/// Returns the list of issues found.
pub fn run_checks(
    config: &GeneretoConfig,
    checks: &[Check],
    output_dir: &Path,
) -> Vec<VerifyIssue> {
    let checks = if checks.is_empty() {
        Check::all()
    } else {
        checks.to_vec()
    };

    let mut issues = Vec::new();

    for check in &checks {
        let check_issues = match check {
            Check::Assets => assets::check(config),
            Check::CoverImage => cover_image::check(config),
            Check::DateMismatch => date_mismatch::check(config),
            Check::EmptyLinks => empty_links::check(config),
            Check::InternalLinks => internal_links::check(output_dir),
            Check::ExternalLinks => external_links::check(config, output_dir),
        };
        issues.extend(check_issues);
    }

    issues
}

/// Print issues to stderr and return the count.
pub fn report_issues(issues: &[VerifyIssue]) -> usize {
    for issue in issues {
        match issue.severity {
            Severity::Warning => warn!("{}", issue),
            Severity::Error => error!("{}", issue),
        }
    }
    issues.len()
}
