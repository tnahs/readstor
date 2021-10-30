#! /bin/zsh

export READSTOR_TESTING=1

# https://unix.stackexchange.com/a/115431
root=${0:A:h:h}

tmp="$root/tmp"
output="$tmp/run"
template="$root/tests/data/templates/valid.md"

rm -r $output
mkdir $tmp
mkdir $output

cargo run -- \
    $output \
    --force \
    --backup \
    --template $template \
    -vvv \

rm -r $output
mkdir $tmp
mkdir $output

cargo run -- \
    $output \
    --force \
    --backup \
    -vvv \
