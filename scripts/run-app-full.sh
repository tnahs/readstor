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
    export \
    --output $output \
    -vvv \

cargo run -- \
    render \
    --output $output \
    -vvv \

cargo run -- \
    backup \
    --output $output \
    -vvv \
