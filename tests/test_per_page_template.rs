use std::fs;
use tempfile::tempdir;

/// Test that pages can specify a custom template via `template_file` frontmatter
#[test]
fn test_page_with_custom_template() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
title: Custom Template Test
url: https://test.example.com
description: Testing per-page templates
"#,
    )
    .unwrap();

    // Create default template (index.html)
    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>DEFAULT TEMPLATE: $GENERETO['title']</title></head>
<body>
<!-- start_content --><!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    // Create custom landing template
    fs::write(
        project_path.join("templates/main/landing.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>LANDING TEMPLATE: $GENERETO['title']</title></head>
<body class="landing-page">
<!-- start_content --><!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    // Create a page using default template
    fs::write(
        project_path.join("content/normal-page.md"),
        r#"---
title: Normal Page
publish_date: 2024-01-01
description: A page using default template
---

# Normal Page Content

This page uses the default template.
"#,
    )
    .unwrap();

    // Create a page using custom template
    fs::write(
        project_path.join("content/landing-page.md"),
        r#"---
title: Landing Page
publish_date: 2024-01-02
description: A page using custom landing template
template_file: landing.html
---

# Landing Page Content

This page uses the landing template.
"#,
    )
    .unwrap();

    // Run genereto
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check normal page uses default template
    let normal_page = fs::read_to_string(project_path.join("output/normal-page.html"))
        .expect("Normal page should exist");
    assert!(
        normal_page.contains("DEFAULT TEMPLATE: Normal Page"),
        "Normal page should use default template: {}",
        normal_page
    );

    // Check landing page uses custom template
    let landing_page = fs::read_to_string(project_path.join("output/landing-page.html"))
        .expect("Landing page should exist");
    assert!(
        landing_page.contains("LANDING TEMPLATE: Landing Page"),
        "Landing page should use landing template: {}",
        landing_page
    );
    assert!(
        landing_page.contains("class=\"landing-page\""),
        "Landing page should have landing-page class: {}",
        landing_page
    );
}

/// Test that blog posts can also use custom templates
#[test]
fn test_blog_post_with_custom_template() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content/blog")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
title: Blog Template Test
url: https://test.example.com
description: Testing per-page templates in blog
blog:
  base_template: blog-index.html
  index_name: index.html
  destination: blog
  generate_single_pages: true
"#,
    )
    .unwrap();

    // Create default page template (required for compile_pages)
    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>$GENERETO['title']</title></head>
<body><!-- start_content --><!-- end_content --></body>
</html>"#,
    )
    .unwrap();

    // Create default blog template
    fs::write(
        project_path.join("templates/main/blog.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>BLOG TEMPLATE: $GENERETO['title']</title></head>
<body>
<!-- start_content --><!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    // Create blog index template
    fs::write(
        project_path.join("templates/main/blog-index.html"),
        r#"<!DOCTYPE html>
<html>
<body>
<!-- start_content -->
<div class="post"><a href="$GENERETO['file_name']">$GENERETO['title']</a></div>
<!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    // Create custom featured template
    fs::write(
        project_path.join("templates/main/featured.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>FEATURED TEMPLATE: $GENERETO['title']</title></head>
<body class="featured-post">
<!-- start_content --><!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    // Create a normal blog post
    fs::write(
        project_path.join("content/blog/normal-post.md"),
        r#"---
title: Normal Post
publish_date: 2024-01-01
description: A normal blog post
---

# Normal Post Content
"#,
    )
    .unwrap();

    // Create a featured blog post with custom template
    fs::write(
        project_path.join("content/blog/featured-post.md"),
        r#"---
title: Featured Post
publish_date: 2024-01-02
description: A featured blog post
template_file: featured.html
---

# Featured Post Content
"#,
    )
    .unwrap();

    // Run genereto
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check normal post uses default blog template
    let normal_post = fs::read_to_string(project_path.join("output/blog/normal-post.html"))
        .expect("Normal post should exist");
    assert!(
        normal_post.contains("BLOG TEMPLATE: Normal Post"),
        "Normal post should use blog template: {}",
        normal_post
    );

    // Check featured post uses custom template
    let featured_post = fs::read_to_string(project_path.join("output/blog/featured-post.html"))
        .expect("Featured post should exist");
    assert!(
        featured_post.contains("FEATURED TEMPLATE: Featured Post"),
        "Featured post should use featured template: {}",
        featured_post
    );
    assert!(
        featured_post.contains("class=\"featured-post\""),
        "Featured post should have featured-post class: {}",
        featured_post
    );
}

/// Test that missing template produces a clear error
#[test]
fn test_missing_template_error() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
title: Missing Template Test
url: https://test.example.com
description: Testing missing template error
"#,
    )
    .unwrap();

    // Create default template only
    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>$GENERETO['title']</title></head>
<body><!-- start_content --><!-- end_content --></body>
</html>"#,
    )
    .unwrap();

    // Create a page referencing a non-existent template
    fs::write(
        project_path.join("content/broken-page.md"),
        r#"---
title: Broken Page
publish_date: 2024-01-01
description: A page with missing template
template_file: nonexistent.html
---

# Content
"#,
    )
    .unwrap();

    // Run genereto - should fail
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_err(), "Should fail when template is missing");

    let error_message = result.unwrap_err().to_string();
    assert!(
        error_message.contains("nonexistent.html") || error_message.contains("template"),
        "Error should mention the missing template: {}",
        error_message
    );
}

/// Test per-page templates with Jinja2 mode enabled
#[test]
fn test_per_page_template_with_jinja() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config with jinja enabled
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
enable_jinja: true
title: Jinja Template Test
url: https://test.example.com
description: Testing per-page templates with Jinja
"#,
    )
    .unwrap();

    // Create default template with Jinja syntax
    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>DEFAULT: {{ page.title }}</title></head>
<body>{{ content }}</body>
</html>"#,
    )
    .unwrap();

    // Create custom template with Jinja syntax
    fs::write(
        project_path.join("templates/main/special.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>SPECIAL: {{ page.title }}</title></head>
<body class="special">{{ content }}</body>
</html>"#,
    )
    .unwrap();

    // Create a page using custom template
    fs::write(
        project_path.join("content/special-page.md"),
        r#"---
title: Special Page
publish_date: 2024-01-01
description: A special page
template_file: special.html
---

# Special Content
"#,
    )
    .unwrap();

    // Run genereto
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check special page uses custom template with Jinja rendering
    let special_page = fs::read_to_string(project_path.join("output/special-page.html"))
        .expect("Special page should exist");
    assert!(
        special_page.contains("<title>SPECIAL: Special Page</title>"),
        "Special page should use Jinja special template: {}",
        special_page
    );
    assert!(
        special_page.contains("class=\"special\""),
        "Special page should have special class: {}",
        special_page
    );
}

/// Test template path resolution for nested templates
#[test]
fn test_nested_template_path() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure with nested templates
    fs::create_dir_all(project_path.join("content")).unwrap();
    fs::create_dir_all(project_path.join("templates/main/layouts")).unwrap();

    // Create config
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
title: Nested Template Test
url: https://test.example.com
description: Testing nested template paths
"#,
    )
    .unwrap();

    // Create default template
    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>DEFAULT: $GENERETO['title']</title></head>
<body><!-- start_content --><!-- end_content --></body>
</html>"#,
    )
    .unwrap();

    // Create nested template
    fs::write(
        project_path.join("templates/main/layouts/gallery.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>GALLERY: $GENERETO['title']</title></head>
<body class="gallery"><!-- start_content --><!-- end_content --></body>
</html>"#,
    )
    .unwrap();

    // Create a page using nested template path
    fs::write(
        project_path.join("content/my-gallery.md"),
        r#"---
title: My Gallery
publish_date: 2024-01-01
description: A gallery page
template_file: layouts/gallery.html
---

# Gallery Content
"#,
    )
    .unwrap();

    // Run genereto
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check gallery page uses nested template
    let gallery_page = fs::read_to_string(project_path.join("output/my-gallery.html"))
        .expect("Gallery page should exist");
    assert!(
        gallery_page.contains("GALLERY: My Gallery"),
        "Gallery page should use nested gallery template: {}",
        gallery_page
    );
    assert!(
        gallery_page.contains("class=\"gallery\""),
        "Gallery page should have gallery class: {}",
        gallery_page
    );
}
