#! /bin/zsh

# https://unix.stackexchange.com/a/115431
root=${0:A:h:h}
bin="readstor"
mac_bin="$bin-mac"
mac_bin_archive="$mac_bin.tar.gz"
mac_bin_sha256="$mac_bin.sha256"

cd "$root/target/release"
tar -czf $mac_bin_archive $bin
shasum -a 256 $mac_bin_archive >> $mac_bin_sha256
