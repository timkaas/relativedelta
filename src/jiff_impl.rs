// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use jiff::civil::{Date, DateTime, Time};
use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp, ZonedDateTime};
use num_integer::Integer;

use crate::from_error::FromError;
use crate::relativedelta::MonthsType;
use crate::shift::sealed::Shift;
use crate::{RelativeDelta, Weekday};

/// Returns the absolute field if set, else the field of the source, as jiff's `i8`.
fn field(absolute: Option<u8>, source: i8) -> Option<i8> {
	absolute.map_or(Some(source), |value| i8::try_from(value).ok())
}

/// Returns `days` 24-hour days as an exact duration, or `None` on overflow.
fn days(days: i64) -> Option<SignedDuration> {
	days.checked_mul(86_400).map(SignedDuration::from_secs)
}

/// Returns `source` with the delta's absolute date fields applied and its years and months added,
/// the day clamped to the length of the resulting month.
fn anchor_date(delta: &RelativeDelta, source: Date) -> Option<Date> {
	let year = delta
		.year()
		.unwrap_or(source.year().into())
		.checked_add(delta.years())?;
	let month = delta
		.month()
		.map_or(source.month().into(), MonthsType::from)
		.checked_add(delta.months())?;
	let (extra_years, month_offset) = (month - 1).div_mod_floor(&12);
	let year = i16::try_from(year.checked_add(extra_years)?).ok()?;
	let month = i8::try_from(month_offset + 1).ok()?;
	let days_in_month = Date::new(year, month, 1).ok()?.days_in_month();
	let day = field(delta.day(), source.day())?.min(days_in_month);
	Date::new(year, month, day).ok()
}

/// Returns `source` with the delta's absolute time fields applied.
fn anchor_time(delta: &RelativeDelta, source: Time) -> Option<Time> {
	let nanosecond = delta
		.nanosecond()
		.map_or(Some(source.subsec_nanosecond()), |value| {
			i32::try_from(value).ok()
		})?;
	Time::new(
		field(delta.hour(), source.hour())?,
		field(delta.minute(), source.minute())?,
		field(delta.second(), source.second())?,
		nanosecond,
	)
	.ok()
}

/// Returns `source` with the delta's absolute fields applied and its years and months added.
fn anchor_datetime(delta: &RelativeDelta, source: DateTime) -> Option<DateTime> {
	Some(DateTime::from_parts(
		anchor_date(delta, source.date())?,
		anchor_time(delta, source.time())?,
	))
}

/// Returns the delta's days through nanoseconds as one exact duration, or `None` on overflow.
fn exact_duration(delta: &RelativeDelta) -> Option<SignedDuration> {
	let seconds = delta
		.days()
		.checked_mul(86_400)?
		.checked_add(delta.hours().checked_mul(3_600)?)?
		.checked_add(delta.minutes().checked_mul(60)?)?
		.checked_add(delta.seconds())?;
	SignedDuration::from_secs(seconds).checked_add(SignedDuration::from_nanos(delta.nanoseconds()))
}

/// Returns the exact duration from a date falling on `current` to the delta's weekday, which is
/// zero if the delta has none.
fn weekday_jump(delta: &RelativeDelta, current: jiff::civil::Weekday) -> Option<SignedDuration> {
	let Some((weekday, nth)) = delta.weekday()
	else {
		return Some(SignedDuration::ZERO);
	};
	// The Monday-zero offset is in 0..=6, so its absolute value is the value itself.
	let current = Weekday::from(current.to_monday_zero_offset().unsigned_abs());
	days(weekday.jump_days_from(current, nth))
}

impl Shift for Date {
	/// Uses only the date fields and the days of the delta, like `chrono::NaiveDate`.
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		let date = anchor_date(delta, *self)?
			.checked_add(days(delta.days())?)
			.ok()?;
		date.checked_add(weekday_jump(delta, date.weekday())?).ok()
	}
}

impl Shift for DateTime {
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		let datetime = anchor_datetime(delta, *self)?
			.checked_add(exact_duration(delta)?)
			.ok()?;
		datetime
			.checked_add(weekday_jump(delta, datetime.weekday())?)
			.ok()
	}
}

impl Shift for Timestamp {
	/// Works on the UTC calendar date and time of the timestamp, like `chrono::DateTime<Utc>`.
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		let datetime = TimeZone::UTC.to_datetime(*self).checked_shift(delta)?;
		TimeZone::UTC.to_timestamp(datetime).ok()
	}
}

impl Shift for ZonedDateTime {
	/// Applies the absolute fields, years and months to the local date and time, then adds days
	/// through nanoseconds and the weekday jump as exact durations, like `chrono::DateTime<Tz>`.
	/// Returns `None` if the local date and time falls in a gap or fold of the time zone.
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		let local = anchor_datetime(delta, self.datetime())?;
		let zoned = self
			.time_zone()
			.to_ambiguous_zoned_datetime(local)
			.unambiguous()
			.ok()?
			.checked_add(exact_duration(delta)?)
			.ok()?;
		let jump = weekday_jump(delta, zoned.weekday())?;
		zoned.checked_add(jump).ok()
	}
}

impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &Date| -> Date { lhs.checked_add(rhs).expect("could not add RelativeDelta to date") });
impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &DateTime| -> DateTime { lhs.checked_add(rhs).expect("could not add RelativeDelta to datetime") });
impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &Timestamp| -> Timestamp { lhs.checked_add(rhs).expect("could not add RelativeDelta to timestamp") });
impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &ZonedDateTime| -> ZonedDateTime { lhs.checked_add(rhs).expect("could not add RelativeDelta to zoned datetime") });

impl_op_ex!(-|lhs: &Date, rhs: &RelativeDelta| -> Date { lhs + (-rhs) });
impl_op_ex!(-|lhs: &DateTime, rhs: &RelativeDelta| -> DateTime { lhs + (-rhs) });
impl_op_ex!(-|lhs: &Timestamp, rhs: &RelativeDelta| -> Timestamp { lhs + (-rhs) });
impl_op_ex!(-|lhs: &ZonedDateTime, rhs: &RelativeDelta| -> ZonedDateTime { lhs + (-rhs) });

impl TryFrom<RelativeDelta> for DateTime {
	type Error = FromError;

	fn try_from(delta: RelativeDelta) -> Result<Self, Self::Error> {
		let year = delta.year().ok_or(FromError::MissingYear)?;
		let month = delta.month().ok_or(FromError::MissingMonth)?;
		let day = delta.day().ok_or(FromError::MissingDay)?;

		let date = Date::new(
			i16::try_from(year).map_err(|_| FromError::InvalidDateComponents)?,
			i8::try_from(month).map_err(|_| FromError::InvalidDateComponents)?,
			i8::try_from(day).map_err(|_| FromError::InvalidDateComponents)?,
		)
		.map_err(|_| FromError::InvalidDateComponents)?;
		let time = anchor_time(&delta, Time::midnight()).ok_or(FromError::InvalidTimeComponents)?;
		Ok(date.to_datetime(time))
	}
}

#[cfg(test)]
mod tests {
	use jiff::civil::{Date, DateTime, date};
	use jiff::tz::{self, TimeZone};
	use jiff::{Timestamp, ZonedDateTime};
	use similar_asserts::assert_eq;

	use crate::{RelativeDelta, Weekday};

	/// 2020-04-28 12:35:48, the start of the chrono backend's test cases.
	fn start() -> DateTime {
		date(2020, 4, 28).at(12, 35, 48, 0)
	}

	fn at_start_time(year: i16, month: i8, day: i8) -> DateTime {
		date(year, month, day).at(12, 35, 48, 0)
	}

	fn utc(datetime: DateTime) -> Timestamp {
		TimeZone::UTC.to_timestamp(datetime).unwrap()
	}

	fn zoned(datetime: DateTime) -> ZonedDateTime {
		datetime
			.to_zoned_datetime(TimeZone::fixed(tz::offset(2)))
			.unwrap()
	}

	/// The time zone of New York, whose 2024 DST gap is 2024-03-10 02:00..03:00 and whose fold is
	/// 2024-11-03 01:00..02:00.
	fn new_york() -> TimeZone {
		TimeZone::posix("EST5EDT,M3.2.0,M11.1.0").unwrap()
	}

	/// Asserts that adding `delta` to `start()` gives `expected` for every jiff date-time type.
	fn assert_add(delta: &RelativeDelta, expected: DateTime) {
		assert_eq!(start() + delta, expected, "DateTime + {delta:?}");
		assert_eq!(utc(start()) + delta, utc(expected), "Timestamp + {delta:?}");
		assert_eq!(
			zoned(start()) + delta,
			zoned(expected),
			"ZonedDateTime + {delta:?}"
		);
	}

	/// Asserts that subtracting `delta` from `start()` gives `expected` for every jiff date-time type.
	fn assert_sub(delta: &RelativeDelta, expected: DateTime) {
		assert_eq!(start() - delta, expected, "DateTime - {delta:?}");
		assert_eq!(utc(start()) - delta, utc(expected), "Timestamp - {delta:?}");
		assert_eq!(
			zoned(start()) - delta,
			zoned(expected),
			"ZonedDateTime - {delta:?}"
		);
	}

	#[test]
	fn test_add() {
		let quarter_end = |month| {
			RelativeDelta::with_day(1)
				.and_days(-1)
				.and_month(Some(month))
				.and_months(1)
				.build()
		};
		let cases = [
			(
				RelativeDelta::with_years(1).build(),
				at_start_time(2021, 4, 28),
			),
			(
				RelativeDelta::with_years(-1).build(),
				at_start_time(2019, 4, 28),
			),
			(
				RelativeDelta::with_year(2010).build(),
				at_start_time(2010, 4, 28),
			),
			(
				RelativeDelta::with_year(-1).build(),
				at_start_time(-1, 4, 28),
			),
			(
				RelativeDelta::with_months(69).build(),
				at_start_time(2026, 1, 28),
			),
			(
				RelativeDelta::with_months(-6).build(),
				at_start_time(2019, 10, 28),
			),
			(
				RelativeDelta::with_months(-47).build(),
				at_start_time(2016, 5, 28),
			),
			(
				RelativeDelta::with_days(400).build(),
				at_start_time(2021, 6, 2),
			),
			(
				RelativeDelta::with_days(-400).build(),
				at_start_time(2019, 3, 25),
			),
			(
				RelativeDelta::with_years(1)
					.and_weekday(Some((Weekday::Mon, 1)))
					.build(),
				at_start_time(2021, 5, 3),
			),
			(
				RelativeDelta::with_years(1)
					.and_weekday(Some((Weekday::Mon, -1)))
					.build(),
				at_start_time(2021, 4, 26),
			),
			(quarter_end(3), at_start_time(2020, 3, 31)),
			(quarter_end(6), at_start_time(2020, 6, 30)),
			(quarter_end(9), at_start_time(2020, 9, 30)),
			(quarter_end(12), at_start_time(2020, 12, 31)),
			(
				RelativeDelta::with_hours(1).and_nanoseconds(5).build(),
				date(2020, 4, 28).at(13, 35, 48, 5),
			),
			(
				RelativeDelta::with_hour(0)
					.and_minute(Some(0))
					.and_second(Some(0))
					.build(),
				date(2020, 4, 28).at(0, 0, 0, 0),
			),
		];
		for (delta, expected) in &cases {
			assert_add(delta, *expected);
		}
	}

	#[test]
	fn test_sub() {
		let cases = [
			(
				RelativeDelta::with_years(1).build(),
				at_start_time(2019, 4, 28),
			),
			(
				RelativeDelta::with_years(-1).build(),
				at_start_time(2021, 4, 28),
			),
			(
				RelativeDelta::with_year(2010).build(),
				at_start_time(2010, 4, 28),
			),
			(
				RelativeDelta::with_year(-1).build(),
				at_start_time(-1, 4, 28),
			),
			(
				RelativeDelta::with_months(-6).build(),
				at_start_time(2020, 10, 28),
			),
			(
				RelativeDelta::with_months(-47).build(),
				at_start_time(2024, 3, 28),
			),
		];
		for (delta, expected) in &cases {
			assert_sub(delta, *expected);
		}
	}

	#[test]
	#[allow(
		clippy::op_ref,
		reason = "exercises every owned and borrowed operator impl"
	)]
	fn test_operators_take_owned_and_borrowed_operands() {
		let delta = RelativeDelta::with_years(1).build();
		let expected = at_start_time(2021, 4, 28);
		let start = start();
		assert_eq!(delta + start, expected);
		assert_eq!(&delta + start, expected);
		assert_eq!(delta + &start, expected);
		assert_eq!(&delta + &start, expected);
		assert_eq!(start + delta, expected);
		assert_eq!(&start + &delta, expected);
		assert_eq!(expected - delta, start);
		assert_eq!(&expected - &delta, start);

		let zoned_start = zoned(start);
		assert_eq!(&zoned_start + &delta, zoned(expected));
		assert_eq!(zoned_start + delta, zoned(expected));
	}

	#[test]
	fn test_date() {
		let date_ = date(2020, 4, 28);
		assert_eq!(
			date_ + RelativeDelta::with_years(1).build(),
			date(2021, 4, 28)
		);
		assert_eq!(
			date_ - RelativeDelta::with_years(1).build(),
			date(2019, 4, 28)
		);
		assert_eq!(
			date_ - RelativeDelta::with_years(-1).build(),
			date(2021, 4, 28)
		);
		assert_eq!(
			date_ + RelativeDelta::with_months(1).build(),
			date(2020, 5, 28)
		);
		assert_eq!(
			date_ + RelativeDelta::with_days(1).build(),
			date(2020, 4, 29)
		);
		assert_eq!(date_ + RelativeDelta::with_day(1).build(), date(2020, 4, 1));
		// A date only takes the date fields and days of a delta, like `chrono::NaiveDate`.
		assert_eq!(date_ + RelativeDelta::with_hours(1).build(), date_);
		assert_eq!(
			date_.at(12, 35, 48, 0) + RelativeDelta::with_hours(1).build(),
			date_.at(13, 35, 48, 0)
		);
	}

	#[test]
	fn test_month_end_is_clamped() {
		let next_month = RelativeDelta::with_months(1).build();
		assert_eq!(date(2024, 1, 31) + next_month, date(2024, 2, 29));
		assert_eq!(date(2023, 1, 31) + next_month, date(2023, 2, 28));
		assert_eq!(
			date(2024, 2, 29) + RelativeDelta::with_years(1).build(),
			date(2025, 2, 28)
		);
		assert_eq!(
			date(2020, 4, 30) + RelativeDelta::with_day(31).build(),
			date(2020, 4, 30)
		);
	}

	#[test]
	#[cfg(feature = "serde")]
	fn test_dateutil_mix_of_absolute_and_relative_fields() -> anyhow::Result<()> {
		// Set the day to the 1st and the month to March, add a month, then go back a day.
		let last_of_march: RelativeDelta =
			serde_json::from_str(r#"{"months": 1, "days": -1, "month": 3, "day": 1}"#)?;
		assert_eq!(date(2020, 4, 28) + last_of_march, date(2020, 3, 31));
		assert_eq!(date(2021, 1, 15) + last_of_march, date(2021, 3, 31));
		assert_eq!(
			utc(start()) + last_of_march,
			utc(at_start_time(2020, 3, 31))
		);
		Ok(())
	}

	#[test]
	fn test_weekday_follows_dateutil() {
		let monday = date(2021, 5, 3);
		let with_weekday = |weekday, nth| RelativeDelta::with_weekday(weekday, nth).build();
		// The date itself is the first occurrence of its own weekday, in either direction.
		assert_eq!(monday + with_weekday(Weekday::Mon, 1), monday);
		assert_eq!(monday + with_weekday(Weekday::Mon, -1), monday);
		assert_eq!(monday + with_weekday(Weekday::Mon, 2), date(2021, 5, 10));
		assert_eq!(monday + with_weekday(Weekday::Mon, 0), monday);
		assert_eq!(monday + with_weekday(Weekday::Tue, 1), date(2021, 5, 4));
		assert_eq!(monday + with_weekday(Weekday::Tue, -1), date(2021, 4, 27));
		assert_eq!(monday + with_weekday(Weekday::Sun, -2), date(2021, 4, 25));
		assert_eq!(
			monday.at(9, 0, 0, 0) + with_weekday(Weekday::Fri, 1),
			date(2021, 5, 7).at(9, 0, 0, 0)
		);
	}

	#[test]
	fn test_try_from() -> anyhow::Result<()> {
		let delta = RelativeDelta::with_year(2020)
			.and_month(Some(4))
			.and_day(Some(28))
			.and_hour(Some(12))
			.and_minute(Some(35))
			.and_second(Some(48))
			.build();
		assert_eq!(DateTime::try_from(delta)?, start());

		let date_only = RelativeDelta::with_year(2020)
			.and_month(Some(4))
			.and_day(Some(28))
			.build();
		assert_eq!(
			DateTime::try_from(date_only)?,
			date(2020, 4, 28).at(0, 0, 0, 0)
		);

		assert!(DateTime::try_from(RelativeDelta::with_year(2020).build()).is_err());
		assert!(
			DateTime::try_from(
				RelativeDelta::with_year(20_000)
					.and_month(Some(1))
					.and_day(Some(1))
					.build()
			)
			.is_err()
		);
		Ok(())
	}

	#[test]
	fn test_checked_add_returns_none_out_of_range() {
		let one_day = RelativeDelta::with_days(1).build();
		assert_eq!(one_day.checked_add(&Date::MAX), None);
		assert_eq!(one_day.checked_add(&DateTime::MAX), None);
		assert_eq!(one_day.checked_add(&Timestamp::MAX), None);
		assert_eq!(
			RelativeDelta::with_year(10_000)
				.build()
				.checked_add(&start()),
			None
		);
		assert_eq!(
			RelativeDelta::with_days(i64::MAX)
				.build()
				.checked_add(&start()),
			None
		);
		assert_eq!(
			RelativeDelta::with_weekday(Weekday::Mon, i64::MAX)
				.build()
				.checked_add(&start()),
			None
		);
	}

	#[test]
	fn test_zoned_local_time_in_gap_or_fold_is_none() {
		let one_month = RelativeDelta::with_months(1).build();
		let before_gap = date(2024, 2, 10)
			.at(2, 30, 0, 0)
			.to_zoned_datetime(new_york())
			.unwrap();
		assert_eq!(one_month.checked_add(&before_gap), None);

		let before_fold = date(2024, 10, 3)
			.at(1, 30, 0, 0)
			.to_zoned_datetime(new_york())
			.unwrap();
		assert_eq!(one_month.checked_add(&before_fold), None);
	}

	#[test]
	fn test_zoned_days_are_exact_24_hours() {
		// Like `chrono::DateTime<Tz>`, days through nanoseconds are an exact duration, so a day across
		// the start of DST lands an hour later on the wall clock.
		let noon = date(2024, 3, 9)
			.at(12, 0, 0, 0)
			.to_zoned_datetime(new_york())
			.unwrap();
		let expected = date(2024, 3, 10)
			.at(13, 0, 0, 0)
			.to_zoned_datetime(new_york())
			.unwrap();
		assert_eq!(noon + RelativeDelta::with_days(1).build(), expected);
	}
}
