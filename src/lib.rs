// This is a part of relativedelta.
// See README.md and LICENSE.txt for details.

//! # relativedelta: Relative Date and Time for Rust
//!
//! ## Usage
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! relativedelta = "0.1"
//! ```

#[macro_use]
extern crate impl_ops;

pub mod relativedelta;

#[cfg(test)]
mod tests {
	use crate::relativedelta::DateTime;
	use chrono::{TimeZone, Utc};

	#[test]
	fn test_add() {
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
		let ddt = DateTime::years(1)
				.with_month(month2)
				.with_months(months)
				.with_days(days)
				.with_nanoseconds(n_secs)
				.new();

		let add_1_year = DateTime::years(1).new();
		assert_eq!(
			dt + add_1_year,
			Utc.ymd(2021, month, day).and_hms(hour, min, sec)
		);

		let sub_1_year = DateTime::years(-1).new();
		assert_eq!(
			dt + sub_1_year,
			Utc.ymd(2019, month, day).and_hms(hour, min, sec)
		);

		let set_year = DateTime::year(2010).new();
		assert_eq!(
			dt + set_year,
			Utc.ymd(2010, month, day).and_hms(hour, min, sec)
		);

		let set_year = DateTime::year(-1).new();
		assert_eq!(
			dt + set_year,
			Utc.ymd(-1, month, day).and_hms(hour, min, sec)
		);

		let add_69_months = DateTime::months(69).new();
		// Expected after fix
		assert_eq!(add_69_months.years, 5);
		assert_eq!(add_69_months.months, 9);
		assert_eq!(
			dt + add_69_months,
			Utc.ymd(2026, 1, day).and_hms(hour, min, sec)
		);

		let sub_6_months = DateTime::months(-6).new();
		assert_eq!(
			dt + sub_6_months,
			Utc.ymd(2019, 10, day).and_hms(hour, min, sec)
		);

		let sub_47_months = DateTime::months(-47).new();
		// Expected after fix
		assert_eq!(sub_47_months.years, -3);
		assert_eq!(sub_47_months.months, -11);
		assert_eq!(
			dt + sub_47_months,
			Utc.ymd(2016, 5, day).and_hms(hour, min, sec)
		);

		let add_400_days = DateTime::days(400).new();
		assert_eq!(
			dt + add_400_days,
			Utc.ymd(2021, 6, 2).and_hms(hour, min, sec)
		);

		let sub_400_days = DateTime::days(-400).new();
		assert_eq!(
			dt + sub_400_days,
			Utc.ymd(2019, 3, 25).and_hms(hour, min, sec)
		);

		let pay1 = DateTime::day(1)
				.with_days(-1)
				.with_month(3)
				.with_months(1)
				.new();
		assert_eq!(dt + pay1, Utc.ymd(2020, 3, 31).and_hms(hour, min, sec));

		let pay2 = DateTime::day(1)
				.with_days(-1)
				.with_month(6)
				.with_months(1)
				.new();
		assert_eq!(dt + pay2, Utc.ymd(2020, 6, 30).and_hms(hour, min, sec));

		let pay3 = DateTime::day(1)
				.with_days(-1)
				.with_month(9)
				.with_months(1)
				.new();
		assert_eq!(dt + pay3, Utc.ymd(2020, 9, 30).and_hms(hour, min, sec));

		let pay4 = DateTime::day(1)
				.with_days(-1)
				.with_month(12)
				.with_months(1)
				.new();
		assert_eq!(dt + pay4, Utc.ymd(2020, 12, 31).and_hms(hour, min, sec));
	}

	#[test]
	fn test_mul() {
		let ddt = DateTime::years(10)
				.and_months(6)
				.and_days(-15)
				.and_hours(23)
				.new();
		let r = ddt * 0.5_f64;


		assert_eq!(r, DateTime::yysmmsdds(None, 5, None, 3, None, -7).and_minutes(-30).new());
	}

	#[test]
	fn test_init_with_float() {
		let ddt = DateTime::ysmsdshsmsssns_f(1.5, -18.0, 0.0, 0.0, 0.0, 0.0, 0).new();
		assert_eq!(
			ddt,
			DateTime::yysmmsdds(None, 0, None, 0, None, 0)
					.and_hhsmmssss(None, 0, None, 0, None, 0)
					.new()
		);

		let ddt = DateTime::ysmsdshsmsssns_f(-0.42, -15.7, -12.3, -5.32, 3.14, 0.15, 22232).new();
		let r = DateTime { years: -1, months: -8, months_f: -0.7399999999999984, days: -12, hours: -12, minutes: -28, seconds: -3, nanoseconds: -449977768, year: None, month: None, day: None, hour: None, minute: None, second: None, nanosecond: None, weekday: None };
		assert_eq!(ddt, r);
	}
}
