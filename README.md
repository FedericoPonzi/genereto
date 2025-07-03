# Genereto

A simple static site generator to handle different kinds of simple static websites.

📖 **[View Documentation](https://federicoPonzi.github.io/genereto)** - Complete guide and tutorials

[<img src="https://github.com/FedericoPonzi/genereto/raw/main/assets/genereto-logo.jpg" width="300" align="center">](https://github.com/FedericoPonzi/genereto/raw/main/assets/genereto-logo.jpg)

## Quick Start

```bash
# Clone and build
git clone https://github.com/FedericoPonzi/genereto.git
cd genereto
cargo build --release

# Create a new project
./target/release/genereto generate-project --project-path ./my-site

# Build your site
./target/release/genereto --project-path ./my-site
```

## Features

- Generate static websites and blogs
- Markdown content with YAML frontmatter
- Simple templating system
- RSS feed generation
- Draft support and TODOs
- Fast compilation

For complete documentation, tutorials, and examples, visit **[federicoPonzi.github.io/genereto](https://federicoPonzi.github.io/genereto)**