---
title: Hello World with Jinja2
publish_date: 2024-05-04
description: Welcome to my new blog built with Jinja2 templates!
keywords: hello, jinja2, genereto
show_table_of_contents: true
---

## Introduction

Welcome to my first blog post using Jinja2 templates in Genereto! This post demonstrates the power and flexibility of using Jinja2 for static site generation.

## Why Jinja2?

Jinja2 templates offer several advantages:

1. **Clean syntax** - Use `{{ variable }}` instead of `$GENERETO['field']`
2. **Powerful loops** - Iterate over articles with `{% for article in articles %}`
3. **Conditionals** - Show content conditionally with `{% if condition %}`
4. **Template inheritance** - Build complex layouts easily

## Example Code

Here's a simple example of Jinja2 syntax:

```html
{% for article in articles %}
<article>
    <h2>{{ article.title }}</h2>
    <p>{{ article.description }}</p>
</article>
{% endfor %}
```

## Conclusion

Jinja2 templates make it easy to build beautiful, dynamic static sites. Give it a try!
