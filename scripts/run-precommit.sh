#! /bin/zsh

cargo fmt \
&& cargo clippy \
&& cargo build \
&& cargo test \
