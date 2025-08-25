---
title: "Welcome to Genereto with Jinja2!"
description: "Experience the power of Jinja2 templating with this advanced static site generator"
keywords: "genereto, jinja2, templates, rust, static site generator"
author: "Genereto Team"
---

# Welcome to Genereto with Jinja2! 🚀

This demonstrates a **Genereto** static site with **Jinja2 templating enabled**. 

## Key Features

- **Fast Generation**: Built in Rust for maximum performance
- **Jinja2 Templates**: Advanced templating with conditionals and loops  
- **Blog Support**: Built-in blog functionality with RSS feeds
- **Markdown Support**: Write content in Markdown with rich frontmatter

## Jinja2 Template Features

With Jinja2 enabled, you can use:

### Variables
```html
{{page.title}} - {{site.title}}
```

### Conditionals  
```html
{% if page.author %}
<p>By {{page.author}}</p>
{% endif %}
```

### Loops
```html
{% for tag in page.keywords.split(', ') %}
<span>{{tag}}</span>
{% endfor %}
```

## Quick Start

1. **Edit Content**: Modify files in `content/` directory
2. **Customize Templates**: Edit `templates/main/*.html.jinja` files
3. **Configure**: Update `config.yml` settings
4. **Generate**: Run `genereto --project-path .`

Check out the [blog](blog.html) for more examples!