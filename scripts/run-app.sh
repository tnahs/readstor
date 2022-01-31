#! /bin/zsh

export DEV_READSTOR=1

# https://unix.stackexchange.com/a/115431
ROOT_DIR=${0:A:h:h}

function run {
    tmp_dir="$ROOT_DIR/tmp"
    output_dir="$tmp_dir/output"

    rm -r $output_dir
    mkdir $tmp_dir
    mkdir $output_dir

    cargo run -- \
        --output $output \
        "$1"
        -vvv \
}

function main {
    choices=(export render backup)
    error="Error: Please provide at least one command to run: [$choices]."

    if [[ $# -lt 1 ]] then;
        echo $error
        exit 2
    # https://stackoverflow.com/a/30647954
    elif [[ ! ${choices[(r)$1]} ]] then;
        echo $error
    else
        run $1
    fi
}

main $@