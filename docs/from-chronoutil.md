# Coming from chronoutil

[chronoutil](https://crates.io/crates/chronoutil) adds calendar months to chrono. relativedelta
covers the same ground and adds rules that set fields, weekday targeting, and the same API on jiff
and time, which lets a codebase leave chrono without rewriting its date rules. This guide translates
chronoutil 0.2 to relativedelta 1.0.

```toml
[dependencies]
relativedelta = { version = "1", features = ["chrono"] }
```

## `RelativeDuration` becomes `RelativeDelta`

Both apply months first, clamping a day that does not exist in the target month to the month's
last day, and then add the rest as an exact duration. A rule built the same way gives the same
date.

| chronoutil | relativedelta |
|------------|---------------|
| `RelativeDuration::years(n)` | `RelativeDelta::with_years(n).build()` |
| `RelativeDuration::months(n)` | `RelativeDelta::with_months(n).build()` |
| `RelativeDuration::weeks(n)` | `RelativeDelta::with_days(7 * n).build()` |
| `RelativeDuration::days(n)` | `RelativeDelta::with_days(n).build()` |
| `RelativeDuration::hours(n)`, `minutes`, `seconds`, `nanoseconds` | `with_hours(n)`, `with_minutes`, `with_seconds`, `with_nanoseconds` |
| `RelativeDuration::zero()` | `RelativeDelta::default()` |
| `rd.with_duration(Duration::days(1))` | `.and_days(1)` on the builder |
| `date + rd`, `date - rd` | `date + delta`, `date - delta` |
| `rd * 2` | `delta * 2.0` |
| `rd1 + rd2`, `-rd` | `delta1 + delta2`, `-delta` |

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

// chronoutil: RelativeDuration::months(1).with_duration(Duration::days(1))
let month_and_a_day = RelativeDelta::with_months(1).and_days(1).build();
assert_eq!(ymd(2020, 1, 31) + month_and_a_day, ymd(2020, 3, 1));
```

The `+` operator panics when the result is out of range, as chronoutil's does. Use
`delta.checked_add(&date)` to get an `Option` instead.

chronoutil's ISO 8601 parsing and formatting has no counterpart. To store rules, enable the `serde`
feature and use any serde format.

## The `delta` functions

| chronoutil | relativedelta |
|------------|---------------|
| `shift_months(date, n)` | `date + RelativeDelta::with_months(n).build()` |
| `shift_years(date, n)` | `date + RelativeDelta::with_years(n).build()` |
| `with_day(date, day)` | `date + RelativeDelta::with_day(day).build()` |
| `with_month(date, month)` | `date + RelativeDelta::with_month(month).build()` |
| `with_year(date, year)` | `date + RelativeDelta::with_year(year).build()` |
| `shift_months_opt`, `with_year_opt`, ... | `RelativeDelta::...build().checked_add(&date)` |
| `is_leap_year(year)` | the backend's own, e.g. `NaiveDate::leap_year` |

Setting a day, month or year clamps the day to the month's length, as chronoutil does. A
relativedelta rule can also set and shift in one value, which chronoutil needs several calls for:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

assert_eq!(ymd(2020, 2, 1) + RelativeDelta::with_day(31).build(), ymd(2020, 2, 29));
assert_eq!(ymd(2020, 1, 31) + RelativeDelta::with_month(2).build(), ymd(2020, 2, 29));
assert_eq!(ymd(2020, 2, 29) + RelativeDelta::with_year(2021).build(), ymd(2021, 2, 28));

// chronoutil: shift_months(with_day(date, 1).unwrap(), 1) - Duration::days(1)
let month_end = RelativeDelta::with_day(1).and_months(1).and_days(-1).build();
assert_eq!(ymd(2020, 2, 10) + month_end, ymd(2020, 2, 29));
```

## `DateRule` becomes an iterator over multiples

relativedelta has no `DateRule`; a range of multiples of a rule does the same job. Each date is
computed from the start, so month ends do not drift, matching `DateRule::monthly`.

| chronoutil | relativedelta |
|------------|---------------|
| `DateRule::monthly(start)` | `(0..).map(\|n\| start + RelativeDelta::with_months(n).build())` |
| `DateRule::yearly(start)` | `... RelativeDelta::with_years(n) ...` |
| `DateRule::weekly(start)` | `... RelativeDelta::with_days(7 * n) ...` |
| `DateRule::new(start, rd)` | `... rule * n as f64 ...`, or build the rule from `n` |
| `.with_count(k)` | `.take(k)` |
| `.with_end(end)` | `.take_while(\|date\| *date < end)` |
| `.with_rolling_day(day)` | add `.and_day(day)` to the rule |

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let start = ymd(2020, 1, 31);

// chronoutil: DateRule::monthly(start).with_count(3)
let monthly: Vec<_> = (0..3).map(|n| start + RelativeDelta::with_months(n).build()).collect();
assert_eq!(monthly, [ymd(2020, 1, 31), ymd(2020, 2, 29), ymd(2020, 3, 31)]);

// chronoutil: DateRule::monthly(ymd(2020, 2, 29)).with_rolling_day(31).unwrap().with_count(3)
let month_ends: Vec<_> = (0..3)
    .map(|n| ymd(2020, 2, 29) + RelativeDelta::with_months(n).and_day(31).build())
    .collect();
assert_eq!(month_ends, [ymd(2020, 2, 29), ymd(2020, 3, 31), ymd(2020, 4, 30)]);
```

## What relativedelta adds

- **Weekdays.** `and_weekday((Weekday::Tue, 2))` moves to the second Tuesday on or after the date.
- **Setting fields inside a rule.** A single value can say "the 15th of next month" or "the last
  day of March".
- **Other backends.** The same rules work on jiff and time, so you can move off chrono without
  touching them; chrono itself is flagged unmaintained
  ([RUSTSEC-2026-0543](https://rustsec.org/advisories/RUSTSEC-2026-0543)).
- **`no_std`** without an allocator.
