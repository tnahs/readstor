#! /bin/zsh

# https://unix.stackexchange.com/a/115431
root=${0:A:h:h}

"$root/scripts/run-app.sh" export
"$root/scripts/run-app.sh" render
"$root/scripts/run-app.sh" backup