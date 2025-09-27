#!/usr/bin/env zsh


# shellcheck disable=2296
NAME=$(basename "${(%):-%N}")


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
    local archive="$1"

    echo "Restoring archived Apple Books library..."
    echo "This may take a few minutes..."

    # The trailing forward-slashes at the end of the source directories is
    # important. It tells 'rsync' to move the archive directory's *contents*
    # into the target directory. Otherwise it would move the source *directory*
    # into the target directory.

    rsync                           \
        --archive                   \
        --extended-attributes       \
        "$archive/Containers/"      \
        "$HOME/Library/Containers/" \
        || {
            echo "Error: encountered error during rsync for 'Containers'"
            return 1
        }

    rsync                                 \
        --archive                         \
        --extended-attributes             \
        "$archive/Group Containers/"      \
        "$HOME/Library/Group Containers/" \
        || {
            echo "Error: encountered error during rsync for 'Group Containers'"
            return 1
        }

        echo "Restore complete!"
        echo "Please restart before running Apple Books."
}


function check_requirements_are_installed {
    if ! hash rsync 2> /dev/null; then
        echo "Error: 'rsync' not installed."
        return 1
    fi
}


function print_help {
    echo -e "Restore a previously archived macOS Apple Books library.

\033[4mUsage:\033[0m ${NAME} [OPTIONS] PATH

\033[4mArguments:\033[0m
  path  Path to restore archive from

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
    delete_old_library
    restore_library "$1"
}


main "$@"
