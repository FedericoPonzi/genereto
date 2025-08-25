---
title: "Welcome to Genereto (Legacy Templates)"
description: "A modern static site generator using traditional template syntax"
keywords: "genereto, static site generator, legacy, simple templates"
author: "Genereto Team"
---

# Welcome to Genereto with Legacy Templates! 📄

Congratulations! You've successfully set up a **Genereto** static site using **traditional legacy templates**. This demonstrates the simple `$GENERETO['field']` syntax for straightforward static site generation.

## What's Included

This starter template includes:

<div class="feature-grid">
<div class="feature-card">
<h3>🎨 Modern CSS</h3>
<p>Beautiful, responsive design with dark mode support and CSS variables for easy customization.</p>
</div>

<div class="feature-card">
<h3>📝 Markdown Support</h3>
<p>Write your content in Markdown with YAML frontmatter for metadata like title, description, and custom fields.</p>
</div>

<div class="feature-card">
<h3>🔧 Jinja2 Templates</h3>
<p>Advanced templating with loops, conditionals, inheritance, and more. Enable with <code>enable_jinja: true</code>.</p>
</div>

<div class="feature-card">
<h3>📱 Responsive Design</h3>
<p>Mobile-first design that looks great on all devices with automatic dark mode support.</p>
</div>

<div class="feature-card">
<h3>🚀 Fast Generation</h3>
<p>Lightning-fast site generation built in Rust with minimal dependencies.</p>
</div>

<div class="feature-card">
<h3>📊 Blog Support</h3>
<p>Built-in blog functionality with RSS feeds, drafts, and automatic metadata extraction.</p>
</div>
</div>

## Quick Start

### 1. Build Your Site

```bash
# Generate the site
genereto --project-path .

# Or for development with drafts visible
genereto --project-path . --drafts-options dev
```

### 2. Customize Your Content

Edit the Markdown files in the `content/` directory:

- `content/index.md` - This homepage
- `content/jinja-templates.md` - Jinja2 documentation
- `content/blog/` - Blog posts

### 3. Modify Templates

Templates are in `templates/main/`:

- `index.html` - Regular template
- `index.html.jinja` - Jinja2 template (when enabled)
- `res/styles.css` - CSS styles

### 4. Configure Your Site

Edit `config.yml` to customize:

```yaml
title: 'Your Site Name'
description: 'Your site description'
url: 'https://yoursite.com'
enable_jinja: true  # Enable advanced templating
```

## Next Steps

<div class="alert alert-info">
<strong>🎯 Ready to dive deeper?</strong> Check out the <a href="jinja-templates.html">Jinja2 Templates Guide</a> to learn about advanced templating features.
</div>

### Learn Advanced Features

1. **[Jinja2 Templates](jinja-templates.html)** - Learn powerful templating
2. **Blog Setup** - Explore the `content/blog/` directory
3. **Styling** - Customize `templates/main/res/styles.css`
4. **Configuration** - Review `config.yml` options

### Example Custom Metadata

This page demonstrates custom metadata in the frontmatter:

```yaml
---
title: "Welcome to Your Genereto Site"
description: "A modern static site generator"
keywords: "genereto, static site generator"
author: "Genereto Team"
---
```

You can access these in Jinja2 templates as:
- `{{page.author}}` → "{{author}}"
- `{{page.keywords}}` → "{{keywords}}"

## Get Help

- **Documentation**: Check the [Jinja2 guide](jinja-templates.html)
- **GitHub**: [FedericoPonzi/genereto](https://github.com/FedericoPonzi/genereto)
- **Examples**: This starter template is your reference

---

Happy building! 🚀