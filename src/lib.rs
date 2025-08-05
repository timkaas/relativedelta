// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! > Rust implementation of `relativedelta` known from Python's [dateutil](https://pypi.org/project/python-dateutil/) library. Calculate dates by adding relative and offset values to a datetime instance. Currently, the [time](https://crates.io/crates/time) and [chrono](https://crates.io/crates/chrono) crates are supported.
//!
//! ## Usage
//!
//! Run `cargo add relativedelta` to add this crate to your project or put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! relativedelta = "0.3"
//! ```
//!
//! ### Minimum Supported Rust Version (MSRV)
//!
//! This crate supports Rust 1.85.0 or later.
//!
//! ### Optional features
//!
//! - **chrono**: Enable support for the [chrono](https://crates.io/crates/chrono) crate.
//! - **time**: Enable support for the [time](https://crates.io/crates/time) crate.
//! - **serde**: Enable serialization/deserialization via [serde](https://crates.io/crates/serde).
//!
//! ## Overview
//!
//! The `RelativeDelta` datatype holds both relative and absolute values for *year*, *month*, *day*, *hour*, *minute*,
//! *second* and
//! *nanosecond*.
//!
//! Relative parts are manipulated and accessed through methods typically ending in "*s*" (e.g. `::with_years`,
//! `.and_days`).
//! Absolute values without "*s*" (e.g. `::with_year`, `.and_day`).
//!
//! All relative values represents an offset to date and time and therefore can take on both positive and negative values,
//! and can take on any value within its datatypes limitations. On construction, a **Builder** will attempt to aggregate
//! values
//! up, so e.g. if *hours* are not in the range \[-23;23]\, the final instance will be updated to instead add or subtract
//! extra
//! *days*, with only the remainder as *hours*.
//! All offsets are set to zero as default.
//!
//! Absolute values represents an explicit *year*, *month*, *day* and so on. So if one e.g. always seeks a certain day in a
//! month, one would use the `::with_day()` or `.and_day()` method. All absolute values are **Options** and defaults to
//! `None`.
//!
//! `RelativeDelta` also holds a weekday value, which is an Option of a tuple with `(Weekday, nth)`. This allows one to e.g.
//! ask for the second tuesday one year from today,
//! with `Utc::now() + RelativeDelta::with_years(1).and_weekday(Some(Weekday::Tue, 2)).build()`.
//!
//! ### Examples
//!
//! Here are some examples of how to use the `RelativeDelta` library:
//!
//! #### Basic Construction
//!
//! Create a `RelativeDelta` representing 1 year:
//!
//! ```rust
//! # use relativedelta::RelativeDelta;
//! let years1 = RelativeDelta::with_years(1).build();
//! ```
//!
//! Create a `RelativeDelta` representing 12 months (equivalent to 1 year):
//!
//! ```rust
//! # use relativedelta::RelativeDelta;
//! let months12 = RelativeDelta::with_months(12).build();
//! ```
//!
//! Combining relative values:
//!
//! ```rust
//! # use relativedelta::RelativeDelta;
//! let one_year_32_days = RelativeDelta::with_years(1).and_days(32).build();
//! ```
//!
//! If the same parameter is specified twice, only the latest is applied:
//!
//! ```rust
//! # use relativedelta::RelativeDelta;
//! let months6 = RelativeDelta::with_months(12).and_months(6).build();
//! ```
//!
//! Combining absolute and relative values:
//!
//! ```rust
//! # use relativedelta::RelativeDelta;
//! // Set year to 2020, add 1 year, set month to January, add 3 months, add 12 days
//! let complex = RelativeDelta::with_year(2020)
//! .and_years(1)
//! .and_month(Some(1))
//! .and_months(3)
//! .and_days(12)
//! .build();
//! ```
//!
//! #### Operations Between `RelativeDelta` Instances
//!
//! Addition and subtraction (note that absolute values are lost in these operations):
//!
//! ```rust
//! # use relativedelta::RelativeDelta;
//! let delta1 = RelativeDelta::with_years( - 4).and_months(3).build();
//! let delta2 = RelativeDelta::with_years(1).and_months(42).build();
//!
//! // Addition
//! let sum = delta1 + delta2; // RelativeDelta::with_years(-3).and_months(45).build()
//!
//! // Subtraction
//! let diff = delta1 - delta2; // RelativeDelta::with_years(-5).and_months(-39).build()
//!
//! // Negation and addition
//! let neg_sum = - delta1 + delta2; // RelativeDelta::with_years(5).and_months(39).build()
//! ```
//!
//! Multiplication with a float:
//!
//! ```rust
//! # use relativedelta::RelativeDelta;
//! let delta = RelativeDelta::with_years(4).and_months(6).build();
//! let half = delta * 0.5; // RelativeDelta::with_years(2).and_months(3).build()
//! ```
//!
//! #### Modifying an Existing `RelativeDelta`
//!
//! Use the `.builder()` method to create a Builder from an existing `RelativeDelta`:
//!
//! ```rust
//! # use relativedelta::RelativeDelta;
//! let original = RelativeDelta::with_years(1).build();
//! let modified = original.builder().and_months(6).and_days( - 5).build();
//! ```
//!
//! #### Using `RelativeDelta` with Datetime Libraries
#![cfg_attr(
	feature = "chrono",
	doc = r"
With the chrono crate (requires the **chrono** feature):
```
# use relativedelta::{RelativeDelta, Weekday};
use chrono;
// Get the last day of the current month
let last_day_of_month = chrono::Utc::now() + RelativeDelta::with_day(1)
.and_months(1)
.and_days(-1)
.build();
// Get the first Monday after one year from a specific date
let first_monday_after_one_year = chrono::Utc::now() + RelativeDelta::with_years(1)
.and_weekday(Some((Weekday::Mon, 1)))
.build();
```
"
)]
#![cfg_attr(
	feature = "time",
	doc = r"
With the time crate (requires the **time** feature):

```
# use relativedelta::RelativeDelta;
use time;
// Add 1 year, 3 months, and 15 days to a date
let delta = RelativeDelta::with_years(1).and_months(3).and_days(15).build();
let result = time::UtcDateTime::now() + delta;
// Get the last day of a month
let last_day = time::UtcDateTime::now() + RelativeDelta::with_day(1)
.and_months(1)
.and_days(-1)
.build();
```
"
)]
//!
//! #### Working with Weekdays
//!
//! Get the 3rd Tuesday of next month:
//!
//! ```rust
//! # use relativedelta::{RelativeDelta, Weekday};
//! let third_tuesday_next_month = RelativeDelta::with_months(1)
//! .and_weekday(Some((Weekday::Tue, 3)))
//! .build();
//! ```

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(deprecated)]
#![deny(unused)]
#![cfg_attr(not(test), no_std)]

/// `RelativeDelta` is a relative date time representation for smart date calculations.
pub mod relativedelta;

/// Weekday is a module that provides functionality for working with weekdays in the context of `RelativeDelta`.
pub mod weekday;
pub use crate::weekday::Weekday;

#[cfg(feature = "chrono")]
/// `chrono_impl` is a module that provides implementations for `RelativeDelta` using the chrono crate.
mod chrono_impl;

#[cfg(feature = "time")]
/// `time_impl` is a module that provides implementations for `RelativeDelta` using the time crate.
mod time_impl;

#[cfg(any(feature = "time", feature = "chrono"))]
mod from_error;

pub use crate::relativedelta::RelativeDelta;

#[cfg(test)]
mod tests {
	use super::*;
	use similar_asserts::assert_eq;

	#[test]
	fn test_add_self() {
		let lhs = RelativeDelta::with_years(1).build();
		let rhs = RelativeDelta::with_years(2).build();

		assert_eq!(lhs + rhs, RelativeDelta::with_years(3).build());
		assert_eq!(lhs + rhs, rhs + lhs);
	}

	#[test]
	fn test_mul() {
		let ddt = RelativeDelta::with_years(10)
			.and_months(6)
			.and_days(-15)
			.and_hours(23)
			.build();
		let r = ddt * 0.5_f64;

		assert_eq!(
			r,
			RelativeDelta::yysmmsdds(None, 5, None, 3, None, -7)
				.and_minutes(-30)
				.build()
		);

		let rhs = RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 42, None, 0).build();
		assert_eq!(
			rhs * 0.5,
			RelativeDelta::with_years(2)
				.and_year(Some(2020))
				.and_months(3)
				.and_month(Some(1))
				.build()
		);
	}

	#[test]
	fn test_init_with_float() {
		let ddt = RelativeDelta::ysmsdshsmsssns_f(1.5, -18.0, 0.0, 0.0, 0.0, 0.0, 0).build();
		assert_eq!(
			ddt,
			RelativeDelta::yysmmsdds(None, 0, None, 0, None, 0)
				.and_hhsmmssss(None, 0, None, 0, None, 0)
				.build()
		);

		let ddt =
			RelativeDelta::ysmsdshsmsssns_f(-0.42, -15.7, -12.3, -5.32, 3.14, 0.15, 22232).build();
		let r = RelativeDelta::with_years(-1)
			.and_months(-8)
			.and_months_f(-0.7399999999999984)
			.and_days(-12)
			.and_hours(-12)
			.and_minutes(-28)
			.and_seconds(-3)
			.and_nanoseconds(-449977768)
			.build();
		assert_eq!(ddt, r);
	}

	#[test]
	fn test_is_empty() {
		let empty = RelativeDelta::default();
		assert!(empty.is_empty())
	}
}
