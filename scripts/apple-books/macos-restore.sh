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


function delete_old_library {
    echo "Deleting old Apple Books library..."
    rm -rf "$HOME/Library/Containers/com.apple.BK*"
    rm -rf "$HOME/Library/Containers/com.apple.iBooks*"
    rm -rf "$HOME/Library/Group Containers/group.com.apple.iBooks"
}


# Additional rsync options:
#     --progress    prints per-file transfer progress
#     --stats       prints file transfer stats on completion
function restore_library {
    # The trailing forward-slash here is important. It tells 'rsync' to move the
    # archive directory's *contents* into the target. Otherwise it would
    # move the archive *directory* into the target.
    local archive="$1"/

    echo "Restoring archived Apple Books library..."
    echo "This may take a few minutes..."

    rsync                     \
        --archive             \
        --extended-attributes \
        "$archive/Containers" \
        "$HOME/Library/Containers/"

    rsync                           \
        --archive                   \
        --extended-attributes       \
        "$archive/Group Containers" \
        "$HOME/Library/Group Containers/"
}


function print_help {
    echo -e "Restore a previously archived macOS Apple Books library

\e[4mUsage:\e[0m ${script_name} [PATH]

\e[4mArguments:\e[0m
  PATH   Path to restore archive from

\e[4mOptions:\e[0m
  -h, --help   Show help"
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
        delete_old_library
        restore_library "$1"
        echo "Restore complete!"
        echo "Please restart before running Apple Books."
    fi
}


main "$@"
