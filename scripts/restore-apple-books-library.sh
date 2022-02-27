#!/bin/zsh


TEMPDIR=$(mktemp -d 2> /dev/null)


function quit_applebooks {
    echo "Quitting Apple Books..."
    osascript -e 'tell application "Books" to quit'
}

# NOTE This spits out an error when extracting:
#
#    tar: Special header too large
#    tar: Error exit delayed from previous errors.
#
# However after some research it seems like it's not a fatal error and
# restoration works as expected. Some further research is necessary.
function extract_archive {
    echo "Extracting Apple Books library archive to ${TEMPDIR}..."
    tar \
        --extract \
        --file="$1" \
        --directory=$TEMPDIR
}


function restore_library {
    echo "Deleting Apple Books library..."
    rm -rf "$HOME/Library/Containers/com.apple.BK*"
    rm -rf "$HOME/Library/Containers/com.apple.iBooks*"
    rm -rf "$HOME/Library/Group Containers/group.com.apple.iBooks"
    echo "Restoring Apple Books library..."
    mv "$TEMPDIR/com.apple.*" "$HOME/Library/Containers"
}


function delete_tempdir {
    echo "Deleting ${TEMPDIR}..."
    rm -r $TEMPDIR
}


function main {
    if [[ $# -ne 1 ]] then;
        echo "Error: Please provide a single path to an Apple Books library archive."
        exit 2
    else
        quit_applebooks
        extract_archive "$1"
        restore_library
        delete_tempdir
        echo "Apple Books library restored!"
        echo "Please restart before running Apple Books."
    fi
}


main "$@"
