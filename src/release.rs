use crate::{
	case_map::CaseMap,
	errors::{APTError, MissingKeyError, ParseError},
	parse_kv,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ReleaseHash {
	pub filename: String,
	pub hash: String,
	pub size: u64,
}

pub struct Release {
	pub(crate) map: CaseMap,
	pub architectures: Vec<String>,
	pub no_support_for_architecture_all: Option<bool>,
	pub description: Option<String>,
	pub origin: Option<String>,
	pub label: Option<String>,
	pub suite: Option<String>,
	pub version: Option<String>,
	pub codename: Option<String>,
	pub date: Option<String>,
	pub valid_until: Option<String>,
	pub components: Vec<String>,
	pub md5sum: Option<Vec<ReleaseHash>>,
	pub sha1sum: Option<Vec<ReleaseHash>>,
	pub sha256sum: Option<Vec<ReleaseHash>>,
	pub sha512sum: Option<Vec<ReleaseHash>>,
	pub not_automatic: Option<bool>,
	pub but_automatic_upgrades: Option<bool>,
	pub acquire_by_hash: Option<bool>,
	pub signed_by: Option<String>,
	pub packages_require_authorization: Option<bool>,
}

impl Release {
	pub fn from(data: &str) -> Result<Release, APTError> {
		let map = match parse_kv(data) {
			Ok(map) => map,
			Err(err) => return Err(APTError::KVError(err)),
		};

		let architectures = match map.get("Architectures") {
			Some(architectures) => architectures
				.split_whitespace()
				.map(|x| x.to_string())
				.collect(),
			None => {
				return Err(APTError::MissingKeyError(MissingKeyError::new(
					"Architectures",
					data,
				)))
			}
		};

		let components = match map.get("Components") {
			Some(components) => components
				.split_whitespace()
				.map(|x| x.to_string())
				.collect(),
			None => {
				return Err(APTError::MissingKeyError(MissingKeyError::new(
					"Components",
					data,
				)))
			}
		};

		let description = map.get("Description").map(|x| {
			if x.ends_with('\n') {
				x.trim_end_matches('\n').to_string()
			} else {
				x.to_string()
			}
		});

		const HASHES: [&str; 4] = ["MD5Sum", "SHA1", "SHA256", "SHA512"];
		let mut hash_map = HashMap::<String, Vec<ReleaseHash>>::new();

		for (key, value) in map.iter() {
			if !HASHES.contains(&key.as_str()) {
				continue;
			}

			let chunks = value.split_whitespace().collect::<Vec<&str>>();
			let chunks = chunks.chunks(3);

			let mut hashes = Vec::new();
			for chunk in chunks {
				let chunk = chunk.to_vec();
				if chunk.len() != 3 {
					return Err(APTError::ParseError(ParseError));
				}

				let size = match chunk[1].parse::<u64>() {
					Ok(size) => size,
					Err(_) => return Err(APTError::ParseError(ParseError)),
				};

				hashes.push(ReleaseHash {
					filename: chunk[2].to_string(),
					hash: chunk[0].to_string(),
					size,
				});
			}

			hash_map.insert(key.to_string(), hashes);
		}

		Ok(Release {
			map: map.clone(),
			architectures,
			no_support_for_architecture_all: map
				.get("No-Support-for-Architecture-all")
				.map(|x| x == "yes"),
			description,
			origin: map.get("Origin").cloned(),
			label: map.get("Label").cloned(),
			suite: map.get("Suite").cloned(),
			version: map.get("Version").cloned(),
			codename: map.get("Codename").cloned(),
			date: map.get("Date").cloned(),
			valid_until: map.get("Valid-Until").cloned(),
			components,
			md5sum: hash_map.get("MD5Sum").cloned(),
			sha1sum: hash_map.get("SHA1").cloned(),
			sha256sum: hash_map.get("SHA256").cloned(),
			sha512sum: hash_map.get("SHA512").cloned(),
			not_automatic: map.get("NotAutomatic").map(|x| x == "yes"),
			but_automatic_upgrades: map.get("ButAutomaticUpgrades").map(|x| x == "yes"),
			acquire_by_hash: map.get("Acquire-By-Hash").map(|x| x == "yes"),
			signed_by: map.get("Signed-By").cloned(),
			packages_require_authorization: map
				.get("Packages-Require-Authorization")
				.map(|x| x == "yes"),
		})
	}

	pub fn get(&self, key: &str) -> Option<&str> {
		self.map.get(key).map(|x| &**x)
	}
}

#[cfg(test)]
mod tests {
	use super::{Release, ReleaseHash};
	use std::fs::read_to_string;

	#[test]
	fn release_chariz() {
		let file = "./test/chariz.release";
		let data = match read_to_string(file) {
			Ok(data) => data,
			Err(err) => panic!("Failed to read file: {}", err),
		};

		let release = match Release::from(&data) {
			Ok(release) => release,
			Err(err) => panic!("Failed to parse release: {}", err),
		};

		assert_eq!(release.architectures, vec!["iphoneos-arm"]);
		assert_eq!(release.no_support_for_architecture_all, None);
		assert_eq!(
			release.description,
			Some(
				"Check out what’s new and download purchases from the Chariz marketplace!"
					.to_owned()
			)
		);
		assert_eq!(release.origin, Some("Chariz".to_owned()));
		assert_eq!(release.label, Some("Chariz".to_owned()));
		assert_eq!(release.suite, Some("stable".to_owned()));
		assert_eq!(release.version, Some("0.9".to_owned()));
		assert_eq!(release.codename, Some("hbang".to_owned()));
		assert_eq!(
			release.date,
			Some("Thu, 13 Jan 2022 07:15:42 +0000".to_owned())
		);
		assert_eq!(release.valid_until, None);
		assert_eq!(release.components, vec!["main"]);

		assert_eq!(
			release.md5sum,
			Some(vec![
				ReleaseHash {
					filename: "Packages".to_owned(),
					hash: "e95ba4e016983b6145b3de3b535bf5e9".to_owned(),
					size: 368031,
				},
				ReleaseHash {
					filename: "Packages.bz2".to_owned(),
					hash: "1c1be6a4f557dc99335cc03c2d2aec3c".to_owned(),
					size: 41023,
				},
				ReleaseHash {
					filename: "Packages.lzma".to_owned(),
					hash: "eb1e7b1c68981be1fe4eeefb7a95f393".to_owned(),
					size: 39736,
				},
				ReleaseHash {
					filename: "Packages.xz".to_owned(),
					hash: "10ad7b7937ab117be9db77b47c74eaf4".to_owned(),
					size: 39360,
				},
				ReleaseHash {
					filename: "Packages.zst".to_owned(),
					hash: "627771b17cc4b50b130cbf5b85f22965".to_owned(),
					size: 42508,
				}
			])
		);

		assert_eq!(release.sha1sum, None);
		assert_eq!(release.sha256sum, None);

		assert_eq!(
			release.sha512sum,
			Some(vec![
				ReleaseHash {
					filename: "Packages".to_owned(),
					hash: "3b7029624379049caff7181a464841fd823c8ce6a7c41c653fcddaeb3215880c5ef5c33347726a44d76c9fed6e74dd3511f9e53e497fa275db04c907c5c44ed0".to_owned(),
					size: 368031,
				},
				ReleaseHash {
					filename: "Packages.bz2".to_owned(),
					hash: "45637f123591db0c8c0483671ec7bbd73c87b8b7c4d03f0968f007a8bf413ed371c965224f2a5652054c0b4605b2766496c7d182a6b81107c032d8daf3eb20d4".to_owned(),
					size: 41023,
				},
				ReleaseHash {
					filename: "Packages.lzma".to_owned(),
					hash: "5881f263d9d8dcc99eb8aea1cc95a380d02b1f6b6512b61603f2a63b446b596cd5080f67bbe04f05c6c74c69caebc2d988eedd33d7d616d9aec17253752c4ef8".to_owned(),
					size: 39736,
				},
				ReleaseHash {
					filename: "Packages.xz".to_owned(),
					hash: "373d79126d59f28c555f4582d84836b2dd66995f6fd3d4d3c737089c1f9226ae29af5923c4ca59c848451bd7ef1b43e828c7bb96dc448482cbd3aa99a456262b".to_owned(),
					size: 39360,
				},
				ReleaseHash {
					filename: "Packages.zst".to_owned(),
					hash: "c858de6a346a1e540f426e9b14ce8680f43dcfe3fa754dc6a0c4f1c4cfb025b82819330176b6847773b38080f370868f387e435c6aeda65ced5eaf242ce4a075".to_owned(),
					size: 42508,
				}
			])
		);

		assert_eq!(release.not_automatic, None);
		assert_eq!(release.but_automatic_upgrades, None);
		assert_eq!(release.acquire_by_hash, None);
		assert_eq!(release.signed_by, None);
		assert_eq!(release.packages_require_authorization, None);
	}

	#[test]
	fn release_jammy() {
		let file = "./test/jammy.release";
		let data = match read_to_string(file) {
			Ok(data) => data,
			Err(err) => panic!("Failed to read file: {}", err),
		};

		let release = match Release::from(&data) {
			Ok(release) => release,
			Err(err) => panic!("Failed to parse release: {}", err),
		};

		assert_eq!(
			release.architectures,
			vec!["amd64", "arm64", "armhf", "i386", "ppc64el", "riscv64", "s390x"]
		);
		assert_eq!(release.no_support_for_architecture_all, None);
		assert_eq!(release.description, Some("Ubuntu Jammy 22.04".to_owned()));
		assert_eq!(release.origin, Some("Ubuntu".to_owned()));
		assert_eq!(release.label, Some("Ubuntu".to_owned()));
		assert_eq!(release.suite, Some("jammy".to_owned()));
		assert_eq!(release.version, Some("22.04".to_owned()));
		assert_eq!(release.codename, Some("jammy".to_owned()));
		assert_eq!(
			release.date,
			Some("Sat, 15 Jan 2022 22:01:06 UTC".to_owned())
		);
		assert_eq!(release.valid_until, None);
		assert_eq!(
			release.components,
			vec!["main", "restricted", "universe", "multiverse"]
		);

		// Hashes are too long to test
		assert_eq!(release.not_automatic, None);
		assert_eq!(release.but_automatic_upgrades, None);
		assert_eq!(release.acquire_by_hash, Some(true));
		assert_eq!(release.signed_by, None);
		assert_eq!(release.packages_require_authorization, None);
	}
}
