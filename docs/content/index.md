title: Genereto Documentation
description: A simple static site generator for different kinds of static websites
add_title: true
---

Welcome to the Genereto documentation! Genereto is a simple static site generator that can handle different kinds of simple static websites.

## Features

With Genereto, you can:
* Generate a simple static website with a single page or multiple pages
* Generate a blog with articles
* Generate a tumblr style website
* Generate a simple static website along with a blog

## Quick Start

1. Create a new project:
```bash
cargo run -- generate-project --project-path ./my-site
```

2. Add content in `my-site/content/`
3. Build your site:
```bash
cargo run -- --project-path ./my-site
```

For detailed information, check out the [GitHub repository](https://github.com/FedericoPonzi/genereto).