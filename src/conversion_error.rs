// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// The error returned when a [`RelativeDelta`](crate::RelativeDelta) is converted to a date-time
/// with `TryFrom`.
///
/// The conversion reads the delta's absolute fields: year, month and day are required, and a
/// missing hour, minute, second or nanosecond counts as zero. New variants may be added in a minor
/// release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[non_exhaustive]
pub enum ConversionError {
	/// The delta has no absolute year.
	#[error("the delta has no absolute year")]
	MissingYear,
	/// The delta has no absolute month.
	#[error("the delta has no absolute month")]
	MissingMonth,
	/// The delta has no absolute day.
	#[error("the delta has no absolute day")]
	MissingDay,
	/// The absolute year, month and day are not a date the target type can represent.
	#[error("the absolute year, month and day are not a valid date for the target type")]
	InvalidDate,
	/// The absolute hour, minute, second and nanosecond are not a valid time of day.
	#[error("the absolute hour, minute, second and nanosecond are not a valid time")]
	InvalidTime,
}
