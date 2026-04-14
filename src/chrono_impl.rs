// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::ops;

use chrono::{Datelike, Timelike};
use num_integer::Integer;

use crate::RelativeDelta;
use crate::from_error::FromError;
use crate::relativedelta::{MonthType, MonthsType, num_days_in_month};

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

impl RelativeDelta {
	fn time_duration(&self) -> chrono::TimeDelta {
		chrono::Duration::days(self.days())
			+ chrono::Duration::hours(self.hours())
			+ chrono::Duration::minutes(self.minutes())
			+ chrono::Duration::seconds(self.seconds())
			+ chrono::Duration::nanoseconds(self.nanoseconds())
	}

	/// Calculates the adjusted year, month, and day from a `RelativeDelta` and source date
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn date_components<D: Datelike>(&self, source: &D) -> DateComponents {
		let mut year = self.year().unwrap_or(source.year().into()) + self.years();
		let month = self.month().map_or(source.month().into(), MonthsType::from) + self.months();
		let (mut extra_years, mut relative_month) = month.div_rem(&12);
		if relative_month <= 0 {
			extra_years -= 1;
			relative_month += 12;
		}
		year += extra_years;

		let num_days_in_month = u32::from(num_days_in_month(year, relative_month as MonthType));
		let day = num_days_in_month.min(self.day().map_or(source.day(), u32::from));

		DateComponents {
			year: year as i32,
			month: relative_month as u32,
			day,
		}
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
	fn apply_weekday_adjustment<D>(&self, result: D) -> D
	where
		D: Datelike + ops::Add<chrono::Duration, Output = D>,
	{
		if let Some((weekday, nth)) = self.weekday() {
			let mut jumpdays = (nth.abs() - 1) * 7;
			if nth > 0 {
				jumpdays += i64::from(
					7 - result.weekday().num_days_from_monday() + u32::from(weekday.num_days_from_monday()),
				);
			}
			else {
				jumpdays += i64::from(
					(result.weekday().num_days_from_monday() - u32::from(weekday.num_days_from_monday())) % 7,
				);
				jumpdays *= -1;
			}
			result + chrono::Duration::days(jumpdays)
		}
		else {
			result
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
		let DateComponents {
			year,
			month,
			day,
		} = self.date_components(rhs);
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
			.and_then(|d| d.with_nanosecond(nanosecond))
			.expect("could not create datetime");

		let ret = datetime + self.time_duration();
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
		let DateComponents {
			year,
			month,
			day,
		} = self.date_components(rhs);
		let TimeComponents {
			hour,
			minute,
			second,
			nanosecond,
		} = self.time_components(rhs);

		let datetime = chrono::NaiveDate::from_ymd_opt(year, month, day)
			.expect("could not create date")
			.and_hms_nano_opt(hour, minute, second, nanosecond)
			.expect("could not create datetime");

		let ret = datetime + self.time_duration();

		self.apply_weekday_adjustment(ret)
	}
}

impl_add_sub!(chrono::NaiveDateTime, chrono::NaiveDateTime);

impl ops::Add<&chrono::NaiveDate> for &RelativeDelta {
	type Output = chrono::NaiveDate;

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn add(self, rhs: &chrono::NaiveDate) -> Self::Output {
		let DateComponents {
			year,
			month,
			day,
		} = self.date_components(rhs);

		let date = chrono::NaiveDate::from_ymd_opt(year, month, day).expect("could not create date");

		let ret = date + chrono::Duration::days(self.days());
		self.apply_weekday_adjustment(ret)
	}
}

impl_add_sub!(chrono::NaiveDate, chrono::NaiveDate);

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
	use similar_asserts::assert_eq;

	use crate::{RelativeDelta, Weekday};

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
	fn test_naive() -> anyhow::Result<()> {
		let nd = chrono::NaiveDate::from_ymd_opt(2020, 4, 28).unwrap();
		let ndt = nd.and_hms_opt(12, 35, 48).unwrap();

		let add_1_year = RelativeDelta::with_years(1).build();
		assert_eq!(
			nd + add_1_year,
			chrono::NaiveDate::from_ymd_opt(2021, 4, 28).unwrap()
		);
		assert_eq!(
			ndt + add_1_year,
			chrono::NaiveDate::from_ymd_opt(2021, 4, 28)
				.unwrap()
				.and_hms_opt(12, 35, 48)
				.unwrap()
		);

		let sub_1_year = RelativeDelta::with_years(1).build();
		assert_eq!(
			nd - sub_1_year,
			chrono::NaiveDate::from_ymd_opt(2019, 4, 28).unwrap()
		);
		assert_eq!(
			ndt - sub_1_year,
			chrono::NaiveDate::from_ymd_opt(2019, 4, 28)
				.unwrap()
				.and_hms_opt(12, 35, 48)
				.unwrap()
		);

		let sub_1_year = RelativeDelta::with_years(-1).build();
		assert_eq!(
			nd - sub_1_year,
			chrono::NaiveDate::from_ymd_opt(2021, 4, 28).unwrap()
		);
		assert_eq!(
			ndt - sub_1_year,
			chrono::NaiveDate::from_ymd_opt(2021, 4, 28)
				.unwrap()
				.and_hms_opt(12, 35, 48)
				.unwrap()
		);

		let add_1_month = RelativeDelta::with_months(1).build();
		assert_eq!(
			nd + add_1_month,
			chrono::NaiveDate::from_ymd_opt(2020, 5, 28).unwrap()
		);

		let add_1_day = RelativeDelta::with_days(1).build();
		assert_eq!(
			nd + add_1_day,
			chrono::NaiveDate::from_ymd_opt(2020, 4, 29).unwrap()
		);

		let set_day = RelativeDelta::with_day(1).build();
		assert_eq!(
			nd + set_day,
			chrono::NaiveDate::from_ymd_opt(2020, 4, 1).unwrap()
		);

		let add_1_hour = RelativeDelta::with_hours(1).build();
		// NaiveDate + 1 hour should still be same NaiveDate because it only adds days
		assert_eq!(nd + add_1_hour, nd);
		assert_eq!(ndt + add_1_hour, nd.and_hms_opt(13, 35, 48).unwrap());

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
