// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use num_integer::Integer;

use crate::relativedelta::{MonthType, MonthsType, num_days_in_month};
use crate::shift::sealed::Shift;
use crate::{ConversionError, RelativeDelta, Weekday};

/// Returns `source` with the delta's absolute date fields applied and its years and months added,
/// the day clamped to the length of the resulting month.
fn anchor_date(delta: &RelativeDelta, source: time::Date) -> Option<time::Date> {
	let year = delta
		.year()
		.unwrap_or(source.year().into())
		.checked_add(delta.years())?;
	let month = delta
		.month()
		.map_or(MonthsType::from(u8::from(source.month())), MonthsType::from)
		.checked_add(delta.months())?;
	let (extra_years, month_offset) = (month - 1).div_mod_floor(&12);
	let year = year.checked_add(extra_years)?;
	let month = MonthType::try_from(month_offset + 1).ok()?;
	let day = delta
		.day()
		.unwrap_or(source.day())
		.min(num_days_in_month(year, month));
	time::Date::from_calendar_date(
		year.try_into().ok()?,
		time::Month::try_from(month).ok()?,
		day,
	)
	.ok()
}

/// Returns `days` as an exact duration, or `None` on overflow.
fn days(days: i64) -> Option<time::Duration> {
	Some(time::Duration::seconds(days.checked_mul(86_400)?))
}

/// Returns the exact duration from a date falling on `current` to the delta's weekday, which is
/// zero if the delta has none.
fn weekday_jump(delta: &RelativeDelta, current: time::Weekday) -> Option<time::Duration> {
	let Some((weekday, nth)) = delta.weekday()
	else {
		return Some(time::Duration::ZERO);
	};
	days(weekday.jump_days_from(Weekday::from(current), nth))
}

/// Adds `delta` to `rhs`, using only the date fields and the days of the delta, like
/// `chrono::NaiveDate`.
fn add_date(delta: &RelativeDelta, rhs: time::Date) -> Option<time::Date> {
	let anchored = anchor_date(delta, rhs)?;
	let days = days(delta.days())?;
	let date = anchored.checked_add(days)?;
	let jump = weekday_jump(delta, date.weekday())?;
	date.checked_add(jump)
}

/// Defines `$fname`, which adds a delta to a `$datetime`; `$ctor` builds the result from the new
/// date, the new wall-clock time and the source, whose offset it keeps.
macro_rules! impl_add {
	($fname:ident, $datetime:ty, $ctor:expr) => {
		fn $fname(lhs: &RelativeDelta, rhs: &$datetime) -> Option<$datetime> {
			let date = anchor_date(lhs, rhs.date())?;
			let time = time::Time::from_hms_nano(
				lhs.hour().unwrap_or(rhs.hour()),
				lhs.minute().unwrap_or(rhs.minute()),
				lhs.second().unwrap_or(rhs.second()),
				lhs.nanosecond().unwrap_or(rhs.nanosecond()),
			)
			.ok()?;
			let hour_seconds = lhs.hours().checked_mul(3_600)?;
			let minute_seconds = lhs.minutes().checked_mul(60)?;
			let seconds = hour_seconds.checked_add(minute_seconds)?;
			let seconds = seconds.checked_add(lhs.seconds())?;
			let days = days(lhs.days())?;
			let anchored = $ctor(date, time, rhs);
			let ret = anchored.checked_add(days)?;
			let ret = ret.checked_add(time::Duration::seconds(seconds))?;
			let ret = ret.checked_add(time::Duration::nanoseconds(lhs.nanoseconds()))?;
			let jump = weekday_jump(lhs, ret.weekday())?;
			ret.checked_add(jump)
		}
	};
}

impl_add!(add_primitive, time::PrimitiveDateTime, |date, time, _| {
	time::PrimitiveDateTime::new(date, time)
});
impl_add!(
	add_offset,
	time::OffsetDateTime,
	|date, time, rhs: &time::OffsetDateTime| {
		time::OffsetDateTime::new_in_offset(date, time, rhs.offset())
	}
);
impl_add!(add_utc, time::UtcDateTime, |date, time, _| {
	time::UtcDateTime::new(date, time)
});

impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &time::Date| -> time::Date { add_date(lhs, *rhs).expect("could not add RelativeDelta to date") });
impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &time::PrimitiveDateTime| -> time::PrimitiveDateTime { add_primitive(lhs, rhs).expect("could not add RelativeDelta to datetime") });
impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &time::OffsetDateTime| -> time::OffsetDateTime { add_offset(lhs, rhs).expect("could not add RelativeDelta to datetime") });
impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &time::UtcDateTime| -> time::UtcDateTime { add_utc(lhs, rhs).expect("could not add RelativeDelta to datetime") });

impl RelativeDelta {
	/// Returns `rhs` moved by this delta, or `None` if the result is not representable.
	#[must_use]
	#[deprecated(
		since = "1.0.0",
		note = "use `RelativeDelta::checked_add`, which takes every supported type"
	)]
	pub fn checked_add_primitive_datetime(
		&self,
		rhs: &time::PrimitiveDateTime,
	) -> Option<time::PrimitiveDateTime> {
		self.checked_add(rhs)
	}

	/// Returns `rhs` moved by this delta, or `None` if the result is not representable.
	#[must_use]
	#[deprecated(
		since = "1.0.0",
		note = "use `RelativeDelta::checked_add`, which takes every supported type"
	)]
	pub fn checked_add_offset_datetime(
		&self,
		rhs: &time::OffsetDateTime,
	) -> Option<time::OffsetDateTime> {
		self.checked_add(rhs)
	}

	/// Returns `rhs` moved by this delta, or `None` if the result is not representable.
	#[must_use]
	#[deprecated(
		since = "1.0.0",
		note = "use `RelativeDelta::checked_add`, which takes every supported type"
	)]
	pub fn checked_add_utc_datetime(&self, rhs: &time::UtcDateTime) -> Option<time::UtcDateTime> {
		self.checked_add(rhs)
	}
}

impl_op_ex!(
	-|lhs: &time::PrimitiveDateTime, rhs: &RelativeDelta| -> time::PrimitiveDateTime { lhs + (-rhs) }
);
impl_op_ex!(
	-|lhs: &time::OffsetDateTime, rhs: &RelativeDelta| -> time::OffsetDateTime { lhs + (-rhs) }
);
impl_op_ex!(-|lhs: &time::Date, rhs: &RelativeDelta| -> time::Date { lhs + (-rhs) });
impl_op_ex!(-|lhs: &time::UtcDateTime, rhs: &RelativeDelta| -> time::UtcDateTime { lhs + (-rhs) });

/// Converts time's weekday to the same day.
impl From<time::Weekday> for Weekday {
	fn from(weekday: time::Weekday) -> Self {
		match weekday {
			time::Weekday::Monday => Self::Mon,
			time::Weekday::Tuesday => Self::Tue,
			time::Weekday::Wednesday => Self::Wed,
			time::Weekday::Thursday => Self::Thu,
			time::Weekday::Friday => Self::Fri,
			time::Weekday::Saturday => Self::Sat,
			time::Weekday::Sunday => Self::Sun,
		}
	}
}

impl Shift for time::Date {
	/// Uses only the date fields and the days of the delta, like `chrono::NaiveDate`.
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		add_date(delta, *self)
	}
}

impl Shift for time::PrimitiveDateTime {
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		add_primitive(delta, self)
	}
}

impl Shift for time::OffsetDateTime {
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		add_offset(delta, self)
	}
}

impl Shift for time::UtcDateTime {
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		add_utc(delta, self)
	}
}

impl TryFrom<RelativeDelta> for time::PrimitiveDateTime {
	type Error = ConversionError;

	fn try_from(rddt: RelativeDelta) -> Result<Self, Self::Error> {
		let year = rddt.year().ok_or(ConversionError::MissingYear)?;
		let year = year.try_into().map_err(|_| ConversionError::InvalidDate)?;
		let month = rddt.month().ok_or(ConversionError::MissingMonth)?;
		let day = rddt.day().ok_or(ConversionError::MissingDay)?;

		let month = time::Month::try_from(month).map_err(|_| ConversionError::InvalidDate)?;

		let date =
			time::Date::from_calendar_date(year, month, day).map_err(|_| ConversionError::InvalidDate)?;

		let hour = rddt.hour().unwrap_or(0);
		let minute = rddt.minute().unwrap_or(0);
		let second = rddt.second().unwrap_or(0);
		let nanosecond = rddt.nanosecond().unwrap_or(0);

		date
			.with_hms_nano(hour, minute, second, nanosecond)
			.map_err(|_| ConversionError::InvalidTime)
	}
}

#[cfg(test)]
mod tests {
	use similar_asserts::assert_eq;
	use time::PrimitiveDateTime;

	use crate::{ConversionError, RelativeDelta};

	#[test]
	fn test_try_from() {
		let rddt = RelativeDelta::with_year(2020)
			.and_month(4)
			.and_day(28)
			.and_hour(12)
			.and_minute(35)
			.and_second(48)
			.build();

		let naive_dt: PrimitiveDateTime = rddt.try_into().expect("Failed to convert");

		assert_eq!(naive_dt.year(), 2020);
		assert_eq!(naive_dt.month(), time::Month::April);
		assert_eq!(naive_dt.day(), 28);
		assert_eq!(naive_dt.hour(), 12);
		assert_eq!(naive_dt.minute(), 35);
		assert_eq!(naive_dt.second(), 48);
	}

	#[test]
	fn test_weekday_from_time() {
		let mut day = time::Weekday::Monday;
		for _ in 0..7 {
			let converted = crate::Weekday::from(day);
			assert_eq!(
				converted.num_days_from_monday(),
				day.number_days_from_monday()
			);
			day = day.next();
		}
	}

	#[test]
	fn test_try_from_reports_the_failing_field() {
		let missing_day = RelativeDelta::with_year(2020).and_month(1).build();
		assert_eq!(
			PrimitiveDateTime::try_from(missing_day),
			Err(ConversionError::MissingDay)
		);
		let out_of_range = RelativeDelta::with_year(1_000_000)
			.and_month(1)
			.and_day(1)
			.build();
		assert_eq!(
			PrimitiveDateTime::try_from(out_of_range),
			Err(ConversionError::InvalidDate)
		);
	}
}
