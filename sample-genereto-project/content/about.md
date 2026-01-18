---
title: About Us
description: Learn more about this sample Genereto website
keywords: about, genereto, static site
template_file: landing.html
---

## Our Story

This page demonstrates the **per-page template selection** feature in Genereto. Notice how this page uses a different layout than the home page - it has a hero section with a gradient background!

## How It Works

To use a custom template for any page, simply add `template_file` to your frontmatter:

```yaml
---
title: About Us
description: Learn more about us
template_file: landing.html
---
```

The `template_file` path is relative to your template directory. You can also use nested paths like `layouts/gallery.html`.

## Features

- Different visual style from the default template
- Hero section with gradient background
- Same header and footer (via includes)
- Full SEO metadata support

[Back to Home](index.html) | [Visit the Blog](blog/index.html)
