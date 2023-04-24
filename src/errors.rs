use std::{
	error::Error,
	fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct KVError;

impl Error for KVError {}

impl Display for KVError {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
		write!(formatter, "Failed to parse APT key-value data")
	}
}

#[derive(Debug)]
pub struct ParseError;

impl Error for ParseError {}

impl Display for ParseError {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
		write!(formatter, "Failed to parse an APT value")
	}
}

#[derive(Debug)]
pub struct MissingKeyError {
	pub key: String,
	pub data: String,
	details: String,
}

impl MissingKeyError {
	pub fn new(key: &str, data: &str) -> MissingKeyError {
		MissingKeyError {
			key: key.to_owned(),
			data: data.to_owned(),
			details: format!("Failed to find key {0}", key),
		}
	}
}

impl Display for MissingKeyError {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
		write!(formatter, "{}", self.details)
	}
}

impl Error for MissingKeyError {
	fn description(&self) -> &str {
		&self.details
	}
}

#[derive(Debug)]
pub enum APTError {
	KVError(KVError),
	ParseError(ParseError),
	MissingKeyError(MissingKeyError),
}

impl Error for APTError {}

impl Display for APTError {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
		match self {
			APTError::KVError(err) => write!(formatter, "{}", err),
			APTError::ParseError(err) => write!(formatter, "{}", err),
			APTError::MissingKeyError(err) => write!(formatter, "{}", err),
		}
	}
}

impl From<ParseError> for APTError {
	fn from(err: ParseError) -> APTError {
		APTError::ParseError(err)
	}
}

impl From<KVError> for APTError {
	fn from(err: KVError) -> APTError {
		APTError::KVError(err)
	}
}

impl From<MissingKeyError> for APTError {
	fn from(err: MissingKeyError) -> APTError {
		APTError::MissingKeyError(err)
	}
}
