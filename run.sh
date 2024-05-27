#!/usr/bin/env bash
# simple run script

(cd blog/ && git submodule update --init --remote --recursive)
(cd fponzi.me/ && git submodule update --init --remote --recursive)

cargo run -- --project-path blog/genereto-project --drafts-options dev
cargo run -- --project-path fponzi.me/genereto-project --drafts-options dev
