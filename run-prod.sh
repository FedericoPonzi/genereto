#!/usr/bin/env bash
# Production run script

# Update fponzi.me repository
(cd fponzi.me/)

# Run genereto for all projects in production mode
cargo run -- --project-path fponzi.me/blog
cargo run -- --project-path fponzi.me/home
cargo run -- --project-path fponzi.me/swag
cargo run -- --project-path distsys.fponzi.me/

