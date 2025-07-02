---
title: Getting Started with Genereto
publish_date: 2024-01-15
description: Learn how to create your first static site with Genereto
---

# Getting Started with Genereto

This guide will walk you through creating your first static site with Genereto.

## Installation

First, make sure you have Rust installed, then clone the repository:

```bash
git clone https://github.com/FedericoPonzi/genereto.git
cd genereto
cargo build --release
```

## Your First Site

Create a new project:

```bash
./target/release/genereto generate-project --project-path ./my-site
```

This creates the basic structure you need to get started.