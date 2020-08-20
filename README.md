relativedelta
=============

[![GitHub Workflow Status](https://img.shields.io/travis/com/timkaas/relativedelta/master?style=flat-square)](https://github.com/stepancheg/rust-protobuf/actions?query=workflow%3ACI)
[![License](https://img.shields.io/github/license/timkaas/relativedelta?style=flat-square)](https://github.com/timkaas/relativedelta/blob/master/LICENSE)

Extention to the Duration from the the [time](https://github.com/rust-lang-deprecated/time) library, allowing calculating datetimes based on a relative representation of datetime.

## Usage

If you cannot wait until proper crate/first release, put this in your `Cargo.toml`:

```toml
[dependencies]
relativedelta = {git = "https://github.com/timkaas/relativedelta"}
```

Optional features:
- [`serde`][]: Enable serialization/deserialization via serde.

[`serde`]: https://github.com/serde-rs/serde

In the works:
- [X] Hook up to [travis.com](https://travis-ci.com/github/timkaas/relativedelta).
- [ ] Mitigation of month rounding error when init with floats or mul with floats.
- [ ] Create a proper crate and publish on [crates.io](https://crates.io/).
- [ ] Documentation and doctest.

Examples:

```rust
use relativedelta;

let year = 2020;
let month = 4;
let month2 = 3;
let months = -11;
let day = 28;
let days = 31;
let hour = 12;
let min = 35;
let sec = 48;
let n_secs = -11_111_111_111;
let dt = Utc.ymd(year, month, day).and_hms(hour, min, sec);
let ddt = RelativeDeltaDateTime::years(1)
.with_month(month2)
.with_months(months)
.with_days(days)
.with_nanoseconds(n_secs)
.new();

let add_1_year = RelativeDeltaDateTime::years(1).new();
assert_eq!(dt + add_1_year, Utc.ymd(2021, month, day).and_hms(hour, min, sec));

let sub_1_year = RelativeDeltaDateTime::years(-1).new();
assert_eq!(dt + sub_1_year, Utc.ymd(2019, month, day).and_hms(hour, min, sec));

let set_year = RelativeDeltaDateTime::year(2010).new();
assert_eq!(dt + set_year, Utc.ymd(2010, month, day).and_hms(hour, min, sec));

let set_year = RelativeDeltaDateTime::year(-1).new();
assert_eq!(dt + set_year, Utc.ymd(-1, month, day).and_hms(hour, min, sec));

let add_69_months = RelativeDeltaDateTime::months(69).new();
// Expected after fix
assert_eq!(add_69_months.years, 5);
assert_eq!(add_69_months.months, 9);
assert_eq!(dt + add_69_months, Utc.ymd(2026, 1, day).and_hms(hour, min, sec));

let sub_6_months = RelativeDeltaDateTime::months(-6).new();
assert_eq!(dt + sub_6_months, Utc.ymd(2019, 10, day).and_hms(hour, min, sec));

let sub_47_months = RelativeDeltaDateTime::months(-47).new();
// Expected after fix
assert_eq!(sub_47_months.years, -3);
assert_eq!(sub_47_months.months, -11);
assert_eq!(dt + sub_47_months, Utc.ymd(2016, 5, day).and_hms(hour, min, sec));

let add_400_days = RelativeDeltaDateTime::days(400).new();
assert_eq!(dt + add_400_days, Utc.ymd(2021, 6, 2).and_hms(hour, min, sec));

let sub_400_days = RelativeDeltaDateTime::days(-400).new();
assert_eq!(dt + sub_400_days, Utc.ymd(2019, 3, 25).and_hms(hour, min, sec));

let pay1 = RelativeDeltaDateTime::day(1).with_days(-1).with_month(3).with_months(1).new();
assert_eq!(dt + pay1, Utc.ymd(2020, 3, 31).and_hms(hour, min, sec));

let pay2 = RelativeDeltaDateTime::day(1).with_days(-1).with_month(6).with_months(1).new();
assert_eq!(dt + pay2, Utc.ymd(2020, 6, 30).and_hms(hour, min, sec));

let pay3 = RelativeDeltaDateTime::day(1).with_days(-1).with_month(9).with_months(1).new();
assert_eq!(dt + pay3, Utc.ymd(2020, 9, 30).and_hms(hour, min, sec));

let pay4 = RelativeDeltaDateTime::day(1).with_days(-1).with_month(12).with_months(1).new();
assert_eq!(dt + pay4, Utc.ymd(2020, 12, 31).and_hms(hour, min, sec));

// Multiplication

let ddt = RelativeDeltaDateTime::years(10).and_months(6).and_days(-15).and_hours(23).new();
let r = ddt * 0.42_f64;
println!("{:?}", r);

// Init with floats
let ddt = RelativeDeltaDateTime::ysmsdshsmsssns_f(-0.42, -15.7, -12.3, -5.32, 3.14, 0.15, 22232).new();
println!("test_init_with_float {:?}", ddt);

let ddt = RelativeDeltaDateTime::ysmsdshsmsssns_f(1.5, -18.0, 0.0, 0.0, 0.0, 0.0, 0).new();
assert_eq!(
    ddt,
    RelativeDeltaDateTime::yysmmsdds(None, 0, None, 0, None, 0)
        .and_hhsmmssss(None, 0, None, 0, None, 0)
        .new()
);
```
