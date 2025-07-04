use chrono::{Duration, Local};
use genereto::DraftsOptions;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_future_date_as_draft_integration() -> anyhow::Result<()> {
    let tmp_dir = TempDir::with_prefix("example")?;
    let project_path = tmp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("templates/default"))?;
    fs::create_dir_all(project_path.join("content/blog"))?;

    // Create config.yml
    let config_content = r#"
template: default
title: Test Blog
url: http://test.com
description: Test blog description
blog:
  base_template: blog.html
  index_name: blog.html
  destination: blog
  generate_single_pages: true
  default_cover_image: cover.jpg
"#;
    fs::write(project_path.join("config.yml"), config_content)?;

    // Get tomorrow's date
    let tomorrow = Local::now().date_naive() + Duration::days(1);
    let tomorrow_str = tomorrow.format("%Y-%m-%d").to_string();

    // Get yesterday's date
    let yesterday = Local::now().date_naive() - Duration::days(1);
    let yesterday_str = yesterday.format("%Y-%m-%d").to_string();

    // Create a blog post with future date
    let future_post_content = format!(
        r#"title: Future Post
publish_date: '{}'
keywords: test
description: This is a post with a future date
---
# Future Post

This post has a future publish date and should be treated as a draft.
"#,
        tomorrow_str
    );

    // Create a blog post with past date
    let past_post_content = format!(
        r#"title: Past Post
publish_date: '{}'
keywords: test
description: This is a post with a past date
---
# Past Post

This post has a past publish date and should be published normally.
"#,
        yesterday_str
    );

    fs::write(
        project_path.join("content/blog/future-post.md"),
        future_post_content,
    )?;
    fs::write(
        project_path.join("content/blog/past-post.md"),
        past_post_content,
    )?;

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

    // Run genereto with Build drafts option
    genereto::run(project_path.into(), DraftsOptions::Build)?;

    // Verify output
    let blog_index = fs::read_to_string(project_path.join("output/blog/blog.html"))?;
    let future_post = fs::read_to_string(project_path.join("output/blog/future-post.html"))?;
    let past_post = fs::read_to_string(project_path.join("output/blog/past-post.html"))?;

    // Future post should be built but not linked in the index
    assert!(future_post.contains("[DRAFT] Future Post"));
    assert!(!blog_index.contains("Future Post"));

    // Past post should be built and linked in the index
    assert!(past_post.contains("Past Post"));
    assert!(!past_post.contains("[DRAFT]"));
    assert!(blog_index.contains("Past Post"));

    // Run genereto with Dev drafts option
    genereto::run(project_path.into(), DraftsOptions::Dev)?;

    // Verify output with Dev option
    let blog_index_dev = fs::read_to_string(project_path.join("output/blog/blog.html"))?;

    // With Dev option, future post should be linked in the index
    assert!(blog_index_dev.contains("[DRAFT] Future Post"));

    Ok(())
}
