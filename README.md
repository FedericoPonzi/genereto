# Genereto

A simple static site generator to handle different kinds of simple static websites.

📖 **[View Documentation](https://federicoPonzi.github.io/genereto)** - Complete guide and tutorials

[<img src="https://github.com/FedericoPonzi/genereto/raw/main/assets/genereto-logo.jpg" width="300" align="center">](https://github.com/FedericoPonzi/genereto/raw/main/assets/genereto-logo.jpg)

## Quick Start

### Download Pre-built Binary (Recommended)

1. Download the latest release for Linux from [GitHub Releases](https://github.com/FedericoPonzi/genereto/releases/latest)
2. Extract the archive: `tar -xzf genereto-*.tar.gz`
3. Use the binary:

```bash
# Create a new project
./genereto generate-project --project-path ./my-site

# Build your site
./genereto --project-path ./my-site
```

### Build from Source

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

## GitHub Action

Use Genereto in your CI/CD pipeline with our official GitHub Action:

```yaml
- name: Build site
  uses: FedericoPonzi/genereto/.github/actions/build-site@v0.1.0-ga
  with:
    project-path: './docs'
```

Perfect for:
- 🚀 **GitHub Pages deployment** - No need to build from source
- ⚡ **Fast CI builds** - Uses pre-built binaries
- 🔧 **Easy setup** - Just one step in your workflow

[View Action Documentation](./.github/actions/build-site/README.md)

## Features

- Generate static websites and blogs
- Markdown content with YAML frontmatter
- Simple templating system
- RSS feed generation
- Draft support and TODOs
- Fast compilation
- GitHub Action for CI/CD

For complete documentation, tutorials, and examples, visit **[federicoPonzi.github.io/genereto](https://federicoPonzi.github.io/genereto)**