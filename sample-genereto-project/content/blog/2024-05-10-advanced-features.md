---
title: Advanced Features Guide
description: Learn about custom metadata, drafts, and more
keywords: genereto, advanced, custom metadata, drafts
publish_date: 2024-05-10
cover_image: res/default-cover.jpg
author: Your Name
category: Tutorial
---

This post demonstrates advanced Genereto features.

## Custom Metadata

You can add any custom fields to the frontmatter:

```yaml
---
title: My Post
author: Your Name
category: Tutorial
custom_field: Any value you want
---
```

Access them in templates with `$GENERETO['author']` or `{{ page.author }}` (Jinja2).

## Draft Posts

Mark posts as drafts to hide them from the index:

```yaml
---
title: Work in Progress
is_draft: true
---
```

Build modes:
- `--drafts-options build` - Build drafts but don't link them (default)
- `--drafts-options dev` - Include drafts normally (for development)
- `--drafts-options hide` - Skip drafts entirely

## Future-Dated Posts

Posts with a future `publish_date` are automatically treated as drafts.

## Table of Contents

Headings automatically generate a table of contents, available via `$GENERETO['table_of_contents']`.

## Template Includes

Reuse components across templates:

```html
$GENERETO_INCLUDE['header.html']
$GENERETO_INCLUDE['footer.html']
```

## RSS Feed

An RSS feed (`rss.xml`) is generated automatically in the blog output directory.
