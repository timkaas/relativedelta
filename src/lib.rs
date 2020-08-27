// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # relativedelta: Relative date time representation for smart date calculations
//!
//! ## Usage
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! relativedelta = "0.2"
//! ```
//!
//! ### Optional features:
//! - [`serde1`][]: Enable serialization/deserialization via serde.
//!
//! [`serde1`]: https://github.com/serde-rs/serde
//!
//! ## Overview
//!
//! The `RelativeDelta` datatype holds both relative and absolute values for year, month, day, hour, minute, seconds and
//! nanosecond.
//!
//! Relative parts are manipulated and accessed through methods typically ending in "s" (e.g. `::with_years`, `.and_days`).
//! Absolute values without "s".
//!
//! All relative values represents an offset to date and time and therefore can take on both positive and negative values,
//! and can take on any value within its datatypes limitations.
//! On creation, the `Builder` will attempt to aggregate values up, so e.g. if hours are not in the range \[-23;23]\,
//! the datatype will be update to instead add or subtract extra days, with only the remainder as hours.
//! All offsets are set to zero as default.
//!
//! Absolute values represents explicit years, months, days and so on. So if one e.g. always seeks a certain day in the
//! month, one would use the `::with_month` or `.and_month` method. All absolute values are Options and set to `None` as
//! default.
//!
//! `RelativeDelta` also holds a weekday value, which is an Option of a tuple with `(Weekday, nth)`. This allows one to
//! e.g. ask for the second tuesday one year from today, with
//! `Utc::now() + RelativeDelta::with_years(1).and_weekday(Some(Weekday::Tue, 2)).new()`.
//!
//! ### Examples:
//!
//! ```rust
//! // Construction
//! # use relativedelta::RelativeDelta;
//! # use chrono::{Utc, TimeZone, Weekday};
//! let years1 = RelativeDelta::with_years(1).new();
//! let months12 = RelativeDelta::with_months(12).new();
//! assert_eq!(years1, months12);
//!
//! let years1 = RelativeDelta::with_years(1).and_days(32).new();
//! // If same parameter is specified twice, only the latest is applied.
//! let months6 = RelativeDelta::with_months(12).with_months(6).new();
//! assert_eq!(months6, RelativeDelta::with_months(6).new());
//! // Below is identical to: RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 3, None, 12).new();
//! let rddt = RelativeDelta::with_year(2020).and_years(1).and_month(Some(1)).and_months(3).and_days(12).new();
//!
//! // Two or more RelativeDeltas can be added and substracted. However, note that constants are lost in the process.
//! let lhs = RelativeDelta::yysmmsdds(Some(2020), -4, Some(1), 3, None, 0).new();
//! let rhs = RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 42, None, 0).new();
//! assert_eq!(lhs + rhs, RelativeDelta::with_years(-3).and_months(45).new());
//! assert_eq!(lhs - rhs, RelativeDelta::with_years(-5).and_months(-39).new());
//! assert_eq!(-lhs + rhs, RelativeDelta::with_years(5).and_months(39).new());
//!
//! // The RelativeDelta can be multiplied with a f64.
//! assert_eq!(rhs * 0.5, RelativeDelta::with_years(2).and_year(Some(2020)).and_months(3).and_month(Some(1)).new());
//!
//! // This crates party piece is the ability to calculate dates based on already existing chrono::DateTime
//! // If one would like to get the last day of the month that one is currently in, it could be done with:
//! println!("{}", Utc::now() + RelativeDelta::with_day(1).and_months(1).and_days(-1).new());
//! // Above first sets the day of the month to the 1st, then adds a month and subtracts a day.
//!
//! // One could also request the first monday after one year by
//! let first_monday_after_one_year = RelativeDelta::with_years(1).and_weekday(Some((Weekday::Mon, 1))).new();
//! let d = Utc.ymd(2020, 1, 1).and_hms(0,0,0) + first_monday_after_one_year;
//! assert_eq!(d, Utc.ymd(2021, 1, 4).and_hms(0,0,0));
//! ```

#[macro_use] extern crate impl_ops;

pub mod relativedelta;
pub use crate::relativedelta::RelativeDelta;

#[cfg(test)]
mod tests {
	use crate::relativedelta::RelativeDelta;
	use chrono::{TimeZone, Utc, Weekday};

	#[test]
	fn test_add_self() {
		let lhs = RelativeDelta::with_years(1).new();
		let rhs = RelativeDelta::with_years(2).new();

		assert_eq!(lhs + rhs, RelativeDelta::with_years(3).new());
		assert_eq!(lhs + rhs, rhs + lhs);
	}

	#[test]
	fn test_add() {
		let year = 2020;
		let month = 4;
		//let month2 = 3;
		//let months = -11;
		let day = 28;
		//let days = 31;
		let hour = 12;
		let min = 35;
		let sec = 48;
		//let n_secs : i64 = -11_111_111_111;
		let dt = Utc.ymd(year, month, day).and_hms(hour, min, sec);
//		let ddt = RelativeDelta::years(1)
//				.with_month(month2)
//				.with_months(months)
//				.with_days(days)
//				.with_nanoseconds(n_secs)
//				.new();

		let add_1_year = RelativeDelta::with_years(1).new();
		assert_eq!(
			dt + add_1_year,
			Utc.ymd(2021, month, day).and_hms(hour, min, sec)
		);

		let sub_1_year = RelativeDelta::with_years(-1).new();
		assert_eq!(
			dt + sub_1_year,
			Utc.ymd(2019, month, day).and_hms(hour, min, sec)
		);

		let set_year = RelativeDelta::with_year(2010).new();
		assert_eq!(
			dt + set_year,
			Utc.ymd(2010, month, day).and_hms(hour, min, sec)
		);

		let set_year = RelativeDelta::with_year(-1).new();
		assert_eq!(
			dt + set_year,
			Utc.ymd(-1, month, day).and_hms(hour, min, sec)
		);

		let add_69_months = RelativeDelta::with_months(69).new();
		// Expected after fix
		assert_eq!(add_69_months.years(), 5);
		assert_eq!(add_69_months.months(), 9);
		assert_eq!(
			dt + add_69_months,
			Utc.ymd(2026, 1, day).and_hms(hour, min, sec)
		);

		let sub_6_months = RelativeDelta::with_months(-6).new();
		assert_eq!(
			dt + sub_6_months,
			Utc.ymd(2019, 10, day).and_hms(hour, min, sec)
		);

		let sub_47_months = RelativeDelta::with_months(-47).new();
		// Expected after fix
		assert_eq!(sub_47_months.years(), -3);
		assert_eq!(sub_47_months.months(), -11);
		assert_eq!(
			dt + sub_47_months,
			Utc.ymd(2016, 5, day).and_hms(hour, min, sec)
		);

		let add_400_days = RelativeDelta::with_days(400).new();
		assert_eq!(
			dt + add_400_days,
			Utc.ymd(2021, 6, 2).and_hms(hour, min, sec)
		);

		let sub_400_days = RelativeDelta::with_days(-400).new();
		assert_eq!(
			dt + sub_400_days,
			Utc.ymd(2019, 3, 25).and_hms(hour, min, sec)
		);

		let monday = RelativeDelta::with_years(1).and_weekday(Some((Weekday::Mon, 1))).new();
		print!("{}", dt + monday);

		let pay1 = RelativeDelta::with_day(1)
				.with_days(-1)
				.with_month(Some(3))
				.with_months(1)
				.new();
		assert_eq!(dt + pay1, Utc.ymd(2020, 3, 31).and_hms(hour, min, sec));

		let pay2 = RelativeDelta::with_day(1)
				.with_days(-1)
				.with_month(Some(6))
				.with_months(1)
				.new();
		assert_eq!(dt + pay2, Utc.ymd(2020, 6, 30).and_hms(hour, min, sec));

		let pay3 = RelativeDelta::with_day(1)
				.with_days(-1)
				.with_month(Some(9))
				.with_months(1)
				.new();
		assert_eq!(dt + pay3, Utc.ymd(2020, 9, 30).and_hms(hour, min, sec));

		let pay4 = RelativeDelta::with_day(1)
				.with_days(-1)
				.with_month(Some(12))
				.with_months(1)
				.new();
		assert_eq!(dt + pay4, Utc.ymd(2020, 12, 31).and_hms(hour, min, sec));

	}

	#[test]
	fn test_mul() {
		let ddt = RelativeDelta::with_years(10)
				.and_months(6)
				.and_days(-15)
				.and_hours(23)
				.new();
		let r = ddt * 0.5_f64;


		assert_eq!(r, RelativeDelta::yysmmsdds(None, 5, None, 3, None, -7).and_minutes(-30).new());

		let rhs = RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 42, None, 0).new();
		assert_eq!(rhs * 0.5, RelativeDelta::with_years(2).and_year(Some(2020)).and_months(3).and_month(Some(1)).new());
	}

	#[test]
	fn test_init_with_float() {
		let ddt = RelativeDelta::ysmsdshsmsssns_f(1.5, -18.0, 0.0, 0.0, 0.0, 0.0, 0).new();
		assert_eq!(
			ddt,
			RelativeDelta::yysmmsdds(None, 0, None, 0, None, 0)
					.and_hhsmmssss(None, 0, None, 0, None, 0)
					.new()
		);

		let ddt = RelativeDelta::ysmsdshsmsssns_f(-0.42, -15.7, -12.3, -5.32, 3.14, 0.15, 22232).new();
		let r = RelativeDelta::with_years(-1).and_months(-8).and_months_f( -0.7399999999999984).and_days(-12).and_hours(-12).and_minutes(-28).and_seconds(-3).and_nanoseconds(-449977768).new();
		assert_eq!(ddt, r);
	}
}
