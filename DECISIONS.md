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
