#!/usr/bin/env bash
# Builds the library for a bare-metal target, which has no `std`, with each backend. The `serde`
# and `schemars` features turn on `std`, so they are left out.
#   scripts/no-std.sh            build
#   scripts/no-std.sh install    add the target to the toolchain
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

target=thumbv7em-none-eabihf
if [ "${1:-}" = install ]; then
	rustup target add "$target"
	exit
fi
if ! rustup target list --installed | grep -qx "$target"; then
	echo "The no_std target is missing; run: scripts/no-std.sh install" >&2
	exit 1
fi

for features in "" chrono time jiff "chrono time jiff"; do
	echo "-- features: ${features:-(none)}"
	cargo build --lib --target "$target" --features "$features"
done
