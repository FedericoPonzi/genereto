use genereto::{GeneretoConfig, GeneretoConfigBlog};
use std::io;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_load_config_sample_genereto_project_works() {
    let config_path = "sample-genereto-project/";

    let config: GeneretoConfig = GeneretoConfig::load_from_folder(PathBuf::from(config_path))
        .expect("Failed to deserialize config");
    assert_eq!(config.title, "My Website");
    assert_eq!(config.description, "The best website in the world.");
    assert_eq!(config.url, "https://blog.fponzi.me");
}

#[test]
fn test_load_config_without_blog() {
    let sample_full_config = r#"
        template: test_template
        template_base_path: /custom/template/path
        title: Test title
        url: XXXXXXXXXXXXXXXX
        description: Test description
        blog:
            base_template: blog-index.html
            index_name: blog.html
            destination: some/directory/folder
            generate_single_pages: false
            title: Custom Blog Title
            default_cover_image: Something.jpg
        "#;
    let expected_full_config = GeneretoConfig {
        template_dir_path: PathBuf::from("/custom/template/path/test_template"),
        output_dir_path: "output".into(),
        project_path: "".into(),
        content_path: "content".into(),
        template: "test_template".into(),
        template_base_path: Some(PathBuf::from("/custom/template/path")),
        title: "Test title".into(),
        url: "XXXXXXXXXXXXXXXX".into(),
        description: "Test description".into(),
        blog: GeneretoConfigBlog {
            base_template: "blog-index.html".into(),
            index_name: "blog.html".into(),
            default_cover_image: Some("Something.jpg".into()),
            destination: "some/directory/folder".into(),
            generate_single_pages: false,
            title: Some("Custom Blog Title".into()),
        },
    };

    let expected_no_blog = GeneretoConfig {
        template_dir_path: "template".into(),
        output_dir_path: "output".into(),
        project_path: "".into(),
        content_path: "content".into(),
        template: "test_template".into(),
        template_base_path: None,
        title: "Test title".into(),
        url: "XXXXXXXXXXXXXXXX".into(),
        description: "Test description".into(),
        blog: GeneretoConfigBlog {
            base_template: "index.html".into(),
            index_name: "index.html".into(),
            destination: "".into(),
            generate_single_pages: true,
            title: None,
            default_cover_image: Some("".into()),
        },
    };

    let no_blog = r#"
        template: test_template
        title: Test title
        url: XXXXXXXXXXXXXXXX
        description: Test description
        default_cover_image: Something.jpg
        "#;

    let relative_template_path = r#"
        template: test_template
        template_base_path: custom/template/path
        title: Test title
        url: XXXXXXXXXXXXXXXX
        description: Test description
        "#;

    let empty_template = r#"
        template: ""
        template_base_path: /custom/template/path
        title: Test title
        url: XXXXXXXXXXXXXXXX
        description: Test description
        "#;

    for (expected, cfg) in [
        (expected_full_config.clone(), sample_full_config),
        (expected_no_blog.clone(), no_blog),
    ] {
        let temp = store_config(cfg).unwrap();
        let received = GeneretoConfig::load_from_folder(temp.path()).unwrap();
        // generate assert_equals for each field:
        for p in [
            received.project_path,
            received.content_path,
            received.blog.destination,
        ] {
            assert!(
                p.starts_with(temp.path()),
                "Path '{}' should start with temp path: {}",
                p.display(),
                temp.path().display()
            );
        }
        assert!(
            received.template_dir_path.starts_with(temp.path())
                || received.template_base_path.is_some(),
            "Path should start with temp path: {}",
            received.template_dir_path.display()
        );
        assert_eq!(expected.template, received.template);
        assert_eq!(expected.template_base_path, received.template_base_path);
        assert_eq!(expected.title, received.title);
        assert_eq!(expected.url, received.url);
        assert_eq!(expected.description, received.description);
        assert_eq!(
            expected.blog.default_cover_image,
            received.blog.default_cover_image
        );
        assert_eq!(expected.blog.title, received.blog.title);
    }

    // Test relative template path
    let temp = store_config(relative_template_path).unwrap();
    let received = GeneretoConfig::load_from_folder(temp.path()).unwrap();
    assert_eq!(
        received.template_base_path,
        Some(PathBuf::from("custom/template/path"))
    );
    assert_eq!(
        received.template_dir_path,
        temp.path().join("custom/template/path/test_template")
    );

    // Test empty template with template_path
    let temp = store_config(empty_template).unwrap();
    let received = GeneretoConfig::load_from_folder(temp.path()).unwrap();
    assert_eq!(
        received.template_base_path,
        Some(PathBuf::from("/custom/template/path"))
    );
    assert_eq!(
        received.template_dir_path,
        PathBuf::from("/custom/template/path")
    );
}
fn store_config(cfg: &str) -> io::Result<TempDir> {
    let temp = TempDir::with_prefix("genereto")?;
    let config_path = temp.path().join("config.yml");
    std::fs::write(config_path, cfg)?;
    Ok(temp)
}
