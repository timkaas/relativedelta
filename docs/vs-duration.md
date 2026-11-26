# RelativeDelta vs Duration: When to Use Each

This guide compares fixed-length durations (`std::time::Duration`, `chrono::TimeDelta`,
`time::Duration`, `jiff::SignedDuration`) with `RelativeDelta` to help you choose the right tool for
your date/time needs. The examples use chrono and fixed dates so every result can be checked.

## Quick Comparison

| Feature | Duration | RelativeDelta |
|---------|----------|---------------|
| **Fixed time intervals** | ✅ Excellent | ✅ Days, hours, minutes, seconds and nanoseconds |
| **Adding months/years** | ❌ Not possible | ✅ Core strength |
| **Month boundaries** | ❌ Breaks | ✅ Handles automatically |
| **Lock to specific day** | ❌ Not available | ✅ `.and_day(Some(15))` |
| **Target weekdays** | ❌ Not available | ✅ `.and_weekday(Some((Weekday::Tue, 2)))` |
| **Serialization** | ✅ Via serde | ✅ Via the serde feature |
| **No-std support** | ✅ Yes | ✅ Yes |

## Detailed Comparisons

### Problem 1: Adding Months to a Date

**The Issue**: What should "add 1 month to January 31st" return?

**Duration approach (approximate)**: a duration can't represent "1 month", only a fixed number of
days, so the result depends on the month you start in:

```rust
use chrono::{NaiveDate, TimeDelta};

let jan_31 = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
let roughly_a_month_later = jan_31 + TimeDelta::days(30);
assert_eq!(roughly_a_month_later, NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()); // not February
```

**RelativeDelta approach**:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let jan_31 = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
let one_month_later = jan_31 + RelativeDelta::with_months(1).build();
assert_eq!(one_month_later, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()); // leap year, clamped
```

**Why this matters**: Month length varies (28-31 days). RelativeDelta handles boundary logic automatically.

### Problem 2: Get Last Day of Month

**Duration approach (tedious)**:

```rust
use chrono::{Datelike, NaiveDate, TimeDelta};

let today = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
// Jump far enough into next month, go to its 1st, then back a day
let next_month = today.with_day(1).unwrap() + TimeDelta::days(31);
let last_day = next_month.with_day(1).unwrap() - TimeDelta::days(1);
assert_eq!(last_day, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
```

**RelativeDelta approach (clear intent)**:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let today = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
// Set day=1, add 1 month, subtract 1 day = last day of month
let last_day = today + RelativeDelta::with_day(1).and_months(1).and_days(-1).build();
assert_eq!(last_day, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
```

**Why this matters**: Semantic clarity. The code reads like the business logic.

### Problem 3: Find Nth Weekday of Month

**Duration approach (manual loop)**:

```rust
use chrono::{Datelike, Months, NaiveDate, TimeDelta, Weekday};

let today = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();
// 2nd Tuesday of next month: find the first Tuesday, then add a week
let mut date = today.with_day(1).unwrap() + Months::new(1);
while date.weekday() != Weekday::Tue {
    date += TimeDelta::days(1);
}
let second_tuesday = date + TimeDelta::days(7);
assert_eq!(second_tuesday, NaiveDate::from_ymd_opt(2024, 4, 9).unwrap());
```

**RelativeDelta approach (direct)**:

```rust
use chrono::NaiveDate;
use relativedelta::{RelativeDelta, Weekday};

let today = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();
// Next month, from its 1st, at the 2nd Tuesday
let second_tuesday = today
    + RelativeDelta::with_months(1)
        .and_day(Some(1))
        .and_weekday(Some((Weekday::Tue, 2)))
        .build();
assert_eq!(second_tuesday, NaiveDate::from_ymd_opt(2024, 4, 9).unwrap());
```

**Why this matters**: Intent is immediately clear. No loops or off-by-one risk.

## Decision Matrix

### Use Duration when:

- ✅ You need **fixed time intervals** (seconds, hours, fixed days)
- ✅ You're measuring **elapsed time** between two instants
- ✅ You're working with **timers** or **timeouts**
- Examples: "wait 500ms", "session expires in 30 seconds", "elapsed time since start"

### Use RelativeDelta when:

- ✅ You need **calendar semantics** (add months/years)
- ✅ You're dealing with **business logic** (monthly billing, quarterly reports)
- ✅ You need **month-boundary handling** (Jan 31 + 1 month = Feb 28/29)
- ✅ You want to **target specific dates** (day 15 of month, 2nd Tuesday)
- ✅ You need **absolute values** (lock year/month/day/hour to specific values)
- Examples: "next salary review", "quarterly payment date", "fiscal year end"

## Common Use Cases

### Case 1: Monthly Subscription Billing

A 30-day assumption drifts across month boundaries; a calendar month keeps the billing day:

```rust
use chrono::{NaiveDate, TimeDelta};
use relativedelta::RelativeDelta;

let signup = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

// Wrong: lands on Feb 14
assert_eq!(signup + TimeDelta::days(30), NaiveDate::from_ymd_opt(2024, 2, 14).unwrap());

// Right: always the 15th
assert_eq!(
    signup + RelativeDelta::with_months(1).build(),
    NaiveDate::from_ymd_opt(2024, 2, 15).unwrap()
);
```

### Case 2: Weekly Schedules

For a week, both give the same date: RelativeDelta adds days as exact 24-hour periods, just like a
duration. On a time-zone-aware date-time, a week across a DST change therefore lands an hour off
on the wall clock with either approach. Prefer whichever reads better next to the rest of your
code.

### Case 3: Calendar Year End

**Duration**: Not applicable (calendar concept).

**RelativeDelta**:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let today = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();
let year_end = today + RelativeDelta::with_month(12).and_day(Some(31)).build();
assert_eq!(year_end, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
```

### Case 4: API Response Timeout

**Duration**: Right tool. A timeout is a fixed span of elapsed time, not a calendar position:

```rust
use std::time::Duration;

let request_timeout = Duration::from_secs(30);
assert_eq!(request_timeout.as_millis(), 30_000);
```

**RelativeDelta**: Wrong tool.

## Migration Example

Suppose you have buggy code that assumes 30 days per month:

```rust
use chrono::{NaiveDate, TimeDelta};
use relativedelta::RelativeDelta;

let today = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

// Buggy: skips February entirely
let next_billing = today + TimeDelta::days(30);
assert_eq!(next_billing, NaiveDate::from_ymd_opt(2024, 3, 1).unwrap());

// Fixed: respects the calendar
let next_billing = today + RelativeDelta::with_months(1).build();
assert_eq!(next_billing, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
```

## See Also

- [RelativeDelta API Docs](https://docs.rs/relativedelta/)
- [Chrono TimeDelta Docs](https://docs.rs/chrono/latest/chrono/struct.TimeDelta.html)
- [Time Duration Docs](https://docs.rs/time/latest/time/struct.Duration.html)
- [Jiff SignedDuration Docs](https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html)
