// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::RelativeDelta;

/// Holds the method behind [`Shift`], out of reach of other crates so that only this crate can
/// implement [`Shift`] and the method is reachable only through [`RelativeDelta::checked_add`].
pub(crate) mod sealed {
	use crate::RelativeDelta;

	pub trait Shift: Sized {
		/// Returns `self` moved by `delta`, or `None` if the result is not representable.
		fn checked_shift(&self, delta: &RelativeDelta) -> Option<Self>;
	}
}

/// A date or date-time type that a [`RelativeDelta`] can be added to with
/// [`RelativeDelta::checked_add`].
///
/// The trait is sealed: this crate implements it for each supported type, depending on the
/// enabled features.
///
/// - **chrono**: `chrono::NaiveDate`, `chrono::NaiveDateTime` and `chrono::DateTime<Tz>`.
/// - **time**: `time::Date`, `time::PrimitiveDateTime`, `time::OffsetDateTime` and
///   `time::UtcDateTime`.
/// - **jiff**: `jiff::civil::Date`, `jiff::civil::DateTime`, `jiff::Timestamp` and
///   `jiff::ZonedDateTime`.
///
/// Code that calls [`RelativeDelta::checked_add`] or the `+`/`-` operators compiles unchanged
/// when switching between these backends.
pub trait Shift: sealed::Shift {}

impl<T: sealed::Shift> Shift for T {}

impl RelativeDelta {
	/// Adds this delta to `rhs` without panicking.
	///
	/// This is the fallible form of `rhs + delta`. Absolute fields replace the matching fields of
	/// `rhs` first, then years and months are added with the day clamped to the length of the
	/// resulting month, then days through nanoseconds are added as an exact duration, and finally
	/// the weekday, if any, moves the result to its `nth` occurrence.
	///
	/// Returns `None` if the result is outside the range of `T`, if a time zone transition makes the
	/// resulting local time ambiguous or nonexistent, or if a duration overflows.
	#[cfg_attr(
		feature = "jiff",
		doc = r"

# Examples

The last day of March, from any date in the year:

```
use jiff::civil::date;
use relativedelta::RelativeDelta;

let last_of_march = RelativeDelta::with_month(3)
	.and_day(Some(1))
	.and_months(1)
	.and_days(-1)
	.build();
assert_eq!(last_of_march.checked_add(&date(2020, 4, 28)), Some(date(2020, 3, 31)));

let one_day = RelativeDelta::with_days(1).build();
assert_eq!(one_day.checked_add(&jiff::civil::Date::MAX), None);
```
"
	)]
	#[must_use]
	pub fn checked_add<T: Shift>(&self, rhs: &T) -> Option<T> {
		rhs.checked_shift(self)
	}
}
