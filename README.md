relativedelta
=============

[![codecov](https://codecov.io/github/LunaPinc/relativedelta/graph/badge.svg?token=JN92RJTYZ1)](https://codecov.io/github/LunaPinc/relativedelta)
[![checks](https://github.com/LunaPinc/relativedelta/actions/workflows/checks.yml/badge.svg)](https://github.com/LunaPinc/relativedelta/actions/workflows/checks.yml)
[![License](https://img.shields.io/github/license/LunaPinc/relativedelta?style=flat-square)](https://github.com/LunaPinc/relativedelta/blob/main/LICENSE)
[![crates.io](https://img.shields.io/crates/v/relativedelta?style=flat-square)](https://crates.io/crates/relativedelta)
[![docs.rs](https://img.shields.io/badge/documentation-docs.rs-orange.svg?style=flat-square)](https://docs.rs/relativedelta/)

> **Date rules as values.** A `RelativeDelta` sets some fields of a date and shifts others: "the
> 1st of next month", "the last day of March", "the second Tuesday a year from now". One rule works
> on [jiff](https://crates.io/crates/jiff), [time](https://crates.io/crates/time) and
> [chrono](https://crates.io/crates/chrono) alike. A Rust take on Python's
> [`dateutil.relativedelta`](https://dateutil.readthedocs.io/en/stable/relativedelta.html).

```rust
use jiff::civil::date;
use relativedelta::{RelativeDelta, Weekday};

// Set the month to March and the day to the 1st, add a month, subtract a day:
// the last day of March, whatever date it is applied to.
let end_of_march = RelativeDelta::with_month(3).and_day(1).and_months(1).and_days(-1).build();
assert_eq!(date(2024, 7, 15) + end_of_march, date(2024, 3, 31));

// Months are calendar months: a month after January 31st is the end of February.
assert_eq!(date(2024, 1, 31) + RelativeDelta::with_months(1).build(), date(2024, 2, 29));

// The second Tuesday of next month.
let second_tuesday = RelativeDelta::with_months(1).and_day(1).and_weekday((Weekday::Tue, 2)).build();
assert_eq!(date(2024, 3, 20) + second_tuesday, date(2024, 4, 9));
```

## Contents

- [Install](#install)
- [How a rule is applied](#how-a-rule-is-applied)
- [One rule, every backend](#one-rule-every-backend)
- [Building and combining rules](#building-and-combining-rules)
- [Storing rules](#storing-rules)
- [Guides](#guides)
- [MSRV policy](#msrv-policy)
- [Acknowledgments](#acknowledgments)

## Install

```toml
[dependencies]
relativedelta = { version = "1", features = ["jiff"] }
```

Enable the backend you use; any combination of backends compiles. Without one, the crate still
builds and combines rules, ready for a backend chosen elsewhere.

| Feature | Adds |
|---------|------|
| `jiff` | `civil::Date`, `civil::DateTime`, `Timestamp` and `ZonedDateTime`. Recommended for new code. |
| `time` | `Date`, `PrimitiveDateTime`, `OffsetDateTime` and `UtcDateTime`. |
| `chrono` | `NaiveDate`, `NaiveDateTime` and `DateTime<Tz>`. |
| `serde` | Serializing and deserializing rules. Turns on `std`. |
| `schemars` | A JSON schema for rules. Turns on `std`. |
| `std` | The standard library. Without it the crate is `no_std` and needs no allocator. |

## How a rule is applied

A `RelativeDelta` holds *absolute* fields (`year`, `month`, `day`, `hour`, `minute`, `second`,
`nanosecond`), which **set** part of a date, and *relative* fields (`years`, `months`, `days`,
`hours`, `minutes`, `seconds`, `nanoseconds`), which **shift** it. It can also hold a weekday. Adding
a rule to a date applies them in this order, as dateutil does:

1. Each absolute field that is set replaces the matching field of the date.
2. Years and months are added. If the day does not exist in the resulting month, it becomes the
   month's last day.
3. Days through nanoseconds are added as an exact duration.
4. The weekday, if any, moves the date to its `nth` occurrence, counting the date itself:
   `(Weekday::Mon, 1)` is the next Monday or the date itself, `(Weekday::Mon, -1)` the previous one.

```rust
use jiff::civil::date;
use relativedelta::{RelativeDelta, Weekday};

let monday = date(2024, 4, 1);
assert_eq!(monday + RelativeDelta::with_weekday(Weekday::Mon, 1).build(), monday);
assert_eq!(monday + RelativeDelta::with_weekday(Weekday::Fri, 1).build(), date(2024, 4, 5));
assert_eq!(monday + RelativeDelta::with_weekday(Weekday::Fri, -1).build(), date(2024, 3, 29));
```

## One rule, every backend

The same rule applies to every supported type. `checked_add` returns `None` instead of panicking
when the result is out of range, and it is generic, so code written against it does not change
when you switch backends.

```rust
use relativedelta::RelativeDelta;

let next_month = RelativeDelta::with_months(1).build();

let jiff_date = jiff::civil::date(2024, 1, 31);
assert_eq!(next_month.checked_add(&jiff_date), Some(jiff::civil::date(2024, 2, 29)));

let time_date = time::Date::from_calendar_date(2024, time::Month::January, 31).unwrap();
assert_eq!(
    next_month.checked_add(&time_date),
    time::Date::from_calendar_date(2024, time::Month::February, 29).ok()
);

let chrono_date = chrono::NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
assert_eq!(next_month.checked_add(&chrono_date), chrono::NaiveDate::from_ymd_opt(2024, 2, 29));
assert_eq!(next_month.checked_add(&chrono::NaiveDate::MAX), None);
```

Date types use only the date fields and the days of a rule. Types with a time zone or offset apply
the absolute fields, years and months to the local time, then add the rest as exact durations.

## Building and combining rules

`RelativeDelta::with_*` starts a builder and `.and_*` adds to it; `build()` normalizes the
relative fields, so twelve months equal one year. Absolute setters take a value or an `Option`, and
`None` clears the field.

```rust
use relativedelta::RelativeDelta;

assert_eq!(RelativeDelta::with_months(12).build(), RelativeDelta::with_years(1).build());

let payday = RelativeDelta::with_day(15).and_months(1).build();
let cleared = payday.builder().and_day(None).build();
assert_eq!(cleared, RelativeDelta::with_months(1).build());
```

Rules combine like dateutil's. `+` and `-` combine the relative fields; an absolute field set on
both sides comes from the right operand of `+` and the left operand of `-`. Negation and
multiplication keep the absolute fields.

```rust
use relativedelta::RelativeDelta;

let delta1 = RelativeDelta::with_years(-4).and_months(3).and_day(1).build();
let delta2 = RelativeDelta::with_years(1).and_months(42).and_day(15).build();

assert_eq!(delta1 + delta2, RelativeDelta::with_years(-3).and_months(45).and_day(15).build());
assert_eq!(delta1 - delta2, RelativeDelta::with_years(-5).and_months(-39).and_day(1).build());

let delta = RelativeDelta::with_years(4).and_months(6).and_day(1).build();
assert_eq!(delta * 0.5, RelativeDelta::with_years(2).and_months(3).and_day(1).build());
assert_eq!(-delta, RelativeDelta::with_years(-4).and_months(-6).and_day(1).build());
```

## Storing rules

With the `serde` feature a rule serializes to any serde format, so schedules can live in a
database or a config file. Unset and zero fields are left out.

```rust
use jiff::civil::date;
use relativedelta::RelativeDelta;

let end_of_march: RelativeDelta =
    serde_json::from_str(r#"{"month": 3, "day": 1, "months": 1, "days": -1}"#).unwrap();
assert_eq!(date(2024, 7, 15) + end_of_march, date(2024, 3, 31));
```

## Guides

- **[Migrating from 0.3 to 1.0](docs/migrating-to-1.0.md)**: every breaking change, with the
  replacement.
- **[Coming from chronoutil](docs/from-chronoutil.md)**: `RelativeDuration`, `DateRule` and the
  `delta` functions, translated.
- **[Cookbook](docs/cookbook.md)**: recipes for payroll, billing, scheduling and calendars.
- **[RelativeDelta vs Duration](docs/vs-duration.md)**: when a calendar rule fits and when a fixed
  duration does.
- **[Changelog](CHANGELOG.md)**.

## MSRV policy

The minimum supported Rust version is the `rust-version` in `Cargo.toml`, which crates.io and
docs.rs show, and CI tests the crate with exactly that version. It is raised only in a minor release,
when a newer Rust feature earns it, and the changelog says so.

## Acknowledgments

The model of absolute and relative fields, the weekday rules and the order in which a rule is
applied come from [python-dateutil](https://github.com/dateutil/dateutil)'s `relativedelta`. This is
an independent Rust implementation.

`weekday.rs` is derived from [chrono](https://github.com/chronotope/chrono)'s, licensed under
MIT/Apache-2.0; see [NOTICE](NOTICE).

## Contributing

Contributions are welcome. Please open an issue before a large change, and add tests with any
change in behaviour.

## License

[Mozilla Public License 2.0](LICENSE).
