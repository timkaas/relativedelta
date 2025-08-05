// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::ops;
#[allow(clippy::wildcard_imports)]
use impl_ops::*;
use num_integer::Integer;
use num_traits::{ToPrimitive, Zero};
use paste::paste;

use crate::weekday::Weekday;

const MAX_MONTHS: MonthsType = 12;
const MONTH_RANGE: ops::RangeInclusive<MonthType> = 1..=12;
const DAY_RANGE: ops::RangeInclusive<DayType> = 1..=31;
const MAX_HOURS: HoursType = 24;
const HOUR_RANGE: ops::RangeInclusive<HourType> = 0..=23;
const MAX_MINUTES: MinutesType = 60;
const MINUTE_RANGE: ops::RangeInclusive<MinuteType> = 0..=59;
const MAX_SECONDS: SecondsType = 60;
const SECOND_RANGE: ops::RangeInclusive<SecondType> = 0..=59;
const MAX_NANOSECONDS: NanosecondsType = 1_000_000_000;
const NANOSECONDS_RANGE: ops::RangeInclusive<NanosecondType> = 0..=999_999_999;

pub(crate) type YearType = i64;
pub(crate) type YearsType = i64;
pub(crate) type MonthType = u8;
pub(crate) type MonthsType = i64;
pub(crate) type DayType = u8;
pub(crate) type DaysType = i64;
pub(crate) type HourType = u8;
pub(crate) type HoursType = i64;
pub(crate) type MinuteType = u8;
pub(crate) type MinutesType = i64;
pub(crate) type SecondType = u8;
pub(crate) type SecondsType = i64;
pub(crate) type NanosecondType = u32;
pub(crate) type NanosecondsType = i64;

/// Builder for `RelativeDelta`
///
/// Batch creation and further modification of relative and constant time parameters before normalization and fixing of
/// parameters keeping them within meaningful boundaries.
///
/// You should not need to construct the builder manually but use the convenience construction methods on `RelativeDelta`.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(default))]
pub struct Builder {
	years: YearsType,
	months: MonthsType,
	months_f: f64,
	days: DaysType,
	hours: HoursType,
	minutes: MinutesType,
	seconds: SecondsType,
	nanoseconds: NanosecondsType,
	year: Option<YearType>,
	month: Option<MonthType>,
	day: Option<DayType>,
	weekday: Option<(Weekday, i64)>,
	hour: Option<HourType>,
	minute: Option<MinuteType>,
	second: Option<SecondType>,
	nanosecond: Option<NanosecondType>,
}

macro_rules! impl_and {
	($key:ident, $ty:ty, $doc_val:expr) => {
		paste! {
			#[doc = concat!("Set or update ", stringify!($key), " on the builder. Previously set values are overwritten for ", stringify!($key), ".")]
			#[doc = "# Arguments"]
			#[doc = concat!("* `", stringify!($key), "` - The ", stringify!($key), " to set.")]
			#[doc = "# Returns"]
			#[doc = "Self by value to further chain value-setting calls using the `and_*()` methods on the builder."]
			#[doc = "# Examples"]
			#[doc = "```"]
			#[doc = concat!("# use relativedelta::{RelativeDelta, Weekday};")]
			#[doc = concat!("let rd = RelativeDelta::new_builder().and_", stringify!($key), "(", stringify!($doc_val) ,").build();")]
			#[doc = concat!("assert_eq!(rd.", stringify!($key), "(), ", stringify!($doc_val), ");")]
			#[doc = "```"]
			#[inline]
			#[must_use]
			pub const fn [<and_ $key>](mut self, $key: $ty) -> Self {
				self.$key = $key;
				self
			}
		}
	};
}

impl Builder {
	// mut Relatives
	impl_and!(year, Option<YearType>, Some(2020));

	impl_and!(years, YearsType, 39);

	impl_and!(month, Option<MonthType>, Some(3));

	impl_and!(months, MonthsType, 6);

	impl_and!(months_f, f64, 0.5);

	impl_and!(day, Option<DayType>, Some(6));

	impl_and!(days, DaysType, 6);

	impl_and!(weekday, Option<(Weekday, i64)>, Some((Weekday::Fri, 2)));

	impl_and!(hour, Option<HourType>, Some(12));

	impl_and!(hours, HoursType, 12);

	impl_and!(minute, Option<MinuteType>, Some(30));

	impl_and!(minutes, MinutesType, 30);

	impl_and!(second, Option<SecondType>, Some(45));

	impl_and!(seconds, SecondsType, 45);

	impl_and!(nanosecond, Option<NanosecondType>, Some(123456789));

	impl_and!(nanoseconds, NanosecondsType, 123456789);

	/// Construct new `RelativeDelta`
	///
	/// Returns a fixed `RelativeDelta` where time parameters are within meaningfull boundaries.
	#[inline]
	#[deprecated(
		since = "0.3.0",
		note = "Use build() instead. This method will be removed in a future version."
	)]
	#[allow(clippy::new_ret_no_self)]
	#[must_use]
	pub fn new(&self) -> RelativeDelta {
		self.build()
	}

	/// Build a `RelativeDelta` from set parameters.
	/// This method normalizes the parameters and fixes them to be within meaningful boundaries.
	/// # Returns
	/// `RelativeDelta` with all parameters set and fixed.
	/// # Examples
	/// ```
	/// # use relativedelta::RelativeDelta;
	/// let rd = RelativeDelta::with_years(1)
	///   .and_months(3)
	///   .and_days(12)
	///   .and_hours(5)
	///   .and_minutes(30)
	///   .and_seconds(45)
	///   .and_nanoseconds(123456789)
	///   .build();
	#[must_use]
	pub fn build(&self) -> RelativeDelta {
		let mut ddt = RelativeDelta {
			years: self.years,
			months: self.months,
			months_f: self.months_f,
			days: self.days,
			hours: self.hours,
			minutes: self.minutes,
			seconds: self.seconds,
			nanoseconds: self.nanoseconds,
			year: self.year,
			month: self.month,
			day: self.day,
			weekday: self.weekday,
			hour: self.hour,
			minute: self.minute,
			second: self.second,
			nanosecond: self.nanosecond,
		};
		Self::fix(&mut ddt);
		ddt
	}

	/// Convenience method to set or update year, relative years, month, relative months, day and relative days while building a `RelativeDelta`.
	///
	/// # Arguments
	/// * `year` - Optional year to set. If None, the year is not set.
	/// * `years` - Relative years to add or substract.
	/// * `month` - Optional month to set. If None, the month is not set.
	/// * `months` - Relative months to add or substract.
	/// * `day` - Optional day to set. If None, the day is not set.
	/// * `days` - Relative days to add or substract.
	///
	/// # Returns
	/// Self by value to further chain value-setting calls using the `and_*()` methods on the builder.
	///
	/// # Examples
	/// ```
	/// # use relativedelta::RelativeDelta;
	/// let rd = RelativeDelta::new_builder().and_yysmmsdds(Some(2020), 1, Some(1), 3, None, 12).build();
	/// assert_eq!(rd.years(), 1);
	/// assert_eq!(rd.months(), 3);
	/// assert_eq!(rd.days(), 12);
	/// let rd = RelativeDelta::new_builder().and_yysmmsdds(None, -1, None, 2, Some(15), 0).build();
	/// assert_eq!(rd.years(), -1);
	/// assert_eq!(rd.months(), 2);
	/// assert_eq!(rd.day(), Some(15));
	/// assert_eq!(rd.days(), 0);
	/// ```
	#[inline]
	#[must_use]
	pub fn and_yysmmsdds(
		mut self,
		year: Option<YearType>,
		years: YearsType,
		month: Option<MonthType>,
		months: MonthsType,
		day: Option<DayType>,
		days: DaysType,
	) -> Self {
		self.year = year;
		self.years = years;
		self.month = month;
		self.months = months;
		self.day = day;
		self.days = days;
		self
	}

	/// Convenience method to set or override hour, relative hours, minute, relative minutes, second and relative seconds while building a `RelativeDelta`.
	///
	/// # Arguments
	/// * `hour` - Optional hour to set. If None, the hour is not set.
	/// * `hours` - Relative hours to add or substract.
	/// * `minute` - Optional minute to set. If None, the minute is not set.
	/// * `minutes` - Relative minutes to add or substract.
	/// * `second` - Optional second to set. If None, the second is not set.
	/// * `seconds` - Relative seconds to add or substract.
	///
	/// # Returns
	/// Self by value to further chain value-setting calls using the `and_*()` methods on the builder.
	///
	/// # Examples
	/// ```
	/// # use relativedelta::RelativeDelta;
	/// let rd = RelativeDelta::new_builder().and_hhsmmssss(Some(12), 1, Some(30), 15, Some(45), 20).build();
	/// assert_eq!(rd.hours(), 1);
	/// assert_eq!(rd.minutes(), 15);
	/// assert_eq!(rd.seconds(), 20);
	/// let rd = RelativeDelta::new_builder().and_hhsmmssss(None, -1, None, 30, Some(45), 0).build();
	/// assert_eq!(rd.hours(), -1);
	/// assert_eq!(rd.minutes(), 30);
	/// assert_eq!(rd.second(), Some(45));
	/// assert_eq!(rd.seconds(), 0);
	/// ```
	#[inline]
	#[must_use]
	pub fn and_hhsmmssss(
		mut self,
		hour: Option<HourType>,
		hours: HoursType,
		minute: Option<MinuteType>,
		minutes: MinutesType,
		second: Option<SecondType>,
		seconds: SecondsType,
	) -> Self {
		self.hour = hour;
		self.hours = hours;
		self.minute = minute;
		self.minutes = minutes;
		self.second = second;
		self.seconds = seconds;
		self
	}

	#[inline]
	fn fix(ddt: &mut RelativeDelta) {
		assert!(
			ddt.month.is_none_or(|m| MONTH_RANGE.contains(&m)),
			"invalid month {}",
			ddt.month.unwrap()
		);
		assert!(
			ddt.day.is_none_or(|d| DAY_RANGE.contains(&d)),
			"invalid day {}",
			ddt.day.unwrap()
		);
		assert!(
			ddt.hour.is_none_or(|h| HOUR_RANGE.contains(&h)),
			"invalid hour {}",
			ddt.hour.unwrap()
		);
		assert!(
			ddt.minute.is_none_or(|m| MINUTE_RANGE.contains(&m)),
			"invalid minute {}",
			ddt.minute.unwrap()
		);
		assert!(
			ddt.second.is_none_or(|s| SECOND_RANGE.contains(&s)),
			"invalid second {}",
			ddt.second.unwrap()
		);
		assert!(
			ddt
				.nanosecond
				.is_none_or(|n| NANOSECONDS_RANGE.contains(&n)),
			"invalid nanosecond {}",
			ddt.nanosecond.unwrap()
		);

		if ddt.nanoseconds.abs() >= MAX_NANOSECONDS {
			let s = ddt.nanoseconds.signum();
			let (div, rem) = (ddt.nanoseconds * s).div_rem(&MAX_NANOSECONDS);
			ddt.nanoseconds = rem * s;
			ddt.seconds += div * s;
		}
		if ddt.seconds.abs() >= MAX_SECONDS {
			let s = ddt.seconds.signum();
			let (div, rem) = (ddt.seconds * s).div_rem(&MAX_SECONDS);
			ddt.seconds = rem * s;
			ddt.minutes += div * s;
		}
		if ddt.minutes.abs() >= MAX_MINUTES {
			let s = ddt.minutes.signum();
			let (div, rem) = (ddt.minutes * s).div_rem(&MAX_MINUTES);
			ddt.minutes = rem * s;
			ddt.hours += div * s;
		}
		if ddt.hours.abs() >= MAX_HOURS {
			let s = ddt.hours.signum();
			let (div, rem) = (ddt.hours * s).div_rem(&MAX_HOURS);
			ddt.hours = rem * s;
			ddt.days += div * s;
		}
		if ddt.months.abs() >= MAX_MONTHS {
			let s = ddt.months.signum();
			let (div, rem) = (ddt.months * s).div_rem(&MAX_MONTHS);
			ddt.months = rem * s;
			ddt.years += div * s;
		}
	}

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_precision_loss)]
	fn normalize(
		years: f64,
		months: f64,
		days: f64,
		hours: f64,
		minutes: f64,
		seconds: f64,
		nanoseconds: i64,
	) -> Self {
		let years_total = years;
		let years = years.trunc();

		let months_f = (years_total - years) * MAX_MONTHS as f64;
		let months_total = months_f + months;
		let months = months_total.trunc();
		let months_remainder = months_total - months;
		// Unfortunately we might loose some days here, as we do not know the number of days in a relative month

		let days_total = days;
		let days = days_total.trunc();

		let hours_f = (days_total - days) * MAX_HOURS as f64;
		let hours_total = hours_f + hours;
		let hours = hours_total.trunc();

		let minutes_f = (hours_total - hours) * MAX_MINUTES as f64;
		let minutes_total = minutes_f + minutes;
		let minutes = minutes_total.trunc();

		let seconds_f = (minutes_total - minutes) * MAX_SECONDS as f64;
		let seconds_total = seconds_f + seconds;
		let seconds = seconds_total.trunc();

		let nanosecs_f = ((seconds_total - seconds) * 1_000_000_000_f64).trunc() as NanosecondsType;
		let nanosecs = nanosecs_f + nanoseconds;

		Self {
			years: years as YearsType,
			months: months as MonthsType,
			months_f: months_remainder,
			days: days as DaysType,
			hours: hours as HoursType,
			minutes: minutes as MinutesType,
			seconds: seconds as SecondsType,
			nanoseconds: nanosecs,
			..Self::default()
		}
	}
}

/// `RelativeDelta` holding all data about the relative delta datetime.
///
/// If the relative delta date time is simple e.g. manipulating only a sigle time parameter, use one of the convenience
/// methods to create a builder, and then call `build()` to get the final `RelativeDelta`.
///
/// The builder is convenient for an ongoing and more complex construction of `RelativeDelta`, as all time parameters are
/// normalized and only calculated once.
///
/// # Examples
///
/// Simple construction
/// ```
/// use relativedelta::RelativeDelta;
/// let years1 = RelativeDelta::with_years(1).build();
/// let months12 = RelativeDelta::with_months(12).build();
/// assert_eq!(years1, months12);
///
/// // date and time parameters are cleverly put within meaning full boundaries on creation where possible.
/// let months47 = RelativeDelta::with_months(47).build();
/// assert_eq!(months47.years(), 3);
/// assert_eq!(months47.months(), 11);
///
/// // This also eases comparison of two RelativeDeltas.
/// assert_eq!(RelativeDelta::with_months(47).build(), RelativeDelta::with_years(3).and_months(11).build());
/// ```
///
/// More complex constructions
/// ```
/// # use relativedelta::RelativeDelta;
/// // If same parameter is specified twice, only the latest is applied.
/// let months6 = RelativeDelta::with_months(12).and_months(6).build();
/// assert_eq!(months6, RelativeDelta::with_months(6).build());
///
/// // For shortcut construction of RelativeDelta, use the `yysmmsdds` and/or `hhsmmssss` methods.
/// let rddt2 = RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 3, None, 12).build();
/// let rddt = RelativeDelta::with_year(2020).and_years(1).and_month(Some(1)).and_months(3).and_days(12).build();
/// assert_eq!(rddt2, rddt);
/// ```
///
/// Implemented operators
/// ```
/// # use relativedelta::{RelativeDelta, Weekday};
/// // Two or more RelativeDeltas can be added and substracted. However, note that constants are lost in the process.
/// let lhs = RelativeDelta::yysmmsdds(Some(2020), -4, Some(1), 3, None, 0).build();
/// let rhs = RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 42, None, 0).build();
/// assert_eq!(lhs + rhs, RelativeDelta::with_years(-3).and_months(45).build());
/// assert_eq!(lhs - rhs, RelativeDelta::with_years(-5).and_months(-39).build());
/// assert_eq!(-lhs + rhs, RelativeDelta::with_years(5).and_months(39).build());
/// // The RelativeDelta can be multiplied with a f64.
/// assert_eq!(rhs * 0.5, RelativeDelta::with_years(2).and_year(Some(2020)).and_months(3).and_month(Some(1)).build());
/// assert_eq!(rhs * 0.5, 0.5 * rhs);
/// ```
/// This crates party piece is the ability to calculate dates based on already existing `chrono::DateTime`
#[cfg_attr(
	feature = "chrono",
	doc = r#"
```
# use relativedelta::{RelativeDelta, Weekday};
use chrono::{Utc, TimeZone, Datelike, DateTime};

// If one would like to get the last day of the month that one is currently in, it could be done with:
println!("{}", Utc::now() + RelativeDelta::with_months(1).and_day(Some(1)).and_days(-1).build());
// Above first sets the day of the month to the 1st, then adds a month and subtracts a day.
// If one were to get all quarters for the current year, one could do so by:
let dt = Utc.with_ymd_and_hms(2020, 1, 1,0,0,0).unwrap();
let quarters : Vec<DateTime<Utc>> = (3..=12).step_by(3).map(|month| dt + RelativeDelta::with_month(month).and_day(Some(1)).build()).collect();
assert_eq!(quarters.len(), 4);
assert_eq!(quarters[0], Utc.with_ymd_and_hms(2020, 3, 1,0,0,0).unwrap());
assert_eq!(quarters[1], Utc.with_ymd_and_hms(2020, 6, 1,0,0,0).unwrap());
assert_eq!(quarters[2], Utc.with_ymd_and_hms(2020, 9, 1,0,0,0).unwrap());
assert_eq!(quarters[3], Utc.with_ymd_and_hms(2020, 12, 1,0,0,0).unwrap());
// One could also request the first monday after one year by
let first_monday_after_one_year = RelativeDelta::with_years(1).and_weekday(Some((Weekday::Mon, 1))).build();
let d = dt + first_monday_after_one_year;
assert_eq!(d, Utc.with_ymd_and_hms(2021, 1, 4,0,0,0).unwrap());
```
"#
)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(default))]
pub struct RelativeDelta {
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "YearType::is_zero"))]
	years: YearsType,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "MonthsType::is_zero"))]
	months: MonthsType,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "f64::is_zero"))]
	months_f: f64,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "DaysType::is_zero"))]
	days: DaysType,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "HoursType::is_zero"))]
	hours: HoursType,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "MinutesType::is_zero"))]
	minutes: MinutesType,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "SecondsType::is_zero"))]
	seconds: SecondsType,
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "NanosecondsType::is_zero")
	)]
	nanoseconds: NanosecondsType,

	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	year: Option<YearType>,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	month: Option<MonthType>,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	day: Option<DayType>,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	hour: Option<HourType>,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	minute: Option<MinuteType>,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	second: Option<SecondType>,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	nanosecond: Option<NanosecondType>,
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
	weekday: Option<(Weekday, i64)>,
}

macro_rules! impl_with {
	($key:ident, opt $ty:ty, $doc_val:literal) => {
		impl_with!($key, Some($key), $ty, $doc_val, Some($doc_val));
	};
	($key:ident, $ty:ty, $doc_val:literal) => {
		impl_with!($key, $key, $ty, $doc_val, $doc_val);
	};
	($key:ident, $val:expr, $ty:ty, $doc_val:literal, $doc_val_eq:expr) => {
		paste! {
			#[doc = "Create a new `RelativeDelta` builder."]
			#[doc = concat!("Convenience method to create a `RelativeDelta` builder with the ", stringify!($key), " set.")]
			#[doc = "# Arguments"]
			#[doc = concat!("* `", stringify!($key), "` - The ", stringify!($key), " to set.")]
			#[doc = "# Returns"]
			#[doc = "The builder by value to further chain value-setting calls using the `and_*()` methods on the builder."]
			#[doc = "# Examples"]
			#[doc = "```"]
			#[doc = concat!("# use relativedelta::RelativeDelta;")]
			#[doc = concat!("let rd = RelativeDelta::with_", stringify!($key), "(", stringify!($doc_val), ").build();")]
			#[doc = concat!("assert_eq!(rd.", stringify!($key), "(), ", stringify!($doc_val_eq), ");")]
			#[doc = "```"]
			#[allow(clippy::redundant_field_names)]
			#[inline]
			#[must_use]
			pub fn [<with_ $key>]($key: $ty) -> Builder {
				Builder { $key: $val, ..Default::default() }
			}
		}
	};
}

macro_rules! impl_getter {
	($key:ident, $ty:ty) => {
		#[doc = concat!("Getter for the ", stringify!($key), " value of the RelativeDelta.")]
		#[inline]
		#[must_use]
		pub const fn $key(&self) -> $ty {
			self.$key
		}
	};
}

impl RelativeDelta {
	impl_with!(year, opt YearType, 2020);

	impl_with!(years, YearsType, 5);

	impl_with!(month, opt MonthType, 3);

	impl_with!(months, MonthsType, 3);

	impl_with!(day, opt DayType, 6);

	impl_with!(days, DaysType, 15);

	impl_with!(hour, opt HourType, 12);

	impl_with!(hours, HoursType, 12);

	impl_with!(minute, opt MinuteType, 30);

	impl_with!(minutes, MinutesType, 30);

	impl_with!(second, opt SecondType, 45);

	impl_with!(seconds, SecondsType, 45);

	impl_with!(nanosecond, opt NanosecondType, 123456789);

	impl_with!(nanoseconds, NanosecondsType, 123456789);

	impl_getter!(year, Option<YearType>);

	impl_getter!(years, YearsType);

	impl_getter!(month, Option<MonthType>);

	impl_getter!(months, MonthsType);

	impl_getter!(months_f, f64);

	impl_getter!(day, Option<DayType>);

	impl_getter!(days, DaysType);

	impl_getter!(hour, Option<HourType>);

	impl_getter!(hours, HoursType);

	impl_getter!(minute, Option<MinuteType>);

	impl_getter!(minutes, MinutesType);

	impl_getter!(second, Option<SecondType>);

	impl_getter!(seconds, SecondsType);

	impl_getter!(nanosecond, Option<NanosecondType>);

	impl_getter!(nanoseconds, NanosecondsType);

	impl_getter!(weekday, Option<(Weekday, i64)>);

	/// Create a new `RelativeDelta` builder
	/// Convenience method to create a `RelativeDelta` builder with default values.
	///
	/// # Returns
	/// A new `RelativeDelta` builder with all parameters set to zero or None.
	/// # Examples
	/// ```
	/// # use relativedelta::RelativeDelta;
	/// let rd_builder = RelativeDelta::new_builder();
	/// let rd = rd_builder.build();
	/// assert!(rd.is_empty())
	/// ```
	#[inline]
	#[must_use]
	pub fn new_builder() -> Builder {
		Builder::default()
	}

	/// Create a `RelativeDelta` builder with all parameters set to the current `RelativeDelta`.
	/// This method is useful for cloning the current `RelativeDelta` and modifying it further.
	/// # Returns
	/// A `RelativeDelta` builder with all parameters set to the current `RelativeDelta`.
	/// # Examples
	/// ```
	/// # use relativedelta::RelativeDelta;
	/// let rd = RelativeDelta::with_years(1).and_months(2).and_days(3).build();
	/// assert_eq!(rd.years(), 1);
	/// assert_eq!(rd.months(), 2);
	/// assert_eq!(rd.days(), 3);
	/// let rd_builder = rd.builder().and_years(5);
	/// let rd2 = rd_builder.build();
	/// assert_eq!(rd2.years(), 5);
	/// assert_eq!(rd2.months(), 2);
	/// assert_eq!(rd2.days(), 3);
	/// assert_ne!(rd, rd2);
	#[inline]
	#[must_use]
	pub const fn builder(&self) -> Builder {
		Builder {
			years: self.years,
			months: self.months,
			months_f: self.months_f,
			days: self.days,
			hours: self.hours,
			minutes: self.minutes,
			seconds: self.seconds,
			nanoseconds: self.nanoseconds,
			year: self.year,
			month: self.month,
			day: self.day,
			hour: self.hour,
			minute: self.minute,
			second: self.second,
			nanosecond: self.nanosecond,
			weekday: self.weekday,
		}
	}

	/// Convenience construction of a `RelativeDelta` (Builder) with all parameters set to the given values.
	/// This method normalizes the parameters and fixes them to be within meaningful boundaries.
	/// The method is mainly used when multiplying a `RelativeDelta` with a f64, as it allows for fractional years, months, days, hours, minutes and seconds.
	/// # Arguments
	/// * `years` - The number of years to add or substract.
	/// * `months` - The number of months to add or substract.
	/// * `days` - The number of days to add or substract.
	/// * `hours` - The number of hours to add or substract.
	/// * `minutes` - The number of minutes to add or substract.
	/// * `seconds` - The number of seconds to add or substract.
	/// * `nanoseconds` - The number of nanoseconds to add or substract.
	/// # Returns
	/// A `RelativeDelta` builder with all parameters set to the given values.
	/// # Examples
	/// ```
	/// # use relativedelta::RelativeDelta;
	/// let rd = RelativeDelta::ysmsdshsmsssns_f(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 123456789).build();
	/// assert_eq!(rd.years(), 1);
	/// assert_eq!(rd.months(), 2);
	/// assert_eq!(rd.days(), 3);
	/// assert_eq!(rd.hours(), 4);
	/// assert_eq!(rd.minutes(), 5);
	/// assert_eq!(rd.seconds(), 6);
	/// assert_eq!(rd.nanoseconds(), 123456789);
	/// ```
	#[inline]
	#[must_use]
	pub fn ysmsdshsmsssns_f(
		years: f64,
		months: f64,
		days: f64,
		hours: f64,
		minutes: f64,
		seconds: f64,
		nanoseconds: i64,
	) -> Builder {
		Builder::normalize(years, months, days, hours, minutes, seconds, nanoseconds)
	}

	/// Convenience construction of a `RelativeDelta` (Builder) with only date parameters
	/// # Arguments
	/// * `year` - Optional year to set. If None, the year is not set.
	/// * `years` - Relative years to add or substract.
	/// * `month` - Optional month to set. If None, the month is not set.
	/// * `months` - Relative months to add or substract.
	/// * `day` - Optional day to set. If None, the day is not set.
	/// * `days` - Relative days to add or substract.
	/// # Returns
	/// A `RelativeDelta` builder with all date parameters set to the given values.
	/// # Examples
	/// ```
	/// # use relativedelta::{RelativeDelta};
	/// let rd = RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 3, None, 12).build();
	/// assert_eq!(rd.year(), Some(2020));
	/// assert_eq!(rd.years(), 1);
	/// assert_eq!(rd.month(), Some(1));
	/// assert_eq!(rd.months(), 3);
	/// assert_eq!(rd.day(), None);
	/// assert_eq!(rd.days(), 12);
	/// ```
	#[inline]
	#[must_use]
	pub fn yysmmsdds(
		year: Option<YearType>,
		years: YearsType,
		month: Option<MonthType>,
		months: MonthsType,
		day: Option<DayType>,
		days: DaysType,
	) -> Builder {
		Builder {
			years,
			months,
			days,
			year,
			month,
			day,
			..Default::default()
		}
	}

	/// Convenience construction of a `RelativeDelta` (Builder) with only time parameters
	/// # Arguments
	/// * `hour` - Optional hour to set. If None, the hour is not set.
	/// * `hours` - Relative hours to add or substract.
	/// * `minute` - Optional minute to set. If None, the minute is not set.
	/// * `minutes` - Relative minutes to add or substract.
	/// * `second` - Optional second to set. If None, the second is not set.
	/// * `seconds` - Relative seconds to add or substract.
	/// # Returns
	/// A `RelativeDelta` builder with all time parameters set to the given values.
	/// # Examples
	/// ```
	/// # use relativedelta::{RelativeDelta};
	/// let rd = RelativeDelta::hhsmmssss(
	///   Some(12),
	///   1,
	///    Some(30),
	///   15,
	///    Some(45),
	///   20,
	/// ).build();
	/// assert_eq!(rd.hour(), Some(12));
	/// assert_eq!(rd.hours(), 1);
	/// assert_eq!(rd.minute(), Some(30));
	/// assert_eq!(rd.minutes(), 15);
	/// assert_eq!(rd.second(), Some(45));
	/// assert_eq!(rd.seconds(), 20);
	/// ```
	#[inline]
	#[must_use]
	pub fn hhsmmssss(
		hour: Option<HourType>,
		hours: HoursType,
		minute: Option<MinuteType>,
		minutes: MinutesType,
		second: Option<SecondType>,
		seconds: SecondsType,
	) -> Builder {
		Builder {
			hours,
			minutes,
			seconds,
			hour,
			minute,
			second,
			..Default::default()
		}
	}

	/// Convenience construction of a `RelativeDelta` (Builder) with a specific weekday and nth weekday.
	/// # Arguments
	/// * `weekday` - The weekday to set.
	/// * `nth` - The nth occurrence of the weekday in the month.
	/// # Returns
	/// A new `RelativeDelta` builder with the weekday and nth weekday set.
	/// # Examples
	/// ```
	/// # use relativedelta::{RelativeDelta, Weekday};
	/// let rd = RelativeDelta::with_weekday(Weekday::Mon, 1).build();
	/// assert_eq!(rd.weekday(), Some((Weekday::Mon, 1)));
	/// let rd = RelativeDelta::with_weekday(Weekday::Fri, -2).build();
	/// assert_eq!(rd.weekday(), Some((Weekday::Fri, -2)));
	/// ```
	#[inline]
	#[must_use]
	pub fn with_weekday(weekday: Weekday, nth: i64) -> Builder {
		Builder {
			weekday: Some((weekday, nth)),
			..Default::default()
		}
	}

	/// Calculate total relative months given the current months and years
	/// # Returns
	/// Total months as i64, calculated as `years * 12 + months`.
	/// # Examples
	/// ```
	/// # use relativedelta::RelativeDelta;
	/// let rd = RelativeDelta::with_years(2).and_months(6).build();
	/// assert_eq!(rd.total_months(), 30);
	/// let rd = RelativeDelta::with_years(1).and_months(0).build();
	/// assert_eq!(rd.total_months(), 12);
	/// let rd = RelativeDelta::with_years(0).and_months(5).build();
	/// assert_eq!(rd.total_months(), 5);
	/// let rd = RelativeDelta::with_years(0).and_months(0).build();
	/// assert_eq!(rd.total_months(), 0);
	/// ```
	#[inline]
	#[must_use]
	pub fn total_months(&self) -> i64 {
		self.years * 12 + self.months
	}

	/// Check if the `RelativeDelta` is empty
	/// # Returns
	/// true if the `RelativeDelta` has no set parameters, i.e. all time parameters are None or zero, false otherwise.
	/// # Examples
	/// ```
	/// # use relativedelta::RelativeDelta;
	/// let rd = RelativeDelta::new_builder().build();
	/// assert!(rd.is_empty());
	/// let rd = RelativeDelta::with_years(1).build();
	/// assert!(!rd.is_empty());
	/// ```
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.year.is_none()
			&& [self.month, self.day, self.hour, self.minute, self.second]
				.iter()
				.all(Option::is_none)
			&& self.nanosecond.is_none()
			&& self.years.is_zero()
			&& [
				self.months,
				self.days,
				self.hours,
				self.minutes,
				self.seconds,
				self.nanoseconds,
			]
			.iter()
			.all(MonthsType::is_zero)
			&& self.months_f.is_zero()
			&& self.weekday.is_none()
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RelativeDelta {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		// Deserialize into Builder and then call build()
		let builder = Builder::deserialize(deserializer)?;
		Ok(builder.build())
	}
}

/// Calculate the number of days in a month
/// # Arguments
/// * `year` - The year of the month
/// * `month` - The month of the year (1-12)
/// # Returns
/// The number of days in the month, or 0 if the month is invalid (e.g. 13).
/// # Examples
/// ```
/// # use relativedelta::relativedelta::num_days_in_month;
/// assert_eq!(num_days_in_month(2020, 1), 31); // January
/// assert_eq!(num_days_in_month(2020, 2), 29); //
/// // February in a leap year
/// assert_eq!(num_days_in_month(2021, 2), 28); // February in a non-leap year
/// assert_eq!(num_days_in_month(2020, 3), 31); //
/// ```
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_sign_loss)]
#[cfg(all(feature = "chrono", not(feature = "time")))]
#[must_use]
pub fn num_days_in_month(year: YearType, month: MonthType) -> DayType {
	chrono::NaiveDate::from_ymd_opt(year as i32, u32::from(month), 1)
		.and_then(|d| {
			d.checked_add_months(chrono::Months::new(1))
				.map(|nm| nm.signed_duration_since(d).num_days() as DayType)
		})
		.unwrap_or(0)
}

/// Calculate the number of days in a month
/// # Arguments
/// * `year` - The year of the month
/// * `month` - The month of the year (1-12)
/// # Returns
/// The number of days in the month, or 0 if the month is invalid (e.g. 13).
/// # Examples
/// ```
/// # use relativedelta::relativedelta::num_days_in_month;
/// assert_eq!(num_days_in_month(2020, 1), 31); // January
/// assert_eq!(num_days_in_month(2020, 2), 29); // February in a leap year
/// assert_eq!(num_days_in_month(2021, 2), 28); // February in a non-leap year
///
/// ```
#[allow(clippy::cast_possible_truncation)]
#[cfg(feature = "time")]
#[must_use]
pub fn num_days_in_month(year: YearType, month: MonthType) -> DayType {
	time_core::util::days_in_month(month, year as i32)
}

impl_op_ex!(-|rhs: &RelativeDelta| -> RelativeDelta {
	RelativeDelta {
		years: -rhs.years,
		months: -rhs.months,
		days: -rhs.days,
		hours: -rhs.hours,
		minutes: -rhs.minutes,
		seconds: -rhs.seconds,
		nanoseconds: -rhs.nanoseconds,
		..*rhs
	}
});

// Add (commutative)
impl_op_ex!(+|lhs: &RelativeDelta, rhs: &RelativeDelta| -> RelativeDelta {
		Builder {years: lhs.years + rhs.years, months: lhs.months + rhs.months, days: lhs.days + rhs.days, hours: lhs.hours + rhs.hours, minutes: lhs.minutes + rhs.minutes, seconds: lhs.seconds + rhs.seconds, nanoseconds: lhs.nanoseconds + rhs.nanoseconds, ..Default::default()}.build()
});

impl_op_ex!(-|lhs: &RelativeDelta, rhs: &RelativeDelta| -> RelativeDelta { -rhs + lhs });

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
fn mul(lhs: &RelativeDelta, rhs: f64) -> RelativeDelta {
	// Calculate relatives
	let years = lhs.years as f64 * rhs;
	let months = lhs.months as f64 * rhs;
	let days = lhs.days as f64 * rhs;
	let hours = lhs.hours as f64 * rhs;
	let minutes = lhs.minutes as f64 * rhs;
	let seconds = lhs.seconds as f64 * rhs;
	let nanoseconds = lhs.nanoseconds as f64 * rhs;
	let mut rddt_mul = RelativeDelta::ysmsdshsmsssns_f(
		years,
		months,
		days,
		hours,
		minutes,
		seconds,
		nanoseconds as i64,
	);
	// Copy over constants
	rddt_mul.year = lhs.year;
	rddt_mul.month = lhs.month;
	rddt_mul.day = lhs.day;
	rddt_mul.hour = lhs.hour;
	rddt_mul.minute = lhs.minute;
	rddt_mul.second = lhs.second;
	rddt_mul.nanosecond = lhs.nanosecond;
	rddt_mul.build()
}

impl_op_ex_commutative!(*|lhs: &RelativeDelta, rhs: f64| -> RelativeDelta { mul(lhs, rhs) });

impl_op_ex!(/ |lhs: &RelativeDelta, rhs: f64| -> RelativeDelta {
		let reciprocal = 1_f64 / rhs;
		lhs * reciprocal
});

impl_op_ex!(/ |lhs: &RelativeDelta, rhs: f32| -> RelativeDelta {
	lhs / f64::from(rhs)
});

impl_op_ex!(/ |lhs: &RelativeDelta, rhs: usize| -> RelativeDelta {
	let rhs = unsafe {rhs.to_f64().unwrap_unchecked()};
	lhs / rhs
});

#[cfg(test)]
mod tests {
	use super::*;
	use similar_asserts::assert_eq;

	#[test]
	fn test_negate() {
		let rd = RelativeDelta::with_year(2020).and_years(1).build();
		let negated = -rd;
		assert_eq!(negated.year, Some(2020));
		assert_eq!(negated.years, -1);
	}
	#[test]
	#[cfg(any(feature = "time", feature = "chrono"))]
	fn test_num_days_in_month() {
		assert_eq!(num_days_in_month(2000, 1), 31);
		// Year 2000 was a leap year
		assert_eq!(num_days_in_month(2000, 2), 29);
		assert_eq!(num_days_in_month(2001, 2), 28);

		assert_eq!(num_days_in_month(2000, 3), 31);
		assert_eq!(num_days_in_month(2000, 4), 30);
		assert_eq!(num_days_in_month(2000, 5), 31);
		assert_eq!(num_days_in_month(2000, 6), 30);
		assert_eq!(num_days_in_month(2000, 7), 31);
		assert_eq!(num_days_in_month(2000, 8), 31);
		assert_eq!(num_days_in_month(2000, 9), 30);
		assert_eq!(num_days_in_month(2000, 10), 31);
		assert_eq!(num_days_in_month(2000, 11), 30);
		assert_eq!(num_days_in_month(2000, 12), 31);
	}

	#[test]
	fn test_with() {
		let months6 = RelativeDelta::with_months(12).and_months(6).build();
		assert_eq!(months6.months, 6);
		assert_eq!(months6.months_f, 0.0);
		assert_eq!(months6.years, 0);
		assert_eq!(months6.year, None);
	}

	#[test]
	fn test_builder() {
		let rd = RelativeDelta::new_builder()
			.and_yysmmsdds(Some(2020), 1, Some(1), 3, None, 12)
			.and_hhsmmssss(Some(12), 1, Some(30), 15, Some(45), 20)
			.build();
		assert_eq!(rd.years, 1);
		assert_eq!(rd.months, 3);
		assert_eq!(rd.days, 12);
		assert_eq!(rd.hours, 1);
		assert_eq!(rd.minutes, 15);
		assert_eq!(rd.seconds, 20);
		assert_eq!(rd.year, Some(2020));
		assert_eq!(rd.month, Some(1));
		assert_eq!(rd.day, None);
		assert_eq!(rd.hour, Some(12));
		assert_eq!(rd.minute, Some(30));
		assert_eq!(rd.second, Some(45));
		assert_eq!(rd.nanoseconds, 0);

		let rd_rebuilt = rd.builder().build();
		assert_eq!(rd_rebuilt, rd);
	}

	#[test]
	#[cfg(feature = "serde")]
	fn test_deserialize() {
		use serde_json::json;

		let rd = RelativeDelta::with_years(1)
			.and_months(2)
			.and_days(3)
			.build();
		let serialized = serde_json::to_string(&rd).unwrap();
		let deserialized: RelativeDelta = serde_json::from_str(&serialized).unwrap();
		assert_eq!(rd, deserialized);

		let json_data = json!({
			"years": 1,
			"months": 2,
			"days": 3
		});
		let deserialized_from_json: RelativeDelta = serde_json::from_value(json_data).unwrap();
		assert_eq!(rd, deserialized_from_json);

		let json_data = json!({
			"years": 1,
			"months": 15,
			"days": 3,
			"hours": 5,
			"minutes": -65,
		});
		let deserialized_from_json: RelativeDelta = serde_json::from_value(json_data).unwrap();
		assert_eq!(deserialized_from_json.years, 2);
		assert_eq!(deserialized_from_json.months, 3);
		assert_eq!(deserialized_from_json.days, 3);
		assert_eq!(deserialized_from_json.hours, 4);
		assert_eq!(deserialized_from_json.minutes, -5);
	}
}
