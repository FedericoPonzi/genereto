use genereto::{run, DraftsOptions};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_jinja_integration_end_to_end() {
    let temp_dir = TempDir::with_prefix("jinja_integration").unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config with Jinja enabled
    let config_content = r#"
template: 'main'
output_dirname: 'output'
title: "Test Jinja Site"
description: "Testing Jinja templates"
url: "https://test.com"
enable_jinja: true
"#;
    fs::write(project_path.join("config.yml"), config_content).unwrap();

    // Create a regular HTML template (fallback)
    let html_template = r#"<!DOCTYPE html>
<html>
<head>
    <title>$GENERETO['title']</title>
</head>
<body>
    <h1>Regular Template</h1>
    <!-- start_content -->
    <!-- end_content -->
</body>
</html>"#;
    fs::write(project_path.join("templates/main/index.html"), html_template).unwrap();

    // Create a Jinja template that will override the HTML template
    let jinja_template = r#"<!DOCTYPE html>
<html>
<head>
    <title>{{site.title}}</title>
</head>
<body>
    <h1>Jinja Template: {{page.title}}</h1>
    <p>Description: {{page.description}}</p>
    <div class="content">{{content}}</div>
    <footer>Site URL: {{site.url}}</footer>
</body>
</html>"#;
    fs::write(project_path.join("templates/main/index.html.jinja"), jinja_template).unwrap();

    // Create a test page
    let page_content = r#"---
title: "Test Page"
description: "This is a test page"
---

# Hello World

This is a **test** page with _markdown_ content."#;
    fs::write(project_path.join("content/test.md"), page_content).unwrap();

    // Run genereto
    let result = run(project_path.to_path_buf(), DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check that output was created
    let output_path = project_path.join("output/test.html");
    assert!(output_path.exists(), "Output HTML file should exist");

    let output_content = fs::read_to_string(&output_path).unwrap();
    
    // Verify Jinja template was used (not regular template)
    assert!(output_content.contains("Jinja Template: Test Page"), 
            "Should use Jinja template, got: {}", output_content);
    assert!(output_content.contains("Description: This is a test page"),
            "Should include page description from Jinja template");
    assert!(output_content.contains("Site URL: https://test.com"),
            "Should include site URL from Jinja template");
    assert!(output_content.contains("<h1 id=\"hello-world\">Hello World</h1>"),
            "Should include rendered markdown content");
    assert!(output_content.contains("<strong>test</strong>"),
            "Should render markdown formatting");
    
    // Verify regular template was NOT used
    assert!(!output_content.contains("Regular Template"),
            "Should not use regular template when Jinja is available");
}

#[test]
fn test_jinja_fallback_to_regular_template() {
    let temp_dir = TempDir::with_prefix("jinja_fallback").unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config with Jinja enabled
    let config_content = r#"
template: 'main'
output_dirname: 'output'
title: "Test Jinja Site"
description: "Testing Jinja fallback"
url: "https://test.com"
enable_jinja: true
"#;
    fs::write(project_path.join("config.yml"), config_content).unwrap();

    // Create ONLY a regular HTML template (no Jinja template)
    let html_template = r#"<!DOCTYPE html>
<html>
<head>
    <title>$GENERETO['title']</title>
</head>
<body>
    <h1>Regular Template Fallback</h1>
    <!-- start_content -->
    <!-- end_content -->
</body>
</html>"#;
    fs::write(project_path.join("templates/main/index.html"), html_template).unwrap();

    // Create a test page
    let page_content = r#"---
title: "Fallback Test"
description: "This tests fallback"
---

# Fallback Content"#;
    fs::write(project_path.join("content/test.md"), page_content).unwrap();

    // Run genereto
    let result = run(project_path.to_path_buf(), DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check that output was created
    let output_path = project_path.join("output/test.html");
    assert!(output_path.exists(), "Output HTML file should exist");

    let output_content = fs::read_to_string(&output_path).unwrap();
    
    // Verify regular template was used as fallback
    assert!(output_content.contains("Regular Template Fallback"),
            "Should fallback to regular template when no Jinja template exists");
    assert!(output_content.contains("<title>Fallback Test</title>"),
            "Should use regular template variable substitution");
    assert!(output_content.contains("<h1 id=\"fallback-content\">Fallback Content</h1>"),
            "Should include rendered markdown content");
}

#[test]
fn test_jinja_disabled_uses_regular_template() {
    let temp_dir = TempDir::with_prefix("jinja_disabled").unwrap();
    let project_path = temp_dir.path();

    // Create project structure
    fs::create_dir_all(project_path.join("content")).unwrap();
    fs::create_dir_all(project_path.join("templates/main")).unwrap();

    // Create config with Jinja DISABLED
    let config_content = r#"
template: 'main'
output_dirname: 'output'
title: "Test Regular Site"
description: "Testing regular templates"
url: "https://test.com"
enable_jinja: false
"#;
    fs::write(project_path.join("config.yml"), config_content).unwrap();

    // Create both templates
    let html_template = r#"<!DOCTYPE html>
<html>
<head>
    <title>$GENERETO['title']</title>
</head>
<body>
    <h1>Regular Template Used</h1>
    <!-- start_content -->
    <!-- end_content -->
</body>
</html>"#;
    fs::write(project_path.join("templates/main/index.html"), html_template).unwrap();

    let jinja_template = r#"<!DOCTYPE html>
<html>
<head>
    <title>{{site.title}}</title>
</head>
<body>
    <h1>Jinja Template Should Not Be Used</h1>
</body>
</html>"#;
    fs::write(project_path.join("templates/main/index.html.jinja"), jinja_template).unwrap();

    // Create a test page
    let page_content = r#"---
title: "Disabled Test"
description: "This tests disabled Jinja"
---

# Regular Content"#;
    fs::write(project_path.join("content/test.md"), page_content).unwrap();

    // Run genereto
    let result = run(project_path.to_path_buf(), DraftsOptions::Build);
    assert!(result.is_ok(), "Failed to generate site: {:?}", result);

    // Check that output was created
    let output_path = project_path.join("output/test.html");
    assert!(output_path.exists(), "Output HTML file should exist");

    let output_content = fs::read_to_string(&output_path).unwrap();
    
    // Verify regular template was used even though Jinja template exists
    assert!(output_content.contains("Regular Template Used"),
            "Should use regular template when Jinja is disabled");
    assert!(output_content.contains("<title>Disabled Test</title>"),
            "Should use regular template variable substitution");
    assert!(!output_content.contains("Jinja Template Should Not Be Used"),
            "Should NOT use Jinja template when disabled");
}