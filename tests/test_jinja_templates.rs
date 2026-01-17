use std::fs;
use tempfile::tempdir;

#[test]
fn test_jinja_page_template_rendering() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content/blog")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config with jinja enabled
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
enable_jinja: true
title: Test Jinja Site
url: https://test.example.com
description: A test site for Jinja templates
blog:
  base_template: index.html
  index_name: index.html
  destination: blog
  generate_single_pages: true
"#,
    )
    .unwrap();

    // Create blog post template with Jinja syntax
    fs::write(
        project_path.join("templates/main/blog.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>{{ page.title }} - {{ site.title }}</title></head>
<body>
<h1>{{ page.title }}</h1>
<p>Published: {{ page.publish_date }}</p>
<p>{{ page.read_time_minutes }} min read</p>
<article>{{ content }}</article>
<footer>&copy; {{ site.current_year }} {{ site.title }}</footer>
</body>
</html>"#,
    )
    .unwrap();

    // Create blog index template with Jinja syntax
    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>{{ site.title }}</title></head>
<body>
<h1>{{ site.title }}</h1>
<p>{{ site.description }}</p>
{% for article in articles %}
<article>
<h2><a href="{{ article.file_name }}">{{ article.title }}</a></h2>
<p>{{ article.publish_date }}</p>
<p>{{ article.description }}</p>
</article>
{% endfor %}
</body>
</html>"#,
    )
    .unwrap();

    // Create a blog post
    fs::write(
        project_path.join("content/blog/2024-01-15-test-post.md"),
        r#"---
title: Test Blog Post
publish_date: 2024-01-15
description: This is a test blog post for Jinja templates
keywords: test, jinja
---

## Hello World

This is the content of the test blog post.
"#,
    )
    .unwrap();

    // Run genereto
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check the blog post was rendered with Jinja
    let blog_post = fs::read_to_string(project_path.join("output/blog/2024-01-15-test-post.html"))
        .expect("Blog post should exist");

    assert!(
        blog_post.contains("<title>Test Blog Post - Test Jinja Site</title>"),
        "Title should contain page and site titles"
    );
    assert!(
        blog_post.contains("<h1>Test Blog Post</h1>"),
        "H1 should contain page title"
    );
    assert!(
        blog_post.contains("Published: 2024-01-15"),
        "Should contain publish date"
    );
    assert!(
        blog_post.contains("<h2 id=\"hello-world\">Hello World</h2>"),
        "Content should be rendered: {}",
        blog_post
    );
    assert!(
        blog_post.contains("Test Jinja Site"),
        "Footer should contain site title"
    );

    // Check the blog index was rendered with Jinja
    let blog_index = fs::read_to_string(project_path.join("output/blog/index.html"))
        .expect("Blog index should exist");

    assert!(
        blog_index.contains("<title>Test Jinja Site</title>"),
        "Index title should be site title"
    );
    assert!(
        blog_index.contains("<h1>Test Jinja Site</h1>"),
        "H1 should be site title"
    );
    assert!(
        blog_index.contains("A test site for Jinja templates"),
        "Should contain site description"
    );
    assert!(
        blog_index.contains("<a href=\"2024-01-15-test-post.html\">Test Blog Post</a>"),
        "Should contain link to blog post"
    );
    assert!(
        blog_index.contains("This is a test blog post for Jinja templates"),
        "Should contain blog post description"
    );
}

#[test]
fn test_jinja_multiple_articles_in_index() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content/blog")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config with jinja enabled
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
enable_jinja: true
title: Multi Article Test
url: https://test.com
description: Testing multiple articles
blog:
  base_template: index.html
  index_name: index.html
  destination: blog
  generate_single_pages: true
"#,
    )
    .unwrap();

    // Create templates
    fs::write(
        project_path.join("templates/main/blog.html"),
        r#"<!DOCTYPE html><html><body>{{ content }}</body></html>"#,
    )
    .unwrap();

    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<body>
<p>Article count: {{ articles|length }}</p>
{% for article in articles %}
<div class="article-{{ loop.index }}">{{ article.title }}</div>
{% endfor %}
</body>
</html>"#,
    )
    .unwrap();

    // Create multiple blog posts
    for i in 1..=3 {
        fs::write(
            project_path.join(format!("content/blog/2024-01-0{}-post-{}.md", i, i)),
            format!(
                r#"---
title: Post Number {}
publish_date: 2024-01-0{}
description: Description for post {}
---

Content of post {}.
"#,
                i, i, i, i
            ),
        )
        .unwrap();
    }

    // Run genereto
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check the blog index contains all articles
    let blog_index = fs::read_to_string(project_path.join("output/blog/index.html"))
        .expect("Blog index should exist");

    assert!(
        blog_index.contains("Article count: 3"),
        "Should show 3 articles: {}",
        blog_index
    );
    assert!(
        blog_index.contains("Post Number 1"),
        "Should contain Post 1"
    );
    assert!(
        blog_index.contains("Post Number 2"),
        "Should contain Post 2"
    );
    assert!(
        blog_index.contains("Post Number 3"),
        "Should contain Post 3"
    );
}

#[test]
fn test_jinja_custom_metadata() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content/blog")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config with jinja enabled
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
enable_jinja: true
title: Custom Meta Test
url: https://test.com
description: Testing custom metadata
blog:
  base_template: index.html
  index_name: index.html
  destination: blog
  generate_single_pages: true
"#,
    )
    .unwrap();

    // Create template that uses custom metadata
    fs::write(
        project_path.join("templates/main/blog.html"),
        r#"<!DOCTYPE html>
<html>
<body>
<p>Author: {{ page.author }}</p>
<p>Category: {{ page.category }}</p>
{{ content }}
</body>
</html>"#,
    )
    .unwrap();

    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html><html><body>{% for a in articles %}{{ a.title }}{% endfor %}</body></html>"#,
    )
    .unwrap();

    // Create blog post with custom metadata
    fs::write(
        project_path.join("content/blog/2024-01-15-custom.md"),
        r#"---
title: Custom Metadata Post
publish_date: 2024-01-15
description: Testing custom fields
author: John Doe
category: Technology
---

Content here.
"#,
    )
    .unwrap();

    // Run genereto
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check custom metadata is rendered
    let blog_post = fs::read_to_string(project_path.join("output/blog/2024-01-15-custom.html"))
        .expect("Blog post should exist");

    assert!(
        blog_post.contains("Author: John Doe"),
        "Should contain author: {}",
        blog_post
    );
    assert!(
        blog_post.contains("Category: Technology"),
        "Should contain category: {}",
        blog_post
    );
}

#[test]
fn test_jinja_disabled_uses_traditional_rendering() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content/blog")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config with jinja DISABLED (default)
    fs::write(
        project_path.join("config.yml"),
        r#"
template: main
title: Traditional Test
url: https://test.com
description: Testing traditional rendering
blog:
  base_template: index.html
  index_name: index.html
  destination: blog
  generate_single_pages: true
"#,
    )
    .unwrap();

    // Create template with traditional syntax
    fs::write(
        project_path.join("templates/main/blog.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>$GENERETO['title']</title></head>
<body>
<!-- start_content --><!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    fs::write(
        project_path.join("templates/main/index.html"),
        r#"<!DOCTYPE html>
<html>
<head><title>$GENERETO['title']</title></head>
<body>
<!-- start_content -->
<div class="post">
<h2>$GENERETO['title']</h2>
<p>$GENERETO['description']</p>
</div>
<!-- end_content -->
</body>
</html>"#,
    )
    .unwrap();

    // Create blog post
    fs::write(
        project_path.join("content/blog/2024-01-15-traditional.md"),
        r#"---
title: Traditional Post
publish_date: 2024-01-15
description: Using traditional syntax
---

Content here.
"#,
    )
    .unwrap();

    // Run genereto
    let result = genereto::run(project_path.to_path_buf(), genereto::DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check traditional rendering works
    let blog_post =
        fs::read_to_string(project_path.join("output/blog/2024-01-15-traditional.html"))
            .expect("Blog post should exist");

    assert!(
        blog_post.contains("<title>Traditional Post</title>"),
        "Title should be substituted: {}",
        blog_post
    );
    assert!(
        blog_post.contains("Content here."),
        "Content should be rendered"
    );
}
