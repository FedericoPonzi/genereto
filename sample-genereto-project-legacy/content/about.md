---
title: "About This Site"
description: "Learn about Genereto and how this site demonstrates its features"
keywords: "about, genereto, static site generator, features"
---

# About This Site

This site is built with **Genereto**, a modern static site generator that combines the speed of Rust with the power of Jinja2 templating.

## What You're Looking At

This starter template demonstrates:

### 🎨 **Modern Design System**
- Clean, responsive layout
- CSS custom properties for theming
- Automatic dark mode support
- Professional typography

### ⚡ **Performance First**
- Generated in milliseconds with Rust
- Optimized CSS and minimal JavaScript
- Static files for maximum speed
- SEO-friendly markup

### 🔧 **Developer Experience**
- Simple Markdown authoring
- Rich YAML frontmatter support
- Live development with draft preview
- Git-friendly workflow

## Jinja2 Template Features

When you enable Jinja2 templates (`enable_jinja: true`), you unlock powerful features:

### Smart Conditionals
```html
{% if page.author %}
<meta name="author" content="{{page.author}}">
{% endif %}
```

### Dynamic Navigation
```html
<a href="about.html" 
   {% if page.title == "About This Site" %}class="active"{% endif %}>
   About
</a>
```

### Custom Metadata Display
```html
{% if page.custom_metadata %}
{% for key, value in page.custom_metadata %}
<div class="meta-item">
    <strong>{{key | title}}:</strong> {{value}}
</div>
{% endfor %}
{% endif %}
```

### Rich Social Meta Tags
```html
<meta property="og:title" content="{{page.title if page.title else site.title}}">
<meta property="og:description" content="{{page.description if page.description else site.description}}">
{% if page.cover_image %}
<meta property="og:image" content="{{site.url}}/{{page.cover_image}}">
{% endif %}
```

## Try It Yourself

1. **Enable Jinja2**: Uncomment `enable_jinja: true` in `config.yml`
2. **Regenerate**: Run `genereto --project-path .`
3. **See the Magic**: Notice the enhanced navigation, metadata display, and conditional content

<div class="alert alert-info">
<strong>💡 Tip:</strong> Compare the generated HTML before and after enabling Jinja2 to see the difference!
</div>

## What's Next?

- **Customize the Design**: Edit `templates/main/res/styles.css`
- **Add Your Content**: Replace these sample pages with your own
- **Explore Templates**: Check out the Jinja2 templates in `templates/main/`
- **Set Up Blog**: Uncomment the blog configuration in `config.yml`

---

Ready to build something amazing? Start editing and make this site your own! 🚀