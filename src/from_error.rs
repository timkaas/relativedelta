#[derive(thiserror::Error, Debug)]
pub enum FromError {
	#[error("Expected an absolute value for year, but got None")]
	MissingYear,
	#[error("Expected an absolute value for month, but got None")]
	MissingMonth,
	#[error("Expected an absolute value for day, but got None")]
	MissingDay,
	#[error("Unable to convert RelativeDelta to NaiveDateTime due to invalid date components")]
	InvalidTimeComponents,
	#[error("Unable to convert RelativeDelta to NaiveDateTime due to invalid date components")]
	InvalidDateComponents,
}
