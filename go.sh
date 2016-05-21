#!/usr/bin/env bash

set -e

function main {
  case "$1" in

  check)
    check
    ;;

  test)
    test
    ;;

  run)
    run
    ;;

  *)
    help
    exit 1
    ;;

  esac
}

function help {
  echo "Usage:"
  echo " check   checks prerequisites are installed"
  echo " test    runs the test suite"
  echo " run     run the application"
}

function check {
  path_to_cargo=$(which cargo || echo "")
  if [ ! -x "$path_to_cargo" ]; then
    echo "Rust not found. (Try running 'brew update && brew install rust'.)"
    exit 1
  fi

  echo "Everything looks OK!"
  echo "Run \`./go.sh run <args>\` to run the application."
}

function test {
  cargo test
}

function run {
  cargo run --quiet
}

main "$@"
