#!/usr/bin/env bash
# HARNESS — do not edit.
# Usage: ./run.sh            run everything
#        ./run.sh p03        run one problem (runtime tests + compile checks)
set -uo pipefail
cd "$(dirname "$0")"
pkg=lifetimes-2026-10
if [[ $# -eq 0 ]]; then
  cargo test -p "$pkg" --no-fail-fast --tests; a=$?
  cargo test -p "$pkg" --doc; b=$?
else
  p=$1
  cargo test -p "$pkg" --test "${p}_*"; a=$?
  cargo test -p "$pkg" --doc -- "checks::${p}::"; b=$?
fi
echo
[[ $a -eq 0 ]] && echo "runtime tests:  PASS" || echo "runtime tests:  FAIL"
[[ $b -eq 0 ]] && echo "compile checks: PASS" || echo "compile checks: FAIL"
exit $(( a || b ))
