use std::fs;
use std::path::Path;
use tempfile::{tempdir_in, TempDir};

fn project(jinja: bool) -> TempDir {
    let dir = tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let p = dir.path();
    fs::create_dir_all(p.join("content/blog")).unwrap();
    fs::create_dir_all(p.join("templates/main")).unwrap();
    fs::write(
        p.join("config.yml"),
        format!("template: main\nenable_jinja: {jinja}\ntitle: Test\nurl: https://example.com/site/\ndescription: Test\nblog:\n  base_template: index.html\n  destination: blog\n  generate_single_pages: true\n"),
    ).unwrap();
    let template = if jinja {
        "{{ page.title|e }}|{{ page.permalink }}|{{ page.file_name }}|{{ page.page_name }}|{{ page.last_modified_date }}|{{ page.cover_image }}|{{ content }}{% for comment in page.comments %}{{ comment.author|e }}:{{ comment.replies[0].text|e }}{% endfor %}{% if page.tags %}|{{ page.tags[0] }}|{{ page.categories.group[0] }}|{{ page.rating }}{% endif %}"
    } else {
        "$GENERETO['permalink']|<!-- start_content --><!-- end_content -->"
    };
    fs::write(p.join("templates/main/blog.html"), template).unwrap();
    fs::write(
        p.join("templates/main/index.html"),
        if jinja {
            "{% for article in articles %}{{ article.permalink }}{% endfor %}"
        } else {
            "<!-- start_content -->$GENERETO['permalink']<!-- end_content -->"
        },
    )
    .unwrap();
    dir
}

fn article(p: &Path, name: &str, extra: &str, body: &str) {
    let path = p.join("content/blog").join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, format!("---\ntitle: Imported <title>\npublish_date: 2020-01-02\ndescription: Imported\n{extra}\n---\n{body}")).unwrap();
}

#[test]
fn html_bundle_preserves_body_metadata_assets_and_canonical_route() {
    let dir = project(true);
    let p = dir.path();
    let body =
        "<!-- preserved -->\n<h2 id=\"original\">Heading</h2>\n# not markdown\n<p>Body</p>\n";
    article(p, "stable-42/index.md", "permalink: /old/nested/\ncontent_format: html\nlast_modified_date: 2019-12-31\ncover_image: /wp-content/uploads/cover.jpg\ncomments:\n  - author: '<Alice>'\n    replies:\n      - text: '<reply>'\ntags: [rust]\ncategories: {group: [legacy]}\nrating: 3", body);
    fs::write(p.join("content/blog/stable-42/photo.jpg"), "photo").unwrap();
    article(p, "legacy.md", "", "## Legacy");
    fs::create_dir_all(p.join("content/blog/legacy-assets")).unwrap();
    fs::write(p.join("content/blog/legacy-assets/file.txt"), "asset").unwrap();
    genereto::run(p.into(), genereto::DraftsOptions::Build).unwrap();
    let html = fs::read_to_string(p.join("output/old/nested/index.html")).unwrap();
    assert!(html.contains(body), "{html}");
    assert!(html.contains("Imported &lt;title&gt;|/old/nested/|index.html|index|2019-12-31|/wp-content/uploads/cover.jpg"), "{html}");
    assert!(
        html.contains("&lt;Alice&gt;:&lt;reply&gt;|rust|legacy|3"),
        "{html}"
    );
    assert_eq!(
        fs::read_to_string(p.join("output/old/nested/photo.jpg")).unwrap(),
        "photo"
    );
    assert!(!p.join("output/old/nested/index.md").exists());
    assert!(!p.join("output/blog/stable-42").exists());
    assert!(p.join("output/blog/legacy.html").is_file());
    assert!(p.join("output/blog/legacy-assets/file.txt").is_file());
    assert!(fs::read_to_string(p.join("output/blog/index.html"))
        .unwrap()
        .contains("/old/nested/"));
    let rss = fs::read_to_string(p.join("output/rss.xml")).unwrap();
    assert!(
        rss.contains("https://example.com/site/old/nested/"),
        "{rss}"
    );
}

#[test]
fn html_add_title_renders_an_escaped_heading() {
    let dir = project(false);
    let p = dir.path();
    article(
        p,
        "heading/index.md",
        "content_format: html\nadd_title: true",
        "<p>Body</p>",
    );
    genereto::run(p.into(), genereto::DraftsOptions::Build).unwrap();
    let html = fs::read_to_string(p.join("output/blog/heading/index.html")).unwrap();
    assert!(html.contains("<h1>Imported &lt;title&gt;</h1>"), "{html}");
    assert!(html.contains("<p>Body</p>"), "{html}");
}

#[test]
fn bundle_defaults_and_html_file_permalink() {
    let dir = project(false);
    let p = dir.path();
    article(p, "default/index.md", "", "## Markdown");
    article(
        p,
        "file/index.md",
        "permalink: /archive/article.html",
        "Body",
    );
    genereto::run(p.into(), genereto::DraftsOptions::Build).unwrap();
    assert!(p.join("output/blog/default/index.html").is_file());
    assert!(fs::read_to_string(p.join("output/archive/article.html"))
        .unwrap()
        .contains("/archive/article.html"));
}

#[test]
fn bundle_covers_resolve_beside_canonical_pages_without_changing_flat_covers() {
    for jinja in [false, true] {
        let dir = project(jinja);
        let p = dir.path();
        fs::write(
            p.join("templates/main/blog.html"),
            if jinja {
                "<img src=\"{{ page.cover_image }}\">{{ content }}"
            } else {
                "<img src=\"$GENERETO['cover_image']\"><!-- start_content --><!-- end_content -->"
            },
        )
        .unwrap();
        fs::write(
            p.join("templates/main/index.html"),
            if jinja {
                "{% for article in articles %}<img src=\"{{ article.cover_image }}\">{% endfor %}"
            } else {
                "<!-- start_content --><img src=\"$GENERETO['cover_image']\"><!-- end_content -->"
            },
        )
        .unwrap();
        let cases = [
            (
                "id",
                Some("/hello/"),
                "cover.jpg",
                "/hello/cover.jpg",
                "hello/index.html",
            ),
            (
                "nested",
                Some("/old/hello/"),
                "images/cover.jpg",
                "/old/hello/images/cover.jpg",
                "old/hello/index.html",
            ),
            (
                "file",
                Some("/archive/post.html"),
                "cover.jpg",
                "/archive/cover.jpg",
                "archive/post.html",
            ),
            (
                "unicode",
                Some("/caff%c3%a8/"),
                "cover.jpg",
                "/caff%c3%a8/cover.jpg",
                "caffè/index.html",
            ),
            (
                "default",
                None,
                "cover.jpg",
                "/blog/default/cover.jpg",
                "blog/default/index.html",
            ),
            (
                "rooted",
                Some("/rooted/"),
                "/wp-content/uploads/cover.jpg",
                "/wp-content/uploads/cover.jpg",
                "rooted/index.html",
            ),
            (
                "external",
                Some("/external/"),
                "https://example.com/cover.jpg",
                "https://example.com/cover.jpg",
                "external/index.html",
            ),
        ];
        for (id, permalink, cover, _, _) in cases {
            let extra = format!(
                "{}cover_image: {cover}\nlast_modified_date: 2020-01-02",
                permalink
                    .map(|value| format!("permalink: {value}\n"))
                    .unwrap_or_default()
            );
            article(p, &format!("{id}/index.md"), &extra, "Body");
            if !cover.starts_with('/') && !cover.starts_with("http") {
                let asset = p.join("content/blog").join(id).join(cover);
                fs::create_dir_all(asset.parent().unwrap()).unwrap();
                fs::write(asset, "cover").unwrap();
            }
        }
        article(
            p,
            "legacy.md",
            "cover_image: cover.jpg\nlast_modified_date: 2020-01-02",
            "Legacy",
        );
        genereto::run(p.into(), genereto::DraftsOptions::Build).unwrap();
        let index = fs::read_to_string(p.join("output/blog/index.html")).unwrap();
        for (_, _, cover, resolved, output) in cases {
            let expected = format!("<img src=\"{resolved}\">");
            let destination = p.join("output").join(output);
            let html = fs::read_to_string(&destination).unwrap();
            assert!(html.contains(&expected), "{output}: {html}");
            assert!(index.contains(&expected), "{index}");
            if !cover.starts_with('/') && !cover.starts_with("http") {
                assert_eq!(
                    fs::read_to_string(destination.parent().unwrap().join(cover)).unwrap(),
                    "cover"
                );
            }
        }
        let legacy = "<img src=\"legacy/cover.jpg\">";
        assert!(fs::read_to_string(p.join("output/blog/legacy.html"))
            .unwrap()
            .contains(legacy));
        assert!(index.contains(legacy), "{index}");
    }
}

#[test]
fn encoded_unicode_permalinks_preserve_public_urls_and_decode_output_paths() {
    for jinja in [false, true] {
        for (route, output) in [
            ("/caff%c3%a8/", "caffè/index.html"),
            ("/archivio/%E6%97%A5%E6%9C%AC/", "archivio/日本/index.html"),
            ("/archive/caff%C3%A8.html", "archive/caffè.html"),
            ("/name%2Bplus/", "name+plus/index.html"),
        ] {
            let dir = project(jinja);
            let p = dir.path();
            article(
                p,
                "stable-id/index.md",
                &format!("permalink: '{route}'\nlast_modified_date: 2019-12-31"),
                "Body",
            );
            fs::write(p.join("content/blog/stable-id/photo.jpg"), "photo").unwrap();
            genereto::run(p.into(), genereto::DraftsOptions::Build).unwrap();
            let destination = p.join("output").join(output);
            let html = fs::read_to_string(&destination).unwrap();
            assert!(html.contains(route), "{html}");
            assert_eq!(
                fs::read_to_string(destination.parent().unwrap().join("photo.jpg")).unwrap(),
                "photo"
            );
            assert!(fs::read_to_string(p.join("output/blog/index.html"))
                .unwrap()
                .contains(route));
            let rss = fs::read_to_string(p.join("output/rss.xml")).unwrap();
            assert!(
                rss.contains(&format!("https://example.com/site{route}")),
                "{rss}"
            );
        }
    }
}

#[test]
fn encoded_route_aliases_cannot_collide_with_articles_or_static_assets() {
    for alias in [Some("/caffè/"), Some("/caff%C3%A8/"), None] {
        let dir = project(false);
        let p = dir.path();
        article(p, "one/index.md", "permalink: /caff%c3%a8/", "One");
        if let Some(alias) = alias {
            article(p, "two/index.md", &format!("permalink: '{alias}'"), "Two");
        } else {
            fs::create_dir_all(p.join("content/caffè")).unwrap();
            fs::write(p.join("content/caffè/index.html"), "static").unwrap();
        }
        fs::create_dir_all(p.join("output")).unwrap();
        fs::write(p.join("output/sentinel"), "old").unwrap();
        assert!(genereto::run(p.into(), genereto::DraftsOptions::Build).is_err());
        assert_eq!(
            fs::read_to_string(p.join("output/sentinel")).unwrap(),
            "old"
        );
    }
}

#[test]
fn structured_metadata_is_rejected_only_when_used_as_a_marker() {
    let dir = project(false);
    let p = dir.path();
    article(p, "post.md", "comments: [{author: Alice}]", "Body");
    genereto::run(p.into(), genereto::DraftsOptions::Build).unwrap();
    fs::write(
        p.join("templates/main/blog.html"),
        "$GENERETO['comments']<!-- start_content --><!-- end_content -->",
    )
    .unwrap();
    let err = genereto::run(p.into(), genereto::DraftsOptions::Build).unwrap_err();
    assert!(format!("{err:#}").contains("comments"), "{err:#}");
}

#[test]
fn unsafe_routes_and_collisions_preserve_previous_output() {
    for route in [
        "/../escape/",
        "//evil/",
        "/bad\\path/",
        "/bad%2fpath/",
        "/bad%2Fpath/",
        "/%2fescape/",
        "/bad%5cpath/",
        "/%2e/",
        "/%2E%2e/",
        "/.%2e/",
        "/nested/%2e%2e/escape/",
        "/incomplete%/",
        "/incomplete%c/",
        "/invalid%gg/",
        "/invalid%+f/",
        "/invalid%c3/",
        "/invalid%c0%af/",
        "/invalid%ed%a0%80/",
        "/null%00/",
        "/query%3fstring/",
        "/fragment%23part/",
        "/scheme%3apath/",
        "/percent%25/",
        "/bad%252epath/",
        "/%25c3%25a8/",
        "/x?y/",
        "/x#z/",
        "https://evil/",
        "/blog/",
        "/rss.xml",
        "/assets/file.txt",
        "/template/file.txt",
        "/taken.html",
    ] {
        let dir = project(false);
        let p = dir.path();
        article(p, "post/index.md", &format!("permalink: '{route}'"), "Body");
        fs::create_dir_all(p.join("content/assets")).unwrap();
        fs::write(p.join("content/assets/file.txt"), "static").unwrap();
        fs::create_dir_all(p.join("templates/main/template")).unwrap();
        fs::write(p.join("templates/main/template/file.txt"), "template").unwrap();
        fs::write(p.join("content/taken.md"), "title: Taken\n---\nTaken").unwrap();
        fs::create_dir_all(p.join("output")).unwrap();
        fs::write(p.join("output/sentinel"), "old").unwrap();
        assert!(
            genereto::run(p.into(), genereto::DraftsOptions::Build).is_err(),
            "{route}"
        );
        assert_eq!(
            fs::read_to_string(p.join("output/sentinel")).unwrap(),
            "old",
            "{route}"
        );
    }
}

#[test]
fn duplicate_routes_and_bundle_asset_collisions_are_rejected() {
    for asset_collision in [false, true] {
        let dir = project(false);
        let p = dir.path();
        article(p, "one/index.md", "permalink: /same/", "One");
        if asset_collision {
            fs::write(p.join("content/blog/one/index.html"), "asset").unwrap();
        } else {
            article(p, "two/index.md", "permalink: /same/", "Two");
        }
        assert!(genereto::run(p.into(), genereto::DraftsOptions::Build).is_err());
    }
}

#[test]
fn yaml_entries_cannot_hide_a_paginated_index_collision() {
    let dir = project(false);
    let p = dir.path();
    let config = fs::read_to_string(p.join("config.yml")).unwrap();
    fs::write(
        p.join("config.yml"),
        format!("{config}  max_entries_per_page: 1\n"),
    )
    .unwrap();
    fs::write(
        p.join("content/blog.yml"),
        "entries:\n  - title: Linked\n    publish_date: 2020-01-01\n",
    )
    .unwrap();
    article(p, "post.md", "permalink: /blog/index-page-2.html", "Body");
    fs::create_dir_all(p.join("output")).unwrap();
    fs::write(p.join("output/sentinel"), "old").unwrap();
    assert!(genereto::run(p.into(), genereto::DraftsOptions::Build).is_err());
    assert!(p.join("output/sentinel").is_file());
}

#[test]
fn structured_markers_inserted_by_scalar_metadata_are_rejected() {
    let dir = project(false);
    let p = dir.path();
    article(
        p,
        "post.md",
        "comments: []\nreference: \"$GENERETO['comments']\"",
        "Body",
    );
    fs::write(
        p.join("templates/main/blog.html"),
        "$GENERETO['reference']<!-- start_content --><!-- end_content -->",
    )
    .unwrap();
    assert!(genereto::run(p.into(), genereto::DraftsOptions::Build).is_err());
}

#[test]
fn default_marker_index_links_use_the_canonical_route() {
    let dir = project(false);
    let p = dir.path();
    article(p, "original.md", "permalink: /canonical/", "Body");
    fs::write(
        p.join("templates/main/index.html"),
        "<!-- start_content --><!-- end_content -->",
    )
    .unwrap();
    genereto::run(p.into(), genereto::DraftsOptions::Build).unwrap();
    let index = fs::read_to_string(p.join("output/blog/index.html")).unwrap();
    assert!(index.contains("href=\"/canonical/\""), "{index}");
}

#[test]
fn standalone_blog_generation_also_checks_collisions() {
    let dir = project(false);
    let p = dir.path();
    article(p, "one.md", "permalink: /same/", "One");
    article(p, "two.md", "permalink: /same/", "Two");
    let config = genereto::GeneretoConfig::load_from_folder(p).unwrap();
    assert!(genereto::blog::generate_blog(&config, &genereto::DraftsOptions::Build).is_err());
    assert!(!p.join("output/same/index.html").exists());
}

#[test]
fn hidden_drafts_do_not_reserve_output_files_or_pagination_pages() {
    let dir = project(false);
    let p = dir.path();
    let config = fs::read_to_string(p.join("config.yml")).unwrap();
    fs::write(
        p.join("config.yml"),
        format!("{config}  max_entries_per_page: 1\n"),
    )
    .unwrap();
    article(p, "index.md", "is_draft: true", "Hidden");
    article(p, "live.md", "permalink: /blog/index-page-2.html", "Live");
    genereto::run(p.into(), genereto::DraftsOptions::Hide).unwrap();
    assert!(p.join("output/blog/index-page-2.html").is_file());
    assert!(!fs::read_to_string(p.join("output/blog/index.html"))
        .unwrap()
        .contains("[DRAFT]"));
}

#[cfg(unix)]
#[test]
fn symlinks_cannot_escape_inputs_or_redirect_output_cleanup() {
    use std::os::unix::fs::symlink;
    for output_link in [false, true] {
        let dir = project(false);
        let p = dir.path();
        fs::create_dir_all(p.join("outside")).unwrap();
        fs::write(p.join("outside/keep"), "keep").unwrap();
        if output_link {
            symlink(p.join("outside"), p.join("output")).unwrap();
        } else {
            article(p, "post/index.md", "permalink: /post/", "Body");
            symlink(p.join("outside"), p.join("content/blog/post/escape")).unwrap();
        }
        assert!(genereto::run(p.into(), genereto::DraftsOptions::Build).is_err());
        assert_eq!(fs::read_to_string(p.join("outside/keep")).unwrap(), "keep");
    }
}
