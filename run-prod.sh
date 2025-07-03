#!/usr/bin/env bash
# simple run script

(cd blog/ && git submodule update --init --remote --recursive)
(cd fponzi.me/ && git submodule update --init --remote --recursive)

cargo run -- --project-path blog/genereto-project 
cargo run -- --project-path fponzi.me/home
cargo run -- --project-path fponzi.me/swag 
cargo run -- --project-path distsys.fponzi.me/

