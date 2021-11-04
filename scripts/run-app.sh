#! /bin/zsh

export DEV_READSTOR=1

# https://unix.stackexchange.com/a/115431
root=${0:A:h:h}

tmp="$root/tmp"
output="$tmp/run"

rm -r $output
mkdir $tmp
mkdir $output

cargo run -- \
    --output $output \
    --export \
    --backup \
    --force \
    -vvv \
