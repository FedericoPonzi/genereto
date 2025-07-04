title: Templating System
publish_date: 2024-01-05
description: Understanding Genereto's simple but powerful templating system
---

# Templating System

Genereto uses a simple variable replacement system for templates.

## Basic Usage

Variables are accessed using the `&#36;GENERETO['variable_name']` syntax:

```html
<title>&#36;GENERETO['title']</title>
<meta name="description" content="&#36;GENERETO['description']">
```

## Content Replacement

The main content is replaced between special markers:

```html
<!-- start_content -->
This content will be replaced
<!-- end_content -->
```