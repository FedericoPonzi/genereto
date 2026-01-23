use genereto::DraftsOptions;
use std::fs;
use tempfile::TempDir;

/// Test that blog pagination generates multiple pages when max_entries_per_page is set
#[test]
fn test_blog_pagination_multiple_pages() -> anyhow::Result<()> {
    let tmp_dir = TempDir::with_prefix("pagination")?;
    let project_path = tmp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("templates/default"))?;
    fs::create_dir_all(project_path.join("content/blog"))?;

    // Create config.yml with max_entries_per_page: 2
    let config_content = r#"
template: default
title: Test Blog
url: http://test.com
description: Test blog description
blog:
  base_template: blog.html
  index_name: index.html
  destination: ""
  generate_single_pages: true
  max_entries_per_page: 2
"#;
    fs::write(project_path.join("config.yml"), config_content)?;

    // Create 5 blog posts with distinct dates so ordering is deterministic
    for i in 1..=5 {
        let content = format!(
            r#"---
title: Post {}
publish_date: 2024-01-{:02}
description: Description for post {}
keywords: test
---

Content for post {}.
"#,
            i, i, i, i
        );
        fs::write(
            project_path.join(format!("content/blog/2024-01-{:02}-post-{}.md", i, i)),
            content,
        )?;
    }

    // Create blog template with pagination placeholder
    let blog_template = r#"<!DOCTYPE html><html><body>
<h1>$GENERETO['title']</h1>
<!-- start_content -->
<div class="post">
<h2><a href="$GENERETO['file_name']">$GENERETO['title']</a></h2>
<p>$GENERETO['description']</p>
</div>
<!-- end_content -->
$GENERETO['pagination']
</body></html>"#;
    fs::write(
        project_path.join("templates/default/blog.html"),
        blog_template,
    )?;

    // Create a default index.html template (required by genereto)
    let index_template = r#"<!DOCTYPE html><html><body>
<!-- start_content -->
<!-- end_content -->
</body></html>"#;
    fs::write(
        project_path.join("templates/default/index.html"),
        index_template,
    )?;

    // Run genereto
    genereto::run(project_path.into(), DraftsOptions::Build)?;

    // Verify page 1 (index.html) exists and has exactly 2 posts
    // Posts are sorted by date descending, so page 1 has posts 5 and 4
    let page1 = fs::read_to_string(project_path.join("output/index.html"))?;
    assert!(page1.contains("Post 5"), "Page 1 should contain Post 5");
    assert!(page1.contains("Post 4"), "Page 1 should contain Post 4");
    assert!(!page1.contains("Post 3"), "Page 1 should NOT contain Post 3");
    assert!(!page1.contains("Post 2"), "Page 1 should NOT contain Post 2");
    assert!(!page1.contains("Post 1"), "Page 1 should NOT contain Post 1");

    // Verify page 1 has "Next" link but no "Previous" link
    assert!(
        page1.contains("index-page-2.html"),
        "Page 1 should link to page 2"
    );
    assert!(
        !page1.contains("pagination-prev"),
        "Page 1 should NOT have a previous link"
    );

    // Verify page 2 (index-page-2.html) exists and has posts 3 and 2
    let page2 = fs::read_to_string(project_path.join("output/index-page-2.html"))?;
    assert!(page2.contains("Post 3"), "Page 2 should contain Post 3");
    assert!(page2.contains("Post 2"), "Page 2 should contain Post 2");
    assert!(!page2.contains("Post 5"), "Page 2 should NOT contain Post 5");
    assert!(!page2.contains("Post 4"), "Page 2 should NOT contain Post 4");
    assert!(!page2.contains("Post 1"), "Page 2 should NOT contain Post 1");

    // Verify page 2 has both prev and next links
    assert!(
        page2.contains("index.html"),
        "Page 2 should link back to page 1"
    );
    assert!(
        page2.contains("index-page-3.html"),
        "Page 2 should link to page 3"
    );

    // Verify page 3 (index-page-3.html) exists and has post 1
    let page3 = fs::read_to_string(project_path.join("output/index-page-3.html"))?;
    assert!(page3.contains("Post 1"), "Page 3 should contain Post 1");
    assert!(!page3.contains("Post 5"), "Page 3 should NOT contain Post 5");

    // Verify page 3 has "Previous" link but no "Next" link
    assert!(
        page3.contains("index-page-2.html"),
        "Page 3 should link back to page 2"
    );
    assert!(
        !page3.contains("pagination-next"),
        "Page 3 should NOT have a next link"
    );

    // Verify page info text
    assert!(page1.contains("Page 1 of 3"), "Page 1 should show 'Page 1 of 3'");
    assert!(page2.contains("Page 2 of 3"), "Page 2 should show 'Page 2 of 3'");
    assert!(page3.contains("Page 3 of 3"), "Page 3 should show 'Page 3 of 3'");

    Ok(())
}

/// Test backward compatibility: no max_entries_per_page generates single index
#[test]
fn test_blog_no_pagination_backward_compatible() -> anyhow::Result<()> {
    let tmp_dir = TempDir::with_prefix("no_pagination")?;
    let project_path = tmp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("templates/default"))?;
    fs::create_dir_all(project_path.join("content/blog"))?;

    // Config without max_entries_per_page
    let config_content = r#"
template: default
title: Test Blog
url: http://test.com
description: Test blog description
blog:
  base_template: blog.html
  index_name: index.html
  destination: ""
  generate_single_pages: true
"#;
    fs::write(project_path.join("config.yml"), config_content)?;

    // Create 3 blog posts
    for i in 1..=3 {
        let content = format!(
            r#"---
title: Post {}
publish_date: 2024-01-{:02}
description: Description for post {}
keywords: test
---

Content for post {}.
"#,
            i, i, i, i
        );
        fs::write(
            project_path.join(format!("content/blog/2024-01-{:02}-post-{}.md", i, i)),
            content,
        )?;
    }

    // Create templates
    let blog_template = r#"<!DOCTYPE html><html><body>
<h1>$GENERETO['title']</h1>
<!-- start_content -->
<div class="post">
<h2><a href="$GENERETO['file_name']">$GENERETO['title']</a></h2>
</div>
<!-- end_content -->
$GENERETO['pagination']
</body></html>"#;
    fs::write(
        project_path.join("templates/default/blog.html"),
        blog_template,
    )?;
    let index_template = r#"<!DOCTYPE html><html><body>
<!-- start_content -->
<!-- end_content -->
</body></html>"#;
    fs::write(
        project_path.join("templates/default/index.html"),
        index_template,
    )?;

    // Run genereto
    genereto::run(project_path.into(), DraftsOptions::Build)?;

    // Verify single index.html with all posts
    let page = fs::read_to_string(project_path.join("output/index.html"))?;
    assert!(page.contains("Post 1"));
    assert!(page.contains("Post 2"));
    assert!(page.contains("Post 3"));

    // Verify no extra pages
    assert!(
        !project_path.join("output/index-page-2.html").exists(),
        "No second page should be created without pagination"
    );

    // Verify no pagination nav
    assert!(
        !page.contains("pagination"),
        "No pagination HTML should be present without pagination config"
    );

    Ok(())
}

/// Test pagination with custom index_name
#[test]
fn test_blog_pagination_custom_index_name() -> anyhow::Result<()> {
    let tmp_dir = TempDir::with_prefix("pagination_custom")?;
    let project_path = tmp_dir.path();

    fs::create_dir_all(project_path.join("templates/default"))?;
    fs::create_dir_all(project_path.join("content/blog"))?;

    let config_content = r#"
template: default
title: Test Blog
url: http://test.com
description: Test blog description
blog:
  base_template: blog.html
  index_name: blog.html
  destination: ""
  generate_single_pages: true
  max_entries_per_page: 2
"#;
    fs::write(project_path.join("config.yml"), config_content)?;

    for i in 1..=3 {
        let content = format!(
            r#"---
title: Post {}
publish_date: 2024-01-{:02}
description: Description for post {}
keywords: test
---

Content for post {}.
"#,
            i, i, i, i
        );
        fs::write(
            project_path.join(format!("content/blog/2024-01-{:02}-post-{}.md", i, i)),
            content,
        )?;
    }

    let blog_template = r#"<!DOCTYPE html><html><body>
<!-- start_content -->
<div class="post"><h2>$GENERETO['title']</h2></div>
<!-- end_content -->
$GENERETO['pagination']
</body></html>"#;
    fs::write(
        project_path.join("templates/default/blog.html"),
        blog_template,
    )?;
    let index_template = r#"<!DOCTYPE html><html><body>
<!-- start_content -->
<!-- end_content -->
</body></html>"#;
    fs::write(
        project_path.join("templates/default/index.html"),
        index_template,
    )?;

    genereto::run(project_path.into(), DraftsOptions::Build)?;

    // With index_name: blog.html, pages should be blog.html, blog-page-2.html
    assert!(project_path.join("output/blog.html").exists());
    assert!(project_path.join("output/blog-page-2.html").exists());

    let page1 = fs::read_to_string(project_path.join("output/blog.html"))?;
    assert!(page1.contains("blog-page-2.html"), "Page 1 should link to blog-page-2.html");

    let page2 = fs::read_to_string(project_path.join("output/blog-page-2.html"))?;
    assert!(page2.contains("blog.html"), "Page 2 should link back to blog.html");

    Ok(())
}
