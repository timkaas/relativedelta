// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Date rules as values, in the style of Python's
//! [`dateutil.relativedelta`](https://dateutil.readthedocs.io/en/stable/relativedelta.html).
//!
//! A [`RelativeDelta`] **sets** some fields of a date and **shifts** others: "the 1st of next
//! month", "the last day of March", "the second Tuesday a year from now". The same rule applies to
//! the date and date-time types of [jiff](https://docs.rs/jiff), [time](https://docs.rs/time) and
//! [chrono](https://docs.rs/chrono).
//!
//! # Concepts
//!
//! - **Absolute fields** (`year`, `month`, `day`, `hour`, `minute`, `second`, `nanosecond`) replace
//!   the matching field of a date. Set them with [`RelativeDelta::with_day`],
//!   [`Builder::and_day`](relativedelta::Builder::and_day) and their siblings.
//! - **Relative fields** (`years`, `months`, `days`, ...) shift a date. Months are calendar months:
//!   a day that does not exist in the resulting month becomes that month's last day.
//! - **A weekday** with an `nth` moves the result to that occurrence of the [`Weekday`], counting
//!   the date itself.
//!
//! Adding a rule to a date applies the absolute fields, then years and months, then days through
//! nanoseconds as an exact duration, then the weekday. [`RelativeDelta::checked_add`] does the same
//! for any [`Shift`] type and returns `None` when the result is out of range; the `+` and `-`
//! operators panic instead.
//!
//! # Quick start
//!
//! ```
//! use relativedelta::{RelativeDelta, Weekday};
//!
//! // Set the month to March and the day to the 1st, add a month, subtract a day.
//! let end_of_march = RelativeDelta::with_month(3).and_day(1).and_months(1).and_days(-1).build();
//! assert_eq!(end_of_march.month(), Some(3));
//! assert_eq!(end_of_march.days(), -1);
//!
//! let second_tuesday = RelativeDelta::with_months(1).and_day(1).and_weekday((Weekday::Tue, 2)).build();
//! assert_eq!(second_tuesday.weekday(), Some((Weekday::Tue, 2)));
//! ```
#![cfg_attr(
	feature = "jiff",
	doc = r"
Applied to a date, here with the `jiff` feature:

```
use jiff::civil::date;
use relativedelta::RelativeDelta;

let end_of_march = RelativeDelta::with_month(3).and_day(1).and_months(1).and_days(-1).build();
assert_eq!(date(2024, 7, 15) + end_of_march, date(2024, 3, 31));
let one_day = RelativeDelta::with_days(1).build();
assert_eq!(one_day.checked_add(&jiff::civil::Date::MAX), None);
```
"
)]
//! # Modules
//!
//! - [`relativedelta`]: [`RelativeDelta`], its [`Builder`](relativedelta::Builder) and the
//!   arithmetic between rules.
//! - [`weekday`]: [`Weekday`] and its conversions.
//!
//! # Feature flags
//!
//! None is enabled by default. Without a backend the crate builds and combines rules.
//!
//! - `jiff`: `civil::Date`, `civil::DateTime`, `Timestamp` and `ZonedDateTime`.
//! - `time`: `Date`, `PrimitiveDateTime`, `OffsetDateTime` and `UtcDateTime`.
//! - `chrono`: `NaiveDate`, `NaiveDateTime` and `DateTime<Tz>`.
//! - `serde`: serializing and deserializing rules; turns on `std`.
//! - `schemars`: a JSON schema for rules; turns on `std`.
//! - `std`: the standard library. Without it the crate is `no_std` and needs no allocator.
//!
//! The minimum supported Rust version is the `rust-version` in `Cargo.toml`. It is raised only in a
//! minor release, when a newer Rust feature earns it.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(deprecated)]
#![deny(unused)]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

#[macro_use]
mod vendor;

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

#[cfg(feature = "jiff")]
/// `jiff_impl` is a module that provides implementations for `RelativeDelta` using the jiff crate.
mod jiff_impl;

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
mod conversion_error;
#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
pub use crate::conversion_error::ConversionError;

/// `shift` is a module that defines the backend-agnostic `Shift` trait behind `RelativeDelta::checked_add`.
mod shift;

pub use crate::relativedelta::RelativeDelta;
pub use crate::shift::Shift;

#[cfg(test)]
mod tests {
	use similar_asserts::assert_eq;

	use super::*;

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
			RelativeDelta::with_years(5)
				.and_months(3)
				.and_days(-7)
				.and_minutes(-30)
				.build()
		);

		let rhs = RelativeDelta::with_year(2020)
			.and_years(1)
			.and_month(1)
			.and_months(42)
			.build();
		assert_eq!(
			rhs * 0.5,
			RelativeDelta::with_years(2)
				.and_year(2020)
				.and_months(3)
				.and_month(1)
				.build()
		);
	}

	#[test]
	fn test_is_empty() {
		let empty = RelativeDelta::default();
		assert!(empty.is_empty());
	}
}

/// Compiles and runs the code blocks of the README and guides as doctests, so they can't drift from
/// the API. They use every backend, so they run only with all features enabled.
#[cfg(all(
	doctest,
	feature = "chrono",
	feature = "time",
	feature = "jiff",
	feature = "serde",
	feature = "std"
))]
mod markdown_doctests {
	#[doc = include_str!("../README.md")]
	struct Readme;

	#[doc = include_str!("../docs/cookbook.md")]
	struct Cookbook;

	#[doc = include_str!("../docs/vs-duration.md")]
	struct VsDuration;

	#[doc = include_str!("../docs/migrating-to-1.0.md")]
	struct MigratingTo1;

	#[doc = include_str!("../docs/from-chronoutil.md")]
	struct FromChronoutil;
}
