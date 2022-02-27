#!/bin/zsh


function quit_applebooks {
    echo "Quitting Apple Books..."
    osascript -e 'tell application "Books" to quit'
}

function archive_library {
    echo "Archiving Apple Books library..."
    tar \
        --create \
        --gzip \
        --file="$1" \
        --directory="$HOME/Library/Containers" \
        "com.apple.BK*" \
        "com.apple.iBooksX*"
}


function main {
    if [[ $# -ne 1 ]] then;
        echo "Error: Please provide a single output path."
        exit 2
    else
        quit_applebooks
        archive_library "$1"
        echo "Apple Books library archived!"
    fi
}

main "$@"
