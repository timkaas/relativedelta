// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::RelativeDelta;
use crate::relativedelta::{MonthType, MonthsType, num_days_in_month};
use core::ops;
#[allow(clippy::wildcard_imports)]
use impl_ops::*;
use num_integer::Integer;

use crate::from_error::FromError;

unsafe fn month_unchecked(month: MonthType) -> time::Month {
	// Safety: `month` is guaranteed to be in the range 1..=12
	unsafe { time::Month::try_from(month).unwrap_unchecked() }
}

macro_rules! impl_add {
	($fname:ident, $datetime:ty, $ctor:expr) => {
		#[allow(clippy::cast_possible_truncation)]
		#[allow(clippy::cast_sign_loss)]
		fn $fname(lhs: &RelativeDelta, rhs: &$datetime) -> $datetime {
			let mut year = lhs.year().unwrap_or(rhs.year().into()) + lhs.years();
			let month = lhs
				.month()
				.map(MonthsType::from)
				.unwrap_or(rhs.month() as MonthsType)
				+ lhs.months();
			let (mut extra_years, mut relative_month) = month.div_rem(&12);
			if relative_month <= 0 {
				extra_years -= 1;
				relative_month += 12;
			}
			year += extra_years;

			let tmonth = relative_month as u8;
			let month = unsafe { month_unchecked(tmonth) };
			// Clamp day to max number of days in calculated month
			let ndim = num_days_in_month(year, tmonth);
			// If day is not set, use the day from rhs, otherwise clamp it to
			// the maximum number of days in the month.
			let day = ndim.min(lhs.day().unwrap_or(rhs.day()));
			let hour = lhs.hour().unwrap_or(rhs.hour());
			let minute = lhs.minute().unwrap_or(rhs.minute());
			let second = lhs.second().unwrap_or(rhs.second());
			let nanosecond = lhs.nanosecond().unwrap_or(rhs.nanosecond());

			let date =
				time::Date::from_calendar_date(year as i32, month, day).expect("could not create date");

			let time =
				time::Time::from_hms_nano(hour, minute, second, nanosecond).expect("could not create time");

			let datetime = $ctor(date, time);

			let ret = datetime
				+ time::Duration::days(lhs.days())
				+ time::Duration::hours(lhs.hours())
				+ time::Duration::minutes(lhs.minutes())
				+ time::Duration::seconds(lhs.seconds())
				+ time::Duration::nanoseconds(lhs.nanoseconds());

			if let Some((weekday, nth)) = lhs.weekday() {
				let mut jumpdays = (nth.abs() - 1) * 7;
				if nth > 0 {
					jumpdays +=
						i64::from(7 - ret.weekday().number_days_from_monday() + weekday.num_days_from_monday());
				} else {
					jumpdays += i64::from(
						(ret.weekday().number_days_from_monday() - weekday.num_days_from_monday()) % 7,
					);
					jumpdays *= -1;
				}
				ret + time::Duration::days(jumpdays)
			} else {
				ret
			}
		}
	};
}

impl_add!(
	add_primitive,
	time::PrimitiveDateTime,
	time::PrimitiveDateTime::new
);
impl_add!(
	add_offset,
	time::OffsetDateTime,
	time::OffsetDateTime::new_utc
);
impl_add!(add_utc, time::UtcDateTime, time::UtcDateTime::new);

impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &time::PrimitiveDateTime| -> time::PrimitiveDateTime { add_primitive(lhs, rhs) });
impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &time::OffsetDateTime| -> time::OffsetDateTime { add_offset(lhs, rhs) });
impl_op_ex_commutative!(+|lhs: &RelativeDelta, rhs: &time::UtcDateTime| -> time::UtcDateTime { add_utc(lhs, rhs) });

impl_op_ex!(
	-|lhs: &time::PrimitiveDateTime, rhs: &RelativeDelta| -> time::PrimitiveDateTime { lhs + (-rhs) }
);
impl_op_ex!(
	-|lhs: &time::OffsetDateTime, rhs: &RelativeDelta| -> time::OffsetDateTime { lhs + (-rhs) }
);
impl_op_ex!(-|lhs: &time::UtcDateTime, rhs: &RelativeDelta| -> time::UtcDateTime { lhs + (-rhs) });

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
	use super::*;
	use crate::relativedelta::*;
	use crate::*;

	use similar_asserts::assert_eq;
	use time::PrimitiveDateTime;

	fn primitive_ymd_hmsn(
		year: YearType,
		month: MonthType,
		day: DayType,
		hour: HourType,
		minute: MinuteType,
		second: SecondType,
		nanosecond: NanosecondType,
	) -> anyhow::Result<time::PrimitiveDateTime> {
		let dt = time::Date::from_calendar_date(year as i32, unsafe { month_unchecked(month) }, day)?
			.with_hms_nano(hour, minute, second, nanosecond)?;
		Ok(dt)
	}
	#[test]
	fn test_add() -> anyhow::Result<()> {
		let year = 2020;
		let month = 4;
		let day = 28;
		let hour = 12;
		let min = 35;
		let sec = 48;
		let dt = primitive_ymd_hmsn(year, month, day, hour, min, sec, 0)?;

		let add_1_year = RelativeDelta::with_years(1).build();
		assert_eq!(
			dt + add_1_year,
			primitive_ymd_hmsn(2021, month, day, hour, min, sec, 0)?
		);

		let sub_1_year = RelativeDelta::with_years(-1).build();
		assert_eq!(
			dt + &sub_1_year,
			primitive_ymd_hmsn(2019, month, day, hour, min, sec, 0)?
		);

		let set_year = RelativeDelta::with_year(2010).build();
		assert_eq!(
			&dt + set_year,
			primitive_ymd_hmsn(2010, month, day, hour, min, sec, 0)?
		);

		let set_year = RelativeDelta::with_year(-1).build();
		assert_eq!(
			&dt + &set_year,
			primitive_ymd_hmsn(-1, month, day, hour, min, sec, 0)?
		);

		let add_69_months = RelativeDelta::with_months(69).build();
		// Expected after fix
		assert_eq!(add_69_months.years(), 5);
		assert_eq!(add_69_months.months(), 9);
		assert_eq!(
			add_69_months + dt,
			primitive_ymd_hmsn(2026, 1, day, hour, min, sec, 0)?
		);

		let sub_6_months = RelativeDelta::with_months(-6).build();
		assert_eq!(
			dt + sub_6_months,
			primitive_ymd_hmsn(2019, 10, day, hour, min, sec, 0)?
		);

		let sub_47_months = RelativeDelta::with_months(-47).build();
		// Expected after fix
		assert_eq!(sub_47_months.years(), -3);
		assert_eq!(sub_47_months.months(), -11);
		assert_eq!(
			dt + sub_47_months,
			primitive_ymd_hmsn(2016, 5, day, hour, min, sec, 0)?
		);

		let add_400_days = RelativeDelta::with_days(400).build();
		assert_eq!(
			dt + add_400_days,
			primitive_ymd_hmsn(2021, 6, 2, hour, min, sec, 0)?
		);

		let sub_400_days = RelativeDelta::with_days(-400).build();
		assert_eq!(
			dt + sub_400_days,
			primitive_ymd_hmsn(2019, 3, 25, hour, min, sec, 0)?
		);

		let monday = RelativeDelta::with_years(1)
			.and_weekday(Some((Weekday::Mon, 1)))
			.build();
		assert_eq!(
			dt + monday,
			primitive_ymd_hmsn(2021, 5, 3, hour, min, sec, 0)?
		);

		let monday = RelativeDelta::with_years(1)
			.and_weekday(Some((Weekday::Mon, -1)))
			.build();
		assert_eq!(
			dt + monday,
			primitive_ymd_hmsn(2021, 4, 26, hour, min, sec, 0)?
		);

		let pay1 = RelativeDelta::with_day(1)
			.and_days(-1)
			.and_month(Some(3))
			.and_months(1)
			.build();
		assert_eq!(
			dt + pay1,
			primitive_ymd_hmsn(2020, 3, 31, hour, min, sec, 0)?
		);

		let pay2 = RelativeDelta::with_day(1)
			.and_days(-1)
			.and_month(Some(6))
			.and_months(1)
			.build();
		assert_eq!(
			dt + pay2,
			primitive_ymd_hmsn(2020, 6, 30, hour, min, sec, 0)?
		);

		let pay3 = RelativeDelta::with_day(1)
			.and_days(-1)
			.and_month(Some(9))
			.and_months(1)
			.build();
		assert_eq!(
			dt + pay3,
			primitive_ymd_hmsn(2020, 9, 30, hour, min, sec, 0)?
		);

		let pay4 = RelativeDelta::with_day(1)
			.and_days(-1)
			.and_month(Some(12))
			.and_months(1)
			.build();
		assert_eq!(
			dt + pay4,
			primitive_ymd_hmsn(2020, 12, 31, hour, min, sec, 0)?
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
		let dt = primitive_ymd_hmsn(year, month, day, hour, min, sec, 0)?;

		let sub_1_year = RelativeDelta::with_years(1).build();
		assert_eq!(
			dt - sub_1_year,
			primitive_ymd_hmsn(2019, month, day, hour, min, sec, 0)?
		);

		let add_1_year = RelativeDelta::with_years(-1).build();
		assert_eq!(
			dt - &add_1_year,
			primitive_ymd_hmsn(2021, month, day, hour, min, sec, 0)?
		);

		let set_year = RelativeDelta::with_year(2010).build();
		assert_eq!(
			&dt - set_year,
			primitive_ymd_hmsn(2010, month, day, hour, min, sec, 0)?
		);

		let set_year = RelativeDelta::with_year(-1).build();
		assert_eq!(
			&dt - &set_year,
			primitive_ymd_hmsn(-1, month, day, hour, min, sec, 0)?
		);

		let sub_6_months = RelativeDelta::with_months(-6).build();
		assert_eq!(
			dt - sub_6_months,
			primitive_ymd_hmsn(2020, 10, day, hour, min, sec, 0)?
		);

		let sub_47_months = RelativeDelta::with_months(-47).build();
		assert_eq!(
			dt - sub_47_months,
			primitive_ymd_hmsn(2024, 3, day, hour, min, sec, 0)?
		);
		Ok(())
	}

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
