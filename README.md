relativedelta
=============

[![Travis Status](https://img.shields.io/travis/com/timkaas/relativedelta/master?style=flat-square)](https://travis-ci.com/github/timkaas/relativedelta)
[![License](https://img.shields.io/github/license/timkaas/relativedelta?style=flat-square)](https://github.com/timkaas/relativedelta/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/relativedelta?style=flat-square)](https://crates.io/crates/relativedelta)
[![docs.rs](https://img.shields.io/badge/documentation-docs.rs-orange.svg?style=flat-square)](https://docs.rs/relativedelta/0.2.2/relativedelta/)

Rust implementation of `relativedelta` known from Python's [dateutil](https://pypi.org/project/python-dateutil/) library.
Extension to the `Duration` from the the [time](https://github.com/rust-lang-deprecated/time) library, which allows for calculating datetimes based on a relative representation of date and time.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
relativedelta = "0.2"
```

### Optional features
- [`serde1`][]: Enable serialization/deserialization via serde.

[`serde1`]: https://github.com/serde-rs/serde

In the pipeline:
- [X] Hook up to [travis.com](https://travis-ci.com/github/timkaas/relativedelta).
- [ ] Mitigation of month rounding error when init with floats or mul with floats.
- [X] Create a proper crate and publish on [crates.io](https://crates.io/).
- [X] Documentation and doctest.
- [ ] Improve examples

## Overview

The `RelativeDelta` datatype holds both relative and absolute values for year, month, day, hour, minute, seconds and nanosecond.

Relative parts are manipulated and accessed through methods typically ending in "s" (e.g. `::with_years`, `.and_days`). Absolute values without "s". 

All relative values represents an offset to date and time and therefore can take on both positive and negative values, and can take on any value within its datatypes limitations. On creation, the `Builder` will attempt to aggregate values up, so e.g. if hours are not in the range \[-23;23]\, the datatype will be updated to instead add or subtract extra days, with only the remainder as hours. 
All offsets are set to zero as default. 

Absolute values represents explicit years, months, days and so on. So if one e.g. always seeks a certain day in the month, one would use the `::with_month` or `.and_month` method. All absolute values are Options and set to `None` as default.    

`RelativeDelta` also holds a weekday value, which is an Option of a tuple with `(Weekday, nth)`. This allows one to e.g. ask for the second tuesday one year from today, with `Utc::now() + RelativeDelta::with_years(1).and_weekday(Some(Weekday::Tue, 2)).new()`.


### Examples

```rust
// Construction
let years1 = RelativeDelta::with_years(1).new();
let months12 = RelativeDelta::with_months(12).new();
assert_eq!(years1, months12);

let years1 = RelativeDelta::with_years(1).and_days(32).new();
// If same parameter is specified twice, only the latest is applied.
let months6 = RelativeDelta::with_months(12).with_months(6).new();
assert_eq!(months6, RelativeDelta::with_months(6).new());
// Below is identical to: RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 3, None, 12).new();
let rddt = RelativeDelta::with_year(2020).and_years(1).and_month(Some(1)).and_months(3).and_days(12).new();

// Two or more RelativeDeltas can be added and substracted. However, note that constants are lost in the process.
let lhs = RelativeDelta::yysmmsdds(Some(2020), -4, Some(1), 3, None, 0).new();
let rhs = RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 42, None, 0).new();
assert_eq!(lhs + rhs, RelativeDelta::with_years(-3).and_months(45).new());
assert_eq!(lhs - rhs, RelativeDelta::with_years(-5).and_months(-39).new());
assert_eq!(-lhs + rhs, RelativeDelta::with_years(5).and_months(39).new());

// The RelativeDelta can be multiplied with a f64.
assert_eq!(rhs * 0.5, RelativeDelta::with_years(2).and_year(Some(2020)).and_months(3).and_month(Some(1)).new());

// This crates party piece is the ability to calculate dates based on already existing chrono::DateTime
// If one would like to get the last day of the month that one is currently in, it could be done with:
println!("{}", Utc::now() + RelativeDelta::with_day(1).and_months(1).and_days(-1).new());
// Above first sets the day of the month to the 1st, then adds a month and subtracts a day.

// One could also request the first monday after one year by
let first_monday_after_one_year = RelativeDelta::with_years(1).and_weekday(Some((Weekday::Mon, 1))).new();
let d = Utc.ymd(2020, 1, 1).and_hms(0,0,0) + first_monday_after_one_year;
assert_eq!(d, Utc.ymd(2021, 1, 4).and_hms(0,0,0));
```
