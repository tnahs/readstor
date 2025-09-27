#!/usr/bin/env zsh


# shellcheck disable=2296
NAME=$(basename "${(%):-%N}")


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
        "$archive/Containers"                        \
        || {
            echo "Error: encountered error during rsync for 'Containers'"
            return 1
        }

    rsync                                                       \
        --archive                                               \
        --extended-attributes                                   \
        "$HOME/Library/Group Containers/group.com.apple.iBooks" \
        "$archive/Group Containers"                             \
        || {
            echo "Error: encountered error during rsync for 'Group Containers'"
            return 1
        }

    echo "Archiving complete!"
}


function check_requirements_are_installed {
    if ! hash rsync 2> /dev/null; then
        echo "Error: 'rsync' not installed"
        return 1
    fi
}


function print_help {
    echo -e "Archive macOS's Apple Books library.

\033[4mUsage:\033[0m ${NAME} [OPTIONS] PATH

\033[4mArguments:\033[0m
  path  Path to save archive to

\033[4mOptions:\033[0m
  -h, --help  Show help"
}


function main {
    if [[ "$1" == "--help" ||  "$1" == "-h" ]]; then
        print_help
        exit 0
    fi

    if ! check_requirements_are_installed; then
        exit 1
    fi

    if [[ $# -ne 1 ]]; then
        echo "Error: missing required positional argument 'path'"
        print_help
        exit 1
    fi

    if [[ ! -d "$1" ]]; then
        echo "Error: input path '$1' does not exist"
        exit 1
    fi

    quit_applebooks
    archive_library "$1"
}


main "$@"
