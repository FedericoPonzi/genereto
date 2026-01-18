---
title: Welcome to My Website
description: A sample website built with Genereto using Jinja2 templates
keywords: genereto, static site, blog, jinja2
cover_image: res/cover.jpg
---
# Welcome!

This is a sample website built with [Genereto](https://github.com/FedericoPonzi/genereto) using **Jinja2 templates**.

## Jinja2 Features

With `enable_jinja: true`, you can use:

- **Variables**: `{{ page.title }}`, `{{ site.url }}`
- **Conditionals**: `{% if page.cover_image %}...{% endif %}`
- **Loops**: `{% for article in articles %}...{% endfor %}`
- **Filters**: `{{ articles|length }}`
- **Auto year**: `{{ site.current_year }}`

## Getting Started

1. Edit `content/index.md` for your home page
2. Add posts in `content/blog/`
3. Customize templates in `templates/main/`
4. Build with `genereto --project-path .`

Check out the [blog](blog/index.html) for more examples!
