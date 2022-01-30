#! /bin/zsh

cargo fmt \
&& cargo test \
&& cargo build \
&& cargo lint \
&& cargo doc --no-deps \
