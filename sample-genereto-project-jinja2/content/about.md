---
title: "About"
description: "Learn about Genereto and Jinja2 templating"
keywords: "about, genereto, jinja2, static site generator"
---

# About This Site

This site demonstrates **Genereto** with **Jinja2 templating** enabled.

## What is Genereto?

Genereto is a fast static site generator built in **Rust** that supports:

- **Markdown** content with YAML frontmatter
- **Jinja2** templating for advanced features
- **Blog** functionality with RSS feeds
- **Fast** generation and modern CSS

## Jinja2 Features

With Jinja2 enabled, you can use:

- **Variables**: `{{page.title}}`, `{{site.description}}`
- **Conditionals**: `{% if condition %}...{% endif %}`
- **Loops**: `{% for item in list %}...{% endfor %}`
- **Custom metadata**: Access any YAML field

## Getting Started

1. Edit content in `content/` directory
2. Modify templates in `templates/main/`
3. Configure in `config.yml`
4. Run: `genereto --project-path .`

Visit the [blog](blog.html) to see more Jinja2 examples in action!