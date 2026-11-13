use std::fs;
use tempfile::tempdir;

fn project(root_files: &str) -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    let p = dir.path();
    fs::create_dir_all(p.join("content/blog")).unwrap();
    fs::create_dir_all(p.join("templates/main")).unwrap();
    fs::write(
        p.join("config.yml"),
        format!("template: main\n{root_files}\nblog:\n  base_template: index.html\n"),
    )
    .unwrap();
    for template in ["index.html", "blog.html"] {
        fs::write(
            p.join("templates/main").join(template),
            "<!-- start_content --><!-- end_content -->",
        )
        .unwrap();
    }
    fs::write(
        p.join("content/blog/post.md"),
        "---\ntitle: Post\npublish_date: 2020-01-01\nlast_modified_date: 2020-01-01\ndescription: Post\n---\nBody",
    )
    .unwrap();
    fs::write(p.join("content/robots.txt"), "User-agent: *\nAllow: /\n").unwrap();
    fs::write(
        p.join("content/sitemap.xml"),
        "<urlset>caffè &amp; più</urlset>",
    )
    .unwrap();
    fs::write(p.join("content/unlisted.txt"), "Not public").unwrap();
    dir
}

#[test]
fn root_files_are_copied_exactly_only_when_opted_in() {
    for enabled in [false, true] {
        let dir = project(if enabled {
            "root_files: [robots.txt, sitemap.xml]"
        } else {
            ""
        });
        genereto::run(dir.path().into(), genereto::DraftsOptions::Hide).unwrap();
        for name in ["robots.txt", "sitemap.xml"] {
            let output = dir.path().join("output").join(name);
            assert_eq!(output.exists(), enabled);
            if enabled {
                assert_eq!(
                    fs::read(output).unwrap(),
                    fs::read(dir.path().join("content").join(name)).unwrap()
                );
            }
        }
        assert!(!dir.path().join("output/unlisted.txt").exists());
    }
}

#[test]
fn invalid_or_colliding_root_files_fail_before_output_cleanup() {
    for name in [
        "../outside.txt",
        "/absolute.txt",
        "blog/post.md",
        "page.md",
        "missing.txt",
        "blog",
        "rss.xml",
        "page.html",
        "robots.txt, robots.txt",
    ] {
        let dir = project(&format!("root_files: [{name}]"));
        let p = dir.path();
        fs::write(p.join("content/page.md"), "---\ntitle: Page\n---\nBody").unwrap();
        fs::write(p.join("content/rss.xml"), "collision").unwrap();
        fs::write(p.join("content/page.html"), "collision").unwrap();
        fs::create_dir(p.join("output")).unwrap();
        fs::write(p.join("output/sentinel"), "Keep").unwrap();
        assert!(
            genereto::run(p.into(), genereto::DraftsOptions::Hide).is_err(),
            "Accepted {name}"
        );
        assert_eq!(
            fs::read_to_string(p.join("output/sentinel")).unwrap(),
            "Keep"
        );
    }
}
