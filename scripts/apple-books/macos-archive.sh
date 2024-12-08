#!/usr/bin/env zsh


if [ -n "$ZSH_VERSION" ]; then
    script_name=$(basename "${(%):-%N}")
else
    script_name=$(basename "$0")
fi


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

    rsync                                            \
        --archive                                    \
        --extended-attributes                        \
        "$HOME/Library/Containers/com.apple.BK*"     \
        "$HOME/Library/Containers/com.apple.iBooks*" \
        "$archive/Containers"

    rsync                                                       \
        --archive                                               \
        --extended-attributes                                   \
        "$HOME/Library/Group Containers/group.com.apple.iBooks" \
        "$archive/Group Containers"
}


function print_help {
    cat <<EOF
Archive macOS's Apple Books library

\e[4mUsage:\e[0m ${script_name} [PATH]"

\e[4mArguments:\e[0m
  PATH   Path to save archive to

\e[4mOptions:\e[0m
  -h, --help   Show help
EOF
}


function main {
    if [[ "$1" == "--help" ||  "$1" == "-h" ]]; then
        print_help
        exit 0
    elif [[ $# -lt 1 ]]; then
        echo "Error: Missing required positional argument: PATH"
        echo
        print_help
        exit 1
    elif [[ $# -gt 1 ]]; then
        echo "Error: Invalid or missing arguments"
        echo
        print_help
        exit 1
    else
        quit_applebooks
        archive_library "$1"
        echo "Archiving complete!"
    fi
}


main "$@"
