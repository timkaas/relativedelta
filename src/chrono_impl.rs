// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::ops;
use core::ops::Add;

use crate::RelativeDelta;
use crate::from_error::FromError;
use crate::relativedelta::{MonthType, MonthsType, num_days_in_month};
use chrono::{Datelike, Timelike};
use num_integer::Integer;

// Unfortunately we have to implement them manually as we dont want to restrict ourselves on a timezone
impl<Tz: chrono::TimeZone> Add<&chrono::DateTime<Tz>> for &RelativeDelta {
	type Output = chrono::DateTime<Tz>;

	/// # Panics
	///
	/// Panics if the resulting `DateTime` is invalid due to uncaught cases of invalid additions.
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn add(self, rhs: &chrono::DateTime<Tz>) -> Self::Output {
		let mut year = self.year().unwrap_or(rhs.year().into()) + self.years();
		let month = self.month().map_or(rhs.month().into(), MonthsType::from) + self.months();
		let (mut extra_years, mut relative_month) = month.div_rem(&12);
		if relative_month <= 0 {
			extra_years -= 1;
			relative_month += 12;
		}
		year += extra_years;

		let real_month = relative_month;
		// Clamp day to max number of days in calculated month
		let ndim = u32::from(num_days_in_month(year, relative_month as MonthType));
		// If day is not set, use the day from rhs, otherwise clamp it to
		// the maximum number of days in the month.
		let day = ndim.min(self.day().map_or(rhs.day(), u32::from));
		let hour = self.hour().map_or(rhs.hour(), u32::from);
		let minute = self.minute().map_or(rhs.minute(), u32::from);
		let second = self.second().map_or(rhs.second(), u32::from);
		let nanosecond = self.nanosecond().unwrap_or(rhs.nanosecond());

		let datetime = rhs
			.timezone()
			.with_ymd_and_hms(year as i32, real_month as u32, day, hour, minute, second)
			.single()
			.and_then(|d| d.with_nanosecond(nanosecond))
			.expect("could not create datetime");

		let ret = datetime
			+ chrono::Duration::days(self.days())
			+ chrono::Duration::hours(self.hours())
			+ chrono::Duration::minutes(self.minutes())
			+ chrono::Duration::seconds(self.seconds())
			+ chrono::Duration::nanoseconds(self.nanoseconds());

		if let Some((weekday, nth)) = self.weekday() {
			let mut jumpdays = (nth.abs() - 1) * 7;
			if nth > 0 {
				jumpdays += i64::from(
					7 - ret.weekday().num_days_from_monday() + u32::from(weekday.num_days_from_monday()),
				);
			} else {
				jumpdays += i64::from(
					(ret.weekday().num_days_from_monday() - u32::from(weekday.num_days_from_monday())) % 7,
				);
				jumpdays *= -1;
			}
			ret + chrono::Duration::days(jumpdays)
		} else {
			ret
		}
	}
}

impl<Tz: chrono::TimeZone> Add<&chrono::DateTime<Tz>> for RelativeDelta {
	type Output = chrono::DateTime<Tz>;

	#[allow(clippy::op_ref)]
	fn add(self, rhs: &chrono::DateTime<Tz>) -> Self::Output {
		&self + rhs
	}
}

impl<Tz: chrono::TimeZone> Add<chrono::DateTime<Tz>> for &RelativeDelta {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: chrono::DateTime<Tz>) -> Self::Output {
		self + &rhs
	}
}

impl<Tz: chrono::TimeZone> Add<chrono::DateTime<Tz>> for RelativeDelta {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: chrono::DateTime<Tz>) -> Self::Output {
		self + &rhs
	}
}

impl<Tz: chrono::TimeZone> Add<&RelativeDelta> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: &RelativeDelta) -> Self::Output {
		rhs + self
	}
}

impl<Tz: chrono::TimeZone> Add<RelativeDelta> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: RelativeDelta) -> Self::Output {
		rhs + self
	}
}

impl<Tz: chrono::TimeZone> Add<&RelativeDelta> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: &RelativeDelta) -> Self::Output {
		rhs + self
	}
}
impl<Tz: chrono::TimeZone> Add<RelativeDelta> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: RelativeDelta) -> Self::Output {
		rhs + self
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

	fn sub(self, rhs: RelativeDelta) -> Self::Output {
		&self - rhs
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
	use anyhow::anyhow;
	use chrono::{Datelike, TimeZone, Timelike, Utc};

	use crate::{RelativeDelta, Weekday};
	use similar_asserts::assert_eq;

	#[test]
	fn test_add() -> anyhow::Result<()> {
		let year = 2020;
		let month = 4;
		let day = 28;
		let hour = 12;
		let min = 35;
		let sec = 48;
		let dt = Utc
			.with_ymd_and_hms(year, month, day, hour, min, sec)
			.single()
			.ok_or(anyhow!("Test failed"))?;

		let add_1_year = RelativeDelta::with_years(1).build();
		assert_eq!(
			dt + add_1_year,
			Utc
				.with_ymd_and_hms(2021, month, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let sub_1_year = RelativeDelta::with_years(-1).build();
		assert_eq!(
			dt + &sub_1_year,
			Utc
				.with_ymd_and_hms(2019, month, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let set_year = RelativeDelta::with_year(2010).build();
		assert_eq!(
			&dt + set_year,
			Utc
				.with_ymd_and_hms(2010, month, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let set_year = RelativeDelta::with_year(-1).build();
		assert_eq!(
			&dt + &set_year,
			Utc
				.with_ymd_and_hms(-1, month, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let add_69_months = RelativeDelta::with_months(69).build();
		// Expected after fix
		assert_eq!(add_69_months.years(), 5);
		assert_eq!(add_69_months.months(), 9);
		assert_eq!(
			add_69_months + dt,
			Utc
				.with_ymd_and_hms(2026, 1, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let sub_6_months = RelativeDelta::with_months(-6).build();
		assert_eq!(
			dt + sub_6_months,
			Utc
				.with_ymd_and_hms(2019, 10, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let sub_47_months = RelativeDelta::with_months(-47).build();
		// Expected after fix
		assert_eq!(sub_47_months.years(), -3);
		assert_eq!(sub_47_months.months(), -11);
		assert_eq!(
			dt + sub_47_months,
			Utc
				.with_ymd_and_hms(2016, 5, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let add_400_days = RelativeDelta::with_days(400).build();
		assert_eq!(
			dt + add_400_days,
			Utc
				.with_ymd_and_hms(2021, 6, 2, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let sub_400_days = RelativeDelta::with_days(-400).build();
		assert_eq!(
			dt + sub_400_days,
			Utc
				.with_ymd_and_hms(2019, 3, 25, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let monday = RelativeDelta::with_years(1)
			.and_weekday(Some((Weekday::Mon, 1)))
			.build();
		assert_eq!(
			dt + monday,
			Utc
				.with_ymd_and_hms(2021, 5, 3, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let monday = RelativeDelta::with_years(1)
			.and_weekday(Some((Weekday::Mon, -1)))
			.build();
		assert_eq!(
			dt + monday,
			Utc
				.with_ymd_and_hms(2021, 4, 26, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let pay1 = RelativeDelta::with_day(1)
			.and_days(-1)
			.and_month(Some(3))
			.and_months(1)
			.build();
		assert_eq!(
			dt + pay1,
			Utc
				.with_ymd_and_hms(2020, 3, 31, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let pay2 = RelativeDelta::with_day(1)
			.and_days(-1)
			.and_month(Some(6))
			.and_months(1)
			.build();
		assert_eq!(
			dt + pay2,
			Utc
				.with_ymd_and_hms(2020, 6, 30, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let pay3 = RelativeDelta::with_day(1)
			.and_days(-1)
			.and_month(Some(9))
			.and_months(1)
			.build();
		assert_eq!(
			dt + pay3,
			Utc
				.with_ymd_and_hms(2020, 9, 30, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let pay4 = RelativeDelta::with_day(1)
			.and_days(-1)
			.and_month(Some(12))
			.and_months(1)
			.build();
		assert_eq!(
			dt + pay4,
			Utc
				.with_ymd_and_hms(2020, 12, 31, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);
		Ok(())
	}

	#[test]
	fn test_sub() -> anyhow::Result<()> {
		let year = 2020;
		let month = 4;
		let day = 28;
		let hour = 12;
		let min = 35;
		let sec = 48;
		let dt = Utc
			.with_ymd_and_hms(year, month, day, hour, min, sec)
			.single()
			.ok_or(anyhow!("Test failed"))?;

		let sub_1_year = RelativeDelta::with_years(1).build();
		assert_eq!(
			dt - sub_1_year,
			Utc
				.with_ymd_and_hms(2019, month, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let add_1_year = RelativeDelta::with_years(-1).build();
		assert_eq!(
			dt - &add_1_year,
			Utc
				.with_ymd_and_hms(2021, month, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let set_year = RelativeDelta::with_year(2010).build();
		assert_eq!(
			&dt - set_year,
			Utc
				.with_ymd_and_hms(2010, month, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let set_year = RelativeDelta::with_year(-1).build();
		assert_eq!(
			&dt - &set_year,
			Utc
				.with_ymd_and_hms(-1, month, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let sub_6_months = RelativeDelta::with_months(-6).build();
		assert_eq!(
			dt - sub_6_months,
			Utc
				.with_ymd_and_hms(2020, 10, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);

		let sub_47_months = RelativeDelta::with_months(-47).build();
		assert_eq!(
			dt - sub_47_months,
			Utc
				.with_ymd_and_hms(2024, 3, day, hour, min, sec)
				.single()
				.ok_or(anyhow!("Test failed"))?
		);
		Ok(())
	}

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
}
