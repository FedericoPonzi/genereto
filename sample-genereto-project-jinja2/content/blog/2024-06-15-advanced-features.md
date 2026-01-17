---
title: Advanced Jinja2 Features
publish_date: 2024-06-15
description: Explore advanced Jinja2 features for more powerful templates
keywords: jinja2, templates, advanced
---

## Custom Metadata

With Jinja2 templates, you can use custom metadata fields directly in your templates. Just add any field to your frontmatter and access it via `{{ page.field_name }}`.

## Conditional Rendering

Use conditionals to show or hide content:

```html
{% if page.cover_image %}
<img src="{{ page.cover_image }}" alt="{{ page.title }}">
{% endif %}
```

## Template Filters

Jinja2 supports filters for transforming data:

```html
{{ articles|length }} articles published
{{ page.title|upper }}
```

## Getting Started

To enable Jinja2 templates in your Genereto project, simply add `enable_jinja: true` to your `config.yml` file. That's it!
