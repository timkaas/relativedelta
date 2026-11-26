#!/usr/bin/env bash
# Runs clippy and the tests for each feature set on its own. The org hooks check only
# --all-features, which hides code that compiles only when another feature happens to be on.
# `num-traits/std` without this crate's `std` reproduces a dependent that turns on num-traits' std.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

feature_sets=("" chrono time jiff "chrono time" "chrono jiff" serde schemars std num-traits/std)
for features in "${feature_sets[@]}"; do
	echo "-- features: ${features:-(none)}"
	cargo clippy --all-targets --features "$features" -- -D warnings
	cargo test --features "$features"
done
