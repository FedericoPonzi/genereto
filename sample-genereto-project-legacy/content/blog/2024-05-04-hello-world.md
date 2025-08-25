---
title: "Hello World from Genereto!"
description: "Getting started with the Genereto static site generator - your first blog post"
keywords: "genereto, blog, first post, getting started"
publish_date: "2024-05-04"
---

# Hello World from Genereto! 🌍

Welcome to your first blog post with **Genereto**! This post demonstrates the key features of the static site generator and shows you what's possible.

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
title: "Hello World from Genereto!"
description: "Getting started with Genereto"
keywords: "genereto, blog, first post"
publish_date: "2024-05-04"
---
```

All of these fields are available in your templates!

## Image Support

You can include images in your posts. Here's the Rust mascot, **Ferris the Crab** 🦀:

![Ferris the Rustacean](./rustacean.png)

Images are automatically copied to the output directory and linked correctly.

## Code Highlighting

Genereto supports syntax highlighting for code blocks:

```rust
fn main() {
    println!("Hello, Genereto!");
    
    let features = vec![
        "Fast generation",
        "Markdown support", 
        "Jinja2 templates",
        "Blog functionality",
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
  base_template: blog-index.html
  destination: blog/
  generate_single_pages: true
```

## Typography and Content

Genereto renders beautiful typography out of the box:

### Headers Look Great

From H1 through H6, all headers are properly styled with consistent spacing and hierarchy.

### Lists Are Clean

**Unordered lists:**
- Fast site generation
- Markdown support with frontmatter
- Jinja2 template engine
- Responsive design
- Dark mode support

**Ordered lists:**
1. Create your content in Markdown
2. Configure your site in `config.yml`
3. Run `genereto --project-path .`
4. Deploy your generated site

### Quotes Stand Out

> "Genereto makes static site generation simple and fast. The combination of Rust performance with Jinja2 templating is incredibly powerful."
> 
> — Happy Genereto User

### Tables Work Too

| Feature | Regular Templates | Jinja2 Templates |
|---------|------------------|------------------|
| Variables | `$GENERETO['field']` | `{{page.field}}` |
| Conditionals | ❌ | ✅ |
| Loops | ❌ | ✅ |
| Inheritance | ❌ | ✅ |
| Macros | ❌ | ✅ |

## Next Steps

Now that you've seen your first blog post, here's what to do next:

1. **Explore Templates** - Check out the Jinja2 templates in `templates/main/`
2. **Customize Styling** - Edit `templates/main/res/styles.css`
3. **Add More Content** - Create more Markdown files in `content/`
4. **Enable Jinja2** - Set `enable_jinja: true` in `config.yml`

<div class="alert alert-success">
<strong>🎉 Congratulations!</strong> You're now ready to build amazing static sites with Genereto!
</div>

## Advanced Features Preview

With Jinja2 templates enabled, you can:

- **Use conditionals**: Show content based on page metadata
- **Loop over data**: Generate navigation, tag lists, related posts
- **Template inheritance**: Create consistent layouts
- **Custom filters**: Process content dynamically
- **Macros**: Reusable template components

Check out the [Jinja2 Templates Guide](../jinja-templates.html) to learn more!

---

Ready to create your next post? Just create a new `.md` file in the `content/blog/` directory and start writing! ✍️