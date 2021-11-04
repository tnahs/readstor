#! /bin/zsh

cargo fmt \
&& cargo lint \
&& cargo build \
&& cargo test \
