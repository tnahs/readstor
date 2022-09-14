#!/bin/zsh


function quit_applebooks {
    echo "Quitting Apple Books..."
    osascript -e 'tell application "Books" to quit'
}


function delete_library {
    echo "Deleting Apple Books library..."
    rm -rf $HOME/Library/Containers/com.apple.BK*
    rm -rf $HOME/Library/Containers/com.apple.iBooks*
    rm -rf $HOME/Library/Group Containers/group.com.apple.iBooks
}


function restore_library {

    # The trailing slash here is important. It tells 'rsync' to move the
    # *contents* of this directory into another. Otherwise it would move the
    # directory as a whole.
    archive="$1"/

    echo "Restoring Apple Books library..."
    rsync \
        --verbose \
        --progress \
        --archive \
        "$archive" \
        $HOME/Library/Containers/
}


function main {
    if [[ $# -ne 1 ]] then;
        echo "Error: Please provide a single path to an Apple Books library archive."
        exit 2
    else
        quit_applebooks
        delete_library
        restore_library "$1"
        echo "Apple Books library restored!"
        echo "Please restart before running Apple Books."
    fi
}


main "$@"
