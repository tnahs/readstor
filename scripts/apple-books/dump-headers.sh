#!/usr/bin/env zsh


# shellcheck disable=2296
NAME=$(basename "${(%):-%N}")


function dump_headers {
    local binary="$1"
    local output="$2"

    ipsw class-dump           \
        --arch ARM64e         \
        --headers             \
        --output "$output"    \
        "$binary" 2> /dev/null \
    || {
        return 1
    }

}


function dump_framework_headers {

    echo -e "Dumping Apple Books app frameworks:"

    local output="$1"

    # Note that 'BooksAll.frameworks' will error out and be skipped.
    for root in /System/Applications/Books.app/Contents/Frameworks/*.framework; do

        # `name` -> BKLibrary.framework
        local name="${root:t}"

        # `name` -> BKLibrary
        local name="${name%.framework}"

        local binary="$root/$name"

        if [[ -f "$binary" ]]; then
            echo "  $binary"
            dump_headers "$binary" "$output" || {
                echo -e "  Error: skipping $binary"
            }
        else
            echo -e "  Error: missing $binary"
        fi

    done
}


function dump_private_framework_headers {

    echo -e "Dumping Apple Books private frameworks:"

    local output="$1"

    local binaries=(
        # These were the only Book-related binaries in 'PrivateFrameworks'.
        /System/Library/PrivateFrameworks/BookLibraryCore.framework/Support/bookassetd
        /System/Library/PrivateFrameworks/BookDataStore.framework/Support/bookdatastored
    )

    for binary in "${binaries[@]}"; do

        echo "  $binary"

        dump_headers "$binary" "$output" || {
            echo -e "    Error: skipping $binary"
        }

    done
}


function check_requirements_are_installed {
    if ! hash ipsw 2> /dev/null; then
        echo "Error: 'ipsw' not installed"
        echo "Run 'brew install blacktop/tap/ipsw' to install"
        return 1
    fi
}


function print_help {
    echo -e "Dump macOS's Apple Books ObjC headers.

\e[4mUsage:\e[0m ${NAME} [OPTIONS] PATH

\e[4mArguments:\e[0m
  path  Path to dump headers to

\e[4mOptions:\e[0m
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

    dump_framework_headers "$1/Books-App"
    dump_private_framework_headers "$1/Books-System"
}


main "$@"
