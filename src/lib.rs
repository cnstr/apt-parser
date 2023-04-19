pub mod case_map;
pub mod control;
pub mod errors;
pub mod packages;
pub mod release;

pub use control::*;
pub use packages::*;
pub use release::*;

use case_map::CaseMap;
use errors::KVError;
use regex::Regex;

#[warn(clippy::all)]
#[warn(clippy::correctness)]
#[warn(clippy::suspicious)]
#[warn(clippy::pedantic)]
#[warn(clippy::style)]
#[warn(clippy::complexity)]
#[warn(clippy::perf)]

// HashMap<String, String>
pub fn parse_kv(raw_apt_data: &str) -> Result<CaseMap, KVError> {
	// clean the string
	let binding = raw_apt_data.replace("\r\n", "\n").replace('\0', "");
	let apt_data = binding.trim().split('\n');

	let mut fields = CaseMap::new();
	let mut current_key = "";

	// Compile our regex before-hand
	let regex = match Regex::new(r"^(.*?):\s(.*)$") {
		Ok(regex) => regex,
		Err(_) => return Err(KVError),
	};

	for line in apt_data {
		let line = line.trim();

		if line.is_empty() {
			continue;
		}

		if !regex.is_match(line) {
			if line.ends_with(':') {
				let mut chars = line.chars();
				chars.next_back(); // Pop the last character off

				current_key = chars.as_str();
				fields.insert(current_key, "");
				continue;
			}

			if !current_key.is_empty() {
				let existing_value = match fields.get(current_key) {
					Some(value) => value,
					None => "",
				};

				// On multiline descriptions, the '.' signifies a newline (blank)
				if line == "." {
					let updated_key = format!("{existing_value}\n\n");
					fields.insert(current_key, &updated_key);
				} else {
					let updated_key = match existing_value.ends_with('\n') {
						true => format!("{existing_value}{line}"),
						false => format!("{existing_value} {line}"),
					};

					fields.insert(current_key, &updated_key);
				}

				continue;
			}
		}

		let captures = regex.captures(line);
		if captures.is_none() {
			return Err(KVError);
		}

		let captures = captures.unwrap();
		let key = captures.get(1);
		let value = captures.get(2);

		if key.is_none() || value.is_none() {
			return Err(KVError);
		};

		let key = key.unwrap().as_str();
		let value = value.unwrap().as_str();

		if fields.contains_key(key) {
			continue;
		}

		if key.to_lowercase() == "description" && !value.is_empty() {
			let format = format!("{value}\n");
			fields.insert(key, &format);
		} else {
			if current_key.to_lowercase() == "description" {
				let existing_value = match fields.get(current_key) {
					Some(value) => value,
					None => "",
				};

				if existing_value.ends_with('\n') {
					let substring = existing_value
						.chars()
						.take(existing_value.len() - 1)
						.collect::<String>();

					fields.insert(current_key, &substring);
				}
			}

			fields.insert(key, value);
		}

		current_key = key;
	}

	Ok(fields)
}

pub fn make_array(raw_data: Option<&String>) -> Option<Vec<String>> {
	return match raw_data {
		Some(raw_data) => {
			let mut data = Vec::new();
			for line in raw_data.split(',') {
				data.push(line.trim().to_string());
			}

			Some(data)
		}
		None => None,
	};
}
