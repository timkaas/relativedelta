# RelativeDelta vs Duration: When to Use Each

This guide compares Rust's standard `Duration` type with `RelativeDelta` to help you choose the right tool for your date/time needs.

## Quick Comparison

| Feature | Duration | RelativeDelta |
|---------|----------|---------------|
| **Fixed time intervals** | ✅ Excellent | ✅ Works (converts to fixed) |
| **Day multiplication** | ✅ Perfect | ✅ Supported |
| **Adding months/years** | ❌ Not possible | ✅ Core strength |
| **Month boundaries** | ❌ Breaks | ✅ Handles automatically |
| **Lock to specific day** | ❌ Not available | ✅ `.and_day(Some(15))` |
| **Target weekdays** | ❌ Not available | ✅ `.and_weekday(Some(Weekday::Tue, 2))` |
| **Serialization** | ✅ Built-in | ✅ Via serde feature |
| **No-std support** | ✅ Yes | ✅ Yes |

## Detailed Comparisons

### Problem 1: Adding Months to a Date

**The Issue**: What should "add 1 month to January 31st" return?

**Duration approach (doesn't work)**:
```rust
use std::time::Duration;
use chrono::Utc;

let jan_31 = Utc::now(); // Jan 31, 2024
// Duration cannot represent "1 month" - only fixed 24-hour periods!
// This doesn't exist: let result = jan_31 + Duration::from_secs(30 * 24 * 3600);
// Above would be approximately 30 days, giving us Feb 29 (or 30), not March 31
```

**RelativeDelta approach**:
```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let jan_31 = Utc::now(); // Jan 31, 2024
let feb_29_2024 = jan_31 + RelativeDelta::with_months(1).build();
// Returns Feb 29 (2024 is leap year, automatic boundary handling)
```

**Why this matters**: Month length varies (28-31 days). RelativeDelta handles boundary logic automatically.

### Problem 2: Get Last Day of Month

**Duration approach (tedious)**:
```rust
use chrono::{Utc, Datelike};

let today = Utc::now();
// Calculate "tomorrow + 1 month, minus 1 day"
let next_month = today + chrono::Duration::days(31); // rough estimate
let first_of_next = next_month.with_day(1).unwrap();
let last_day = first_of_next - chrono::Duration::days(1);
```

**RelativeDelta approach (clear intent)**:
```rust
use relativedelta::RelativeDelta;
use chrono::Utc;

let today = Utc::now();
let last_day = today + RelativeDelta::with_day(1)
    .and_months(1)
    .and_days(-1)
    .build();
// Clear: set day=1, add 1 month, subtract 1 day = last day of month
```

**Why this matters**: Semantic clarity. The code reads like the business logic.

### Problem 3: Find Nth Weekday of Month

**Duration approach (not available)**:
```rust
use chrono::{Utc, Weekday, Datelike};

let today = Utc::now();
// No built-in way to say "find 2nd Tuesday next month"
// Manual approach:
let mut date = today + chrono::Duration::days(30); // rough
while date.weekday() != Weekday::Tue {
    date = date + chrono::Duration::days(1);
}
let count = (date.day() - 1) / 7 + 1;
if count != 2 { /* ... more logic ... */ }
```

**RelativeDelta approach (direct)**:
```rust
use relativedelta::{RelativeDelta, Weekday};
use chrono::Utc;

let today = Utc::now();
let second_tuesday = today + RelativeDelta::with_months(1)
    .and_weekday(Some((Weekday::Tue, 2)))
    .build();
// Direct: 1 month forward, at 2nd Tuesday
```

**Why this matters**: Intent is immediately clear. No loops or off-by-one risk.

## Decision Matrix

### Use Duration when:

- ✅ You need **fixed time intervals** (seconds, hours, fixed days)
- ✅ You're measuring **elapsed time** between two dates
- ✅ You need **millisecond precision**
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

**Wrong choice (Duration)**:
```rust
// PROBLEM: 30-day assumption breaks on month boundaries
let billing_date = today + Duration::days(30);
// Jan -> Feb often wrong; Feb -> Mar wrong in leap years
```

**Right choice (RelativeDelta)**:
```rust
// Always same day of month, handles boundaries
let billing_date = today + RelativeDelta::with_months(1).build();
```

### Case 2: Shift Work Scheduling

**Wrong choice (Duration)**:
```rust
// PROBLEM: "next week" isn't exactly 7 * 24 hours
let next_week_estimate = today + Duration::days(7);
```

**Right choice (RelativeDelta)**:
```rust
// Respects calendar week (handles DST, calendar boundaries)
let next_week = today + RelativeDelta::with_weeks(1).build();
```

### Case 3: Fiscal Year End

**Duration**: Not applicable (calendar concept).

**RelativeDelta**:
```rust
// Get fiscal year-end date (Dec 31)
let fiscal_year_end = today 
    + RelativeDelta::with_year(today.year() + 1)
        .and_month(Some(12))
        .and_day(Some(31))
        .build();
```

### Case 4: API Response Timeout

**Duration**: Right tool.

**RelativeDelta**: Wrong tool (use Duration).

```rust
// CORRECT: Use Duration for fixed time measurements
timeout(Duration::from_secs(30), make_request()).await
```

## Performance Notes

- **Duration**: Zero-cost abstraction. Simple arithmetic.
- **RelativeDelta**: Minimal overhead. Building involves integer comparisons for boundary logic. Negligible in most applications.

Typical comparison:
- Duration: <50ns for arithmetic
- RelativeDelta: <200ns for arithmetic + boundary logic

For business applications, this difference is irrelevant. Clarity wins.

## Migration Example

Suppose you have buggy code using Duration:

```rust
// BUGGY: Assumes 30 days per month
let next_billing = today + Duration::days(30);
```

Convert to RelativeDelta:

```rust
// FIXED: Respects calendar
let next_billing = today + RelativeDelta::with_months(1).build();
```

## See Also

- [RelativeDelta API Docs](https://docs.rs/relativedelta/)
- [Chrono Duration Docs](https://docs.rs/chrono/latest/chrono/struct.Duration.html)
- [Time Duration Docs](https://docs.rs/time/latest/time/struct.Duration.html)
