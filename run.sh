#!/usr/bin/env bash
# simple run script

(cd blog/genereto-project/template/main && git pull)
(cd fponzi.me/genereto-project/template/main && git pull)

cargo run -- --project-path blog/genereto-project --drafts-options dev
cargo run -- --project-path fponzi.me/genereto-project --drafts-options dev
