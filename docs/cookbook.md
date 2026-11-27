# RelativeDelta Cookbook

Practical recipes for common date/time scenarios using RelativeDelta. The recipes use chrono and
fixed dates so every result can be checked; they work the same with the time and jiff backends.

## Table of Contents

- [HR & Payroll](#hr--payroll)
- [Finance & Billing](#finance--billing)
- [Scheduling & Deadlines](#scheduling--deadlines)
- [Calendar Operations](#calendar-operations)
- [Time-based Calculations](#time-based-calculations)
- [Tips & Best Practices](#tips--best-practices)

---

## HR & Payroll

### Salary Review Date (N Months After Hire)

Calculate when an employee's next salary review is due (typically 1 month, 3 months, or 1 year after hire).

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let hire_date = ymd(2024, 1, 31);

// 1-month review; February has no 31st, so the day is clamped
let one_month_review = hire_date + RelativeDelta::with_months(1).build();
assert_eq!(one_month_review, ymd(2024, 2, 29));

// 3-month probation end
let probation_end = hire_date + RelativeDelta::with_months(3).build();
assert_eq!(probation_end, ymd(2024, 4, 30));

// Annual review
let annual_review = hire_date + RelativeDelta::with_years(1).build();
assert_eq!(annual_review, ymd(2025, 1, 31));
```

### Last Day of Month (for Payroll)

Get the last calendar day of a month for monthly payroll processing: set the day to the 1st, move
forward a month, then go back a day.

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let today = ymd(2024, 3, 20);

let last_day_this_month = today + RelativeDelta::with_day(1).and_months(1).and_days(-1).build();
assert_eq!(last_day_this_month, ymd(2024, 3, 31));

let last_day_next_month = today + RelativeDelta::with_day(1).and_months(2).and_days(-1).build();
assert_eq!(last_day_next_month, ymd(2024, 4, 30));
```

### Vacation Days Expiry

Track when accrued vacation days expire (typically 1 year from accrual date).

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let accrual_date = ymd(2024, 3, 20);
let expiry_date = accrual_date + RelativeDelta::with_years(1).build();
assert_eq!(expiry_date, ymd(2025, 3, 20));

// Alert within 30 days of expiry
let alert_from = expiry_date - RelativeDelta::with_days(30).build();
assert_eq!(alert_from, ymd(2025, 2, 18));
let today = ymd(2025, 3, 1);
assert!(today >= alert_from);
```

---

## Finance & Billing

### Monthly Recurring Charge (Fixed Day of Month)

Charge a customer on the same day each month.

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let today = ymd(2024, 3, 20);
let charge_day = 15;

let charges: Vec<_> = (1..=3)
    .map(|month| today + RelativeDelta::with_months(month).and_day(charge_day).build())
    .collect();
assert_eq!(charges, [ymd(2024, 4, 15), ymd(2024, 5, 15), ymd(2024, 6, 15)]);
```

The day is clamped to the length of the month, so a charge on the 31st falls on the 30th in April
and on the 28th or 29th in February.

### Quarter Boundaries

Determine the quarter a date falls in and get its first and last day.

```rust
use chrono::{Datelike, NaiveDate};
use relativedelta::RelativeDelta;

fn quarter_bounds(date: NaiveDate) -> (NaiveDate, NaiveDate) {
    let first_month = u8::try_from((date.month0() / 3) * 3 + 1).unwrap();
    let start = date + RelativeDelta::with_month(first_month).and_day(1).build();
    let end = start + RelativeDelta::with_months(3).and_days(-1).build();
    (start, end)
}

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
assert_eq!(quarter_bounds(ymd(2024, 3, 20)), (ymd(2024, 1, 1), ymd(2024, 3, 31)));
assert_eq!(quarter_bounds(ymd(2024, 8, 7)), (ymd(2024, 7, 1), ymd(2024, 9, 30)));
```

### Loan/Mortgage Payment Schedule

Generate payment dates with fixed intervals.

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let loan_start = ymd(2024, 3, 20);

// 12 monthly payments, always on the 1st
let payment_dates: Vec<_> = (1..=12)
    .map(|month| loan_start + RelativeDelta::with_months(month).and_day(1).build())
    .collect();
assert_eq!(payment_dates.first(), Some(&ymd(2024, 4, 1)));
assert_eq!(payment_dates.last(), Some(&ymd(2025, 3, 1)));
```

### Contract Renewal Date

Calculate when a contract renews (often 1 or 3 years from signature).

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let contract_date = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();
let renewal_date = contract_date + RelativeDelta::with_years(3).build();
assert_eq!(renewal_date, NaiveDate::from_ymd_opt(2027, 3, 20).unwrap());
```

---

## Scheduling & Deadlines

### Find Nth Weekday of Month

Find the 2nd Tuesday, 3rd Friday, etc. (common for recurring meetings). Set the day to the 1st,
then ask for the nth weekday: the date itself counts as the first occurrence if it falls on that
weekday, as in dateutil.

```rust
use chrono::NaiveDate;
use relativedelta::{RelativeDelta, Weekday};

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let today = ymd(2024, 3, 20);

// Next month's 2nd Tuesday
let second_tuesday_next_month = today
    + RelativeDelta::with_months(1)
        .and_day(1)
        .and_weekday((Weekday::Tue, 2))
        .build();
assert_eq!(second_tuesday_next_month, ymd(2024, 4, 9));

// This month's 3rd Friday
let third_friday_this_month = today
    + RelativeDelta::with_day(1)
        .and_weekday((Weekday::Fri, 3))
        .build();
assert_eq!(third_friday_this_month, ymd(2024, 3, 15));

// This month's last Friday: go to the last day of the month, then search backwards
let last_friday = today
    + RelativeDelta::with_day(1)
        .and_months(1)
        .and_days(-1)
        .and_weekday((Weekday::Fri, -1))
        .build();
assert_eq!(last_friday, ymd(2024, 3, 29));
```

### Recurring Meeting Schedule

Schedule a meeting every 2 weeks on Wednesday, starting from the first Wednesday on or after a date.

```rust
use chrono::NaiveDate;
use relativedelta::{RelativeDelta, Weekday};

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let start_date = ymd(2024, 3, 18); // a Monday

let meetings: Vec<_> = (0..5)
    .map(|occurrence| {
        start_date
            + RelativeDelta::with_days(14 * occurrence)
                .and_weekday((Weekday::Wed, 1))
                .build()
    })
    .collect();
assert_eq!(
    meetings,
    [ymd(2024, 3, 20), ymd(2024, 4, 3), ymd(2024, 4, 17), ymd(2024, 5, 1), ymd(2024, 5, 15)]
);
```

---

## Calendar Operations

### Fiscal Year Calculations

Calculate fiscal year boundaries (e.g., July 1 - June 30).

```rust
use chrono::{Datelike, NaiveDate};
use relativedelta::RelativeDelta;

fn fiscal_year_start(date: NaiveDate) -> NaiveDate {
    let years_back = if date.month() >= 7 { 0 } else { -1 };
    date + RelativeDelta::with_years(years_back).and_month(7).and_day(1).build()
}

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let today = ymd(2024, 3, 20);

let current_fy_start = fiscal_year_start(today);
assert_eq!(current_fy_start, ymd(2023, 7, 1));

let current_fy_end = current_fy_start + RelativeDelta::with_years(1).and_days(-1).build();
assert_eq!(current_fy_end, ymd(2024, 6, 30));

let next_fy_start = current_fy_start + RelativeDelta::with_years(1).build();
assert_eq!(next_fy_start, ymd(2024, 7, 1));
```

### Age Calculation with Anniversary

Calculate when someone will turn a specific age.

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

let birth_date = ymd(1990, 8, 15);
assert_eq!(birth_date + RelativeDelta::with_years(35).build(), ymd(2025, 8, 15));

// Someone born on a leap day turns 25 on the last day of February
let leap_day_birth = ymd(2000, 2, 29);
assert_eq!(leap_day_birth + RelativeDelta::with_years(25).build(), ymd(2025, 2, 28));
```

---

## Time-based Calculations

### Meeting Duration with Offset

Schedule a meeting and calculate end time.

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let at = |h, m| NaiveDate::from_ymd_opt(2024, 3, 20).unwrap().and_hms_opt(h, m, 0).unwrap();
let meeting_start = at(9, 0);

let meeting_end = meeting_start + RelativeDelta::with_hours(2).build();
assert_eq!(meeting_end, at(11, 0));

let lunch_break = meeting_start + RelativeDelta::with_hours(1).and_minutes(30).build();
assert_eq!(lunch_break, at(10, 30));
```

### Shift Scheduling

Calculate shift times (e.g., 9 AM - 5 PM with breaks). Absolute time fields set the shift start
regardless of the current time.

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let at = |h, m| NaiveDate::from_ymd_opt(2024, 3, 20).unwrap().and_hms_opt(h, m, 0).unwrap();
let now = at(7, 42);

let shift_start = now + RelativeDelta::with_hour(9).and_minute(0).build();
assert_eq!(shift_start, at(9, 0));

let lunch = shift_start + RelativeDelta::with_hours(3).build();
let back_from_lunch = lunch + RelativeDelta::with_hours(1).build();
let shift_end = shift_start + RelativeDelta::with_hours(8).build();
assert_eq!((lunch, back_from_lunch, shift_end), (at(12, 0), at(13, 0), at(17, 0)));
```

### Expiry Windows

Check if something expires within a time window.

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let today = ymd(2024, 3, 20);
let expiry_date = ymd(2024, 4, 19);

// Alert from 7 days before expiry
let alert_date = expiry_date - RelativeDelta::with_days(7).build();
assert_eq!(alert_date, ymd(2024, 4, 12));
assert!(today < alert_date);
assert!(today <= expiry_date);
```

---

## Tips & Best Practices

### 1. Always Build

Finish the builder chain with `.build()`; a `Builder` can't be added to a date:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let today = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();
let next_month = today + RelativeDelta::with_months(1).build();
// `today + RelativeDelta::with_months(1)` doesn't compile
assert_eq!(next_month, NaiveDate::from_ymd_opt(2024, 4, 20).unwrap());
```

### 2. Handle Month Boundaries

RelativeDelta clamps the day to the length of the target month:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let jan_31 = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
let end_of_feb = jan_31 + RelativeDelta::with_months(1).build();
// Feb 28 (or 29 in leap years), NOT Mar 3
assert_eq!(end_of_feb, NaiveDate::from_ymd_opt(2023, 2, 28).unwrap());
```

### 3. Combine Multiple Offsets

You can chain multiple offsets in one delta:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let start = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap().and_hms_opt(9, 0, 0).unwrap();
let later = start
    + RelativeDelta::with_years(1)
        .and_months(2)
        .and_days(3)
        .and_hours(4)
        .and_minutes(30)
        .build();
assert_eq!(later, NaiveDate::from_ymd_opt(2025, 5, 23).unwrap().and_hms_opt(13, 30, 0).unwrap());
```

### 4. Use Absolute Values for Lock Points

Fields without an "s" set a value; fields with an "s" add an offset:

```rust
use chrono::NaiveDate;
use relativedelta::RelativeDelta;

let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
let today = ymd(2024, 3, 20);

// Lock to day 15 of the month (absolute)
assert_eq!(today + RelativeDelta::with_day(15).build(), ymd(2024, 3, 15));

// Add 15 days (relative)
assert_eq!(today + RelativeDelta::with_days(15).build(), ymd(2024, 4, 4));
```

### 5. Feature Flags

Enable only what you need:

```toml
# For chrono support only
relativedelta = { version = "0.3", features = ["chrono"] }

# For jiff and serde
relativedelta = { version = "0.3", features = ["jiff", "serde"] }

# For JSON schema generation
relativedelta = { version = "0.3", features = ["schemars"] }
```

---

## See Also

- [Main Documentation](https://docs.rs/relativedelta)
- [vs Duration Guide](./vs-duration.md) — When to use each approach
- [Examples in README](../README.md#examples)
