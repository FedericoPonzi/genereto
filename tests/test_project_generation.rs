use tempfile::tempdir;

/**
 * tests:
 * * no metadata available
 * * article with todos and is draft equal to true and false should not be compiled.
 */
#[test]
fn test_genereto_should_create_index_file() {}

#[test]
fn test_generate_project_with_override_git() {
    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path().to_path_buf();

    // Should fail without git and without override
    assert!(genereto::generate_project(&project_path, false).is_err());

    // Should succeed with override_git
    assert!(genereto::generate_project(&project_path, true).is_ok());

    // Check if project was created
    assert!(project_path.join("genereto-project").exists());
    assert!(project_path.join("genereto-project/templates").exists());
    assert!(project_path.join("genereto-project/config.yml").exists());
}

#[test]
#[ignore]
fn test_blog_generation_with_single_pages_disabled() {
    use genereto::{GeneretoConfig, GeneretoConfigBlog};
    use std::fs;
    use std::path::PathBuf;

    let temp_dir = tempdir().unwrap();
    let project_path = temp_dir.path().to_path_buf();

    // Create test project structure
    fs::create_dir_all(project_path.join("content/blog")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create a test blog post
    fs::write(
        project_path.join("content/blog/test-post.md"),
        "title: Test Post\ndate: 2024-01-01\n---\nTest content",
    )
    .unwrap();

    // Create test template
    fs::write(
        project_path.join("templates/main/blog.html"),
        "<html><!-- start_content -->{{content}}<!-- end_content --></html>",
    )
    .unwrap();

    // Create config with single pages disabled
    let config = GeneretoConfig {
        template_dir_path: project_path.join("templates/main"),
        output_dir_path: project_path.join("output"),
        project_path: project_path.clone(),
        content_path: project_path.join("content"),
        template: "main".into(),
        template_base_path: None,
        title: "Test".into(),
        url: "http://test.com".into(),
        description: "Test".into(),

        blog: GeneretoConfigBlog {
            base_template: PathBuf::from("blog.html"),
            index_name: PathBuf::from("index.html"),
            destination: PathBuf::from("blog"),
            generate_single_pages: false,
            default_cover_image: Some("Something.jpg".into()),
            title: None,
        },
    };

    // Run blog generation
    let drafts_options = genereto::DraftsOptions::new(false);
    let result = genereto::blog::generate_blog(&config, &drafts_options);
    assert!(result.is_ok(), "Failed to generate blog: {:?}", result);

    // Check that index was created but individual post wasn't
    assert!(project_path.join("output/blog/index.html").exists());
    assert!(!project_path.join("output/blog/test-post.html").exists());
}
