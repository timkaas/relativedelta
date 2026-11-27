// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! One table of cases, run against every date and date-time type of every enabled backend.
//!
//! Each case is written once as a backend-neutral [`Civil`] date-time. [`backend!`] implements
//! [`Backend`] for one type and generates its test, which runs every case the type can represent
//! through [`RelativeDelta::checked_add`] and every form of the `+` and `-` operators.

#![cfg(any(feature = "chrono", feature = "time", feature = "jiff"))]
#![allow(
	clippy::too_many_lines,
	reason = "the case table reads best as one list"
)]

use core::fmt::Debug;

use relativedelta::{RelativeDelta, Shift, Weekday};

/// A date-time independent of any backend, in which every case is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Civil {
	year: i32,
	month: u8,
	day: u8,
	hour: u8,
	minute: u8,
	second: u8,
	nanosecond: u32,
}

/// Returns midnight at the start of the given date.
const fn ymd(year: i32, month: u8, day: u8) -> Civil {
	Civil {
		year,
		month,
		day,
		hour: 0,
		minute: 0,
		second: 0,
		nanosecond: 0,
	}
}

impl Civil {
	/// Returns this date at the given wall-clock time.
	const fn at(self, hour: u8, minute: u8, second: u8) -> Self {
		Self {
			hour,
			minute,
			second,
			..self
		}
	}

	/// Returns this date-time with the given sub-second part.
	const fn with_nanos(self, nanosecond: u32) -> Self {
		Self {
			nanosecond,
			..self
		}
	}

	/// Returns whether this is midnight, so a date-only type can represent it.
	const fn is_midnight(self) -> bool {
		self.hour == 0 && self.minute == 0 && self.second == 0 && self.nanosecond == 0
	}
}

/// What a backend type can represent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Resolution {
	/// A calendar date, which takes only the date fields and the days of a delta.
	Date,
	/// A date and a wall-clock time to the nanosecond.
	DateTime,
}

/// A type the shared cases run against.
trait Backend: Shift + Clone + PartialEq + Debug {
	const RESOLUTION: Resolution;

	/// Returns `civil` as this type, or `None` when it is outside the type's range.
	fn from_civil(civil: Civil) -> Option<Self>;

	/// Returns `self + delta` for every combination of owned and borrowed operands, in both orders.
	fn add_forms(&self, delta: &RelativeDelta) -> [Self; 4];

	/// Returns `self - delta` for every combination of owned and borrowed operands.
	fn sub_forms(&self, delta: &RelativeDelta) -> [Self; 2];
}

/// One expectation: `delta` added to `start` gives `expected`, or `None` when the result is out of
/// range.
struct Case {
	name: &'static str,
	start: Civil,
	delta: RelativeDelta,
	expected: Option<Civil>,
}

/// Returns whether `delta` has any clock field, absolute or relative.
fn touches_clock(delta: &RelativeDelta) -> bool {
	delta.hour().is_some()
		|| delta.minute().is_some()
		|| delta.second().is_some()
		|| delta.nanosecond().is_some()
		|| delta.hours() != 0
		|| delta.minutes() != 0
		|| delta.seconds() != 0
		|| delta.nanoseconds() != 0
}

/// Returns whether a type of `resolution` can run `case`.
fn can_run(resolution: Resolution, case: &Case) -> bool {
	match resolution {
		Resolution::DateTime => true,
		Resolution::Date => {
			case.start.is_midnight()
				&& case.expected.is_none_or(Civil::is_midnight)
				&& !touches_clock(&case.delta)
		}
	}
}

/// 2020-04-28 12:35:48, a Tuesday, where most cases start.
const START: Civil = ymd(2020, 4, 28).at(12, 35, 48);

/// Returns `START`'s time on another date.
const fn at_start_time(year: i32, month: u8, day: u8) -> Civil {
	ymd(year, month, day).at(12, 35, 48)
}

/// Returns the delta that moves any date in the year to the last day of `month`.
fn month_end(month: u8) -> RelativeDelta {
	RelativeDelta::with_day(1)
		.and_days(-1)
		.and_month(Some(month))
		.and_months(1)
		.build()
}

fn cases() -> Vec<Case> {
	let case = |name, start, delta, expected| Case {
		name,
		start,
		delta,
		expected: Some(expected),
	};
	let out_of_range = |name, delta| Case {
		name,
		start: START,
		delta,
		expected: None,
	};
	let monday = ymd(2021, 5, 3);
	let weekday = |weekday, nth| RelativeDelta::with_weekday(weekday, nth).build();
	vec![
		// Relative years, months and days.
		case(
			"add a year",
			START,
			RelativeDelta::with_years(1).build(),
			at_start_time(2021, 4, 28),
		),
		case(
			"subtract a year",
			START,
			RelativeDelta::with_years(-1).build(),
			at_start_time(2019, 4, 28),
		),
		case(
			"add a month",
			START,
			RelativeDelta::with_months(1).build(),
			at_start_time(2020, 5, 28),
		),
		case(
			"add 6 months",
			START,
			RelativeDelta::with_months(6).build(),
			at_start_time(2020, 10, 28),
		),
		case(
			"subtract 6 months across a year",
			START,
			RelativeDelta::with_months(-6).build(),
			at_start_time(2019, 10, 28),
		),
		case(
			"add 47 months",
			START,
			RelativeDelta::with_months(47).build(),
			at_start_time(2024, 3, 28),
		),
		case(
			"subtract 47 months",
			START,
			RelativeDelta::with_months(-47).build(),
			at_start_time(2016, 5, 28),
		),
		case(
			"add 69 months",
			START,
			RelativeDelta::with_months(69).build(),
			at_start_time(2026, 1, 28),
		),
		case(
			"add a day",
			START,
			RelativeDelta::with_days(1).build(),
			at_start_time(2020, 4, 29),
		),
		case(
			"add 400 days",
			START,
			RelativeDelta::with_days(400).build(),
			at_start_time(2021, 6, 2),
		),
		case(
			"subtract 400 days",
			START,
			RelativeDelta::with_days(-400).build(),
			at_start_time(2019, 3, 25),
		),
		// The day is clamped to the length of the resulting month.
		case(
			"a month after Jan 31 in a leap year",
			ymd(2024, 1, 31),
			RelativeDelta::with_months(1).build(),
			ymd(2024, 2, 29),
		),
		case(
			"a month after Jan 31",
			ymd(2023, 1, 31),
			RelativeDelta::with_months(1).build(),
			ymd(2023, 2, 28),
		),
		case(
			"a year after Feb 29",
			ymd(2024, 2, 29),
			RelativeDelta::with_years(1).build(),
			ymd(2025, 2, 28),
		),
		case(
			"set day 31 in a 30-day month",
			ymd(2020, 4, 30),
			RelativeDelta::with_day(31).build(),
			ymd(2020, 4, 30),
		),
		// Absolute fields replace the matching fields before the relative ones are added.
		case(
			"set the year",
			START,
			RelativeDelta::with_year(2010).build(),
			at_start_time(2010, 4, 28),
		),
		case(
			"set a negative year",
			START,
			RelativeDelta::with_year(-1).build(),
			at_start_time(-1, 4, 28),
		),
		case(
			"set the day",
			START,
			RelativeDelta::with_day(1).build(),
			at_start_time(2020, 4, 1),
		),
		case(
			"end of March",
			START,
			month_end(3),
			at_start_time(2020, 3, 31),
		),
		case(
			"end of June",
			START,
			month_end(6),
			at_start_time(2020, 6, 30),
		),
		case(
			"end of September",
			START,
			month_end(9),
			at_start_time(2020, 9, 30),
		),
		case(
			"end of December",
			START,
			month_end(12),
			at_start_time(2020, 12, 31),
		),
		case(
			"end of March from January",
			ymd(2021, 1, 15),
			month_end(3),
			ymd(2021, 3, 31),
		),
		// Clock fields.
		case(
			"add an hour and 5 nanoseconds",
			START,
			RelativeDelta::with_hours(1).and_nanoseconds(5).build(),
			at_start_time(2020, 4, 28).at(13, 35, 48).with_nanos(5),
		),
		case(
			"add minutes and seconds across midnight",
			ymd(2020, 4, 28).at(23, 59, 0),
			RelativeDelta::with_minutes(1).and_seconds(30).build(),
			ymd(2020, 4, 29).at(0, 0, 30),
		),
		case(
			"set midnight",
			START,
			RelativeDelta::with_hour(0)
				.and_minute(Some(0))
				.and_second(Some(0))
				.build(),
			ymd(2020, 4, 28),
		),
		// A weekday moves to its nth occurrence counting the date itself, like dateutil.
		case(
			"next Monday after a year",
			START,
			RelativeDelta::with_years(1)
				.and_weekday(Some((Weekday::Mon, 1)))
				.build(),
			at_start_time(2021, 5, 3),
		),
		case(
			"previous Monday after a year",
			START,
			RelativeDelta::with_years(1)
				.and_weekday(Some((Weekday::Mon, -1)))
				.build(),
			at_start_time(2021, 4, 26),
		),
		case(
			"a Monday is its own next Monday",
			monday,
			weekday(Weekday::Mon, 1),
			monday,
		),
		case(
			"a Monday is its own previous Monday",
			monday,
			weekday(Weekday::Mon, -1),
			monday,
		),
		case(
			"nth 0 behaves as 1",
			monday,
			weekday(Weekday::Mon, 0),
			monday,
		),
		case(
			"second Monday",
			monday,
			weekday(Weekday::Mon, 2),
			ymd(2021, 5, 10),
		),
		case(
			"next Tuesday",
			monday,
			weekday(Weekday::Tue, 1),
			ymd(2021, 5, 4),
		),
		case(
			"previous Tuesday",
			monday,
			weekday(Weekday::Tue, -1),
			ymd(2021, 4, 27),
		),
		case(
			"second previous Sunday",
			monday,
			weekday(Weekday::Sun, -2),
			ymd(2021, 4, 25),
		),
		case(
			"next Friday keeps the time",
			monday.at(9, 0, 0),
			weekday(Weekday::Fri, 1),
			ymd(2021, 5, 7).at(9, 0, 0),
		),
		// Results no backend can represent.
		out_of_range(
			"a million years",
			RelativeDelta::with_years(1_000_000).build(),
		),
		out_of_range(
			"years overflow",
			RelativeDelta::with_years(i64::MAX).build(),
		),
		out_of_range("days overflow", RelativeDelta::with_days(i64::MAX).build()),
		out_of_range(
			"hours overflow",
			RelativeDelta::with_hours(i64::MAX).build(),
		),
		out_of_range("weekday overflow", weekday(Weekday::Mon, i64::MAX)),
	]
}

/// Runs every case `B` can represent, naming the case and type in each failure.
fn run<B: Backend>() {
	let type_name = core::any::type_name::<B>();
	let civil = |civil: Civil, name: &str| {
		B::from_civil(civil)
			.unwrap_or_else(|| panic!("{type_name} cannot represent {civil:?} in case {name:?}"))
	};
	for case in cases().iter().filter(|case| can_run(B::RESOLUTION, case)) {
		let start = civil(case.start, case.name);
		let expected = case.expected.map(|expected| civil(expected, case.name));
		assert_eq!(
			case.delta.checked_add(&start),
			expected.clone(),
			"{type_name}: {}",
			case.name
		);
		if let Some(expected) = expected {
			assert_eq!(
				start.add_forms(&case.delta),
				[0; 4].map(|_| expected.clone()),
				"{type_name} +: {}",
				case.name
			);
			assert_eq!(
				start.sub_forms(&-&case.delta),
				[0; 2].map(|_| expected.clone()),
				"{type_name} -: {}",
				case.name
			);
		}
	}
	if B::RESOLUTION == Resolution::Date {
		let start = civil(ymd(2020, 4, 28), "clock fields");
		let clock = RelativeDelta::with_hours(23).and_minute(Some(5)).build();
		assert_eq!(
			clock.checked_add(&start),
			Some(start.clone()),
			"{type_name} ignores clock fields"
		);
	}
}

/// Implements [`Backend`] for `$type` behind `$feature` and generates the test `$test` for it.
///
/// `$civil` names the [`Civil`] that `$from_civil` converts.
macro_rules! backend {
	($feature:literal, $test:ident, $type:ty, $resolution:expr, | $civil:ident | $from_civil:expr) => {
		#[cfg(feature = $feature)]
		impl Backend for $type {
			const RESOLUTION: Resolution = $resolution;

			fn from_civil($civil: Civil) -> Option<Self> {
				$from_civil
			}

			#[allow(
				clippy::op_ref,
				reason = "exercises every owned and borrowed operator impl"
			)]
			fn add_forms(&self, delta: &RelativeDelta) -> [Self; 4] {
				[
					self.clone() + delta.clone(),
					self + delta,
					delta.clone() + self.clone(),
					delta + self,
				]
			}

			#[allow(
				clippy::op_ref,
				reason = "exercises every owned and borrowed operator impl"
			)]
			fn sub_forms(&self, delta: &RelativeDelta) -> [Self; 2] {
				[self.clone() - delta.clone(), self - delta]
			}
		}

		#[cfg(feature = $feature)]
		#[test]
		fn $test() {
			run::<$type>();
		}
	};
}

#[cfg(feature = "chrono")]
mod chrono_civil {
	use super::Civil;

	pub fn date(civil: Civil) -> Option<chrono::NaiveDate> {
		chrono::NaiveDate::from_ymd_opt(civil.year, civil.month.into(), civil.day.into())
	}

	pub fn date_time(civil: Civil) -> Option<chrono::NaiveDateTime> {
		date(civil)?.and_hms_nano_opt(
			civil.hour.into(),
			civil.minute.into(),
			civil.second.into(),
			civil.nanosecond,
		)
	}
}

backend!(
	"chrono",
	chrono_naive_date,
	chrono::NaiveDate,
	Resolution::Date,
	|civil| chrono_civil::date(civil)
);
backend!(
	"chrono",
	chrono_naive_date_time,
	chrono::NaiveDateTime,
	Resolution::DateTime,
	|civil| chrono_civil::date_time(civil)
);
backend!(
	"chrono",
	chrono_date_time_utc,
	chrono::DateTime<chrono::Utc>,
	Resolution::DateTime,
	|civil| Some(chrono_civil::date_time(civil)?.and_utc())
);
backend!(
	"chrono",
	chrono_date_time_fixed_offset,
	chrono::DateTime<chrono::FixedOffset>,
	Resolution::DateTime,
	|civil| chrono_civil::date_time(civil)?
		.and_local_timezone(chrono::FixedOffset::east_opt(2 * 3600)?)
		.single()
);

#[cfg(feature = "time")]
mod time_civil {
	use super::Civil;

	pub fn date(civil: Civil) -> Option<time::Date> {
		time::Date::from_calendar_date(
			civil.year,
			time::Month::try_from(civil.month).ok()?,
			civil.day,
		)
		.ok()
	}

	pub fn date_time(civil: Civil) -> Option<time::PrimitiveDateTime> {
		date(civil)?
			.with_hms_nano(civil.hour, civil.minute, civil.second, civil.nanosecond)
			.ok()
	}
}

backend!("time", time_date, time::Date, Resolution::Date, |civil| {
	time_civil::date(civil)
});
backend!(
	"time",
	time_primitive_date_time,
	time::PrimitiveDateTime,
	Resolution::DateTime,
	|civil| time_civil::date_time(civil)
);
backend!(
	"time",
	time_offset_date_time,
	time::OffsetDateTime,
	Resolution::DateTime,
	|civil| Some(
		time_civil::date_time(civil)?.assume_offset(time::UtcOffset::from_hms(2, 0, 0).ok()?)
	)
);
backend!(
	"time",
	time_utc_date_time,
	time::UtcDateTime,
	Resolution::DateTime,
	|civil| Some(time_civil::date_time(civil)?.as_utc())
);

#[cfg(feature = "jiff")]
mod jiff_civil {
	use super::Civil;

	pub fn date(civil: Civil) -> Option<jiff::civil::Date> {
		jiff::civil::Date::new(
			civil.year.try_into().ok()?,
			civil.month.try_into().ok()?,
			civil.day.try_into().ok()?,
		)
		.ok()
	}

	pub fn date_time(civil: Civil) -> Option<jiff::civil::DateTime> {
		let time = jiff::civil::Time::new(
			civil.hour.try_into().ok()?,
			civil.minute.try_into().ok()?,
			civil.second.try_into().ok()?,
			civil.nanosecond.try_into().ok()?,
		)
		.ok()?;
		Some(date(civil)?.to_datetime(time))
	}
}

backend!(
	"jiff",
	jiff_date,
	jiff::civil::Date,
	Resolution::Date,
	|civil| jiff_civil::date(civil)
);
backend!(
	"jiff",
	jiff_date_time,
	jiff::civil::DateTime,
	Resolution::DateTime,
	|civil| jiff_civil::date_time(civil)
);
backend!(
	"jiff",
	jiff_timestamp,
	jiff::Timestamp,
	Resolution::DateTime,
	|civil| jiff::tz::TimeZone::UTC
		.to_timestamp(jiff_civil::date_time(civil)?)
		.ok()
);
backend!(
	"jiff",
	jiff_zoned_date_time,
	jiff::ZonedDateTime,
	Resolution::DateTime,
	|civil| jiff_civil::date_time(civil)?
		.to_zoned_datetime(jiff::tz::TimeZone::fixed(jiff::tz::offset(2)))
		.ok()
);
