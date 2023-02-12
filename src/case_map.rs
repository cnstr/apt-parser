use std::collections::{hash_map::Iter, HashMap};

#[derive(Debug, Clone)]
pub struct CaseMap {
	map: HashMap<String, String>,
}

impl CaseMap {
	pub fn new() -> CaseMap {
		CaseMap {
			map: HashMap::new(),
		}
	}

	fn get_proper_key<'a>(&'a self, key: &'a str) -> Option<&str> {
		if self.map.contains_key(key) {
			return Some(key);
		}

		let case_key = format!("__cased__{}", key.to_lowercase());
		if self.map.contains_key(&case_key) {
			let real_key = match self.map.get(&case_key) {
				Some(real_key) => real_key,
				None => {
					return None;
				}
			};

			return Some(real_key);
		}

		None
	}

	pub fn insert(&mut self, key: &str, value: &str) {
		let case_key = format!("__cased__{}", key.to_lowercase());
		self.map.insert(key.to_owned(), value.to_owned());
		self.map.insert(case_key, key.to_owned());
	}

	pub fn get(&self, key: &str) -> Option<&String> {
		match self.get_proper_key(key) {
			Some(key) => self.map.get(key),
			None => None,
		}
	}

	pub fn contains_key(&self, key: &str) -> bool {
		match self.get_proper_key(key) {
			Some(_) => true,
			None => false,
		}
	}

	pub fn len(&self) -> usize {
		self.map.len() / 2
	}

	pub fn iter(&self) -> Iter<'_, String, String> {
		self.map.iter()
	}
}
