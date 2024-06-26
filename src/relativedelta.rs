// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{Datelike, Months, Timelike};
use num_integer::Integer;
use num_traits::Zero;
use std::ops;
use std::ops::Add;

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Builder for RelativeDelta
///
/// Batch creation and further modification of relative and constant time parameters before normalization and fixing of
/// parameters keeping them within meaningfull boundaries.
///
/// You should not need to construct the builder manually but use the convenience construction methods on RelativeDelta.
#[derive(Default)]
pub struct Builder {
    years: i32,
    months: i64,
    months_f: f64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    nanoseconds: i64,
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    weekday: Option<(chrono::Weekday, i64)>,
    hour: Option<u32>,
    minute: Option<u32>,
    second: Option<u32>,
    nanosecond: Option<u32>,
}

impl Builder {
    /// Construct new RelativeDelta
    ///
    /// Returns a fixed RelativeDelta where time parameters are within meaningfull boundaries.
    #[inline]
    pub fn new(&self) -> RelativeDelta {
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

    #[inline]
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

    #[inline]
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
    /// Clone the builder and set years
    #[inline]
    pub fn with_years(&self, years: i32) -> Self {
        Self { years, ..*self }
    }

    /// Clone the builder and set months
    #[inline]
    pub fn with_months(&self, months: i64) -> Self {
        Self { months, ..*self }
    }

    /// Clone the builder and set days
    #[inline]
    pub fn with_days(&self, days: i64) -> Self {
        Self { days, ..*self }
    }

    /// Clone the builder and set hours
    #[inline]
    pub fn with_hours(&self, hours: i64) -> Self {
        Self { hours, ..*self }
    }

    /// Clone the builder and set minutes
    #[inline]
    pub fn with_minutes(&self, minutes: i64) -> Self {
        Self { minutes, ..*self }
    }

    /// Clone the builder and set nanoseconds
    #[inline]
    pub fn with_nanoseconds(&self, nanoseconds: i64) -> Self {
        Self {
            nanoseconds,
            ..*self
        }
    }

    // mut Relatives
    /// Set years on mutable ref and return itself for further chaining
    #[inline]
    pub fn and_years(&mut self, years: i32) -> &mut Self {
        self.years = years;
        self
    }

    /// Set months on mutable ref and return itself for further chaining
    #[inline]
    pub fn and_months(&mut self, months: i64) -> &mut Self {
        self.months = months;
        self
    }

    /// Set month floating part on mutable ref and return itself for further chaining
    #[inline]
    pub fn and_months_f(&mut self, months_f: f64) -> &mut Self {
        self.months_f = months_f;
        self
    }

    /// Set days on mutable ref and return itself for further chaining
    #[inline]
    pub fn and_days(&mut self, days: i64) -> &mut Self {
        self.days = days;
        self
    }

    /// Set hours on mutable ref and return itself for further chaining
    #[inline]
    pub fn and_hours(&mut self, hours: i64) -> &mut Self {
        self.hours = hours;
        self
    }

    /// Set minutes on mutable ref and return itself for further chaining
    #[inline]
    pub fn and_minutes(&mut self, minutes: i64) -> &mut Self {
        self.minutes = minutes;
        self
    }

    /// Set seconds on mutable ref and return itself for further chaining
    #[inline]
    pub fn and_seconds(&mut self, seconds: i64) -> &mut Self {
        self.seconds = seconds;
        self
    }

    /// Set nanoseconds on mutable ref and return itself for further chaining
    #[inline]
    pub fn and_nanoseconds(&mut self, nanoseconds: i64) -> &mut Self {
        self.nanoseconds = nanoseconds;
        self
    }

    // Constants
    /// Clone the builder and set an optional year
    /// If year is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes year and not overwrite it
    #[inline]
    pub fn with_year(self, year: Option<i32>) -> Self {
        Self { year, ..self }
    }

    /// Clone the builder and set an optional month
    /// If month is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes month and not overwrite it
    #[inline]
    pub fn with_month(self, month: Option<u32>) -> Self {
        Self { month, ..self }
    }

    /// Clone the builder and set an optional day
    /// If day is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes day and not overwrite it
    #[inline]
    pub fn with_day(self, day: Option<u32>) -> Self {
        Self { day, ..self }
    }

    /// Clone the builder and set an optional hour
    /// If hour is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes hour and not overwrite it
    #[inline]
    pub fn with_hour(self, hour: Option<u32>) -> Self {
        Self { hour, ..self }
    }

    /// Set year on mutable ref and return itself for further chaining
    /// If year is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes year and not overwrite it
    #[inline]
    pub fn and_year(&mut self, year: Option<i32>) -> &mut Self {
        self.year = year;
        self
    }

    /// Set month on mutable ref and return itself for further chaining
    /// If month is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes month and not overwrite it
    #[inline]
    pub fn and_month(&mut self, month: Option<u32>) -> &mut Self {
        self.month = month;
        self
    }

    /// Set day on mutable ref and return itself for further chaining
    /// If day is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes day and not overwrite it
    #[inline]
    pub fn and_day(&mut self, day: Option<u32>) -> &mut Self {
        self.day = day;
        self
    }

    /// Set hour on mutable ref and return itself for further chaining
    /// If hour is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes hour and not overwrite it
    #[inline]
    pub fn and_hour(&mut self, hour: Option<u32>) -> &mut Self {
        self.hour = hour;
        self
    }

    /// Set minute on mutable ref and return itself for further chaining
    /// If minute is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes minute and not overwrite it
    #[inline]
    pub fn and_minute(&mut self, minute: Option<u32>) -> &mut Self {
        self.minute = minute;
        self
    }

    /// Set second on mutable ref and return itself for further chaining
    /// If second is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes second and not overwrite it
    #[inline]
    pub fn and_second(&mut self, second: Option<u32>) -> &mut Self {
        self.second = second;
        self
    }

    /// Set nanosecond on mutable ref and return itself for further chaining
    /// If nanosecond is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes nanosecond and not overwrite it
    #[inline]
    pub fn and_nanosecond(&mut self, nanosecond: Option<u32>) -> &mut Self {
        self.nanosecond = nanosecond;
        self
    }

    /// Set weekday on mutable ref and return itself for further chaining
    /// If weekday is set to None, addition with e.g. chrono::DateTime will just keep the DateTimes weekday and not overwrite it
    #[inline]
    pub fn and_weekday(&mut self, weekday_nth: Option<(chrono::Weekday, i64)>) -> &mut Self {
        self.weekday = weekday_nth;
        self
    }

    #[inline]
    fn fix(ddt: &mut RelativeDelta) {
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
        let months_remainder = months_total - months;
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
            months_f: months_remainder,
            days: days as i64,
            hours: hours as i64,
            minutes: minutes as i64,
            seconds: seconds as i64,
            nanoseconds: nanosecs as i64,
            ..Self::default()
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
    v.fract() == 0.0
}

/// RelativeDelta holding all data about the relative delta datetime.
///
/// If the relative delta date time is simple e.g. manipulating only a sigle time parameter, use one of the convenience
/// methods to create a builder, and then call new to get the final RelativeDelta.
///
/// The builder is convenient for an ongoing and more complex construction of RelativeDelta, as all time parameters are
/// normalized and only calculated once.
///
/// After creation the RelativeDelta can be added or substracted with itself or a chrono::DateTime object.
/// Multiplication with f64 is possible as well. All operators are commutative.
///
/// # Examples
///
/// Simple construction
/// ```edition2018
/// use chrono::{Utc, TimeZone, Datelike};
/// use relativedelta::RelativeDelta;
///
/// let years1 = RelativeDelta::with_years(1).new();
///
/// let months12 = RelativeDelta::with_months(12).new();
/// assert_eq!(years1, months12);
///
/// // date and time parameters are cleverly put within meaning full boundaries on creation where possible.
/// let months47 = RelativeDelta::with_months(47).new();
/// assert_eq!(months47.years(), 3);
/// assert_eq!(months47.months(), 11);
///
/// // This also eases comparison of two RelativeDeltas.
/// assert_eq!(RelativeDelta::with_months(47).new(), RelativeDelta::with_years(3).and_months(11).new());
///
/// ```
///
/// More complex constructions
/// ```edition2018
/// # use chrono::{Utc, TimeZone, Datelike};
/// # use relativedelta::RelativeDelta;
///
/// // The and_parm methods should be prefered when possible as it works on mutable references and updates the Builder
/// // in place, where as the with_param methods creates copies and works on immutable references.
/// let years1 = RelativeDelta::with_years(1).and_days(32).new();
///
/// // If same parameter is specified twice, only the latest is applied.
/// let months6 = RelativeDelta::with_months(12).with_months(6).new();
/// assert_eq!(months6, RelativeDelta::with_months(6).new());
///
/// // Below is identical to: RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 3, None, 12).new();
/// let rddt = RelativeDelta::with_year(2020).and_years(1).and_month(Some(1)).and_months(3).and_days(12).new();
/// ```
///
/// Implemented operators
/// ```edition2018
/// # use chrono::{Utc, TimeZone, Datelike, DateTime, Weekday};
/// # use relativedelta::RelativeDelta;
///
/// // Two or more RelativeDeltas can be added and substracted. However, note that constants are lost in the process.
/// let lhs = RelativeDelta::yysmmsdds(Some(2020), -4, Some(1), 3, None, 0).new();
/// let rhs = RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 42, None, 0).new();
///
/// assert_eq!(lhs + rhs, RelativeDelta::with_years(-3).and_months(45).new());
/// assert_eq!(lhs - rhs, RelativeDelta::with_years(-5).and_months(-39).new());
/// assert_eq!(-lhs + rhs, RelativeDelta::with_years(5).and_months(39).new());
///
/// // The RelativeDelta can be multiplied with a f64.
/// assert_eq!(rhs * 0.5, RelativeDelta::with_years(2).and_year(Some(2020)).and_months(3).and_month(Some(1)).new());
/// # assert_eq!(rhs * 0.5, 0.5 * rhs);
///
/// // This crates party piece is the ability to calculate dates based on already existing chrono::DateTime
/// // If one would like to get the last day of the month that one is currently in, it could be done with:
/// println!("{}", Utc::now() + RelativeDelta::with_months(1).and_day(Some(1)).and_days(-1).new());
/// // Above first sets the day of the month to the 1st, then adds a month and subtracts a day.
///
/// // If one were to get all quarters for the current year, one could do so by:
/// let dt = Utc.ymd(2020, 1, 1).and_hms(0,0,0);
/// let quarters : Vec<DateTime<Utc>> = (3..=12).step_by(3).map(|month| dt + RelativeDelta::with_day(1).and_month(Some(month)).new()).collect();
/// assert_eq!(quarters.len(), 4);
/// assert_eq!(quarters[0], Utc.ymd(2020, 3, 1).and_hms(0,0,0));
/// assert_eq!(quarters[1], Utc.ymd(2020, 6, 1).and_hms(0,0,0));
/// assert_eq!(quarters[2], Utc.ymd(2020, 9, 1).and_hms(0,0,0));
/// assert_eq!(quarters[3], Utc.ymd(2020, 12, 1).and_hms(0,0,0));
///
/// // One could also request the first monday after one year by
/// let first_monday_after_one_year = RelativeDelta::with_years(1).and_weekday(Some((Weekday::Mon, 1))).new();
/// let d = dt + first_monday_after_one_year;
/// assert_eq!(d, Utc.ymd(2021, 1, 4).and_hms(0,0,0));
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelativeDelta {
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "is_i32_zero"),
        serde(default)
    )]
    years: i32,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "is_i64_zero"),
        serde(default)
    )]
    months: i64,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "is_f64_zero"),
        serde(default)
    )]
    months_f: f64,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "is_i64_zero"),
        serde(default)
    )]
    days: i64,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "is_i64_zero"),
        serde(default)
    )]
    hours: i64,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "is_i64_zero"),
        serde(default)
    )]
    minutes: i64,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "is_i64_zero"),
        serde(default)
    )]
    seconds: i64,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "is_i64_zero"),
        serde(default)
    )]
    nanoseconds: i64,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none"),
        serde(default)
    )]
    year: Option<i32>,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none"),
        serde(default)
    )]
    month: Option<u32>,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none"),
        serde(default)
    )]
    day: Option<u32>,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none"),
        serde(default)
    )]
    hour: Option<u32>,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none"),
        serde(default)
    )]
    minute: Option<u32>,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none"),
        serde(default)
    )]
    second: Option<u32>,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none"),
        serde(default)
    )]
    nanosecond: Option<u32>,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none"),
        serde(default)
    )]
    weekday: Option<(chrono::Weekday, i64)>,
}

impl RelativeDelta {
    /// Convenience construction of a RelativeDelta (Builder) with float paramters
    ///
    /// Takes only relative date and time parameters, years, months, days, hours, minutes, seconds and nanoseconds
    /// Parameters will be normalized to ints wherever possible
    #[inline]
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

    /// Convenience construction of a RelativeDelta (Builder) with only date parameters
    #[inline]
    pub fn yysmmsdds(
        year: Option<i32>,
        years: i32,
        month: Option<u32>,
        months: i64,
        day: Option<u32>,
        days: i64,
    ) -> Builder {
        Builder {
            year,
            years,
            month,
            months,
            day,
            days,
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only time parameters
    #[inline]
    pub fn hhsmmssss(
        hour: Option<u32>,
        hours: i64,
        minute: Option<u32>,
        minutes: i64,
        second: Option<u32>,
        seconds: i64,
    ) -> Builder {
        Builder {
            hour,
            hours,
            minute,
            minutes,
            second,
            seconds,
            ..Default::default()
        }
    }

    // Relatives
    /// Convenience construction of a RelativeDelta (Builder) with only relative years parameter
    #[inline]
    pub fn with_years(years: i32) -> Builder {
        Builder {
            years,
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only relative months parameter
    #[inline]
    pub fn with_months(months: i64) -> Builder {
        Builder {
            months,
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only relative days parameter
    #[inline]
    pub fn with_days(days: i64) -> Builder {
        Builder {
            days,
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only relative hours parameter
    #[inline]
    pub fn with_hours(hours: i64) -> Builder {
        Builder {
            hours,
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only relative minutes parameter
    #[inline]
    pub fn with_minutes(minutes: i64) -> Builder {
        Builder {
            minutes,
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only relative seconds parameter
    #[inline]
    pub fn with_seconds(seconds: i64) -> Builder {
        Builder {
            seconds,
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only relative nanoseconds parameter
    #[inline]
    pub fn with_nanoseconds(nanoseconds: i64) -> Builder {
        Builder {
            nanoseconds,
            ..Default::default()
        }
    }

    // Constants
    /// Convenience construction of a RelativeDelta (Builder) with only constant year parameter
    #[inline]
    pub fn with_year(year: i32) -> Builder {
        Builder {
            year: Some(year),
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only constant month parameter
    #[inline]
    pub fn with_month(month: u32) -> Builder {
        Builder {
            month: Some(month),
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only constant day parameter
    #[inline]
    pub fn with_day(day: u32) -> Builder {
        Builder {
            day: Some(day),
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only constant hour parameter
    #[inline]
    pub fn with_hour(hour: u32) -> Builder {
        Builder {
            hour: Some(hour),
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only constant minute parameter
    #[inline]
    pub fn with_minute(minute: u32) -> Builder {
        Builder {
            minute: Some(minute),
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only constant second parameter
    #[inline]
    pub fn with_second(second: u32) -> Builder {
        Builder {
            second: Some(second),
            ..Default::default()
        }
    }

    /// Convenience construction of a RelativeDelta (Builder) with only constant nanosecond parameter
    #[inline]
    pub fn with_nanosecond(nanosecond: u32) -> Builder {
        Builder {
            nanosecond: Some(nanosecond),
            ..Default::default()
        }
    }

    #[inline]
    pub fn with_weekday(weekday: chrono::Weekday, nth: i64) -> Builder {
        Builder {
            weekday: Some((weekday, nth)),
            ..Default::default()
        }
    }

    #[inline]
    pub fn years(&self) -> i32 {
        self.years
    }

    #[inline]
    pub fn year(&self) -> Option<i32> {
        self.year
    }

    #[inline]
    pub fn months(&self) -> i64 {
        self.months
    }

    #[inline]
    pub fn month(&self) -> Option<u32> {
        self.month
    }

    #[inline]
    pub fn days(&self) -> i64 {
        self.days
    }

    #[inline]
    pub fn day(&self) -> Option<u32> {
        self.day
    }

    #[inline]
    pub fn hours(&self) -> i64 {
        self.hours
    }

    #[inline]
    pub fn hour(&self) -> Option<u32> {
        self.hour
    }

    #[inline]
    pub fn minutes(&self) -> i64 {
        self.minutes
    }

    #[inline]
    pub fn minute(&self) -> Option<u32> {
        self.minute
    }

    #[inline]
    pub fn seconds(&self) -> i64 {
        self.seconds
    }

    #[inline]
    pub fn second(&self) -> Option<u32> {
        self.second
    }

    #[inline]
    pub fn nanoseconds(&self) -> i64 {
        self.nanoseconds
    }

    #[inline]
    pub fn nanosecond(&self) -> Option<u32> {
        self.nanosecond
    }

    #[inline]
    pub fn weekday(&self) -> Option<(chrono::Weekday, i64)> {
        self.weekday
    }

    /// Calculate total months given the current months and years
    #[inline]
    pub fn total_months(&self) -> i64 {
        (self.years as i64) * 12 + self.months
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.year == None
            && [
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second,
                self.nanosecond,
            ]
            .iter()
            .all(|t| t == &None)
            && self.years == 0
            && [
                self.months,
                self.days,
                self.hours,
                self.minutes,
                self.seconds,
                self.nanoseconds,
            ]
            .iter()
            .all(|t| t == &0)
            && self.months_f.is_zero()
            && self.weekday == None
    }
}

pub fn num_days_in_month(year: i32, month: u32) -> u32 {
    chrono::NaiveDate::from_ymd_opt(year, month, 1)
        .and_then(|d| {
            d.clone()
                .checked_add_months(Months::new(1))
                .map(|nm| nm.signed_duration_since(d).num_days() as u32)
        })
        .unwrap_or(0)
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
impl_op_ex!(+ |lhs: &RelativeDelta, rhs: &RelativeDelta| -> RelativeDelta {
    Builder {years: lhs.years + rhs.years, months: lhs.months + rhs.months, days: lhs.days + rhs.days, hours: lhs.hours + rhs.hours, minutes: lhs.minutes + rhs.minutes, seconds: lhs.seconds + rhs.seconds, nanoseconds: lhs.nanoseconds + rhs.nanoseconds, ..Default::default()}.new()
});

impl_op_ex!(-|lhs: &RelativeDelta, rhs: &RelativeDelta| -> RelativeDelta { -rhs + lhs });

// Unfortunately we have to implement them manually as we dont want to restrict ourselves on a timezone
impl<Tz: chrono::TimeZone> Add<&chrono::DateTime<Tz>> for &RelativeDelta {
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

        let datetime = rhs
            .timezone()
            .with_ymd_and_hms(year, real_month, day, hour, minute, second)
            .single()
            .and_then(|d| d.with_nanosecond(nanosecond))
            .expect(&format!("Could not create DateTime from year: {year}, month: {real_month}, day: {day}, hour: {hour}, minute: {minute}, second: {second}, nano: {nanosecond}."));

        let ret = datetime
            + chrono::Duration::days(self.days)
            + chrono::Duration::hours(self.hours)
            + chrono::Duration::minutes(self.minutes)
            + chrono::Duration::seconds(self.seconds)
            + chrono::Duration::nanoseconds(self.nanoseconds);

        if let Some((weekday, nth)) = self.weekday {
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
        } else {
            ret
        }
    }
}

impl<Tz: chrono::TimeZone> Add<&chrono::DateTime<Tz>> for RelativeDelta {
    type Output = chrono::DateTime<Tz>;

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
        &self + &rhs
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

// Convenient add for builder (experimental)
/*
impl<Tz: chrono::TimeZone> Add<&chrono::DateTime<Tz>> for &Builder {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: &chrono::DateTime<Tz>) -> Self::Output {
        self.new() + rhs
    }
}

impl<Tz: chrono::TimeZone> Add<&chrono::DateTime<Tz>> for Builder {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: &chrono::DateTime<Tz>) -> Self::Output {
        &self + rhs
    }
}

impl<Tz: chrono::TimeZone> Add<chrono::DateTime<Tz>> for &Builder {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: chrono::DateTime<Tz>) -> Self::Output {
        self + &rhs
    }
}

impl<Tz: chrono::TimeZone> Add<chrono::DateTime<Tz>> for &mut Builder {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: chrono::DateTime<Tz>) -> Self::Output {
        let s: &Builder = self;
        s + &rhs
    }
}

impl<Tz: chrono::TimeZone> Add<chrono::DateTime<Tz>> for Builder {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: chrono::DateTime<Tz>) -> Self::Output {
        &self + &rhs
    }
}

impl<Tz: chrono::TimeZone> Add<&Builder> for &chrono::DateTime<Tz> {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: &Builder) -> Self::Output {
        rhs + self
    }
}

impl<Tz: chrono::TimeZone> Add<Builder> for &chrono::DateTime<Tz> {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: Builder) -> Self::Output {
        rhs + self
    }
}

impl<Tz: chrono::TimeZone> Add<&Builder> for chrono::DateTime<Tz> {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: &Builder) -> Self::Output {
        rhs + self
    }
}

impl<Tz: chrono::TimeZone> Add<&mut Builder> for chrono::DateTime<Tz> {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: &mut Builder) -> Self::Output {
        rhs + self
    }
}

impl<Tz: chrono::TimeZone> Add<Builder> for chrono::DateTime<Tz> {
    type Output = chrono::DateTime<Tz>;

    fn add(self, rhs: Builder) -> Self::Output {
        rhs + self
    }
}
*/

/// Sub (non commutative)

impl<Tz: chrono::TimeZone> ops::Sub<&RelativeDelta> for &chrono::DateTime<Tz> {
    type Output = chrono::DateTime<Tz>;

    fn sub(self, rhs: &RelativeDelta) -> Self::Output {
        self + (-rhs)
    }
}

impl<Tz: chrono::TimeZone> ops::Sub<RelativeDelta> for &chrono::DateTime<Tz> {
    type Output = chrono::DateTime<Tz>;

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
        &self - &rhs
    }
}

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
    rddt_mul.new()
}

impl_op_ex_commutative!(*|lhs: &RelativeDelta, rhs: f64| -> RelativeDelta { mul(lhs, rhs) });

/*
impl_op_ex!(/ |lhs: &RelativeDelta, rhs: &RelativeDelta| -> f64 {
    let lhst = lhs.years as i64 * 360 + lhs.months * 30 + lhs.days.min(30);
    let rhst = rhs.years as i64 * 360 + rhs.months * 30 + lhs.days.min(30);
    lhst as f64 / rhst as f64
});
*/

impl_op_ex!(/ |lhs: &RelativeDelta, rhs: f64| -> RelativeDelta {
    let reciprocal = 1_f64 / rhs;
    lhs * reciprocal
});

impl_op_ex!(/ |lhs: &RelativeDelta, rhs: f32| -> RelativeDelta {
    lhs / (rhs as f64)
});

impl_op_ex!(/ |lhs: &RelativeDelta, rhs: usize| -> RelativeDelta {
    lhs / (rhs as f64)
});

/*
impl TryFrom<RelativeDelta> for chrono::NaiveDateTime {
    type Error = ();

    fn try_from(value: RelativeDelta) -> Result<Self, Self::Error> {
        todo!()
    }
}
 */

impl From<RelativeDelta> for Option<chrono::NaiveDateTime> {
    fn from(rddt: RelativeDelta) -> Self {
        match (rddt.year, rddt.month, rddt.day) {
            (Some(year), Some(month), Some(day)) => {
                chrono::NaiveDate::from_ymd_opt(year, month, day).and_then(|d| {
                    d.and_hms_nano_opt(
                        rddt.hour.unwrap_or(0),
                        rddt.minute.unwrap_or(0),
                        rddt.second.unwrap_or(0),
                        rddt.nanosecond.unwrap_or(0),
                    )
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
}
