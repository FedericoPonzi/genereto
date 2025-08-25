---
title: "Hello World from Genereto with Jinja2!"
description: "Getting started with advanced Jinja2 templating - your first enhanced blog post"
keywords: "genereto, jinja2, blog, first post, templating, advanced features"
publish_date: "2024-05-04"
reading_time: "3"
author: "Jinja2 Demo Author"
difficulty: "Beginner"
category: "Tutorial" 
featured: "true"
---

# Hello World from Genereto with Jinja2! 🌍✨

Welcome to your first blog post with **Genereto** and **Jinja2 templating**! This post demonstrates the advanced features available when you enable Jinja2, including rich metadata display, conditionals, and dynamic content generation.

## What Makes Genereto Special?

Genereto is designed to be **fast**, **simple**, and **powerful**:

### 🚀 Lightning Fast
Built in Rust for maximum performance. Generate hundreds of pages in milliseconds.

### 📝 Markdown-First
Write your content in Markdown with rich YAML frontmatter support for metadata.

### 🎨 Flexible Templating
Choose between simple `$GENERETO['field']` templates or powerful Jinja2 templates with conditionals, loops, and inheritance.

### 📱 Modern Web Standards
Responsive design, dark mode, accessibility, and modern CSS out of the box.

## Rich Metadata Support

This post demonstrates various metadata fields in the frontmatter:

```yaml
---
title: "Hello World from Genereto with Jinja2!"
description: "Getting started with advanced Jinja2 templating"
keywords: "genereto, jinja2, blog, first post, templating"
publish_date: "2024-05-04"
reading_time: "3"
author: "Jinja2 Demo Author"
difficulty: "Beginner"
category: "Tutorial" 
featured: "true"
---
```

All of these fields are available in your Jinja2 templates!

## Jinja2 Template Features

With Jinja2 enabled, you can use:

### Conditionals
```html
{% if page.author %}
<meta name="author" content="{{page.author}}">
{% endif %}
```

### Loops
```html
{% for keyword in page.keywords.split(', ') %}
<span class="tag">{{keyword.strip()}}</span>
{% endfor %}
```

### Template Inheritance
```html
{% extends "base.html" %}
{% block content %}
<h1>{{page.title}}</h1>
{% endblock %}
```

## Code Highlighting

Genereto supports syntax highlighting for code blocks:

```rust
fn main() {
    println!("Hello, Genereto with Jinja2!");
    
    let features = vec![
        "Fast generation",
        "Markdown support", 
        "Jinja2 templates",
        "Blog functionality",
        "Conditional logic",
        "Dynamic navigation",
    ];
    
    for feature in features {
        println!("✓ {}", feature);
    }
}
```

```yaml
# config.yml
title: 'My Awesome Site'
description: 'Built with Genereto'
url: 'https://mysite.com'
enable_jinja: true

blog:
  base_template: main/blog-index.html
  destination: blog/
  generate_single_pages: true
```

## Advanced Features Preview

With Jinja2 templates enabled, you can:

- **Use conditionals**: Show content based on page metadata
- **Loop over data**: Generate navigation, tag lists, related posts
- **Template inheritance**: Create consistent layouts
- **Custom filters**: Process content dynamically
- **Macros**: Reusable template components

## Next Steps

Now that you've seen your first Jinja2 blog post, here's what to do next:

1. **Explore Templates** - Check out the Jinja2 templates in `templates/main/`
2. **Customize Styling** - Edit `templates/main/res/styles.css`
3. **Add More Content** - Create more Markdown files in `content/`
4. **Try Advanced Features** - Use conditionals, loops, and inheritance

<div class="alert alert-success">
<strong>🎉 Congratulations!</strong> You're now ready to build amazing static sites with Genereto and Jinja2!
</div>

---

Ready to create your next post? Just create a new `.md` file in the `content/blog/` directory and start writing! ✍️