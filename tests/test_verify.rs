use std::fs;
use tempfile::tempdir;

use genereto::verify::{self, Check};
use genereto::GeneretoConfig;

/// Helper to create a minimal project structure for verify tests.
fn create_test_project(temp_dir: &std::path::Path, blog_files: &[(&str, &str)]) -> GeneretoConfig {
    fs::create_dir_all(temp_dir.join("content/blog")).unwrap();
    fs::create_dir_all(temp_dir.join("templates/main")).unwrap();
    fs::create_dir_all(temp_dir.join("output")).unwrap();

    fs::write(
        temp_dir.join("config.yml"),
        r#"
template: main
title: Test
url: https://test.example.com
description: Test site
blog:
  base_template: index.html
  index_name: index.html
  destination: blog
  generate_single_pages: true
"#,
    )
    .unwrap();

    // Minimal templates
    fs::write(
        temp_dir.join("templates/main/blog.html"),
        "<!-- start_content -->\n<!-- end_content -->",
    )
    .unwrap();
    fs::write(
        temp_dir.join("templates/main/index.html"),
        "<!-- start_content -->\n<!-- end_content -->",
    )
    .unwrap();

    for (name, content) in blog_files {
        fs::write(temp_dir.join("content/blog").join(name), content).unwrap();
    }

    GeneretoConfig::load_from_folder(temp_dir.to_path_buf()).unwrap()
}

#[test]
fn test_verify_assets_missing_image() {
    let temp_dir = tempdir().unwrap();
    let config = create_test_project(
        temp_dir.path(),
        &[(
            "2024-01-01-test.md",
            "---\ntitle: Test\npublish_date: '2024-01-01'\n---\n\n![img](2024-01-01-test/missing.png)\n",
        )],
    );

    let issues = verify::run_checks(&config, &[Check::Assets], &config.output_dir_path);
    assert!(
        issues.iter().any(|i| i.message.contains("Missing image")),
        "Should detect missing image, got: {:?}",
        issues.iter().map(|i| &i.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_assets_existing_image() {
    let temp_dir = tempdir().unwrap();
    let config = create_test_project(
        temp_dir.path(),
        &[(
            "2024-01-01-test.md",
            "---\ntitle: Test\npublish_date: '2024-01-01'\n---\n\n![img](2024-01-01-test/exists.png)\n",
        )],
    );

    // Create the image file
    fs::create_dir_all(temp_dir.path().join("content/blog/2024-01-01-test")).unwrap();
    fs::write(
        temp_dir
            .path()
            .join("content/blog/2024-01-01-test/exists.png"),
        "fake",
    )
    .unwrap();

    let issues = verify::run_checks(&config, &[Check::Assets], &config.output_dir_path);
    assert!(
        issues.is_empty(),
        "Should not report issues for existing image, got: {:?}",
        issues.iter().map(|i| &i.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_cover_image_missing() {
    let temp_dir = tempdir().unwrap();
    let config = create_test_project(
        temp_dir.path(),
        &[(
            "2024-01-01-test.md",
            "---\ntitle: Test\npublish_date: '2024-01-01'\ncover_image: missing.jpg\n---\n\nContent.\n",
        )],
    );

    let issues = verify::run_checks(&config, &[Check::CoverImage], &config.output_dir_path);
    assert!(
        issues
            .iter()
            .any(|i| i.message.contains("Cover image not found")),
        "Should detect missing cover image, got: {:?}",
        issues.iter().map(|i| &i.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_cover_image_exists() {
    let temp_dir = tempdir().unwrap();
    let config = create_test_project(
        temp_dir.path(),
        &[(
            "2024-01-01-test.md",
            "---\ntitle: Test\npublish_date: '2024-01-01'\ncover_image: cover.jpg\n---\n\nContent.\n",
        )],
    );

    fs::create_dir_all(temp_dir.path().join("content/blog/2024-01-01-test")).unwrap();
    fs::write(
        temp_dir
            .path()
            .join("content/blog/2024-01-01-test/cover.jpg"),
        "fake",
    )
    .unwrap();

    let issues = verify::run_checks(&config, &[Check::CoverImage], &config.output_dir_path);
    assert!(
        issues.is_empty(),
        "Should not report issues for existing cover image, got: {:?}",
        issues.iter().map(|i| &i.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_date_mismatch() {
    let temp_dir = tempdir().unwrap();
    let config = create_test_project(
        temp_dir.path(),
        &[(
            "2024-01-01-test.md",
            "---\ntitle: Test\npublish_date: '2024-06-15'\n---\n\nContent.\n",
        )],
    );

    let issues = verify::run_checks(&config, &[Check::DateMismatch], &config.output_dir_path);
    assert!(
        issues
            .iter()
            .any(|i| i.message.contains("2024-01-01") && i.message.contains("2024-06-15")),
        "Should detect date mismatch, got: {:?}",
        issues.iter().map(|i| &i.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_date_matches() {
    let temp_dir = tempdir().unwrap();
    let config = create_test_project(
        temp_dir.path(),
        &[(
            "2024-01-01-test.md",
            "---\ntitle: Test\npublish_date: '2024-01-01'\n---\n\nContent.\n",
        )],
    );

    let issues = verify::run_checks(&config, &[Check::DateMismatch], &config.output_dir_path);
    assert!(
        issues.is_empty(),
        "Should not report issues when dates match, got: {:?}",
        issues.iter().map(|i| &i.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_empty_links() {
    let temp_dir = tempdir().unwrap();
    let config = create_test_project(
        temp_dir.path(),
        &[(
            "2024-01-01-test.md",
            "---\ntitle: Test\npublish_date: '2024-01-01'\n---\n\nCheck this [](https://example.com) link.\n",
        )],
    );

    let issues = verify::run_checks(&config, &[Check::EmptyLinks], &config.output_dir_path);
    assert!(
        issues.iter().any(|i| i.message.contains("Empty link text")),
        "Should detect empty link text, got: {:?}",
        issues.iter().map(|i| &i.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_empty_links_not_images() {
    let temp_dir = tempdir().unwrap();
    let config = create_test_project(
        temp_dir.path(),
        &[(
            "2024-01-01-test.md",
            "---\ntitle: Test\npublish_date: '2024-01-01'\n---\n\n![](image.png)\n",
        )],
    );

    let issues = verify::run_checks(&config, &[Check::EmptyLinks], &config.output_dir_path);
    assert!(
        issues.is_empty(),
        "Should not flag images without alt text as empty links, got: {:?}",
        issues.iter().map(|i| &i.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_internal_links_broken() {
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(
        output_dir.join("index.html"),
        r#"<html><body><a href="nonexistent.html">Link</a></body></html>"#,
    )
    .unwrap();

    let config = create_test_project(temp_dir.path(), &[]);

    let issues = verify::run_checks(&config, &[Check::InternalLinks], &output_dir);
    assert!(
        issues
            .iter()
            .any(|i| i.message.contains("Broken internal link")),
        "Should detect broken internal link, got: {:?}",
        issues.iter().map(|i| &i.message).collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_internal_links_valid() {
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(
        output_dir.join("index.html"),
        r#"<html><body><a href="about.html">About</a></body></html>"#,
    )
    .unwrap();
    fs::write(output_dir.join("about.html"), "<html></html>").unwrap();

    let config = create_test_project(temp_dir.path(), &[]);

    let issues = verify::run_checks(&config, &[Check::InternalLinks], &output_dir);
    let internal_issues: Vec<_> = issues
        .iter()
        .filter(|i| i.check == Check::InternalLinks)
        .collect();
    assert!(
        internal_issues.is_empty(),
        "Should not report issues for valid links, got: {:?}",
        internal_issues
            .iter()
            .map(|i| &i.message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_verify_all_checks_no_crash() {
    let temp_dir = tempdir().unwrap();
    let config = create_test_project(
        temp_dir.path(),
        &[(
            "2024-01-01-test.md",
            "---\ntitle: Test\npublish_date: '2024-01-01'\n---\n\nHello world.\n",
        )],
    );

    // Build the project first
    let output = genereto::run(
        temp_dir.path().to_path_buf(),
        genereto::DraftsOptions::Build,
    )
    .expect("Build should succeed");

    // Run all checks except external-links (needs network)
    let checks = vec![
        Check::Assets,
        Check::CoverImage,
        Check::DateMismatch,
        Check::EmptyLinks,
        Check::InternalLinks,
    ];
    let issues = verify::run_checks(&config, &checks, &output);
    // Just verify it doesn't crash — issue count depends on template
    let _ = verify::report_issues(&issues);
}
