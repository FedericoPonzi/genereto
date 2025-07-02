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

Add any custom fields to your content metadata and access them in templates using `$GENERETO['field_name']`.