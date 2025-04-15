use std::{fs, io};
use tempfile::tempdir;

#[test]
fn test_custom_metadata_in_generated_pages() -> io::Result<()> {
    // Create temporary project directory
    let temp_dir = tempdir()?;
    let project_path = temp_dir.path().to_path_buf();

    // Create project structure
    fs::create_dir_all(project_path.join("content/blog"))?;
    fs::create_dir_all(project_path.join("templates/main"))?;

    // Create a test blog post with custom metadata
    let blog_content = r#"title: Test Post
publish_date: 2024-01-01
keywords: test
author_twitter: "@testauthor"
project_repo: "https://github.com/test/repo"
----
# Test Content

This is a test post with custom metadata."#;

    fs::write(project_path.join("content/blog/test-post.md"), blog_content).unwrap();

    // Create test template that uses custom metadata
    let template_content = r#"<html>
<head>
    <title>$GENERETO['title']</title>
</head>
<body>
    <div class="metadata">
        <p>Author Twitter: $GENERETO['author_twitter']</p>
        <p>Project Repository: $GENERETO['project_repo']</p>
    </div>
    <!-- start_content -->
    {{content}}
    <!-- end_content -->
</body>
</html>"#;

    fs::write(
        project_path.join("templates/main/blog.html"),
        template_content,
    )?;
    fs::write(
        project_path.join("templates/main/index.html"),
        template_content,
    )?;

    // Create config
    let raw_config = r#"
template: main
title: Test
url: http://test.com
description: Test blog description
default_cover_image: cover.jpg
blog:
  base_template: blog.html
  index_name: index.html
  destination: blog
  generate_single_pages: true
"#;
    // write to file:
    fs::write(project_path.join("config.yml"), raw_config)?;

    // Generate blog
    let drafts_options = genereto::DraftsOptions::new(false);
    genereto::run(project_path.clone(), drafts_options).unwrap();

    // Verify output file exists
    let output_file = project_path.join("output/blog/test-post.html");
    assert!(output_file.exists(), "Output file was not created");

    // Read and verify content
    let generated_content = fs::read_to_string(output_file).unwrap();
    assert!(
        generated_content.contains("@testauthor"),
        "Custom metadata 'author_twitter' not found in output"
    );
    assert!(
        generated_content.contains("https://github.com/test/repo"),
        "Custom metadata 'project_repo' not found in output"
    );
    Ok(())
}
