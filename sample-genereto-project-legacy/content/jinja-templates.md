---
title: "Advanced Templating with Jinja2"
description: "Learn how to use Jinja2 templates in Genereto for powerful, dynamic website generation"
keywords: "jinja2, templates, genereto, static site generator"
---

# Advanced Templating with Jinja2

Genereto supports **Jinja2 templates** for advanced templating capabilities beyond the basic `$GENERETO['field']` syntax. This allows you to use loops, conditionals, template inheritance, and other powerful features.

<div class="alert alert-info">
<strong>💡 Pro Tip:</strong> Jinja2 templates are optional and fully backward compatible. Your existing templates will continue to work unchanged.
</div>

## Quick Start

### 1. Enable Jinja2 Templates

Add this line to your `config.yml`:

```yaml
enable_jinja: true
```

### 2. Create Jinja2 Templates

Create templates with `.html.jinja` extension alongside your regular `.html` templates:

```
templates/
├── main/
│   ├── index.html        # Regular template (fallback)
│   └── index.html.jinja  # Jinja2 template (takes precedence)
```

### 3. Use Jinja2 Syntax

Instead of `$GENERETO['title']`, use `{{page.title}}`:

```html
<!DOCTYPE html>
<html>
<head>
    <title>{{page.title}} - {{site.title}}</title>
</head>
<body>
    <h1>{{page.title}}</h1>
    <div class="content">{{content}}</div>
</body>
</html>
```

## Available Variables

### Site Variables

Access global site configuration:

```html
<title>{{site.title}}</title>
<meta name="description" content="{{site.description}}">
<link rel="canonical" href="{{site.url}}">
```

- `{{site.title}}` - Site title from config
- `{{site.description}}` - Site description
- `{{site.url}}` - Site URL
- `{{site.template}}` - Current template name

### Page Variables

Access page-specific metadata:

```html
<article>
    <header>
        <h1>{{page.title}}</h1>
        {% if page.publish_date %}
        <time datetime="{{page.publish_date}}">{{page.publish_date}}</time>
        {% endif %}
        {% if page.reading_time_mins %}
        <span class="reading-time">{{page.reading_time_mins}} min read</span>
        {% endif %}
    </header>
    
    <div class="content">
        {{content}}
    </div>
    
    {% if page.keywords %}
    <div class="tags">
        <strong>Tags:</strong> {{page.keywords}}
    </div>
    {% endif %}
</article>
```

Available page variables:
- `{{page.title}}` - Page title
- `{{page.description}}` - Page description
- `{{page.keywords}}` - Keywords/tags
- `{{page.publish_date}}` - Publication date
- `{{page.reading_time_mins}}` - Estimated reading time
- `{{page.cover_image}}` - Cover image path
- `{{page.is_draft}}` - Boolean draft status
- `{{page.custom_metadata}}` - Dictionary of custom fields

### Content Variables

- `{{content}}` - Rendered HTML content from Markdown
- `{{blog.title}}` - Blog title (if configured)

## Advanced Features

### Conditional Content

Show content based on conditions:

```html
{% if page.cover_image %}
<img src="{{page.cover_image}}" alt="{{page.title}}" class="cover-image">
{% endif %}

{% if page.is_draft %}
<div class="alert alert-warning">
    <strong>Draft:</strong> This page is not published yet.
</div>
{% endif %}
```

### Custom Metadata Loop

Display custom metadata fields:

```html
{% if page.custom_metadata %}
<div class="metadata">
    <h3>Additional Information</h3>
    {% for key, value in page.custom_metadata %}
    <div class="meta-item">
        <strong>{{key}}:</strong> {{value}}
    </div>
    {% endfor %}
</div>
{% endif %}
```

### Template Inheritance

Create a base template (`base.html.jinja`):

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}{{page.title}}{% endblock %} - {{site.title}}</title>
    <meta name="description" content="{% block description %}{{page.description}}{% endblock %}">
    <link rel="stylesheet" href="res/styles.css">
    {% block head %}{% endblock %}
</head>
<body>
    <header>
        <h1><a href="{{site.url}}">{{site.title}}</a></h1>
        <p>{{site.description}}</p>
    </header>
    
    <main>
        {% block content %}{% endblock %}
    </main>
    
    <footer>
        <p>&copy; 2024 {{site.title}}. Generated with Genereto.</p>
    </footer>
</body>
</html>
```

Then extend it in your page template (`index.html.jinja`):

```html
{% extends "base.html.jinja" %}

{% block content %}
<article>
    <header>
        <h1>{{page.title}}</h1>
        {% if page.publish_date %}
        <time datetime="{{page.publish_date}}">{{page.publish_date}}</time>
        {% endif %}
    </header>
    
    <div class="content">
        {{content}}
    </div>
</article>
{% endblock %}
```

### Filters and Functions

Use Jinja2's built-in filters:

```html
<!-- Format text -->
<h1>{{page.title | title}}</h1>
<p>{{page.description | truncate(100)}}</p>

<!-- Format dates (if using date objects) -->
<time>{{page.publish_date | strftime('%B %d, %Y')}}</time>

<!-- Safe HTML rendering -->
<div class="content">{{content | safe}}</div>
```

## Blog Templates

For blog posts, you can create specialized templates:

### Blog Index (`blog-index.html.jinja`)

```html
{% extends "base.html.jinja" %}

{% block title %}{{blog.title if blog.title else site.title}}{% endblock %}

{% block content %}
<div class="blog-header">
    <h1>{{blog.title if blog.title else "Blog"}}</h1>
</div>

<div class="blog-posts">
    <!-- Blog posts will be inserted here by Genereto -->
    <!-- start_content -->
    
    <!-- Default blog post template -->
    <article class="blog-post-preview">
        <header>
            <h2><a href="{{page.file_name}}">{{page.title}}</a></h2>
            <div class="article-meta">
                {% if page.publish_date %}
                <time datetime="{{page.publish_date}}">{{page.publish_date}}</time>
                {% endif %}
                {% if page.reading_time_mins %}
                <span class="reading-time">{{page.reading_time_mins}} min read</span>
                {% endif %}
            </div>
        </header>
        <p>{{page.description}}</p>
        <a href="{{page.file_name}}" class="read-more">Read more →</a>
    </article>
    
    <!-- end_content -->
</div>
{% endblock %}
```

### Individual Blog Post (`blog.html.jinja`)

```html
{% extends "base.html.jinja" %}

{% block head %}
{% if page.cover_image %}
<meta property="og:image" content="{{site.url}}/{{page.cover_image}}">
{% endif %}
{% endblock %}

{% block content %}
<article class="blog-post">
    {% if page.cover_image %}
    <img src="{{page.cover_image}}" alt="{{page.title}}" class="cover-image">
    {% endif %}
    
    <header>
        <h1>{{page.title}}</h1>
        <div class="article-meta">
            {% if page.publish_date %}
            <time datetime="{{page.publish_date}}">{{page.publish_date}}</time>
            {% endif %}
            {% if page.reading_time_mins %}
            <span class="reading-time">{{page.reading_time_mins}} min read</span>
            {% endif %}
        </div>
    </header>
    
    <div class="content">
        {{content}}
    </div>
    
    {% if page.keywords %}
    <footer class="article-footer">
        <div class="tags">
            <strong>Tags:</strong> {{page.keywords}}
        </div>
    </footer>
    {% endif %}
</article>
{% endblock %}
```

## Migration Guide

### From Regular Templates

**Before:**
```html
<title>$GENERETO['title']</title>
<h1>$GENERETO['title']</h1>
<p>$GENERETO['description']</p>
```

**After:**
```html
<title>{{page.title}}</title>
<h1>{{page.title}}</h1>
<p>{{page.description}}</p>
```

### Mixed Usage

You can use both template systems side by side:

1. Keep existing `.html` templates as fallbacks
2. Create new `.html.jinja` templates for enhanced features
3. Migrate gradually as needed

## Best Practices

### 1. Use Template Inheritance

Create a base template and extend it for consistency:

```html
<!-- base.html.jinja -->
{% block head %}{% endblock %}
{% block content %}{% endblock %}

<!-- page.html.jinja -->
{% extends "base.html.jinja" %}
{% block content %}...{% endblock %}
```

### 2. Handle Missing Data

Use conditional blocks to handle optional data:

```html
{% if page.author %}
<p>By {{page.author}}</p>
{% endif %}
```

### 3. Create Reusable Components

Use macros for repeatable elements:

```html
{% macro article_meta(page) %}
<div class="article-meta">
    {% if page.publish_date %}
    <time datetime="{{page.publish_date}}">{{page.publish_date}}</time>
    {% endif %}
    {% if page.reading_time_mins %}
    <span class="reading-time">{{page.reading_time_mins}} min read</span>
    {% endif %}
</div>
{% endmacro %}

<!-- Usage -->
{{ article_meta(page) }}
```

### 4. Escape User Content

Always escape user-provided content:

```html
<!-- Safe for HTML content -->
<div>{{content | safe}}</div>

<!-- Auto-escaped for user input -->
<title>{{page.title}}</title>
```

## Troubleshooting

### Template Not Found

- Ensure `.html.jinja` extension is correct
- Check that `enable_jinja: true` is set in config.yml
- Verify template is in the correct directory

### Syntax Errors

- Use `{% %}` for control structures (if, for, etc.)
- Use `{{ }}` for variables and expressions
- Check bracket matching and proper closing tags

### Fallback Behavior

If Jinja processing fails, Genereto will:
1. Log the error
2. Fall back to the regular `.html` template
3. Continue processing other pages

This ensures your site builds even with template errors.

---

Ready to create powerful, dynamic templates? Enable Jinja2 in your `config.yml` and start building!