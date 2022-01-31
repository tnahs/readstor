#! /bin/zsh

# https://unix.stackexchange.com/a/115431
ROOT_DIR=${0:A:h:h}

function main {
    bin="readstor"
    mac_bin="$bin-mac"                   # readstor-mac
    mac_bin_archive="$mac_bin.tar.gz"    # readstor-mac.tar.gz
    mac_bin_sha256="$mac_bin.sha256"     # readstor-mac.sha256

    target_dir="$ROOT_DIR/target"
    release_dir="$ROOT_DIR/release"

    rm -r $target_dir
    rm -r $release_dir
    mkdir $release_dir

    cargo fmt \
        && cargo build --release \
        && cargo lint \
        && cargo test \

    # Copy binary to 'release' directory and archive it
    cp "$target_dir/release/$bin" $release_dir \
        && cd $release_dir \
        && tar -czf $mac_bin_archive $bin \

    # Generate a hash from the archive
    hash="$(shasum -a 256 $mac_bin_archive)" \
        && echo $hash > $mac_bin_sha256 \

    echo ""
    echo "-------------------------------------------------------------------------------------"
    echo "Build successful! Generated the following archive hash:"
    echo $hash
    echo "-------------------------------------------------------------------------------------"
}

main