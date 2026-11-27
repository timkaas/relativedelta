// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::ops;

use chrono::{Datelike, Timelike};
use num_integer::Integer;

use crate::from_error::FromError;
use crate::relativedelta::{MonthType, MonthsType, num_days_in_month};
use crate::shift::sealed::Shift;
use crate::{RelativeDelta, Weekday};

macro_rules! impl_add_sub {
	($base:ty, $ret:ty) => {
		impl ops::Add<$base> for &$crate::RelativeDelta {
			type Output = $ret;

			fn add(self, rhs: $base) -> Self::Output {
				self + &rhs
			}
		}

		impl ops::Add<&$base> for $crate::RelativeDelta {
			type Output = $ret;

			fn add(self, rhs: &$base) -> Self::Output {
				&self + rhs
			}
		}

		impl ops::Add<$base> for $crate::RelativeDelta {
			type Output = $ret;

			fn add(self, rhs: $base) -> Self::Output {
				&self + &rhs
			}
		}

		impl ops::Add<&$crate::RelativeDelta> for &$base {
			type Output = $ret;

			fn add(self, rhs: &$crate::RelativeDelta) -> Self::Output {
				rhs + self
			}
		}

		impl ops::Add<$crate::RelativeDelta> for &$base {
			type Output = $ret;

			fn add(self, rhs: $crate::RelativeDelta) -> Self::Output {
				&rhs + self
			}
		}

		impl ops::Add<&$crate::RelativeDelta> for $base {
			type Output = $ret;

			fn add(self, rhs: &$crate::RelativeDelta) -> Self::Output {
				rhs + &self
			}
		}

		impl ops::Add<$crate::RelativeDelta> for $base {
			type Output = $ret;

			fn add(self, rhs: $crate::RelativeDelta) -> Self::Output {
				rhs + &self
			}
		}

		impl ops::Sub<&$crate::RelativeDelta> for &$base {
			type Output = $ret;

			fn sub(self, rhs: &$crate::RelativeDelta) -> Self::Output {
				self + &(-rhs)
			}
		}

		impl ops::Sub<$crate::RelativeDelta> for &$base {
			type Output = $ret;

			fn sub(self, rhs: $crate::RelativeDelta) -> Self::Output {
				self - &rhs
			}
		}

		impl ops::Sub<&$crate::RelativeDelta> for $base {
			type Output = $ret;

			fn sub(self, rhs: &$crate::RelativeDelta) -> Self::Output {
				&self - rhs
			}
		}

		impl ops::Sub<$crate::RelativeDelta> for $base {
			type Output = $ret;

			fn sub(self, rhs: $crate::RelativeDelta) -> Self::Output {
				&self - &rhs
			}
		}
	};
}

/// Helper struct to hold calculated date components
struct DateComponents {
	year: i32,
	month: u32,
	day: u32,
}

struct TimeComponents {
	hour: u32,
	minute: u32,
	second: u32,
	nanosecond: u32,
}

trait CheckedAddDuration: Sized {
	fn checked_add_duration(self, duration: chrono::Duration) -> Option<Self>;
}

impl CheckedAddDuration for chrono::NaiveDate {
	fn checked_add_duration(self, duration: chrono::Duration) -> Option<Self> {
		self.checked_add_signed(duration)
	}
}

impl CheckedAddDuration for chrono::NaiveDateTime {
	fn checked_add_duration(self, duration: chrono::Duration) -> Option<Self> {
		self.checked_add_signed(duration)
	}
}

impl<Tz: chrono::TimeZone> CheckedAddDuration for chrono::DateTime<Tz> {
	fn checked_add_duration(self, duration: chrono::Duration) -> Option<Self> {
		self.checked_add_signed(duration)
	}
}

impl RelativeDelta {
	/// Returns the days through nanoseconds as one exact duration, or `None` on overflow.
	fn time_duration(&self) -> Option<chrono::TimeDelta> {
		chrono::TimeDelta::try_days(self.days())?
			.checked_add(&chrono::TimeDelta::try_hours(self.hours())?)?
			.checked_add(&chrono::TimeDelta::try_minutes(self.minutes())?)?
			.checked_add(&chrono::TimeDelta::try_seconds(self.seconds())?)?
			.checked_add(&chrono::TimeDelta::nanoseconds(self.nanoseconds()))
	}

	/// Calculates the adjusted year, month, and day from a `RelativeDelta` and source date
	fn date_components<D: Datelike>(&self, source: &D) -> Option<DateComponents> {
		let year = self
			.year()
			.unwrap_or(source.year().into())
			.checked_add(self.years())?;
		let month = self
			.month()
			.map_or(source.month().into(), MonthsType::from)
			.checked_add(self.months())?;
		let (extra_years, month_offset) = (month - 1).div_mod_floor(&12);
		let year = year.checked_add(extra_years)?;
		let month = MonthType::try_from(month_offset + 1).ok()?;

		let day =
			u32::from(num_days_in_month(year, month)).min(self.day().map_or(source.day(), u32::from));

		Some(DateComponents {
			year: year.try_into().ok()?,
			month: month.into(),
			day,
		})
	}

	fn time_components<D: Timelike>(&self, source: &D) -> TimeComponents {
		TimeComponents {
			hour: self.hour().map_or(source.hour(), u32::from),
			minute: self.minute().map_or(source.minute(), u32::from),
			second: self.second().map_or(source.second(), u32::from),
			nanosecond: self.nanosecond().unwrap_or(source.nanosecond()),
		}
	}

	/// Applies weekday adjustments to a result date
	fn apply_weekday_adjustment<D>(&self, result: D) -> Option<D>
	where
		D: Datelike + CheckedAddDuration,
	{
		if let Some((weekday, nth)) = self.weekday() {
			let current = Weekday::from(u8::try_from(result.weekday().num_days_from_monday()).ok()?);
			result.checked_add_duration(chrono::Duration::try_days(
				weekday.jump_days_from(current, nth),
			)?)
		}
		else {
			Some(result)
		}
	}
}

impl<Tz: chrono::TimeZone> ops::Add<&chrono::DateTime<Tz>> for &RelativeDelta {
	type Output = chrono::DateTime<Tz>;

	/// # Panics
	///
	/// Panics if the resulting `DateTime` is invalid due to uncaught cases of invalid additions.
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn add(self, rhs: &chrono::DateTime<Tz>) -> Self::Output {
		self
			.checked_add_datetime(rhs)
			.expect("could not add RelativeDelta to datetime")
	}
}

impl RelativeDelta {
	/// Add this calendar delta without panicking on an unrepresentable result.
	///
	/// Returns `None` if the resulting date is outside the range supported by
	/// `chrono`, if a timezone transition makes the local time ambiguous, or if
	/// the duration overflows.
	#[must_use]
	pub fn checked_add_datetime<Tz: chrono::TimeZone>(
		&self,
		rhs: &chrono::DateTime<Tz>,
	) -> Option<chrono::DateTime<Tz>> {
		let DateComponents {
			year,
			month,
			day,
		} = self.date_components(rhs)?;
		let TimeComponents {
			hour,
			minute,
			second,
			nanosecond,
		} = self.time_components(rhs);

		let datetime = rhs
			.timezone()
			.with_ymd_and_hms(year, month, day, hour, minute, second)
			.single()
			.and_then(|d| d.with_nanosecond(nanosecond))?;

		let ret = datetime.checked_add_signed(self.time_duration()?)?;
		self.apply_weekday_adjustment(ret)
	}
}

impl<Tz: chrono::TimeZone> ops::Add<chrono::DateTime<Tz>> for &RelativeDelta {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: chrono::DateTime<Tz>) -> Self::Output {
		self + &rhs
	}
}

impl<Tz: chrono::TimeZone> ops::Add<chrono::DateTime<Tz>> for RelativeDelta {
	type Output = chrono::DateTime<Tz>;

	#[allow(clippy::op_ref)]
	fn add(self, rhs: chrono::DateTime<Tz>) -> Self::Output {
		&self + &rhs
	}
}

impl<Tz: chrono::TimeZone> ops::Add<&chrono::DateTime<Tz>> for RelativeDelta {
	type Output = chrono::DateTime<Tz>;

	#[allow(clippy::op_ref)]
	fn add(self, rhs: &chrono::DateTime<Tz>) -> Self::Output {
		&self + rhs
	}
}

impl<Tz: chrono::TimeZone> ops::Add<&RelativeDelta> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: &RelativeDelta) -> Self::Output {
		rhs + self
	}
}

impl<Tz: chrono::TimeZone> ops::Add<RelativeDelta> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	#[allow(clippy::op_ref)]
	fn add(self, rhs: RelativeDelta) -> Self::Output {
		&rhs + self
	}
}

impl<Tz: chrono::TimeZone> ops::Add<&RelativeDelta> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: &RelativeDelta) -> Self::Output {
		rhs + &self
	}
}

impl<Tz: chrono::TimeZone> ops::Add<RelativeDelta> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	#[allow(clippy::op_ref)]
	fn add(self, rhs: RelativeDelta) -> Self::Output {
		&rhs + &self
	}
}

/// Sub (non commutative)
impl<Tz: chrono::TimeZone> ops::Sub<&RelativeDelta> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn sub(self, rhs: &RelativeDelta) -> Self::Output {
		self + (-rhs)
	}
}

impl<Tz: chrono::TimeZone> ops::Sub<RelativeDelta> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	#[allow(clippy::op_ref)]
	fn sub(self, rhs: RelativeDelta) -> Self::Output {
		self - &rhs
	}
}

impl<Tz: chrono::TimeZone> ops::Sub<&RelativeDelta> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn sub(self, rhs: &RelativeDelta) -> Self::Output {
		&self - rhs
	}
}

impl<Tz: chrono::TimeZone> ops::Sub<RelativeDelta> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	#[allow(clippy::op_ref)]
	fn sub(self, rhs: RelativeDelta) -> Self::Output {
		&self - &rhs
	}
}

impl ops::Add<&chrono::NaiveDateTime> for &RelativeDelta {
	type Output = chrono::NaiveDateTime;

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn add(self, rhs: &chrono::NaiveDateTime) -> Self::Output {
		self
			.checked_add_naive_datetime(rhs)
			.expect("could not add RelativeDelta to datetime")
	}
}

impl RelativeDelta {
	/// Add this calendar delta to a `chrono::NaiveDateTime` without panicking.
	#[must_use]
	pub fn checked_add_naive_datetime(
		&self,
		rhs: &chrono::NaiveDateTime,
	) -> Option<chrono::NaiveDateTime> {
		let DateComponents {
			year,
			month,
			day,
		} = self.date_components(rhs)?;
		let TimeComponents {
			hour,
			minute,
			second,
			nanosecond,
		} = self.time_components(rhs);

		let datetime = chrono::NaiveDate::from_ymd_opt(year, month, day)?
			.and_hms_nano_opt(hour, minute, second, nanosecond)?;

		let ret = datetime.checked_add_signed(self.time_duration()?)?;

		self.apply_weekday_adjustment(ret)
	}
}

impl_add_sub!(chrono::NaiveDateTime, chrono::NaiveDateTime);

impl ops::Add<&chrono::NaiveDate> for &RelativeDelta {
	type Output = chrono::NaiveDate;

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn add(self, rhs: &chrono::NaiveDate) -> Self::Output {
		self
			.checked_add_date(rhs)
			.expect("could not add RelativeDelta to date")
	}
}

impl RelativeDelta {
	/// Add this calendar delta to a `chrono::NaiveDate` without panicking.
	#[must_use]
	pub fn checked_add_date(&self, rhs: &chrono::NaiveDate) -> Option<chrono::NaiveDate> {
		let DateComponents {
			year,
			month,
			day,
		} = self.date_components(rhs)?;

		let date = chrono::NaiveDate::from_ymd_opt(year, month, day)?;

		let ret = date.checked_add_signed(chrono::TimeDelta::try_days(self.days())?)?;
		self.apply_weekday_adjustment(ret)
	}
}

impl_add_sub!(chrono::NaiveDate, chrono::NaiveDate);

impl Shift for chrono::NaiveDate {
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		delta.checked_add_date(self)
	}
}

impl Shift for chrono::NaiveDateTime {
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		delta.checked_add_naive_datetime(self)
	}
}

impl<Tz: chrono::TimeZone> Shift for chrono::DateTime<Tz> {
	fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self> {
		delta.checked_add_datetime(self)
	}
}

impl TryFrom<RelativeDelta> for chrono::NaiveDateTime {
	type Error = FromError;

	fn try_from(rddt: RelativeDelta) -> Result<Self, Self::Error> {
		let year = rddt.year().ok_or(FromError::MissingYear)?;
		let year = year.try_into().map_err(|_| FromError::MissingYear)?;
		let month = rddt.month().ok_or(FromError::MissingMonth)?;
		let day = rddt.day().ok_or(FromError::MissingDay)?;

		let date = chrono::NaiveDate::from_ymd_opt(year, month.into(), day.into())
			.ok_or(FromError::InvalidDateComponents)?;

		date
			.and_hms_nano_opt(
				u32::from(rddt.hour().unwrap_or(0)),
				u32::from(rddt.minute().unwrap_or(0)),
				u32::from(rddt.second().unwrap_or(0)),
				rddt.nanosecond().unwrap_or(0),
			)
			.ok_or(FromError::InvalidTimeComponents)
	}
}

#[cfg(test)]
mod tests {
	use chrono::{Datelike, Timelike};
	use similar_asserts::assert_eq;

	use crate::RelativeDelta;

	#[test]
	fn test_try_from() -> anyhow::Result<()> {
		let rddt = RelativeDelta::with_year(2020)
			.and_month(Some(4))
			.and_day(Some(28))
			.and_hour(Some(12))
			.and_minute(Some(35))
			.and_second(Some(48))
			.build();

		let naive_dt: chrono::NaiveDateTime = rddt.try_into()?;

		assert_eq!(naive_dt.year(), 2020);
		assert_eq!(naive_dt.month(), 4);
		assert_eq!(naive_dt.day(), 28);
		assert_eq!(naive_dt.hour(), 12);
		assert_eq!(naive_dt.minute(), 35);
		assert_eq!(naive_dt.second(), 48);
		Ok(())
	}

	#[test]
	fn test_ergonomic_constructors_and_checked_add() {
		let date = chrono::NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
		let next_month = date + RelativeDelta::with_months(1).build();
		assert_eq!(
			next_month,
			chrono::NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
		);

		let maximum = chrono::NaiveDate::MAX;
		assert!(
			RelativeDelta::with_days(1)
				.build()
				.checked_add_date(&maximum)
				.is_none()
		);
	}
}
