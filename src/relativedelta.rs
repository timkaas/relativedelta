use std::ops;
use std::ops::Add;
use chrono::{Datelike, Timelike};
use num_integer::Integer;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Factory {
	pub years: i32,
	pub months: i64,
	pub months_f: f64,
	pub days: i64,
	pub hours: i64,
	pub minutes: i64,
	pub seconds: i64,
	pub nanoseconds: i64,
	pub year: Option<i32>,
	pub month: Option<u32>,
	pub day: Option<u32>,
	pub weekday: Option<(chrono::Weekday, i64)>,
	pub hour: Option<u32>,
	pub minute: Option<u32>,
	pub second: Option<u32>,
	pub nanosecond: Option<u32>,
}

impl Factory {
	pub fn new(&self) -> DateTime {
		let mut ddt = DateTime {
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

	pub fn ysmsdshsmsssns(
		years: i32,
		months: i64,
		days: i64,
		hours: i64,
		minutes: i64,
		seconds: i64,
		nanoseconds: i64,
	) -> Self {
		Self {
			years,
			months,
			days,
			hours,
			minutes,
			seconds,
			nanoseconds,
			..Self::default()
		}
	}

	pub fn ysmsdshsmsssns_f(
		years: f64,
		months: f64,
		days: f64,
		hours: f64,
		minutes: f64,
		seconds: f64,
		nanoseconds: i64,
	) -> Self {
		Self::normalize(years, months, days, hours, minutes, seconds, nanoseconds)
	}

	pub fn yysmmsdds(
		year: Option<i32>,
		years: i32,
		month: Option<u32>,
		months: i64,
		day: Option<u32>,
		days: i64,
	) -> Self {
		Self {
			year,
			years,
			month,
			months,
			day,
			days,
			..Self::default()
		}
	}

	pub fn hhsmmssss(
		hour: Option<u32>,
		hours: i64,
		minute: Option<u32>,
		minutes: i64,
		second: Option<u32>,
		seconds: i64,
	) -> Self {
		Self {
			hour,
			hours,
			minute,
			minutes,
			second,
			seconds,
			..Self::default()
		}
	}

	pub fn and_yysmmsdds(
		&mut self,
		year: Option<i32>,
		years: i32,
		month: Option<u32>,
		months: i64,
		day: Option<u32>,
		days: i64,
	) -> &mut Self {
		self.year = year;
		self.years = years;
		self.month = month;
		self.months = months;
		self.day = day;
		self.days = days;
		self
	}

	pub fn and_hhsmmssss(
		&mut self,
		hour: Option<u32>,
		hours: i64,
		minute: Option<u32>,
		minutes: i64,
		second: Option<u32>,
		seconds: i64,
	) -> &mut Self {
		self.hour = hour;
		self.hours = hours;
		self.minute = minute;
		self.minutes = minutes;
		self.second = second;
		self.seconds = seconds;
		self
	}

	// Relatives
	pub fn with_years(self, years: i32) -> Self {
		Self { years, ..self }
	}

	pub fn with_months(self, months: i64) -> Self {
		Self {
			months: months,
			..self
		}
	}

	pub fn with_days(self, days: i64) -> Self {
		Self { days: days, ..self }
	}

	pub fn with_hours(self, hours: i64) -> Self {
		Self {
			hours: hours,
			..self
		}
	}

	pub fn with_minutes(self, minutes: i64) -> Self {
		Self {
			minutes: minutes,
			..self
		}
	}

	pub fn with_nanoseconds(self, nanoseconds: i64) -> Self {
		Self {
			nanoseconds: nanoseconds,
			..self
		}
	}

	// mut Relatives
	pub fn and_years(&mut self, years: i32) -> &mut Self {
		self.years = years;
		self
	}

	pub fn and_months(&mut self, months: i64) -> &mut Self {
		self.months = months;
		self
	}

	pub fn and_days(&mut self, days: i64) -> &mut Self {
		self.days = days;
		self
	}

	pub fn and_hours(&mut self, hours: i64) -> &mut Self {
		self.hours = hours;
		self
	}

	pub fn and_minutes(&mut self, minutes: i64) -> &mut Self {
		self.minutes = minutes;
		self
	}

	pub fn and_nanoseconds(&mut self, nanoseconds: i64) -> &mut Self {
		self.nanoseconds = nanoseconds;
		self
	}

	// Constants
	pub fn with_year(self, year: i32) -> Self {
		Self {
			year: Some(year),
			..self
		}
	}

	pub fn with_month(self, month: u32) -> Self {
		assert!((1..=12).contains(&month));
		Self {
			month: Some(month),
			..self
		}
	}

	pub fn with_day(self, day: u32) -> Self {
		Self {
			day: Some(day),
			..self
		}
	}

	pub fn with_hour(self, hour: u32) -> Self {
		Self {
			hour: Some(hour),
			..self
		}
	}

	pub fn and_year(&mut self, year: i32) -> &mut Self {
		self.year = Some(year);
		self
	}

	pub fn and_month(&mut self, month: u32) -> &mut Self {
		self.month = Some(month);
		self
	}

	pub fn and_day(&mut self, day: u32) -> &mut Self {
		self.day = Some(day);
		self
	}

	pub fn and_hour(&mut self, hour: u32) -> &mut Self {
		self.hour = Some(hour);
		self
	}

	pub fn and_minute(&mut self, minute: u32) -> &mut Self {
		self.minute = Some(minute);
		self
	}

	pub fn and_second(&mut self, second: u32) -> &mut Self {
		self.second = Some(second);
		self
	}

	pub fn and_nanosecond(&mut self, nanosecond: u32) -> &mut Self {
		self.nanosecond = Some(nanosecond);
		self
	}

	fn fix(ddt: &mut DateTime) {
		assert!(
			ddt.month.map_or(true, |m| (1..=12).contains(&m)),
			"invalid month {}",
			ddt.month.unwrap()
		);
		assert!(
			ddt.day.map_or(true, |d| (1..=31).contains(&d)),
			"invalid day {}",
			ddt.day.unwrap()
		);
		assert!(
			ddt.hour.map_or(true, |h| (0..=23).contains(&h)),
			"invalid hour {}",
			ddt.hour.unwrap()
		);
		assert!(
			ddt.minute.map_or(true, |m| (0..=59).contains(&m)),
			"invalid minute {}",
			ddt.minute.unwrap()
		);
		assert!(
			ddt.second.map_or(true, |s| (0..=59).contains(&s)),
			"invalid second {}",
			ddt.second.unwrap()
		);
		assert!(
			ddt.nanosecond
					.map_or(true, |n| (0..=999_999_999).contains(&n)),
			"invalid nanosecond {}",
			ddt.nanosecond.unwrap()
		);

		if ddt.nanoseconds.abs() > 999_999_999 {
			let s = ddt.nanoseconds.signum();
			let (div, rem) = (ddt.nanoseconds * s).div_rem(&1_000_000_000);
			ddt.nanoseconds = rem * s;
			ddt.seconds += div * s;
		}
		if ddt.seconds.abs() > 59 {
			let s = ddt.seconds.signum();
			let (div, rem) = (ddt.seconds * s).div_rem(&60);
			ddt.seconds = rem * s;
			ddt.minutes += div * s;
		}
		if ddt.minutes.abs() > 59 {
			let s = ddt.minutes.signum();
			let (div, rem) = (ddt.minutes * s).div_rem(&60);
			ddt.minutes = rem * s;
			ddt.hours += div * s;
		}
		if ddt.hours.abs() > 23 {
			let s = ddt.hours.signum();
			let (div, rem) = (ddt.hours * s).div_rem(&24);
			ddt.hours = rem * s;
			ddt.days += div * s;
		}
		if ddt.months.abs() > 11 {
			let s = ddt.months.signum();
			let (div, rem) = (ddt.months * s).div_rem(&12);
			ddt.months = rem * s;
			ddt.years += (div * s) as i32;
		}
		/*
								if (self.hours or self.minutes or self.seconds or self.microseconds
												or self.hour is not None or self.minute is not None or
												self.second is not None or self.microsecond is not None):
										self._has_time = 1
								else:
										self._has_time = 0
		*/
	}

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

		let months_f = (years_total - years) * 12_f64;
		let months_total = months_f + months;
		let months = months_total.trunc();
		let months_f = months_total - months;
		// Unfortunately we might loose some days here, as we do not know the number of days in a relative month

		let days_total = days;
		let days = days_total.trunc();

		let hours_f = (days_total - days) * 24_f64;
		let hours_total = hours_f + hours;
		let hours = hours_total.trunc();

		let minutes_f = (hours_total - hours) * 60_f64;
		let minutes_total = minutes_f + minutes;
		let minutes = minutes_total.trunc();

		let seconds_f = (minutes_total - minutes) * 60_f64;
		let seconds_total = seconds_f + seconds;
		let seconds = seconds_total.trunc();

		let nanosecs_f = ((seconds_total - seconds) * 1_000_000_000_f64).trunc() as i64;
		let nanosecs = nanosecs_f + nanoseconds;


		Self {
			years: years as i32,
			months: months as i64,
			months_f,
			days: days as i64,
			hours: hours as i64,
			minutes: minutes as i64,
			seconds: seconds as i64,
			nanoseconds: nanosecs as i64,
			year: None,
			month: None,
			day: None,
			weekday: None,
			hour: None,
			minute: None,
			second: None,
			nanosecond: None,
		}
	}
}

#[cfg(feature = "serde")]
fn is_i32_zero(v: &i32) -> bool {
	*v == 0
}

#[cfg(feature = "serde")]
fn is_i64_zero(v: &i64) -> bool {
	*v == 0
}

#[cfg(feature = "serde")]
fn is_f64_zero(v: &f64) -> bool {
	*v.fract() == 0.0
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DateTime {
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "is_i32_zero"),
	serde(default)
	)]
	pub years: i32,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "is_i64_zero"),
	serde(default)
	)]
	pub months: i64,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "is_i64_zero"),
	serde(default)
	)]
	pub months_f: f64,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "is_f64_zero"),
	serde(default)
	)]
	pub days: i64,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "is_i64_zero"),
	serde(default)
	)]
	pub hours: i64,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "is_i64_zero"),
	serde(default)
	)]
	pub minutes: i64,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "is_i64_zero"),
	serde(default)
	)]
	pub seconds: i64,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "is_i64_zero"),
	serde(default)
	)]
	pub nanoseconds: i64,

	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "Option::is_none"),
	serde(default)
	)]
	pub year: Option<i32>,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "Option::is_none"),
	serde(default)
	)]
	pub month: Option<u32>,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "Option::is_none"),
	serde(default)
	)]
	pub day: Option<u32>,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "Option::is_none"),
	serde(default)
	)]
	pub hour: Option<u32>,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "Option::is_none"),
	serde(default)
	)]
	pub minute: Option<u32>,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "Option::is_none"),
	serde(default)
	)]
	pub second: Option<u32>,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "Option::is_none"),
	serde(default)
	)]
	pub nanosecond: Option<u32>,
	#[cfg_attr(
	feature = "serde",
	serde(skip_serializing_if = "Option::is_none"),
	serde(default)
	)]
	pub weekday: Option<(chrono::Weekday, i64)>,
}

impl DateTime {
	pub fn ysmsdshsmsssns_f(
		years: f64,
		months: f64,
		days: f64,
		hours: f64,
		minutes: f64,
		seconds: f64,
		nanoseconds: i64,
	) -> Factory {
		Factory::ysmsdshsmsssns_f(years, months, days, hours, minutes, seconds, nanoseconds)
	}

	pub fn yysmmsdds(
		year: Option<i32>,
		years: i32,
		month: Option<u32>,
		months: i64,
		day: Option<u32>,
		days: i64,
	) -> Factory {
		Factory::yysmmsdds(year, years, month, months, day, days)
	}

	pub fn hhsmmssss(
		hour: Option<u32>,
		hours: i64,
		minute: Option<u32>,
		minutes: i64,
		second: Option<u32>,
		seconds: i64,
	) -> Factory {
		Factory::hhsmmssss(hour, hours, minute, minutes, second, seconds)
	}
	// Relatives
	pub fn years(years: i32) -> Factory {
		Factory {
			years: years,
			..Default::default()
		}
	}

	pub fn months(months: i64) -> Factory {
		//assert!((1..=12).contains(&months.abs()));
		Factory {
			months: months,
			..Default::default()
		}
	}

	pub fn days(days: i64) -> Factory {
		//assert!((1..=31).contains(&days.abs()));
		Factory {
			days: days,
			..Default::default()
		}
	}

	pub fn hours(hours: i64) -> Factory {
		//assert!((0..=23).contains(&hours.abs()));
		Factory {
			hours: hours,
			..Default::default()
		}
	}

	pub fn minutes(minutes: i64) -> Factory {
		Factory {
			minutes: minutes,
			..Default::default()
		}
	}

	pub fn seconds(seconds: i64) -> Factory {
		Factory {
			seconds: seconds,
			..Default::default()
		}
	}

	pub fn nanoseconds(nanoseconds: i64) -> Factory {
		Factory {
			nanoseconds: nanoseconds,
			..Default::default()
		}
	}

	/// Constants
	pub fn year(year: i32) -> Factory {
		Factory {
			year: Some(year),
			..Default::default()
		}
	}

	pub fn month(month: u32) -> Factory {
		Factory {
			month: Some(month),
			..Default::default()
		}
	}

	pub fn day(day: u32) -> Factory {
		Factory {
			day: Some(day),
			..Default::default()
		}
	}

	pub fn hour(hour: u32) -> Factory {
		Factory {
			hour: Some(hour),
			..Default::default()
		}
	}

	pub fn minute(minute: u32) -> Factory {
		Factory {
			minute: Some(minute),
			..Default::default()
		}
	}

	pub fn second(second: u32) -> Factory {
		Factory {
			second: Some(second),
			..Default::default()
		}
	}

	pub fn nanosecond(nanosecond: u32) -> Factory {
		Factory {
			nanosecond: Some(nanosecond),
			..Default::default()
		}
	}

	pub fn total_months(&self) -> i64 {
		(self.years as i64) * 12 + self.months
	}
}

pub fn num_days_in_month(year: i32, month: u32) -> u32 {
	let nd = if month == 12 {
		chrono::NaiveDate::from_ymd(year + 1, 1, 1)
	} else {
		chrono::NaiveDate::from_ymd(year, month + 1, 1)
	};

	let r = nd
			.signed_duration_since(chrono::NaiveDate::from_ymd(year, month, 1))
			.num_days();
	assert!((1..=31).contains(&r));
	r as u32
}

impl_op_ex!(-|rhs: &DateTime| -> DateTime {
		DateTime {
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
// Unfortunately we have to implement them manually as we dont want to restrict ourselves on a timezone
impl_op_ex!(+ |lhs: &DateTime, rhs: &DateTime| -> DateTime {
		Factory::ysmsdshsmsssns(lhs.years + rhs.years, lhs.months + rhs.months, lhs.days + rhs.days, lhs.hours + rhs.hours, lhs.minutes + rhs.minutes, lhs.seconds + rhs.seconds, lhs.nanoseconds + rhs.nanoseconds).new()
});

impl<Tz: chrono::TimeZone> Add<&chrono::DateTime<Tz>> for &DateTime {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: &chrono::DateTime<Tz>) -> Self::Output {
		let mut year = self.year.unwrap_or(rhs.year()) + self.years;
		let month = self.month.unwrap_or(rhs.month()) as i64 + self.months;
		let (mut extra_years, mut relative_month) = month.div_rem(&12);
		if relative_month <= 0 {
			extra_years -= 1;
			relative_month = 12 + relative_month;
		}
		assert!(
			(1..=12).contains(&relative_month),
			"relative month was {}",
			relative_month
		);
		year += extra_years as i32;

		let real_month = relative_month as u32;
		// Clamp day to max number of days in calculated month
		let day = num_days_in_month(year, real_month).min(self.day.unwrap_or(rhs.day()));
		let hour = self.hour.unwrap_or(rhs.hour());
		let minute = self.minute.unwrap_or(rhs.minute());
		let second = self.second.unwrap_or(rhs.second());
		let nanosecond = self.nanosecond.unwrap_or(rhs.nanosecond());
		let td = rhs
				.timezone()
				.ymd(year, real_month, day)
				.and_hms_nano(hour, minute, second, nanosecond);
		let ret = td
				+ chrono::Duration::days(self.days)
				+ chrono::Duration::hours(self.hours)
				+ chrono::Duration::minutes(self.minutes)
				+ chrono::Duration::seconds(self.seconds)
				+ chrono::Duration::nanoseconds(self.nanoseconds);

		if self.weekday.is_none() {
			return ret;
		}

		let t = self.weekday.unwrap();
		let weekday = t.0;
		let nth = t.1;
		let mut jumpdays = (nth.abs() - 1) * 7;
		if nth > 0 {
			jumpdays += (7 - ret.weekday().num_days_from_monday()
					+ weekday.num_days_from_monday()) as i64;
		} else {
			jumpdays += ((ret.weekday().num_days_from_monday()
					- weekday.num_days_from_monday())
					% 7) as i64;
			jumpdays *= -1;
		}
		ret + chrono::Duration::days(jumpdays)
	}
}

impl<Tz: chrono::TimeZone> Add<&chrono::DateTime<Tz>> for DateTime {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: &chrono::DateTime<Tz>) -> Self::Output {
		&self + rhs
	}
}

impl<Tz: chrono::TimeZone> Add<chrono::DateTime<Tz>> for &DateTime {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: chrono::DateTime<Tz>) -> Self::Output {
		self + &rhs
	}
}

impl<Tz: chrono::TimeZone> Add<chrono::DateTime<Tz>> for DateTime {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: chrono::DateTime<Tz>) -> Self::Output {
		&self + &rhs
	}
}

impl<Tz: chrono::TimeZone> Add<&DateTime> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: &DateTime) -> Self::Output {
		rhs + self
	}
}

impl<Tz: chrono::TimeZone> Add<DateTime> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: DateTime) -> Self::Output {
		rhs + self
	}
}

impl<Tz: chrono::TimeZone> Add<&DateTime> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: &DateTime) -> Self::Output {
		rhs + self
	}
}

impl<Tz: chrono::TimeZone> Add<DateTime> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn add(self, rhs: DateTime) -> Self::Output {
		rhs + self
	}
}

/// Sub (non commutative)

impl<Tz: chrono::TimeZone> ops::Sub<&DateTime> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn sub(self, rhs: &DateTime) -> Self::Output {
		self + (-rhs)
	}
}

impl<Tz: chrono::TimeZone> ops::Sub<DateTime> for &chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn sub(self, rhs: DateTime) -> Self::Output {
		self - &rhs
	}
}

impl<Tz: chrono::TimeZone> ops::Sub<&DateTime> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn sub(self, rhs: &DateTime) -> Self::Output {
		&self - rhs
	}
}

impl<Tz: chrono::TimeZone> ops::Sub<DateTime> for chrono::DateTime<Tz> {
	type Output = chrono::DateTime<Tz>;

	fn sub(self, rhs: DateTime) -> Self::Output {
		&self - &rhs
	}
}

fn mul(lhs: &DateTime, rhs: f64) -> DateTime {
	let years = lhs.years as f64 * rhs;
	let months = lhs.months as f64 * rhs;
	let days = lhs.days as f64 * rhs;
	let hours = lhs.hours as f64 * rhs;
	let minutes = lhs.minutes as f64 * rhs;
	let seconds = lhs.seconds as f64 * rhs;
	let nanoseconds = lhs.nanoseconds as f64 * rhs;
	let mut rddt_mul = DateTime::ysmsdshsmsssns_f(
		years,
		months,
		days,
		hours,
		minutes,
		seconds,
		nanoseconds as i64,
	);
	rddt_mul.year = lhs.year;
	rddt_mul.month = lhs.month;
	rddt_mul.day = lhs.day;
	rddt_mul.hour = lhs.hour;
	rddt_mul.minute = lhs.minute;
	rddt_mul.second = lhs.second;
	rddt_mul.nanosecond = lhs.nanosecond;
	rddt_mul.new()
}

impl_op_ex_commutative!(*|lhs: &DateTime, rhs: f64| -> DateTime { mul(lhs, rhs) });

impl_op_ex!(/ |lhs: &DateTime, rhs: &DateTime| -> f64 {
let lhst = lhs.years as i64 * 360 + lhs.months * 30 + lhs.days.min(30);
		let rhst = rhs.years as i64 * 360 + rhs.months * 30 + lhs.days.min(30);

		lhst as f64 / rhst as f64
});

impl_op_ex!(/ |lhs: &DateTime, rhs: f64| -> DateTime {
		let reciprocal = 1_f64 / rhs;
				lhs * reciprocal
});

impl_op_ex!(/ |lhs: &DateTime, rhs: f32| -> DateTime {
		lhs / (rhs as f64)
});

impl_op_ex!(/ |lhs: &DateTime, rhs: usize| -> DateTime {
		lhs / (rhs as f64)
});

impl From<DateTime> for Option<chrono::NaiveDateTime> {
	fn from(rddt: DateTime) -> Self {
		match (rddt.year, rddt.month, rddt.day) {
			(Some(year), Some(month), Some(day)) => {
				Some(chrono::NaiveDate::from_ymd(year, month, day).and_hms_nano(
					rddt.hour.unwrap_or(0),
					rddt.minute.unwrap_or(0),
					rddt.second.unwrap_or(0),
					rddt.nanosecond.unwrap_or(0),
				))
			}
			_ => None,
		}
	}
}
