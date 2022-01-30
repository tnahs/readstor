#! /bin/zsh

export DEV_READSTOR=1

function run {
    # https://unix.stackexchange.com/a/115431
    root=${0:A:h:h}

    tmp="$root/tmp"
    output="$tmp/run"

    rm -r $output
    mkdir $tmp
    mkdir $output

    cargo run -- \
        --output $output \
        "$1"
        # -vvv \
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
        run "$1"
    fi
}


main $@