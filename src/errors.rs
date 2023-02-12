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
			details: format!("Failed to find key {0} in data {1}", key, data),
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
pub struct PackagesError {
	pub data: String,
	pub errors: Vec<String>,
	details: String,
}

impl PackagesError {
	pub fn new(data: &str, errors: Vec<String>) -> PackagesError {
		let messages = errors
			.iter()
			.map(|e| e.to_string())
			.collect::<Vec<String>>()
			.join("\n -");

		PackagesError {
			data: data.to_owned(),
			errors,
			details: format!(
				"The following errors occurred while parsing multiple packages {0}: {1}",
				messages, data
			),
		}
	}
}

impl Display for PackagesError {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
		write!(formatter, "{0}", &self.details)
	}
}

impl Error for PackagesError {
	fn description(&self) -> &str {
		&self.details
	}
}

#[derive(Debug)]
pub enum APTError {
	KVError(KVError),
	ParseError(ParseError),
	MissingKeyError(MissingKeyError),
	PackagesError(PackagesError),
}

impl Error for APTError {}

impl Display for APTError {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
		match self {
			APTError::KVError(err) => write!(formatter, "{}", err),
			APTError::ParseError(err) => write!(formatter, "{}", err),
			APTError::MissingKeyError(err) => write!(formatter, "{}", err),
			APTError::PackagesError(err) => write!(formatter, "{}", err),
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

impl From<PackagesError> for APTError {
	fn from(err: PackagesError) -> APTError {
		APTError::PackagesError(err)
	}
}
