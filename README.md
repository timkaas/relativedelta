relativedelta
=============

[![codecov](https://codecov.io/github/timkaas/relativedelta/graph/badge.svg?token=JN92RJTYZ1)](https://codecov.io/github/timkaas/relativedelta)
[![Check, Lint & Test](https://github.com/timkaas/relativedelta/actions/workflows/ci.yml/badge.svg)](https://github.com/timkaas/relativedelta/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/timkaas/relativedelta?style=flat-square)](https://github.com/timkaas/relativedelta/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/relativedelta?style=flat-square)](https://crates.io/crates/relativedelta)
[![docs.rs](https://img.shields.io/badge/documentation-docs.rs-orange.svg?style=flat-square)](https://docs.rs/relativedelta/)

> **Calendar-aware date arithmetic for Rust.** Add or subtract months and years without off-by-one errors, clamp month ends, target weekdays, and express recurring dates clearly. A better alternative to `Duration` for calendar operations.
>
> Works with [chrono](https://crates.io/crates/chrono), [time](https://crates.io/crates/time) and [jiff](https://crates.io/crates/jiff) crates. Inspired by [Python's dateutil.relativedelta](https://pypi.org/project/python-dateutil/).

### **[Documentation](https://docs.rs/crate/relativedelta/latest)**

## Table of Contents

- [Usage](#usage)
- [When RelativeDelta Shines](#when-relativedelta-shines)
- [Features](#optional-features)
- [Examples](#examples)
- [Real-World Examples](#real-world-examples)
- [Guides](#guides)
- [Migration Guide](#migration-from-02x-to-030)
- [Contributing](#contributing)
- [License](#license)

## Usage

For `chrono`:

```toml
[dependencies]
relativedelta = { version = "0.3", features = ["chrono"] }
```

For the `time` or `jiff` crate, use `features = ["time"]` or `features = ["jiff"]`
instead; the backend features can be combined. The crate itself is
feature-free by default so it can be used to construct deltas without choosing
a datetime backend.

### Minimum Supported Rust Version (MSRV)

This crate supports Rust <!-- MSRV_START -->1.88.0<!-- MSRV_END --> or later.

### Optional features

- **chrono**: Enable support for the [chrono](https://crates.io/crates/chrono) crate.
- **time**: Enable support for the [time](https://crates.io/crates/time) crate.
- **jiff**: Enable support for the [jiff](https://crates.io/crates/jiff) crate (`civil::Date`,
  `civil::DateTime`, `Timestamp` and `ZonedDateTime`).
- **serde**: Enable serialization/deserialization via [serde](https://crates.io/crates/serde).
- **schemars**: Enable JSON schema generation via [schemars](https://crates.io/crates/schemars).
- **std**: Enable features that depend on the Rust standard library. Without this feature, the crate operates in `no_std` mode.

## When RelativeDelta Shines

| Problem | Duration Approach | RelativeDelta |
|---------|-------------------|----------------|
| **Add 1 month to Jan 31** | ❌ Breaks (Feb 31 doesn't exist) | ✅ Automatically clamps to Feb 28/29 |
| **Get last day of month** | ❌ Manual: `date + months(1) - days(1)` | ✅ Simple: `.and_day(1).and_months(1).and_days(-1)` |
| **Find 2nd Tuesday next quarter** | ❌ Complex date math + weekday logic | ✅ Direct: `.and_months(3).and_day(Some(1)).and_weekday(Some((Weekday::Tue, 2)))` |
| **Salary review in 1 month** | ❌ Naive addition fails on month boundaries | ✅ Semantic: `.and_months(1)` respects calendar |

## Overview

The **RelativeDelta** datatype holds both relative and absolute values for *year*, *month*, *day*, *hour*, *minute*,
*second* and
*nanosecond*.

Relative parts are manipulated and accessed through methods typically ending in "*s*" (e.g. `::with_years`,
`.and_days`).
Absolute values without "*s*" (e.g. `::with_year`, `.and_day`).

All relative values represent an offset to date and time and therefore can take on both positive and negative values,
and can take on any value within its datatypes limitations. On construction, a **Builder** will attempt to aggregate
values
up, so e.g. if *hours* are not in the range \[-23;23]\, the final instance will be updated to instead add or subtract
extra
*days*, with only the remainder as *hours*.
All offsets are set to zero as default.

Absolute values represent an explicit *year*, *month*, *day* and so on. So if one e.g. always seeks a certain day in a
month, one would use the `::with_day()` or `.and_day()` method. All absolute values are **Options** and defaults to
`None`.

`RelativeDelta` also holds a weekday value, which is an Option of a tuple with `(Weekday, nth)`. This allows one to e.g.
ask for the second Tuesday on or after the same date next year,
with `Utc::now() + RelativeDelta::with_years(1).and_weekday(Some((Weekday::Tue, 2))).build()`.

### Real-World Examples

#### HR & Payroll

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let hire_date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

// Salary review one month after hiring; February has no 31st, so the day is clamped
let salary_review = hire_date + RelativeDelta::with_months(1).build();
assert_eq!(salary_review, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());

// Last day of the month, for the monthly closing
let month_end = hire_date + RelativeDelta::with_day(1).and_months(1).and_days(-1).build();
assert_eq!(month_end, NaiveDate::from_ymd_opt(2024, 1, 31).unwrap());
```

#### Finance & Recurring Billing

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let today = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();

// Next monthly charge, always on the 15th
let next_charge = today + RelativeDelta::with_months(1).and_day(Some(15)).build();
assert_eq!(next_charge, NaiveDate::from_ymd_opt(2024, 4, 15).unwrap());

// Quarterly reporting, three months forward
let next_quarter = today + RelativeDelta::with_months(3).build();
assert_eq!(next_quarter, NaiveDate::from_ymd_opt(2024, 6, 20).unwrap());
```

#### Scheduling & Deadlines

```rust
use chrono::NaiveDate;
use relativedelta::{RelativeDelta, Weekday};

let today = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();

// Second Tuesday of next month
let second_tuesday = today
    + RelativeDelta::with_months(1)
        .and_day(Some(1))
        .and_weekday(Some((Weekday::Tue, 2)))
        .build();
assert_eq!(second_tuesday, NaiveDate::from_ymd_opt(2024, 4, 9).unwrap());

// Deadline on the last day of the month
let deadline = today + RelativeDelta::with_day(1).and_months(1).and_days(-1).build();
assert_eq!(deadline, NaiveDate::from_ymd_opt(2024, 3, 31).unwrap());
```

### Examples

Here is the core use case:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
let next_month = date + RelativeDelta::with_months(1).build();

assert_eq!(next_month, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
```

Unlike a fixed-length `Duration`, adding a calendar month understands that
February has no 31st day.

When the result may be outside the supported datetime range, use
`checked_add` instead of the `+` operator. It works with every supported type of
every backend, and returns `None` instead of panicking on overflow or an invalid
result:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let one_month = RelativeDelta::with_months(1).build();
assert_eq!(
    one_month.checked_add(&NaiveDate::from_ymd_opt(2024, 1, 31).unwrap()),
    NaiveDate::from_ymd_opt(2024, 2, 29)
);
assert_eq!(one_month.checked_add(&NaiveDate::MAX), None);
```

#### Basic Construction

```rust
use relativedelta::RelativeDelta;

// 1 year
let years1 = RelativeDelta::with_years(1).build();

// 12 months, which is the same as 1 year
let months12 = RelativeDelta::with_months(12).build();
assert_eq!(months12, years1);

// Relative values combine
let one_year_32_days = RelativeDelta::with_years(1).and_days(32).build();
assert_eq!(one_year_32_days.days(), 32);

// If the same parameter is set twice, the latest wins
let months6 = RelativeDelta::with_months(12).and_months(6).build();
assert_eq!(months6, RelativeDelta::with_months(6).build());

// Absolute and relative values combine: set the year to 2020 and add 1 year,
// set the month to January and add 3 months, then add 12 days
let complex = RelativeDelta::with_year(2020)
    .and_years(1)
    .and_month(Some(1))
    .and_months(3)
    .and_days(12)
    .build();
assert_eq!(complex.year(), Some(2020));
```

#### Operations Between RelativeDelta Instances

Addition and subtraction (note that absolute values are lost in these operations):

```rust
use relativedelta::RelativeDelta;

let delta1 = RelativeDelta::with_years(-4).and_months(3).build();
let delta2 = RelativeDelta::with_years(1).and_months(42).build();

assert_eq!(delta1 + delta2, RelativeDelta::with_years(-3).and_months(45).build());
assert_eq!(delta1 - delta2, RelativeDelta::with_years(-5).and_months(-39).build());
assert_eq!(-delta1 + delta2, RelativeDelta::with_years(5).and_months(39).build());
```

Multiplication with a float:

```rust
use relativedelta::RelativeDelta;

let delta = RelativeDelta::with_years(4).and_months(6).build();
assert_eq!(delta * 0.5, RelativeDelta::with_years(2).and_months(3).build());
```

#### Modifying an Existing RelativeDelta

Use the `.builder()` method to create a Builder from an existing RelativeDelta:

```rust
use relativedelta::RelativeDelta;

let original = RelativeDelta::with_years(1).build();
let modified = original.builder().and_months(6).and_days(-5).build();
assert_eq!(modified, RelativeDelta::with_years(1).and_months(6).and_days(-5).build());
```

#### Using RelativeDelta with Datetime Libraries

With the chrono crate (requires the **chrono** feature):

```rust
use chrono::{TimeZone, Utc};
use relativedelta::{RelativeDelta, Weekday};

let now = Utc.with_ymd_and_hms(2024, 2, 10, 9, 30, 0).unwrap();

// Last day of the current month
let last_day_of_month = now + RelativeDelta::with_day(1).and_months(1).and_days(-1).build();
assert_eq!(last_day_of_month, Utc.with_ymd_and_hms(2024, 2, 29, 9, 30, 0).unwrap());

// First Monday on or after the same date next year
let first_monday = now
    + RelativeDelta::with_years(1)
        .and_weekday(Some((Weekday::Mon, 1)))
        .build();
assert_eq!(first_monday, Utc.with_ymd_and_hms(2025, 2, 10, 9, 30, 0).unwrap());
```

With the time crate (requires the **time** feature):

```rust
use relativedelta::RelativeDelta;
use time::{Date, Month, PrimitiveDateTime, Time};

let at_half_past_nine = |year, month, day| {
    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month, day).unwrap(),
        Time::from_hms(9, 30, 0).unwrap(),
    )
};
let now = at_half_past_nine(2024, Month::February, 10);

// Add 1 year, 3 months and 15 days
let delta = RelativeDelta::with_years(1).and_months(3).and_days(15).build();
assert_eq!(now + delta, at_half_past_nine(2025, Month::May, 25));

// Last day of the month
let last_day = now + RelativeDelta::with_day(1).and_months(1).and_days(-1).build();
assert_eq!(last_day, at_half_past_nine(2024, Month::February, 29));
```

With the jiff crate (requires the **jiff** feature):

```rust
use jiff::civil::date;
use relativedelta::RelativeDelta;

// Last day of the month
let last_day = date(2024, 2, 10) + RelativeDelta::with_day(1).and_months(1).and_days(-1).build();
assert_eq!(last_day, date(2024, 2, 29));
```

#### Working with Weekdays

The 3rd Tuesday of next month:

```rust
use chrono::NaiveDate;
use relativedelta::{RelativeDelta, Weekday};

let third_tuesday_next_month = RelativeDelta::with_months(1)
    .and_day(Some(1))
    .and_weekday(Some((Weekday::Tue, 3)))
    .build();

let today = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();
assert_eq!(today + third_tuesday_next_month, NaiveDate::from_ymd_opt(2024, 4, 16).unwrap());
```

For more examples, see the [cookbook](docs/cookbook.md).

## Guides

- **[vs Duration](docs/vs-duration.md)** — When to use RelativeDelta vs Duration, with detailed comparisons and decision matrix
- **[Cookbook](docs/cookbook.md)** — Real-world recipes for HR, payroll, finance, scheduling, and calendar operations

## Migration from 0.3.0

**RelativeDelta** has been significantly refactored in version 0.3.0. The main changes are:

1. **Builder Pattern**: The method to finalize a **RelativeDelta** construction using the **Builder** has changed from
   `.new()` to `.build()` more closely
   following [rust convention](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html).

2. **Chrono functionality**: This crate now supports both [time](https://crates.io/crates/time)
   and [chrono](https://crates.io/crates/chrono) for datetime manipulation.
   This means that *chrono* functionality is now hidden behind the **chrono** feature. If you were using the *chrono*
   functionality, you need to explicitly enable it in your **Cargo.toml**:
   ```toml
   [dependencies]
   relativedelta = { version = "0.3", features = ["chrono"] }
   ```
3. **Serde Feature**: The serde feature has been renamed from "**serde1**" to "**serde**".
   If you were using the **serde1** feature, update your **Cargo.toml**:
   ```toml
   [dependencies]
   relativedelta = { version = "0.3", features = ["serde"] }
   ```
4. **Builder & Manipulation Methods**: The methods for creating and manipulating a **RelativeDelta** instance have been
   updated to
   follow a more consistent naming convention.
    - On **RelativeDelta**, all `::with_*()` functions are convenience for creating a new **Builder**. If a *
      *RelativeDelta** needs further manipulation after constuction, call `.builder()` which will copy over all values
      and allows for updating values using the `.and_*()` methods.
    - On the **Builder**, all `.with_*()` methods have been removed in favor of just `.and_*()` methods.

5. **Rust Edition**: Updated to Rust edition 2024 with MSRV <!-- MSRV_START -->1.88.0<!-- MSRV_END -->.

For more details, see the [migration guide](migration_guide.md).

## Acknowledgments

This project's RelativeDelta implementation is inspired by the relativedelta class from python-dateutil:
https://github.com/dateutil/dateutil

While this is a completely new implementation in Rust, the following concepts and design decisions were inspired by
python-dateutil:

- The general concept of relative and absolute time components
- The handling of weekday specifications
- The approach to date calculations with relative components
- The naming conventions for certain methods and attributes

This project includes code derived from [chrono](https://github.com/chronotope/chrono).
Specifically, our
`weekday.rs` implementation is based on chrono's implementation. For more details, see [NOTICE](NOTICE).\
Licensed under MIT/Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the Mozilla Public License, Version 2.0 or later (MPL-2.0+). You can find the full
license text at [https://mozilla.org/MPL/2.0/](https://mozilla.org/MPL/2.0/).
For more details about your rights and obligations, please see the [LICENSE](LICENSE) file in this repository.
