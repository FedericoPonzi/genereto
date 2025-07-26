title: Setting up GitHub Pages with Genereto
publish_date: 2024-01-20
description: Complete guide to deploy your Genereto site to GitHub Pages with automated CI/CD
---

# Setting up GitHub Pages with Genereto

This guide shows you how to automatically deploy your Genereto-generated site to GitHub Pages using GitHub Actions.

## Prerequisites

- A GitHub repository with your Genereto project
- Basic knowledge of GitHub Actions
- Your site content ready in the `content/` directory

## Step 1: Create the GitHub Actions Workflow

Create `.github/workflows/docs.yml` in your repository:

```yaml
name: Deploy Documentation

on:
  push:
    branches: [ "main" ]
    paths:
      - 'docs/**'
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        
      - name: Build site with Genereto
        id: build
        uses: FedericoPonzi/genereto/.github/actions/build-site@v1.0.0-ga
        with:
          project-path: './docs'
        
      - name: Setup Pages
        uses: actions/configure-pages@v4
        
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: ${{ steps.build.outputs.output-path }}

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

## Step 2: Configure GitHub Pages

1. Go to your repository Settings
2. Navigate to Pages in the sidebar
3. Under "Source", select "GitHub Actions"
4. Save the settings

## Step 3: Set up Your Project Structure

Organize your project like this:

```
your-repo/
├── .github/workflows/docs.yml
├── docs/
│   ├── config.yml
│   ├── content/
│   │   ├── index.md
│   │   └── blog/
│   └── templates/main/
│       ├── index.html
│       └── blog.html
├── src/
└── Cargo.toml
```

## Step 4: Configure Your Site

Update your `docs/config.yml`:

```yaml
template: main
title: Your Site Title
url: https://yourusername.github.io/your-repo
description: Your site description
default_cover_image: ../assets/logo.jpg

blog:
  base_template: blog.html
  index_name: index.html
  destination: blog/
  generate_single_pages: true
```

## Step 5: Deploy

Push your changes to the main branch. The workflow will:

1. Download the latest Genereto binary
2. Generate your static site using the custom action
3. Deploy it to GitHub Pages

Your site will be available at `https://yourusername.github.io/your-repo`.

## Benefits of Using the Custom Action

The new Genereto GitHub Action provides several advantages:

- **Faster builds**: No need to compile Rust code from source
- **Simpler setup**: Just one step instead of multiple build steps
- **Automatic updates**: Uses the latest Genereto release by default
- **Better reliability**: Pre-built binaries reduce build failures
- **Version control**: Pin to specific Genereto versions when needed

## Advanced Usage

### Use a Specific Version

```yaml
- name: Build site with specific version
  uses: FedericoPonzi/genereto/.github/actions/build-site@v1.0.0-ga
  with:
    project-path: './docs'
    genereto-version: 'v0.2.0'
```

### Custom Output Path

```yaml
- name: Build site with custom output
  uses: FedericoPonzi/genereto/.github/actions/build-site@v1.0.0-ga
  with:
    project-path: './website'
    output-path: 'dist'
```

## Tips

- Use `paths` in your workflow to only trigger builds when content changes
- Add `workflow_dispatch` to manually trigger deployments
- The action automatically validates your project structure and provides helpful error messages
- Check the [action documentation](https://github.com/FedericoPonzi/genereto/tree/main/.github/actions/build-site) for more examples