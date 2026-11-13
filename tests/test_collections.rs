use std::fs;
use tempfile::tempdir;

fn project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    let p = dir.path();
    fs::create_dir_all(p.join("content/blog")).unwrap();
    fs::create_dir_all(p.join("templates/main")).unwrap();
    fs::write(p.join("config.yml"), "template: main\nenable_jinja: true\ntitle: Test\nurl: https://example.test\nblog:\n  base_template: list.html\n  destination: ''\n  max_entries_per_page: 2\n  pagination_path: /page/{page}/\n  archives_file: archives.yml\n  sort_by: timestamp\n").unwrap();
    fs::write(p.join("templates/main/blog.html"), "{{ content }}").unwrap();
    fs::write(p.join("templates/main/index.html"), "{{ content }}").unwrap();
    fs::write(p.join("templates/main/list.html"), "{{ listing.title }}|{{ listing.permalink }}|{% for post in articles %}{{ post.title }};{% endfor %}{% if pagination %}|{{ pagination.prev_url }}|{{ pagination.next_url }}{% endif %}").unwrap();
    for (id, timestamp) in [
        (1, "2020-01-01 09:00:00"),
        (2, "2020-01-01 12:00:00"),
        (3, "2020-01-02 12:00:00"),
    ] {
        fs::write(p.join(format!("content/blog/{id}.md")), format!("---\ntitle: Post {id}\npublish_date: 2020-01-01\nlast_modified_date: 2020-01-01\npermalink: /post-{id}/\ntimestamp: '{timestamp}'\n---\nBody")).unwrap();
    }
    fs::write(p.join("archives.yml"), "- permalink: /cat/tech/\n  title: Tech\n  articles: [/post-1/, /post-3/, /post-2/]\n- permalink: /tag/single/\n  title: Single\n  articles: [/post-2/]\n").unwrap();
    dir
}

#[test]
fn collections_share_sorting_pagination_and_canonical_routes() {
    let dir = project();
    let p = dir.path();
    genereto::run(p.into(), genereto::DraftsOptions::Hide).unwrap();
    let home = fs::read_to_string(p.join("output/index.html")).unwrap();
    assert!(home.contains("Test|/|Post 3;Post 2;"), "{home}");
    assert!(home.ends_with("||/page/2/"), "{home}");
    let next = fs::read_to_string(p.join("output/page/2/index.html")).unwrap();
    assert!(next.contains("Post 1;|/|"), "{next}");
    let archive = fs::read_to_string(p.join("output/cat/tech/index.html")).unwrap();
    assert!(
        archive.contains("Tech|/cat/tech/|Post 3;Post 2;"),
        "{archive}"
    );
    assert!(archive.ends_with("||/cat/tech/page/2/"), "{archive}");
    assert!(p.join("output/cat/tech/page/2/index.html").is_file());
    assert!(fs::read_to_string(p.join("output/tag/single/index.html"))
        .unwrap()
        .contains("Post 2;"));
    assert!(!p.join("output/index-page-2.html").exists());
}

#[test]
fn bad_collection_members_and_routes_fail_before_output_cleanup() {
    for invalid in [
        "- permalink: /cat/x/\n  title: X\n  articles: [/missing/]\n",
        "- permalink: /post-1/\n  title: X\n  articles: [/post-1/]\n",
        "- permalink: /../escape/\n  title: X\n  articles: []\n",
        "- permalink: /cat/x/\n  title: X\n  articles: [/post-1/, /post-1/]\n",
    ] {
        let dir = project();
        let p = dir.path();
        fs::create_dir(p.join("output")).unwrap();
        fs::write(p.join("output/sentinel"), "old").unwrap();
        fs::write(p.join("archives.yml"), invalid).unwrap();
        assert!(genereto::run(p.into(), genereto::DraftsOptions::Hide).is_err());
        assert_eq!(
            fs::read_to_string(p.join("output/sentinel")).unwrap(),
            "old"
        );
    }
}

#[test]
fn page_bundles_honor_permalinks_and_do_not_publish_markdown() {
    let dir = project();
    let p = dir.path();
    fs::create_dir(p.join("content/page-849")).unwrap();
    fs::write(p.join("content/page-849/index.md"), "---\ntitle: Contact\npublish_date: 2020-01-01\nlast_modified_date: 2020-01-01\npermalink: /info-contatti/\ncontent_format: html\n---\n<p>Contact body</p>").unwrap();
    fs::write(p.join("content/page-849/photo.txt"), "asset").unwrap();
    genereto::run(p.into(), genereto::DraftsOptions::Hide).unwrap();
    assert!(
        fs::read_to_string(p.join("output/info-contatti/index.html"))
            .unwrap()
            .contains("<p>Contact body</p>")
    );
    assert!(p.join("output/info-contatti/photo.txt").is_file());
    assert!(!p.join("output/page-849/index.md").exists());
    fs::create_dir(p.join("content/second")).unwrap();
    fs::write(p.join("content/second/index.md"), "---\ntitle: Second\npublish_date: 2020-01-01\nlast_modified_date: 2020-01-01\ncover_image: photo.txt\n---\nBody").unwrap();
    fs::write(
        p.join("templates/main/index.html"),
        "{{ page.permalink }}|{{ page.cover_image }}|{{ content }}",
    )
    .unwrap();
    genereto::run(p.into(), genereto::DraftsOptions::Hide).unwrap();
    assert!(fs::read_to_string(p.join("output/second/index.html"))
        .unwrap()
        .starts_with("/second/|/second/photo.txt|"));
}

#[test]
fn hidden_articles_invalid_sort_keys_and_invalid_patterns_fail_before_cleanup() {
    for (from, to) in [
        ("timestamp: '2020-01-01 09:00:00'", "timestamp: [invalid]"),
        ("title: Post 1", "title: Post 1\nis_draft: true"),
    ] {
        let dir = project();
        let p = dir.path();
        let source = p.join("content/blog/1.md");
        fs::write(
            &source,
            fs::read_to_string(&source).unwrap().replace(from, to),
        )
        .unwrap();
        fs::create_dir(p.join("output")).unwrap();
        fs::write(p.join("output/sentinel"), "old").unwrap();
        assert!(genereto::run(p.into(), genereto::DraftsOptions::Hide).is_err());
        assert!(p.join("output/sentinel").exists());
    }
    for pattern in ["/page/fixed/", "/../{page}/", "/{page}/{page}/"] {
        let dir = project();
        let p = dir.path();
        let cfg = p.join("config.yml");
        fs::write(
            &cfg,
            fs::read_to_string(&cfg)
                .unwrap()
                .replace("/page/{page}/", pattern),
        )
        .unwrap();
        assert!(genereto::run(p.into(), genereto::DraftsOptions::Hide).is_err());
        assert!(!p.join("output").exists());
    }
}

#[test]
fn empty_indexes_and_sorting_ties_are_deterministic() {
    let dir = project();
    let p = dir.path();
    let one = p.join("content/blog/1.md");
    fs::write(
        &one,
        fs::read_to_string(&one)
            .unwrap()
            .replace("09:00:00", "12:00:00"),
    )
    .unwrap();
    genereto::run(p.into(), genereto::DraftsOptions::Hide).unwrap();
    let home = fs::read_to_string(p.join("output/index.html")).unwrap();
    assert!(home.contains("Post 3;Post 1;"), "{home}");
    for number in 1..=3 {
        fs::remove_file(p.join(format!("content/blog/{number}.md"))).unwrap();
    }

    fs::write(p.join("archives.yml"), "[]").unwrap();
    genereto::run(p.into(), genereto::DraftsOptions::Hide).unwrap();
    assert!(p.join("output/index.html").is_file());
    assert!(!p.join("output/page/2/index.html").exists());
}

#[test]
fn custom_index_names_are_not_mistaken_for_directory_indexes() {
    let dir = project();
    let p = dir.path();
    let cfg = p.join("config.yml");
    fs::write(
        &cfg,
        fs::read_to_string(&cfg).unwrap().replace(
            "  base_template:",
            "  index_name: myindex.html\n  base_template:",
        ),
    )
    .unwrap();
    genereto::run(p.into(), genereto::DraftsOptions::Hide).unwrap();
    assert!(fs::read_to_string(p.join("output/myindex.html"))
        .unwrap()
        .contains("Test|/myindex.html|"));
}
