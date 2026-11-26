#!/usr/bin/env bash
# Installs what relativedelta's own hooks need beyond the org's toolchains: the no_std target and
# the MSRV toolchain.
set -euo pipefail
scripts/no-std.sh install
scripts/msrv.sh install
