# Genereto

A simple static site generator to handle different kinds of simple static websites.

[<img src="https://github.com/FedericoPonzi/genereto/raw/main/assets/genereto-logo.jpg" width="300" align="center">](https://github.com/FedericoPonzi/genereto/raw/main/assets/genereto-logo.jpg)

## Table of Contents
- [Features](#features)
- [Quick Start Tutorial](#quick-start-tutorial)
- [CLI Reference](#cli-reference)
- [Config Reference](#config-reference)
- [Metadata Fields Reference](#metadata-fields-reference)
- [Templating Guide](#templating-guide)
- [Advanced Features](#advanced-features)

## Features

With Genereto, you can:
* Generate a simple static website with a single page or multiple pages
* Generate a blog with articles
* Generate a tumblr style website
* Generate a simple static website along with a blog

You should use genereto if you want:
* A complete and easy way to create a static website
* Metadata stored along with the website content written in Markdown
* A super simple templating system
* Fast compilation
* And more:
    * TODOs and comments embeddable in your pages
    * Drafts support
    * Tumblr style website
    * RSS feed generation

## Quick Start Tutorial

1. Create a new project:
```bash
cargo run -- generate-project --project-path ./my-site
```

2. Add content in `my-site/content/`:
```markdown
---
title: My First Page
description: Welcome to my site
---
# Welcome!
This is my first page.
```

3. Build your site:
```bash
cargo run -- --project-path ./my-site
```

> 💡 **Tip**: Use the sample project as reference: check out `sample-genereto-project` in this repository.

## CLI Reference

```bash
# Basic usage
genereto --project-path <PATH>

# Generate new project
genereto generate-project --project-path <PATH> [--override-git]

# Draft options
genereto --project-path <PATH> --drafts-options <OPTION>
```

Draft options:
- `build` (default): Builds draft pages but doesn't link them
- `dev`: Treats drafts as normal pages
- `hide`: Completely skips draft pages

## Config Reference

The `config.yml` file in your project root defines the site configuration:

```yaml
# Required fields
template: string           # Template directory name to use
title: string             # Website title (used in RSS)
url: string              # Website URL (used in RSS)
description: string      # Website description (used in RSS)
default_cover_image: string # Default image for pages without cover

# Blog configuration (optional)
blog:
  base_template: index.html  # Template for blog pages
  index_name: index.html    # Name of the blog index file
  destination: ""           # Blog output subdirectory
  generate_single_pages: true # Generate individual article pages
  title: string            # Optional blog-specific title
```

### Directory Structure
- `content/`: Markdown files and assets
- `templates/`: HTML templates
- `output/`: Generated site (created automatically)

## Metadata Fields Reference

Available metadata fields for pages and articles:

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `title` | string | Page/article title | Required |
| `publish_date` | string | Publication date (YYYY-mm-dd) | Optional |
| `is_draft` | bool | Draft status | `false` |
| `keywords` | string | Comma-separated keywords | Optional |
| `show_table_of_contents` | bool | Enable ToC generation | `false` |
| `add_title` | bool | Auto-add H1 title from metadata | `false` |
| `description` | string | Brief description (first 150 chars if not provided) | Optional |
| `cover_image` | string | Path to cover image | Optional |
| `url` | string | External URL for the article | Optional |
| `current_year` | string | Current year (auto-generated) | Auto |

> ⚠️ **Notes**: 
> - Articles with TODOs are automatically marked as drafts regardless of `is_draft` setting
> - If no description is provided, the first 150 characters of content will be used
> - Cover images can be relative paths or full URLs

## Templating Guide

Templates require two main files:
- `index.html`: For listing blog articles
- `blog.html`: For individual articles

Content replacement section:
```html
<!-- start_content -->
Content here will be replaced
<!-- end_content -->
```

Variables are accessed using:
```html
$GENERETO['variable_name']
```

> 💡 **Tip**: Use the content between start/end_content markers to preview your template's appearance.

## Advanced Features

### RSS Feed
Genereto automatically generates an RSS feed. Add to your template:
```html
<link rel="alternate" type="application/rss+xml" title="RSS Feed" href="rss.xml" />
```

### TODOs and Comments
Embed TODOs and comments in your content:
```markdown
$GENERETO{TODO: fix this section}
This is my content $GENERETO{add more details here}
```

### Blog YAML Format
For tumblr-style blogs, use `blog.yml`:
```yaml
entries:
  - title: My Post
    publish_date: 2024-01-01
    description: Quick update
```

> 💡 **Tip**: YAML entries support all the same metadata fields as markdown articles.

---

For more details, check out the [introduction article](https://blog.fponzi.me/2023-05-19-one-complex-setup.html).



