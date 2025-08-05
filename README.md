relativedelta
=============

[![codecov](https://codecov.io/github/timkaas/relativedelta/graph/badge.svg?token=JN92RJTYZ1)](https://codecov.io/github/timkaas/relativedelta)
[![Publish](https://github.com/timkaas/relativedelta/actions/workflows/publish.yml/badge.svg)](https://github.com/timkaas/relativedelta/actions/workflows/publish.yml)
[![Coverage](https://github.com/timkaas/relativedelta/actions/workflows/coverage.yml/badge.svg)](https://github.com/timkaas/relativedelta/actions/workflows/coverage.yml)
[![License](https://img.shields.io/github/license/timkaas/relativedelta?style=flat-square)](https://github.com/timkaas/relativedelta/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/relativedelta?style=flat-square)](https://crates.io/crates/relativedelta)
[![docs.rs](https://img.shields.io/badge/documentation-docs.rs-orange.svg?style=flat-square)](https://docs.rs/relativedelta/)

> Rust implementation of `relativedelta` known from Python's [dateutil](https://pypi.org/project/python-dateutil/) library. Calculate dates by adding relative and offset values to a datetime instance. Currently, the [time](https://crates.io/crates/time) and [chrono](https://crates.io/crates/chrono) crates are supported.

### **[Documentation](https://docs.rs/crate/relativedelta/latest)**

## Table of Contents

- [Usage](#usage)
- [Overview](#overview)
- [Features](#optional-features)
- [Examples](#examples)
- [Migration Guide](#migration-from-02x-to-030)
- [Contributing](#contributing)
- [License](#license)

## Usage

Run `cargo add relativedelta` to add this crate to your project or put this in your `Cargo.toml`:

```toml
[dependencies]
relativedelta = "0.3"
```

### Minimum Supported Rust Version (MSRV)

This crate supports Rust 1.85.0 or later.

### Optional features

- **chrono**: Enable support for the [chrono](https://crates.io/crates/chrono) crate.
- **time**: Enable support for the [time](https://crates.io/crates/time) crate.
- **serde**: Enable serialization/deserialization via [serde](https://crates.io/crates/serde).
- **std**: Enable features that depend on the Rust standard library. Without this feature, the crate operates in `no_std` mode.

## Overview

The **RelativeDelta** datatype holds both relative and absolute values for *year*, *month*, *day*, *hour*, *minute*,
*second* and
*nanosecond*.

Relative parts are manipulated and accessed through methods typically ending in "*s*" (e.g. `::with_years`,
`.and_days`).
Absolute values without "*s*" (e.g. `::with_year`, `.and_day`).

All relative values represents an offset to date and time and therefore can take on both positive and negative values,
and can take on any value within its datatypes limitations. On construction, a **Builder** will attempt to aggregate
values
up, so e.g. if *hours* are not in the range \[-23;23]\, the final instance will be updated to instead add or subtract
extra
*days*, with only the remainder as *hours*.
All offsets are set to zero as default.

Absolute values represents an explicit *year*, *month*, *day* and so on. So if one e.g. always seeks a certain day in a
month, one would use the `::with_day()` or `.and_day()` method. All absolute values are **Options** and defaults to
`None`.

`RelativeDelta` also holds a weekday value, which is an Option of a tuple with `(Weekday, nth)`. This allows one to e.g.
ask for the second tuesday one year from today,
with `Utc::now() + RelativeDelta::with_years(1).and_weekday(Some(Weekday::Tue, 2)).build()`.

### Examples

Here are some examples of how to use the RelativeDelta library:

#### Basic Construction

Create a RelativeDelta representing 1 year:

```rust
let years1 = RelativeDelta::with_years(1).build();
```

Create a RelativeDelta representing 12 months (equivalent to 1 year):

```rust
let months12 = RelativeDelta::with_months(12).build();
```

Combining relative values:

```rust
let one_year_32_days = RelativeDelta::with_years(1).and_days(32).build();
```

If the same parameter is specified twice, only the latest is applied:

```rust
let months6 = RelativeDelta::with_months(12).and_months(6).build();
```

Combining absolute and relative values:

```rust
// Set year to 2020, add 1 year, set month to January, add 3 months, add 12 days
let complex = RelativeDelta::with_year(2020)
.and_years(1)
.and_month(Some(1))
.and_months(3)
.and_days(12)
.build();
```

#### Operations Between RelativeDelta Instances

Addition and subtraction (note that absolute values are lost in these operations):

```rust
let delta1 = RelativeDelta::with_years( - 4).and_months(3).build();
let delta2 = RelativeDelta::with_years(1).and_months(42).build();

// Addition
let sum = delta1 + delta2; // RelativeDelta::with_years(-3).and_months(45).build()

// Subtraction
let diff = delta1 - delta2; // RelativeDelta::with_years(-5).and_months(-39).build()

// Negation and addition
let neg_sum = - delta1 + delta2; // RelativeDelta::with_years(5).and_months(39).build()
```

Multiplication with a float:

```rust
let delta = RelativeDelta::with_years(4).and_months(6).build();
let half = delta * 0.5; // RelativeDelta::with_years(2).and_months(3).build()
```

#### Modifying an Existing RelativeDelta

Use the `.builder()` method to create a Builder from an existing RelativeDelta:

```rust
let original = RelativeDelta::with_years(1).build();
let modified = original.builder().and_months(6).and_days( - 5).build();
```

#### Using RelativeDelta with Datetime Libraries

With the chrono crate (requires the **chrono** feature):

```rust
use chrono;
// Get the last day of the current month
let last_day_of_month = chrono::Utc::now() + RelativeDelta::with_day(1)
.and_months(1)
.and_days( - 1)
.build();

// Get the first Monday after one year from a specific date
let first_monday_after_one_year = chrono::Utc::now() + RelativeDelta::with_years(1)
.and_weekday(Some((Weekday::Mon, 1)))
.build();
```

With the time crate (requires the **time** feature):

```rust
use time;

// Add 1 year, 3 months, and 15 days to a date
let delta = RelativeDelta::with_years(1).and_months(3).and_days(15).build();
let result = time::UtcDateTime::now() + delta;

// Get the last day of a month
let last_day = time::UtcDateTime::now() + RelativeDelta::with_day(1)
.and_months(1)
.and_days( - 1)
.build();
```

#### Working with Weekdays

Get the 3rd Tuesday of next month:

```rust
let third_tuesday_next_month = RelativeDelta::with_months(1)
.and_weekday(Some((Weekday::Tue, 3)))
.build();
```

For more examples, see [examples](new_examples.md).

## Migration from 0.2.x to 0.3.0

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

5. **Rust Edition**: Updated to Rust edition 2024 with MSRV 1.85.0.

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