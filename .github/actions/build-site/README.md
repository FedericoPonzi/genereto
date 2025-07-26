# Build Site with Genereto Action

A GitHub Action to build static sites using the [Genereto](https://github.com/FedericoPonzi/genereto) static site generator.

## Features

- 🚀 **Fast Setup**: No need to build from source or manage dependencies
- 📦 **Pre-built Binaries**: Downloads and uses official Genereto releases
- 🔧 **Configurable**: Supports custom project paths and versions
- ✅ **Reliable**: Includes validation and error handling
- 🎯 **Simple**: Just one step to build your site

## Usage

### Basic Usage

```yaml
- name: Build site
  uses: FedericoPonzi/genereto/.github/actions/build-site@v0.1.0-ga
```

### With Custom Configuration

```yaml
- name: Build site
  uses: FedericoPonzi/genereto/.github/actions/build-site@v0.1.0-ga
  with:
    project-path: './my-site'
    genereto-version: 'v0.2.0'
    output-path: 'dist'
```

### Complete GitHub Pages Workflow

```yaml
name: Deploy to GitHub Pages

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
        uses: FedericoPonzi/genereto/.github/actions/build-site@v0.1.0-ga
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

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `project-path` | Path to the Genereto project directory | No | `./docs` |
| `genereto-version` | Version of Genereto to use (e.g., `v0.2.0` or `latest`) | No | `latest` |
| `output-path` | Path where the generated site will be output (relative to project-path) | No | `output` |

## Outputs

| Output | Description |
|--------|-------------|
| `output-path` | Full path to the generated site output directory |

## Examples

### Deploy to GitHub Pages

```yaml
name: Deploy Documentation

on:
  push:
    branches: [ "main" ]
    paths: [ 'docs/**' ]

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Build with Genereto
        id: build
        uses: FedericoPonzi/genereto/.github/actions/build-site@v0.1.0-ga
        
      - name: Setup Pages
        uses: actions/configure-pages@v4
        
      - name: Upload to Pages
        uses: actions/upload-pages-artifact@v3
        with:
          path: ${{ steps.build.outputs.output-path }}
          
      - name: Deploy to Pages
        uses: actions/deploy-pages@v4
```

### Build Multiple Sites

```yaml
name: Build Sites

on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        site: [docs, blog, portfolio]
    steps:
      - uses: actions/checkout@v4
      
      - name: Build ${{ matrix.site }}
        uses: FedericoPonzi/genereto/.github/actions/build-site@v0.1.0-ga
        with:
          project-path: ./${{ matrix.site }}
          
      - name: Upload ${{ matrix.site }} artifact
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.site }}-site
          path: ./${{ matrix.site }}/output
```

### Use Specific Version

```yaml
- name: Build with specific Genereto version
  uses: FedericoPonzi/genereto/.github/actions/build-site@v0.1.0-ga
  with:
    genereto-version: 'v0.1.5'
    project-path: './website'
```

## Requirements

- Linux runner (ubuntu-latest recommended)
- Your repository must contain a valid Genereto project structure:
  ```
  project-path/
  ├── config.yml
  ├── content/
  └── templates/
  ```

## Error Handling

The action includes comprehensive error handling:

- Validates that the project path exists
- Verifies Genereto binary download and installation
- Confirms output directory is created successfully
- Provides clear error messages for troubleshooting

## Troubleshooting

### Common Issues

**Project path not found:**
```
Error: Project path './docs' does not exist
```
- Ensure your project path is correct relative to repository root
- Check that your repository contains the Genereto project files

**Output directory not created:**
```
Error: Output directory './docs/output' was not created
```
- Check your `config.yml` for syntax errors
- Verify your content and template files are valid
- Review Genereto logs for build errors

**Binary download failed:**
- Check if the specified version exists in [releases](https://github.com/FedericoPonzi/genereto/releases)
- Ensure network connectivity in your runner environment

## Contributing

Found a bug or want to contribute? Please visit the [main repository](https://github.com/FedericoPonzi/genereto) to report issues or submit pull requests.

## License

This action is part of the Genereto project. See the [main repository](https://github.com/FedericoPonzi/genereto) for license information.
