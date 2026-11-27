// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use jiff::civil::{Date, DateTime, Time};
use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp, ZonedDateTime};
use num_integer::Integer;

use crate::relativedelta::MonthsType;
use crate::shift::sealed::Shift;
use crate::{ConversionError, RelativeDelta, Weekday};

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
	let year = year.checked_add(extra_years)?;
	let year = i16::try_from(year).ok()?;
	let month = i8::try_from(month_offset + 1).ok()?;
	let first_of_month = Date::new(year, month, 1).ok()?;
	let day = field(delta.day(), source.day())?;
	let day = day.min(first_of_month.days_in_month());
	Date::new(year, month, day).ok()
}

/// Returns `source` with the delta's absolute time fields applied.
fn anchor_time(delta: &RelativeDelta, source: Time) -> Option<Time> {
	let nanosecond = delta
		.nanosecond()
		.map_or(Some(source.subsec_nanosecond()), |value| {
			i32::try_from(value).ok()
		})?;
	let hour = field(delta.hour(), source.hour())?;
	let minute = field(delta.minute(), source.minute())?;
	let second = field(delta.second(), source.second())?;
	Time::new(hour, minute, second, nanosecond).ok()
}

/// Returns `source` with the delta's absolute fields applied and its years and months added.
fn anchor_datetime(delta: &RelativeDelta, source: DateTime) -> Option<DateTime> {
	let date = anchor_date(delta, source.date())?;
	let time = anchor_time(delta, source.time())?;
	Some(DateTime::from_parts(date, time))
}

/// Returns the delta's days through nanoseconds as one exact duration, or `None` on overflow.
fn exact_duration(delta: &RelativeDelta) -> Option<SignedDuration> {
	let day_seconds = delta.days().checked_mul(86_400)?;
	let hour_seconds = delta.hours().checked_mul(3_600)?;
	let minute_seconds = delta.minutes().checked_mul(60)?;
	let seconds = day_seconds.checked_add(hour_seconds)?;
	let seconds = seconds.checked_add(minute_seconds)?;
	let seconds = seconds.checked_add(delta.seconds())?;
	SignedDuration::from_secs(seconds).checked_add(SignedDuration::from_nanos(delta.nanoseconds()))
}

/// Returns the exact duration from a date falling on `current` to the delta's weekday, which is
/// zero if the delta has none.
fn weekday_jump(delta: &RelativeDelta, current: jiff::civil::Weekday) -> Option<SignedDuration> {
	let Some((weekday, nth)) = delta.weekday()
	else {
		return Some(SignedDuration::ZERO);
	};
	days(weekday.jump_days_from(Weekday::from(current), nth))
}

/// Converts jiff's weekday to the same day.
impl From<jiff::civil::Weekday> for Weekday {
	fn from(weekday: jiff::civil::Weekday) -> Self {
		match weekday {
			jiff::civil::Weekday::Monday => Self::Mon,
			jiff::civil::Weekday::Tuesday => Self::Tue,
			jiff::civil::Weekday::Wednesday => Self::Wed,
			jiff::civil::Weekday::Thursday => Self::Thu,
			jiff::civil::Weekday::Friday => Self::Fri,
			jiff::civil::Weekday::Saturday => Self::Sat,
			jiff::civil::Weekday::Sunday => Self::Sun,
		}
	}
}

impl Shift for Date {
	/// Uses only the date fields and the days of the delta, like `chrono::NaiveDate`.
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		let anchored = anchor_date(delta, *self)?;
		let days = days(delta.days())?;
		let date = anchored.checked_add(days).ok()?;
		let jump = weekday_jump(delta, date.weekday())?;
		date.checked_add(jump).ok()
	}
}

impl Shift for DateTime {
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		let anchored = anchor_datetime(delta, *self)?;
		let duration = exact_duration(delta)?;
		let datetime = anchored.checked_add(duration).ok()?;
		let jump = weekday_jump(delta, datetime.weekday())?;
		datetime.checked_add(jump).ok()
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
		let anchored = self
			.time_zone()
			.to_ambiguous_zoned_datetime(local)
			.unambiguous()
			.ok()?;
		let duration = exact_duration(delta)?;
		let zoned = anchored.checked_add(duration).ok()?;
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
	type Error = ConversionError;

	fn try_from(delta: RelativeDelta) -> Result<Self, Self::Error> {
		let year = delta.year().ok_or(ConversionError::MissingYear)?;
		let month = delta.month().ok_or(ConversionError::MissingMonth)?;
		let day = delta.day().ok_or(ConversionError::MissingDay)?;

		let year = i16::try_from(year).map_err(|_| ConversionError::InvalidDate)?;
		let month = i8::try_from(month).map_err(|_| ConversionError::InvalidDate)?;
		let day = i8::try_from(day).map_err(|_| ConversionError::InvalidDate)?;
		let date = Date::new(year, month, day).map_err(|_| ConversionError::InvalidDate)?;
		let time = anchor_time(&delta, Time::midnight()).ok_or(ConversionError::InvalidTime)?;
		Ok(date.to_datetime(time))
	}
}

#[cfg(test)]
mod tests {
	use jiff::Timestamp;
	use jiff::civil::{Date, DateTime, date};
	use jiff::tz::TimeZone;
	use similar_asserts::assert_eq;

	use crate::RelativeDelta;

	/// 2020-04-28 12:35:48.
	fn start() -> DateTime {
		date(2020, 4, 28).at(12, 35, 48, 0)
	}

	/// The time zone of New York, whose 2024 DST gap is 2024-03-10 02:00..03:00 and whose fold is
	/// 2024-11-03 01:00..02:00.
	fn new_york() -> TimeZone {
		TimeZone::posix("EST5EDT,M3.2.0,M11.1.0").unwrap()
	}

	#[test]
	#[cfg(feature = "serde")]
	fn test_dateutil_mix_of_absolute_and_relative_fields() -> anyhow::Result<()> {
		// Set the day to the 1st and the month to March, add a month, then go back a day.
		let last_of_march: RelativeDelta =
			serde_json::from_str(r#"{"months": 1, "days": -1, "month": 3, "day": 1}"#)?;
		assert_eq!(date(2020, 4, 28) + last_of_march, date(2020, 3, 31));
		assert_eq!(date(2021, 1, 15) + last_of_march, date(2021, 3, 31));
		Ok(())
	}

	#[test]
	fn test_try_from() -> anyhow::Result<()> {
		let delta = RelativeDelta::with_year(2020)
			.and_month(4)
			.and_day(28)
			.and_hour(12)
			.and_minute(35)
			.and_second(48)
			.build();
		assert_eq!(DateTime::try_from(delta)?, start());

		let date_only = RelativeDelta::with_year(2020)
			.and_month(4)
			.and_day(28)
			.build();
		assert_eq!(
			DateTime::try_from(date_only)?,
			date(2020, 4, 28).at(0, 0, 0, 0)
		);

		assert!(DateTime::try_from(RelativeDelta::with_year(2020).build()).is_err());
		assert!(
			DateTime::try_from(
				RelativeDelta::with_year(20_000)
					.and_month(1)
					.and_day(1)
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

	#[test]
	fn test_weekday_from_jiff() {
		let mut day = jiff::civil::Weekday::Monday;
		for _ in 0..7 {
			let converted = crate::Weekday::from(day);
			let expected = day.to_monday_zero_offset().unsigned_abs();
			assert_eq!(converted.num_days_from_monday(), expected);
			day = day.next();
		}
	}
}
