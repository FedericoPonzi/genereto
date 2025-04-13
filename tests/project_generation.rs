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
