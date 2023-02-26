#!/usr/bin/env zsh


function print_help {
    cat <<EOF
Archive macOS's Apple Books library

Usage: archive-library.sh [PATH]

Arguments:
    PATH    Path to save archive to

Options:
    -h, --help    Print help information
EOF
}


function print_usage {
    cat <<EOF

Usage: archive-library.sh [PATH]

For more information try '--help'.
EOF
}


function quit_applebooks {
    echo "Quitting Apple Books..."
    osascript -e 'tell application "Books" to quit'
}


# Additional rsync options:
#     --progress        prints per-file transfer progress
#     --stats           prints file transfer stats on completion
#     --delete-after    deletes files from the destination directory that are no
#                       longer in the source directory
function archive_library {
    local archive="$1"

    echo "Archiving the Apple Books library..."
    echo "This may take a few minutes..."

    rsync \
        --archive \
        --extended-attributes \
        $HOME/Library/Containers/com.apple.BK* \
        $HOME/Library/Containers/com.apple.iBooks* \
        "$archive"/Containers

    rsync \
        --archive \
        --extended-attributes \
        $HOME/Library/Group\ Containers/group.com.apple.iBooks \
        "$archive"/Group\ Containers
}


function main {
    if [[ $1 == "--help" ||  $1 == "-h" ]] then;
        print_help
        exit 1
    elif [[ $# -lt 1 ]] then;
        echo "error: Missing required positional argument: PATH"
        print_usage
        exit 2
    elif [[ $# -gt 1 ]] then;
        echo "error: Invalid or missing arguments"
        print_usage
        exit 2
    else
        quit_applebooks
        archive_library "$1"
        echo "Archiving complete!"
    fi
}


main "$@"
