// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// This file is partially derived from the chrono crate
// Copyright (c) 2014, Kang Seonghoon.
// Licensed under:
// - MIT license (http://opensource.org/licenses/MIT)
// - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// Modifications Copyright (c) 2025 Tim Gorm Kaas-Rasmussen Olsen/RelativeDelta
// Original source: https://github.com/chronotope/chrono/blob/main/src/weekday.rs

use core::fmt;

/// The day of week.
///
/// The order of the days of week depends on the context.
/// (This is why this type does *not* implement `PartialOrd` or `Ord` traits.)
/// One should prefer `*_from_monday` or `*_from_sunday` methods to get the correct result.
///
/// # Example
/// ```
/// use relativedelta::Weekday;
///
/// let sunday = Weekday::try_from(6).unwrap();
/// assert_eq!(sunday, Weekday::Sun);
///
/// assert_eq!(sunday.num_days_from_monday(), 6); // starts counting with Monday = 0
/// assert_eq!(sunday.number_from_monday(), 7); // starts counting with Monday = 1
///
/// assert_eq!(sunday.succ(), Weekday::Mon);
/// assert_eq!(sunday.pred(), Weekday::Sat);
/// ```
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Weekday {
	/// Monday.
	#[cfg_attr(feature = "serde", serde(alias = "Monday"))]
	Mon = 0,
	/// Tuesday.
	#[cfg_attr(feature = "serde", serde(alias = "Tuesday"))]
	Tue = 1,
	/// Wednesday.
	#[cfg_attr(feature = "serde", serde(alias = "Wednesday"))]
	Wed = 2,
	/// Thursday.
	#[cfg_attr(feature = "serde", serde(alias = "Thursday"))]
	Thu = 3,
	/// Friday.
	#[cfg_attr(feature = "serde", serde(alias = "Friday"))]
	Fri = 4,
	/// Saturday.
	#[cfg_attr(feature = "serde", serde(alias = "Saturday"))]
	Sat = 5,
	/// Sunday.
	#[cfg_attr(feature = "serde", serde(alias = "Sunday"))]
	Sun = 6,
}

impl Weekday {
	/// The next day in the week.
	///
	/// `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
	/// ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
	/// `w.succ()`: | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun` | `Mon`
	#[inline]
	#[must_use]
	pub const fn succ(&self) -> Self {
		match *self {
			Self::Mon => Self::Tue,
			Self::Tue => Self::Wed,
			Self::Wed => Self::Thu,
			Self::Thu => Self::Fri,
			Self::Fri => Self::Sat,
			Self::Sat => Self::Sun,
			Self::Sun => Self::Mon,
		}
	}

	/// The previous day in the week.
	///
	/// `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
	/// ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
	/// `w.pred()`: | `Sun` | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat`
	#[inline]
	#[must_use]
	pub const fn pred(&self) -> Self {
		match *self {
			Self::Mon => Self::Sun,
			Self::Tue => Self::Mon,
			Self::Wed => Self::Tue,
			Self::Thu => Self::Wed,
			Self::Fri => Self::Thu,
			Self::Sat => Self::Fri,
			Self::Sun => Self::Sat,
		}
	}

	/// Returns a day-of-week number starting from Monday = 1. (ISO 8601 weekday number)
	///
	/// `w`:                      | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
	/// ------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
	/// `w.number_from_monday()`: | 1     | 2     | 3     | 4     | 5     | 6     | 7
	#[inline]
	#[must_use]
	pub const fn number_from_monday(&self) -> u8 {
		self.days_since(Self::Mon) + 1
	}

	/// Returns a day-of-week number starting from Monday = 0.
	///
	/// `w`:                        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
	/// --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
	/// `w.num_days_from_monday()`: | 0     | 1     | 2     | 3     | 4     | 5     | 6
	#[inline]
	#[must_use]
	pub const fn num_days_from_monday(&self) -> u8 {
		self.days_since(Self::Mon)
	}

	/// The number of days since the given day.
	///
	/// # Examples
	///
	/// ```
	/// use relativedelta::Weekday::*;
	/// assert_eq!(Mon.days_since(Mon), 0);
	/// assert_eq!(Sun.days_since(Tue), 5);
	/// assert_eq!(Wed.days_since(Sun), 3);
	/// ```
	#[must_use]
	pub const fn days_since(&self, other: Self) -> u8 {
		let lhs = *self as u8;
		let rhs = other as u8;
		if lhs < rhs { 7 + lhs - rhs } else { lhs - rhs }
	}
}

impl fmt::Display for Weekday {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.pad(match *self {
			Self::Mon => "Mon",
			Self::Tue => "Tue",
			Self::Wed => "Wed",
			Self::Thu => "Thu",
			Self::Fri => "Fri",
			Self::Sat => "Sat",
			Self::Sun => "Sun",
		})
	}
}

/// Any weekday can be represented as an integer from 0 to 6, which equals to
/// [`Weekday::num_days_from_monday`](#method.num_days_from_monday) in this implementation.
/// Do not heavily depend on this though; use explicit methods whenever possible.
impl From<u8> for Weekday {
	fn from(value: u8) -> Self {
		let value = value % 7; // Ensure the value is within 0-6
		match value {
			0 => Self::Mon,
			1 => Self::Tue,
			2 => Self::Wed,
			3 => Self::Thu,
			4 => Self::Fri,
			5 => Self::Sat,
			6 => Self::Sun,
			_ => unreachable!(), // This case should never happen due to the modulo operation
		}
	}
}

/// Any weekday can be represented as an integer from 0 to 6, which equals to
/// [`Weekday::num_days_from_monday`](#method.num_days_from_monday) in this implementation.
/// Do not heavily depend on this though; use explicit methods whenever possible.
impl num_traits::FromPrimitive for Weekday {
	#[inline]
	fn from_i64(n: i64) -> Option<Weekday> {
		match n {
			0 => Some(Self::Mon),
			1 => Some(Self::Tue),
			2 => Some(Self::Wed),
			3 => Some(Self::Thu),
			4 => Some(Self::Fri),
			5 => Some(Self::Sat),
			6 => Some(Self::Sun),
			_ => None,
		}
	}

	#[inline]
	fn from_u64(n: u64) -> Option<Weekday> {
		match n {
			0 => Some(Self::Mon),
			1 => Some(Self::Tue),
			2 => Some(Self::Wed),
			3 => Some(Self::Thu),
			4 => Some(Self::Fri),
			5 => Some(Self::Sat),
			6 => Some(Self::Sun),
			_ => None,
		}
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use num_traits::FromPrimitive;
	use similar_asserts::assert_eq;

	#[test]
	fn test_days_since() {
		for i in 0..7 {
			let base_day = Weekday::from(i);

			assert_eq!(
				base_day.num_days_from_monday(),
				base_day.days_since(Weekday::Mon)
			);

			assert_eq!(base_day.days_since(base_day), 0);

			assert_eq!(base_day.days_since(base_day.pred()), 1);
			assert_eq!(base_day.days_since(base_day.pred().pred()), 2);
			assert_eq!(base_day.days_since(base_day.pred().pred().pred()), 3);
			assert_eq!(base_day.days_since(base_day.pred().pred().pred().pred()), 4);
			assert_eq!(
				base_day.days_since(base_day.pred().pred().pred().pred().pred()),
				5
			);
			assert_eq!(
				base_day.days_since(base_day.pred().pred().pred().pred().pred().pred()),
				6
			);

			assert_eq!(base_day.days_since(base_day.succ()), 6);
			assert_eq!(base_day.days_since(base_day.succ().succ()), 5);
			assert_eq!(base_day.days_since(base_day.succ().succ().succ()), 4);
			assert_eq!(base_day.days_since(base_day.succ().succ().succ().succ()), 3);
			assert_eq!(
				base_day.days_since(base_day.succ().succ().succ().succ().succ()),
				2
			);
			assert_eq!(
				base_day.days_since(base_day.succ().succ().succ().succ().succ().succ()),
				1
			);
		}
	}

	#[test]
	#[cfg(feature = "serde")]
	fn test_serde_serialize() {
		use Weekday::*;
		use serde_json::to_string;
		use std::vec;

		let cases: vec::Vec<(Weekday, &str)> = vec![
			(Mon, "\"Mon\""),
			(Tue, "\"Tue\""),
			(Wed, "\"Wed\""),
			(Thu, "\"Thu\""),
			(Fri, "\"Fri\""),
			(Sat, "\"Sat\""),
			(Sun, "\"Sun\""),
		];

		for (weekday, expected_str) in cases {
			let string = to_string(&weekday).unwrap();
			assert_eq!(string, expected_str);
		}
	}

	#[test]
	#[cfg(feature = "serde")]
	fn test_serde_deserialize() {
		use Weekday::*;
		use serde_json::from_str;
		use std::vec;

		let cases: vec::Vec<(&str, Weekday)> = vec![
			("\"Mon\"", Mon),
			("\"Monday\"", Mon),
			("\"Tue\"", Tue),
			("\"Tuesday\"", Tue),
			("\"Wed\"", Wed),
			("\"Wednesday\"", Wed),
			("\"Thu\"", Thu),
			("\"Thursday\"", Thu),
			("\"Fri\"", Fri),
			("\"Friday\"", Fri),
			("\"Sat\"", Sat),
			("\"Saturday\"", Sat),
			("\"Sun\"", Sun),
			("\"Sunday\"", Sun),
		];

		for (str, expected_weekday) in cases {
			let weekday = from_str::<Weekday>(str).unwrap();
			assert_eq!(weekday, expected_weekday);
		}

		let errors: vec::Vec<&str> = vec![
			"\"not a weekday\"",
			"\"monDAYs\"",
			"\"mond\"",
			"mon",
			"\"thur\"",
			"\"thurs\"",
		];

		for str in errors {
			from_str::<Weekday>(str).unwrap_err();
		}
	}

	#[test]
	fn test_number_from_monday() {
		for i in 0..7 {
			let day = Weekday::from(i);
			assert_eq!(day.number_from_monday(), i + 1);
			assert_eq!(day.num_days_from_monday(), i);
		}
	}

	#[test]
	fn test_from_u64() {
		assert_eq!(Weekday::from_u64(0), Some(Weekday::Mon));
		assert_eq!(Weekday::from_u64(1), Some(Weekday::Tue));
		assert_eq!(Weekday::from_u64(2), Some(Weekday::Wed));
		assert_eq!(Weekday::from_u64(3), Some(Weekday::Thu));
		assert_eq!(Weekday::from_u64(4), Some(Weekday::Fri));
		assert_eq!(Weekday::from_u64(5), Some(Weekday::Sat));
		assert_eq!(Weekday::from_u64(6), Some(Weekday::Sun));
		assert_eq!(Weekday::from_u64(7), None);
	}

	#[test]
	fn test_from_i64() {
		assert_eq!(Weekday::from_i64(0), Some(Weekday::Mon));
		assert_eq!(Weekday::from_i64(1), Some(Weekday::Tue));
		assert_eq!(Weekday::from_i64(2), Some(Weekday::Wed));
		assert_eq!(Weekday::from_i64(3), Some(Weekday::Thu));
		assert_eq!(Weekday::from_i64(4), Some(Weekday::Fri));
		assert_eq!(Weekday::from_i64(5), Some(Weekday::Sat));
		assert_eq!(Weekday::from_i64(6), Some(Weekday::Sun));
		assert_eq!(Weekday::from_i64(7), None);
	}

	#[test]
	#[cfg(feature = "schemars")]
	fn test_schemars_schema() {
		use schemars::JsonSchema;
		use schemars::generate::SchemaGenerator;

		let mut generator = SchemaGenerator::default();
		let schema = Weekday::json_schema(&mut generator);

		// Verify schema is generated successfully with enum variants
		// Enums are represented with oneOf containing const values
		let one_of = schema
			.get("oneOf")
			.and_then(|v| v.as_array())
			.expect("Schema should have oneOf variants");

		assert!(
			one_of.len() >= 7,
			"Schema should contain at least 7 weekday variants"
		);

		// Extract the const values from each variant
		let variants: Vec<String> = one_of
			.iter()
			.filter_map(|v| v.get("const").and_then(|c| c.as_str()).map(String::from))
			.collect();

		// Verify all weekday variants are in the schema
		assert!(
			variants.contains(&"Mon".to_string()),
			"Schema should contain Mon variant"
		);
		assert!(
			variants.contains(&"Tue".to_string()),
			"Schema should contain Tue variant"
		);
		assert!(
			variants.contains(&"Wed".to_string()),
			"Schema should contain Wed variant"
		);
		assert!(
			variants.contains(&"Thu".to_string()),
			"Schema should contain Thu variant"
		);
		assert!(
			variants.contains(&"Fri".to_string()),
			"Schema should contain Fri variant"
		);
		assert!(
			variants.contains(&"Sat".to_string()),
			"Schema should contain Sat variant"
		);
		assert!(
			variants.contains(&"Sun".to_string()),
			"Schema should contain Sun variant"
		);
	}

	#[test]
	#[cfg(all(feature = "schemars", feature = "serde"))]
	fn test_schemars_serde_compatibility() {
		use schemars::JsonSchema;
		use schemars::generate::SchemaGenerator;

		// Create instance and verify it can be serialized while schema is defined
		let weekday = Weekday::Mon;

		// Verify schema generation doesn't fail with serde enabled
		let mut generator = SchemaGenerator::default();
		let schema = Weekday::json_schema(&mut generator);
		assert!(
			schema.get("oneOf").is_some(),
			"Schema should generate successfully with serde"
		);

		// Verify serialization works with schema available
		let serialized = serde_json::to_string(&weekday).unwrap();
		let deserialized: Weekday = serde_json::from_str(&serialized).unwrap();
		assert_eq!(weekday, deserialized, "Serde + Schemars should work together");
	}
}
