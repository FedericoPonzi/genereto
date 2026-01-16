#!/usr/bin/env bash
# Development run script

# Update fponzi.me repository
(cd fponzi.me/)

# Run genereto for all projects with drafts visible
cargo run -- --project-path fponzi.me/blog --drafts-options dev
cargo run -- --project-path fponzi.me/home --drafts-options dev
cargo run -- --project-path fponzi.me/swag --drafts-options dev
