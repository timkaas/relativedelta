// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use num_integer::Integer;

use crate::from_error::FromError;
use crate::relativedelta::{MonthType, MonthsType, num_days_in_month};
use crate::shift::sealed::Shift;
use crate::{RelativeDelta, Weekday};

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
	let current = Weekday::from(current.number_days_from_monday());
	days(weekday.jump_days_from(current, nth))
}

/// Adds `delta` to `rhs`, using only the date fields and the days of the delta, like
/// `chrono::NaiveDate`.
fn add_date(delta: &RelativeDelta, rhs: time::Date) -> Option<time::Date> {
	let date = anchor_date(delta, rhs)?.checked_add(days(delta.days())?)?;
	date.checked_add(weekday_jump(delta, date.weekday())?)
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
			let seconds = lhs
				.hours()
				.checked_mul(3_600)?
				.checked_add(lhs.minutes().checked_mul(60)?)?
				.checked_add(lhs.seconds())?;
			let ret = $ctor(date, time, rhs)
				.checked_add(days(lhs.days())?)?
				.checked_add(time::Duration::seconds(seconds))?
				.checked_add(time::Duration::nanoseconds(lhs.nanoseconds()))?;
			ret.checked_add(weekday_jump(lhs, ret.weekday())?)
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
	/// Add this calendar delta to a `time::PrimitiveDateTime` without panicking.
	#[must_use]
	pub fn checked_add_primitive_datetime(
		&self,
		rhs: &time::PrimitiveDateTime,
	) -> Option<time::PrimitiveDateTime> {
		add_primitive(self, rhs)
	}

	/// Add this calendar delta to a `time::OffsetDateTime` without panicking.
	#[must_use]
	pub fn checked_add_offset_datetime(
		&self,
		rhs: &time::OffsetDateTime,
	) -> Option<time::OffsetDateTime> {
		add_offset(self, rhs)
	}

	/// Add this calendar delta to a `time::UtcDateTime` without panicking.
	#[must_use]
	pub fn checked_add_utc_datetime(&self, rhs: &time::UtcDateTime) -> Option<time::UtcDateTime> {
		add_utc(self, rhs)
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
	type Error = FromError;

	fn try_from(rddt: RelativeDelta) -> Result<Self, Self::Error> {
		let year = rddt.year().ok_or(FromError::MissingYear)?;
		let year = year.try_into().map_err(|_| FromError::MissingYear)?;
		let month = rddt.month().ok_or(FromError::MissingMonth)?;
		let day = rddt.day().ok_or(FromError::MissingDay)?;

		let month = time::Month::try_from(month).map_err(|_| FromError::InvalidDateComponents)?;

		let date = time::Date::from_calendar_date(year, month, day)
			.map_err(|_| FromError::InvalidDateComponents)?;

		let hour = rddt.hour().unwrap_or(0);
		let minute = rddt.minute().unwrap_or(0);
		let second = rddt.second().unwrap_or(0);
		let nanosecond = rddt.nanosecond().unwrap_or(0);

		date
			.with_hms_nano(hour, minute, second, nanosecond)
			.map_err(|_| FromError::InvalidTimeComponents)
	}
}

#[cfg(test)]
mod tests {
	use similar_asserts::assert_eq;
	use time::PrimitiveDateTime;

	use crate::RelativeDelta;

	#[test]
	fn test_try_from() {
		let rddt = RelativeDelta::with_year(2020)
			.and_month(Some(4))
			.and_day(Some(28))
			.and_hour(Some(12))
			.and_minute(Some(35))
			.and_second(Some(48))
			.build();

		let naive_dt: PrimitiveDateTime = rddt.try_into().expect("Failed to convert");

		assert_eq!(naive_dt.year(), 2020);
		assert_eq!(naive_dt.month(), time::Month::April);
		assert_eq!(naive_dt.day(), 28);
		assert_eq!(naive_dt.hour(), 12);
		assert_eq!(naive_dt.minute(), 35);
		assert_eq!(naive_dt.second(), 48);
	}
}
