# Changelog

All notable changes to this crate. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate follows
[Semantic Versioning](https://semver.org/). Upgrading from 0.3: see the
[migration guide](docs/migrating-to-1.0.md).

## [Unreleased]

## [1.0.0-rc.1] - 2026-11-27

The release candidate for 1.0. The repository moved to
[LunaPinc/relativedelta](https://github.com/LunaPinc/relativedelta).

### Added

- A `jiff` backend for `civil::Date`, `civil::DateTime`, `Timestamp` and `ZonedDateTime`.
- `time::Date` support in the `time` backend.
- `RelativeDelta::checked_add`, generic over every supported type through the sealed `Shift`
  trait, so the same code works with any backend.
- `ConversionError`, the public error type of `TryFrom<RelativeDelta>`; it is `#[non_exhaustive]`.
- `TryFrom<u8> for Weekday` with the error `InvalidWeekday`, and `From` for `chrono::Weekday`,
  `time::Weekday` and `jiff::civil::Weekday`.
- A documented [MSRV policy](README.md#msrv-policy), and guides for
  [migrating from 0.3](docs/migrating-to-1.0.md) and
  [coming from chronoutil](docs/from-chronoutil.md).

### Changed

- Absolute setters take `impl Into<Option<T>>`, so `and_day(1)` reads as it should; `Some(..)`
  still compiles. They are no longer `const fn`.
- `a + b` combines absolute fields and the weekday like dateutil: the right operand wins where
  both are set. `a - b` prefers the left operand. Both used to drop them.
- Multiplying a rule keeps its weekday, and negating it negates `months_f`.
- A weekday jump counts the date itself as the first occurrence, in both directions, and treats an
  `nth` of 0 as 1, as dateutil does.
- `TryFrom<RelativeDelta>` errors were renamed (`InvalidDate`, `InvalidTime`), and a year outside
  the target's range is `InvalidDate`.
- The `schemars` feature turns on `std`.
- The licence is `MPL-2.0` (was `MPL-2.0+`).

### Deprecated

- `checked_add_date`, `checked_add_naive_datetime`, `checked_add_datetime`,
  `checked_add_primitive_datetime`, `checked_add_offset_datetime` and `checked_add_utc_datetime`,
  in favour of `checked_add`. They will be removed in 2.0.

### Removed

- `ysmsdshsmsssns_f`, `yysmmsdds`, `hhsmmssss`, `Builder::and_yysmmsdds`,
  `Builder::and_hhsmmssss` and the deprecated `Builder::new`.
- `From<u8> for Weekday`, which wrapped numbers above 6.
- The `paste` dependency.

### Fixed

- Adding a rule to a `time::OffsetDateTime` kept the wall-clock time but moved it to UTC; it now
  keeps the original offset.
- `checked_add` on chrono and time types panicked when years, months or durations overflowed; it
  returns `None`.
- The crate builds without `std` with every backend, which a feature-unification bug broke.

## [0.3.4] - 2026-10-21

- Allow `std` APIs when the `std` feature is on.

## [0.3.3] - 2026-10-21

- `checked_add_*` methods that return `None` instead of panicking.
- `no_std` support, with `serde` turning on `std`.

## [0.3.2] - 2026-04-14

- JSON schema generation behind the `schemars` feature.

## [0.3.1] - 2026-04-09

- Support for chrono's `NaiveDate` and `NaiveDateTime`.
- MSRV raised to 1.88.

## [0.3.0] - 2025-08-06

Added the `time` backend and put chrono behind the `chrono` feature, replaced `Builder::new` with
`build`, renamed the `serde1` feature to `serde`, and moved to edition 2024.

## [0.2.2] - 2020-08-27

- Documentation and a weekday example.

## [0.2.1] - 2020-08-22

- Absolute fields on the builder are `Option`s, so they can be cleared.
- Fixed serializing `months_f`.

## [0.2.0] - 2020-08-21

- All fields are private, and the `serde` feature is named `serde1`.

## [0.1.0] - 2020-08-20

The first release.

[Unreleased]: https://github.com/LunaPinc/relativedelta/compare/v1.0.0-rc.1...HEAD
[1.0.0-rc.1]: https://github.com/LunaPinc/relativedelta/compare/v0.3.4...v1.0.0-rc.1
[0.3.4]: https://github.com/LunaPinc/relativedelta/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/LunaPinc/relativedelta/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/LunaPinc/relativedelta/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/LunaPinc/relativedelta/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/LunaPinc/relativedelta/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/LunaPinc/relativedelta/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/LunaPinc/relativedelta/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/LunaPinc/relativedelta/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/LunaPinc/relativedelta/releases/tag/v0.1.0
