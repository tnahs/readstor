#! /bin/zsh

# https://unix.stackexchange.com/a/115431
ROOT_DIR=${0:A:h:h}

"$ROOT_DIR/scripts/run-app.sh" export
"$ROOT_DIR/scripts/run-app.sh" render
"$ROOT_DIR/scripts/run-app.sh" backup