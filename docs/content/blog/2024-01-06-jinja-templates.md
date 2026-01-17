---
title: Jinja2 Template Support
publish_date: 2024-01-06
description: Learn how to use Jinja2 templates in Genereto for more powerful and flexible templating
keywords: jinja2, templates, minijinja, templating
show_table_of_contents: true
---

## Introduction

Genereto supports optional Jinja2 template processing using the `minijinja` crate. When enabled, you can use Jinja2 syntax (`{{ variable }}`) instead of the traditional `$GENERETO['field']` syntax.

## Enabling Jinja2 Templates

To enable Jinja2 template processing, add `enable_jinja: true` to your `config.yml`:

```yaml
template: main
enable_jinja: true
title: My Website
url: https://example.com
description: My awesome website
```

## Available Variables

### Site Variables

Site-level variables are available in all templates:

| Variable | Description |
|----------|-------------|
| `{{ site.title }}` | Website title from config.yml |
| `{{ site.url }}` | Website URL from config.yml |
| `{{ site.description }}` | Website description from config.yml |
| `{{ site.current_year }}` | Current year (e.g., 2024) |

### Page Variables

Page-level variables are available in individual page templates:

| Variable | Description |
|----------|-------------|
| `{{ page.title }}` | Page title from frontmatter |
| `{{ page.publish_date }}` | Publish date (YYYY-MM-DD) |
| `{{ page.description }}` | Page description |
| `{{ page.keywords }}` | Page keywords |
| `{{ page.file_name }}` | Output filename (e.g., `my-post.html`) |
| `{{ page.cover_image }}` | Cover image path |
| `{{ page.table_of_contents }}` | Generated table of contents HTML |
| `{{ page.read_time_minutes }}` | Estimated reading time |
| `{{ page.last_modified_date }}` | Last modification date |
| `{{ page.url }}` | Article URL (for external links) |

Custom metadata fields from your frontmatter are also available as `{{ page.field_name }}`.

### Content Variable

The rendered markdown content is available as `{{ content }}`.

## Blog Index Template

For blog index pages, use the `articles` loop to iterate over all articles:

```html
<!DOCTYPE html>
<html>
<head><title>{{ site.title }}</title></head>
<body>
  <h1>{{ site.title }}</h1>

  {% for article in articles %}
  <article>
    <h2><a href="{{ article.file_name }}">{{ article.title }}</a></h2>
    <p>{{ article.publish_date }} | {{ article.read_time_minutes }} min read</p>
    <p>{{ article.description }}</p>
  </article>
  {% endfor %}

  {% if articles|length == 0 %}
  <p>No articles yet.</p>
  {% endif %}
</body>
</html>
```

## Single Page Template

For individual blog posts or pages:

```html
<!DOCTYPE html>
<html>
<head>
  <title>{{ page.title }} - {{ site.title }}</title>
  <meta name="description" content="{{ page.description }}">
</head>
<body>
  <h1>{{ page.title }}</h1>
  <p>Published: {{ page.publish_date }}</p>

  {% if page.cover_image %}
  <img src="{{ page.cover_image }}" alt="{{ page.title }}">
  {% endif %}

  {% if page.table_of_contents %}
  <nav class="toc">{{ page.table_of_contents }}</nav>
  {% endif %}

  <article>{{ content }}</article>

  <footer>&copy; {{ site.current_year }} {{ site.title }}</footer>
</body>
</html>
```

## Conditionals

Use Jinja2 conditionals to show or hide content:

```html
{% if page.cover_image %}
<img src="{{ page.cover_image }}" alt="{{ page.title }}">
{% endif %}

{% if page.keywords %}
<meta name="keywords" content="{{ page.keywords }}">
{% endif %}
```

## Filters

Jinja2 filters are supported for transforming data:

```html
{{ articles|length }} articles published
{{ page.title|upper }}
{{ page.description|truncate(100) }}
```

## Custom Metadata

Add any custom field to your frontmatter and access it in templates:

```yaml
---
title: My Post
publish_date: 2024-01-15
author: John Doe
category: Technology
---
```

```html
<p>Author: {{ page.author }}</p>
<p>Category: {{ page.category }}</p>
```

## Comparison with Traditional Syntax

| Feature | Traditional Syntax | Jinja2 Syntax |
|---------|-------------------|---------------|
| Variable | `$GENERETO['title']` | `{{ page.title }}` |
| Site variable | `$GENERETO['current_year']` | `{{ site.current_year }}` |
| Content | Markers: `<!-- start_content -->...<!-- end_content -->` | `{{ content }}` |
| Loops | Not supported | `{% for article in articles %}` |
| Conditionals | Not supported | `{% if condition %}` |
| Filters | Not supported | `{{ value\|filter }}` |

## Migration

To migrate from traditional templates to Jinja2:

1. Add `enable_jinja: true` to your `config.yml`
2. Replace `$GENERETO['field']` with `{{ page.field }}` or `{{ site.field }}`
3. Remove `<!-- start_content -->` and `<!-- end_content -->` markers
4. Add `{{ content }}` where you want the content to appear
5. Optionally use loops and conditionals for more dynamic templates

## Example Project

See the `sample-genereto-project-jinja2` directory in the Genereto repository for a complete working example.
