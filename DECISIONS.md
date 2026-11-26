# Decisions

Choices that deviate from the LunaPinc standards, or that a reader would otherwise question, with
the reason for each.

## Advisory ignores

- **RUSTSEC-2026-0543, chrono unmaintained.** chrono stays an optional backend behind the `chrono`
  feature, because most users, and everyone migrating from chronoutil, still depend on it. The
  README recommends jiff for new code. Review on 2027-06-01, or drop the backend if chrono gets a
  security advisory that won't be fixed.
- **RUSTSEC-2024-0436, paste unmaintained.** paste only builds setter names in macros, and the 1.0
  API cleanup removes it: the macros take full method names instead. Review on 2027-01-31.

## MSRV has one source

The 1.0 plan replaced `build.rs`, which rewrote the MSRV markers in `README.md` and `src/lib.rs` on
every build, with a CI check that the markers match `rust-version`. The markers are gone instead:
the README and crate docs point at `rust-version` in `Cargo.toml`, which crates.io and docs.rs show,
so there is nothing to keep in sync and nothing to check. `scripts/msrv.sh` tests that version.
