use std::fs;
use tempfile::tempdir;

/// Test that $GENERETO['page_name'] is substituted in markdown content before HTML compilation,
/// so image references like $GENERETO['page_name']/image.png resolve to the correct path.
#[test]
fn test_page_name_in_blog_article_images() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content/blog")).unwrap();
    fs::create_dir_all(project_path.join("content/blog/2024-01-15-test-post")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create a fake image in the asset directory
    fs::write(
        project_path.join("content/blog/2024-01-15-test-post/diagram.png"),
        "fake png",
    )
    .unwrap();

    // Create config
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
title: Test Site
url: https://test.example.com
description: A test site
blog:
  base_template: index.html
  index_name: index.html
  destination: blog
  generate_single_pages: true
"#,
    )
    .unwrap();

    // Blog post template
    fs::write(
        project_path.join("templates/main/blog.html"),
        r#"<!DOCTYPE html>
<html>
<body>
<!-- start_content -->
placeholder
<!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    // Blog index template
    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<body>
<!-- start_content -->
placeholder
<!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    // Create a blog post using $GENERETO['page_name'] for image references
    fs::write(
        project_path.join("content/blog/2024-01-15-test-post.md"),
        r#"---
title: Test Post
publish_date: 2024-01-15
description: A test post with images
---

Here is a diagram:

![Diagram]($GENERETO['page_name']/diagram.png)

And some inline HTML:

<img src="$GENERETO['page_name']/diagram.png" alt="Diagram">
"#,
    )
    .unwrap();

    // Run genereto
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Read the generated blog post
    let blog_post = fs::read_to_string(project_path.join("output/blog/2024-01-15-test-post.html"))
        .expect("Blog post should exist");

    // Markdown image: should be resolved to the actual page name
    assert!(
        blog_post.contains("2024-01-15-test-post/diagram.png"),
        "Image src should contain the resolved page name, got: {}",
        blog_post
    );

    // No unresolved $GENERETO['page_name'] should remain
    assert!(
        !blog_post.contains("$GENERETO['page_name']"),
        "page_name placeholder should not remain in output"
    );

    // The image asset directory should be copied to output
    assert!(
        project_path
            .join("output/blog/2024-01-15-test-post/diagram.png")
            .exists(),
        "Image file should be copied to output"
    );
}

/// Test that page_name works for non-blog pages too
#[test]
fn test_page_name_in_regular_page() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    fs::create_dir_all(project_path.join("content/my-page")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    fs::write(
        project_path.join("content/my-page/photo.jpg"),
        "fake jpg",
    )
    .unwrap();

    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
title: Test Site
url: https://test.example.com
description: A test site
"#,
    )
    .unwrap();

    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<body>
<!-- start_content -->
placeholder
<!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    fs::write(
        project_path.join("content/my-page.md"),
        r#"---
title: My Page
description: A page with images
---

![Photo]($GENERETO['page_name']/photo.jpg)
"#,
    )
    .unwrap();

    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    let page = fs::read_to_string(project_path.join("output/my-page.html"))
        .expect("Page should exist");

    assert!(
        page.contains("my-page/photo.jpg"),
        "Image src should contain resolved page name, got: {}",
        page
    );
    assert!(
        !page.contains("$GENERETO['page_name']"),
        "page_name placeholder should not remain in output"
    );
}
