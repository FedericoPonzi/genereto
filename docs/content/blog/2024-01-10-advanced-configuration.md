title: Advanced Configuration
publish_date: 2024-01-10
description: Deep dive into Genereto's configuration options
---

# Advanced Configuration

Genereto offers many configuration options to customize your site generation.

## Template Configuration

You can specify custom template paths and configure blog settings:

```yaml
template: main
template_base_path: ./custom-templates
blog:
  base_template: article.html
  generate_single_pages: true
```

## Custom Metadata

Add any custom fields to your content metadata and access them in templates using `&#36;GENERETO['field_name']`.

## Root Static Files

Non-Markdown files directly under `content/` are ignored unless explicitly
listed in `root_files`:

```yaml
root_files:
  - robots.txt
  - sitemap.xml
```

These files are copied unchanged to the output root. Entries must be existing
regular, non-`.md` files with literal root-level filenames, not paths or encoded
names. Symlinks, duplicate entries and collisions with generated pages, feeds
or other copied assets fail before output cleanup. The default list is empty;
existing asset directories keep their normal recursive-copy behavior.

## Public Routes and Archives

A page or article can set `permalink: /nested/example/` independently of its
source filename. A trailing slash writes `nested/example/index.html`; explicit
`.html` paths also work. A folder containing `index.md` is a page bundle:
its other files are copied beside the generated page. Other folders remain
asset directories. Put article bundles under `content/blog/` and standalone
page bundles directly under `content/`.

Blog archives and directory-style pagination are optional:

```yaml
enable_jinja: true
blog:
  base_template: blog-index.html
  destination: ""
  max_entries_per_page: 10
  pagination_path: /page/{page}/
  archives_file: migration/archives.json
  sort_by: published_at
```

`pagination_path` must contain exactly one `{page}`. Without it, existing
`index-page-N.html` pagination remains unchanged. `sort_by` selects a string
custom-metadata field, required on every blog entry, sorted descending; ties
use permalink and filename. Without it, publication-date sorting is unchanged.
Use consistently formatted timestamps when sorting by date.

The project-relative archive file accepts JSON or YAML:

```json
[
  {
    "permalink": "/category/rust/",
    "title": "Rust articles",
    "description": "Notes about Rust.",
    "articles": ["/nested/example/"]
  }
]
```

Archive paths must end in `/`. Members must be visible articles with an
explicit permalink or a bundle-derived permalink; hidden drafts and duplicate
members are rejected. Archives share the blog template, ordering and page size,
with subsequent pages at `/category/rust/page/N/`.

Jinja listing templates receive `listing.title`, `listing.description`,
`listing.permalink` (the current page), and `listing.is_archive`, alongside
`articles` and `pagination`. Pagination links are root-relative when using
directory-style routes; do not prepend another slash.

Generated routes and copied assets are checked for collisions before existing
output is removed. Archive files cannot use absolute paths, `..` or symlinks.