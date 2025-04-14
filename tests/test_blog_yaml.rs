use genereto::DraftsOptions;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_blog_yaml_integration() -> anyhow::Result<()> {
    let tmp_dir = TempDir::with_prefix("example")?;
    let project_path = tmp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("templates/default"))?;
    fs::create_dir_all(project_path.join("content"))?;

    // Create config.yml
    let config_content = r#"
template: default
title: Test Blog
url: http://test.com
description: Test blog description
default_cover_image: cover.jpg
blog:
  base_template: blog.html
  index_name: blog.html
  destination: blog
  generate_single_pages: true
"#;
    fs::write(project_path.join("config.yml"), config_content)?;

    // Create blog.yml with entries
    let blog_content = r#"
entries:
  - title: Test Article 1
    publish_date: 2024-01-01
    keywords: test
    description: Test description 1
    cover_image: cover1.jpg
  - title: Test Article 2
    publish_date: 2024-01-02
    keywords: test2
    description: Test description 2
    cover_image: cover2.jpg
"#;
    fs::write(project_path.join("content/blog.yml"), blog_content)?;

    // Create blog template
    let blog_template = r#"<!DOCTYPE html><html><body><title>$GENERETO['title']</title>
<!-- start_content -->
<div class="post">
<h2>$GENERETO['title']</h2>
<p>$GENERETO['description']</p>
</div>
<!-- end_content -->
</body></html>"#
        .to_string();
    fs::write(
        project_path.join("templates/default/blog.html"),
        &blog_template,
    )?;
    fs::write(
        project_path.join("templates/default/index.html"),
        blog_template,
    )?;

    // Run genereto
    genereto::run(project_path.into(), DraftsOptions::Build).unwrap();

    // Verify output
    let output = fs::read_to_string(project_path.join("output/blog/blog.html"))?;
    assert!(output.contains("Test Article 1"));
    assert!(output.contains("Test Article 2"));
    assert!(output.contains("Test description 1"));
    assert!(output.contains("Test description 2"));

    Ok(())
}
