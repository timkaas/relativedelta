# RelativeDelta Cookbook

Practical recipes for common date/time scenarios using RelativeDelta.

## Table of Contents

- [HR & Payroll](#hr--payroll)
- [Finance & Billing](#finance--billing)
- [Scheduling & Deadlines](#scheduling--deadlines)
- [Calendar Operations](#calendar-operations)
- [Time-based Calculations](#time-based-calculations)

---

## HR & Payroll

### Salary Review Date (N Months After Hire)

Calculate when an employee's next salary review is due (typically 1 month, 3 months, or 1 year after hire).

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let hire_date = Utc::now();

// 1-month review
let one_month_review = hire_date + RelativeDelta::with_months(1).build();

// 3-month probation end
let probation_end = hire_date + RelativeDelta::with_months(3).build();

// Annual review
let annual_review = hire_date + RelativeDelta::with_years(1).build();
```

### Last Workday of Month (for Payroll)

Get the last day of the month for monthly payroll processing (typically not a weekend in real systems, but this gets the calendar day).

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let today = Utc::now();

// Get last day of current month
let last_day_this_month = today
    + RelativeDelta::with_day(1)
        .and_months(1)
        .and_days(-1)
        .build();

// Get last day of next month
let last_day_next_month = today + RelativeDelta::with_months(2)
    .and_day(1)
    .and_days(-1)
    .build();
```

### Vacation Days Expiry

Track when accrued vacation days expire (typically 1 year from accrual date).

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let accrual_date = Utc::now();
let expiry_date = accrual_date + RelativeDelta::with_years(1).build();

// Alert if within 30 days of expiry
let soon_expiring = today >= (expiry_date + RelativeDelta::with_days(-30).build());
```

---

## Finance & Billing

### Monthly Recurring Charge (Fixed Day of Month)

Charge a customer on the same day each month, handling month-boundary issues (e.g., Jan 31 → Feb 28).

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let today = Utc::now();
let charge_day = 15; // Always on the 15th

// Next charge date (same day next month)
let next_charge = today + RelativeDelta::with_months(1).and_day(Some(charge_day)).build();

// Schedule multiple charges
let charge_1 = today + RelativeDelta::with_months(1).and_day(Some(charge_day)).build();
let charge_2 = today + RelativeDelta::with_months(2).and_day(Some(charge_day)).build();
let charge_3 = today + RelativeDelta::with_months(3).and_day(Some(charge_day)).build();
```

### Quarterly Financial Reporting

Calculate dates for quarterly reports (Q1, Q2, Q3, Q4).

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let today = Utc::now();

// Q1 ends: Mar 31
let q1_end = today
    .with_month(3).unwrap()
    .with_day(31).unwrap()
    .with_hour(0).unwrap()
    .with_minute(0).unwrap()
    .with_second(0).unwrap()
    .with_nanosecond(0).unwrap();

// Or using RelativeDelta to ensure we get the right quarter-end:
let next_quarter = today + RelativeDelta::with_months(3).build();
let quarter_end = next_quarter
    + RelativeDelta::with_month(Some((next_quarter.month() / 3) * 3 + 3))
        .and_day(Some(31))
        .build();
```

### Loan/Mortgage Payment Schedule

Generate payment dates with fixed intervals.

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let loan_start = Utc::now();
let payment_day = 1; // Always on the 1st

// Generate 12 monthly payment dates
let mut payment_dates = Vec::new();
for month in 1..=12 {
    let payment = loan_start
        + RelativeDelta::with_months(month).and_day(Some(payment_day)).build();
    payment_dates.push(payment);
}
```

### Contract Renewal Date

Calculate when a contract renews (often 1 or 3 years from signature).

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let contract_date = Utc::now();
let renewal_term_years = 3;

let renewal_date = contract_date + RelativeDelta::with_years(renewal_term_years).build();
```

---

## Scheduling & Deadlines

### Find Nth Weekday of Month

Find the 2nd Tuesday, 3rd Friday, etc. (common for recurring meetings).

```rust
use relativedelta::{RelativeDelta, Weekday};
use chrono::Utc;

let today = Utc::now();

// Next month's 2nd Tuesday
let second_tuesday_next_month = today
    + RelativeDelta::with_months(1)
        .and_weekday(Some((Weekday::Tue, 2)))
        .build();

// Current month's 3rd Friday
let third_friday_this_month = today
    + RelativeDelta::with_weekday(Some((Weekday::Fri, 3)))
        .build();

// Last working day (Friday) of month - find 4th or 5th Friday
// Note: This is approximate; check actual day to confirm
let last_friday = today
    + RelativeDelta::with_day(1)
        .and_months(1)
        .and_weekday(Some((Weekday::Fri, -1)))  // -1 for last occurrence
        .build();
```

### Deadline: End of Month

Get the last day of the current or next month for submission deadlines.

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let today = Utc::now();

// Deadline: last day of this month
let end_of_month = today
    + RelativeDelta::with_day(1)
        .and_months(1)
        .and_days(-1)
        .build();

// Deadline: last day of next month
let end_of_next_month = today
    + RelativeDelta::with_day(1)
        .and_months(2)
        .and_days(-1)
        .build();
```

### Recurring Meeting Schedule

Schedule a recurring meeting every 2 weeks.

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let start_date = Utc::now();
let meeting_day_of_week = chrono::Weekday::Wed;

// Next 5 occurrences (every 2 weeks on Wednesday)
let mut meetings = Vec::new();
for occurrence in 0..5 {
    let weeks = occurrence * 2;
    let meeting = start_date
        + RelativeDelta::with_weeks(weeks as i32)
            .and_weekday(Some((
                // Convert chrono Weekday to relativedelta Weekday
                match meeting_day_of_week {
                    chrono::Weekday::Mon => relativedelta::Weekday::Mon,
                    chrono::Weekday::Tue => relativedelta::Weekday::Tue,
                    chrono::Weekday::Wed => relativedelta::Weekday::Wed,
                    chrono::Weekday::Thu => relativedelta::Weekday::Thu,
                    chrono::Weekday::Fri => relativedelta::Weekday::Fri,
                    chrono::Weekday::Sat => relativedelta::Weekday::Sat,
                    chrono::Weekday::Sun => relativedelta::Weekday::Sun,
                },
                1,
            )))
            .build();
    meetings.push(meeting);
}
```

---

## Calendar Operations

### Fiscal Year Calculations

Calculate fiscal year boundaries (e.g., July 1 - June 30).

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let today = Utc::now();
let fiscal_start_month = 7;  // July
let fiscal_start_day = 1;

// Current fiscal year start
let current_fy_start = today
    + RelativeDelta::with_month(Some(fiscal_start_month))
        .and_day(Some(fiscal_start_day))
        .build();

// Current fiscal year end
let current_fy_end = today
    + RelativeDelta::with_months(12)
        .and_month(Some(fiscal_start_month))
        .and_day(Some(fiscal_start_day - 1))
        .build();

// Next fiscal year start
let next_fy_start = today
    + RelativeDelta::with_years(1)
        .and_month(Some(fiscal_start_month))
        .and_day(Some(fiscal_start_day))
        .build();
```

### Age Calculation with Anniversary

Calculate when someone will turn a specific age.

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let birth_date = Utc::now() - chrono::Duration::days(365 * 30);  // ~30 years ago
let target_age = 35;

let next_birthday_at_target_age = birth_date
    + RelativeDelta::with_years(target_age).build();
```

### Business Quarter by Date

Determine which quarter a date falls in and get quarter boundaries.

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

fn get_quarter_bounds(date: chrono::DateTime<Utc>) -> (chrono::DateTime<Utc>, chrono::DateTime<Utc>) {
    let month = date.month();
    let quarter = (month - 1) / 3 + 1;  // 1-4
    
    // Quarter start month
    let start_month = (quarter - 1) * 3 + 1;
    let end_month = quarter * 3;
    
    let q_start = date
        + RelativeDelta::with_month(Some(start_month))
            .and_day(Some(1))
            .build();
    
    let q_end = date
        + RelativeDelta::with_month(Some(end_month))
            .and_day(Some(31))
            .build();
    
    (q_start, q_end)
}

let today = Utc::now();
let (q_start, q_end) = get_quarter_bounds(today);
```

---

## Time-based Calculations

### Meeting Duration with Offset

Schedule a meeting and calculate end time.

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let meeting_start = Utc::now();
let duration_hours = 2;

// End time (2 hours later)
let meeting_end = meeting_start
    + RelativeDelta::with_hours(duration_hours).build();

// Lunch break (1 hour after meeting starts)
let lunch_break = meeting_start
    + RelativeDelta::with_hours(1).build();
```

### Shift Scheduling

Calculate shift times (e.g., 9 AM - 5 PM with breaks).

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let shift_start = Utc::now().with_hour(9).unwrap();

// Lunch at 12 PM (3 hours in)
let lunch = shift_start + RelativeDelta::with_hours(3).build();

// Lunch duration 1 hour
let back_from_lunch = lunch + RelativeDelta::with_hours(1).build();

// Shift end 5 PM
let shift_end = shift_start + RelativeDelta::with_hours(8).build();
```

### Expiry windows

Check if something expires within a time window.

```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let expiry_date = Utc::now() + chrono::Duration::days(30);

// Check if expires within 7 days
let alert_date = expiry_date + RelativeDelta::with_days(-7).build();
let should_alert = Utc::now() >= alert_date;

// Check if already expired
let is_expired = Utc::now() > expiry_date;
```

---

## Tips & Best Practices

### 1. Always Build

Don't forget `.build()` at the end of the builder chain:

```rust
// ✅ Correct
let date = today + RelativeDelta::with_months(1).build();

// ❌ Wrong - doesn't compile
let date = today + RelativeDelta::with_months(1);
```

### 2. Handle Month Boundaries

RelativeDelta automatically handles them, but be aware:

```rust
let jan_31 = Utc::now().with_month(1).unwrap().with_day(31).unwrap();
let feb_28 = jan_31 + RelativeDelta::with_months(1).build();
// feb_28 is Feb 28 (or 29 in leap years), NOT Mar 1
```

### 3. Combine Multiple Offsets

You can chain multiple operations easily:

```rust
let date = today
    + RelativeDelta::with_years(1)
        .and_months(2)
        .and_days(3)
        .and_hours(4)
        .and_minutes(30)
        .build();
```

### 4. Use Absolute Values for Lock Points

Set specific values, not offsets:

```rust
// Lock to day 15 of the month (absolute)
let date = today + RelativeDelta::with_day(Some(15)).build();

// Add 15 days (relative)
let date = today + RelativeDelta::with_days(15).build();
```

### 5. Feature Flags

Enable only what you need:

```toml
# For chrono support only
relativedelta = { version = "0.3", features = ["chrono"] }

# For both chrono and serde
relativedelta = { version = "0.3", features = ["chrono", "serde"] }

# For JSON schema generation
relativedelta = { version = "0.3", features = ["schemars"] }
```

---

## See Also

- [Main Documentation](https://docs.rs/relativedelta)
- [vs Duration Guide](./vs-duration.md) — When to use each approach
- [Examples in README](../README.md#examples)
