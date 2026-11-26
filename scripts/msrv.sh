#!/usr/bin/env bash
# Runs the tests with every feature on the minimum supported Rust version, the `rust-version` in
# Cargo.toml, so that version stays accurate.
#   scripts/msrv.sh            test
#   scripts/msrv.sh install    install that toolchain
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

msrv=$(sed -n 's/^rust-version = "\(.*\)"$/\1/p' Cargo.toml)
if [ -z "$msrv" ]; then
	echo "No rust-version in Cargo.toml" >&2
	exit 1
fi
if [ "${1:-}" = install ]; then
	rustup toolchain install "$msrv" --profile minimal
	exit
fi
if ! rustup run "$msrv" rustc --version >/dev/null 2>&1; then
	echo "Rust $msrv is missing; run: scripts/msrv.sh install" >&2
	exit 1
fi

cargo +"$msrv" test --all-features
