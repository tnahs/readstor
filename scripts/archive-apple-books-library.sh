#!/bin/zsh


function quit_applebooks {
    echo "Quitting Apple Books..."
    osascript -e 'tell application "Books" to quit'
}


function archive_library {
    rsync \
        --verbose \
        --progress \
        --archive \
        --extended-attributes \
        $HOME/Library/Containers/com.apple.BK* \
        $HOME/Library/Containers/com.apple.iBooks* \
        "$1"
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
