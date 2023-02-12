use crate::{
	case_map::CaseMap,
	control::Control,
	errors::{APTError, MissingKeyError, PackagesError},
};
use rayon::prelude::*;
use std::ops::Index;

pub struct Package {
	pub(crate) map: CaseMap,
	pub package: String,
	pub source: Option<String>,
	pub version: String,
	pub section: Option<String>,
	pub priority: Option<String>,
	pub architecture: String,
	pub is_essential: Option<bool>,
	pub depends: Option<Vec<String>>,
	pub pre_depends: Option<Vec<String>>,
	pub recommends: Option<Vec<String>>,
	pub suggests: Option<Vec<String>>,
	pub replaces: Option<Vec<String>>,
	pub enhances: Option<Vec<String>>,
	pub breaks: Option<Vec<String>>,
	pub conflicts: Option<Vec<String>>,
	pub installed_size: Option<u64>,
	pub maintainer: Option<String>,
	pub description: Option<String>,
	pub homepage: Option<String>,
	pub built_using: Option<String>,
	pub package_type: Option<String>,
	pub tags: Option<Vec<String>>,
	pub filename: String,
	pub size: i64,
	pub md5sum: Option<String>,
	pub sha1sum: Option<String>,
	pub sha256sum: Option<String>,
	pub sha512sum: Option<String>,
	pub description_md5sum: Option<String>,
}

impl Package {
	pub fn from(data: &str) -> Result<Package, APTError> {
		let control = match Control::from(data) {
			Ok(control) => control,
			Err(err) => return Err(err),
		};

		let map = control.map;

		let filename = match map.get("Filename") {
			Some(filename) => filename.to_owned(),
			None => {
				return Err(APTError::MissingKeyError(MissingKeyError::new(
					"Filename", data,
				)))
			}
		};

		let size = match map.get("Size") {
			Some(size) => size.parse::<i64>().unwrap_or(-1),
			None => {
				return Err(APTError::MissingKeyError(MissingKeyError::new(
					"Size", data,
				)))
			}
		};

		Ok(Package {
			map: map.clone(),
			package: control.package,
			source: control.source,
			version: control.version,
			section: control.section,
			priority: control.priority,
			architecture: control.architecture,
			is_essential: control.is_essential,
			depends: control.depends,
			pre_depends: control.pre_depends,
			recommends: control.recommends,
			suggests: control.suggests,
			replaces: control.replaces,
			enhances: control.enhances,
			breaks: control.breaks,
			conflicts: control.conflicts,
			installed_size: control.installed_size,
			maintainer: control.maintainer,
			description: control.description,
			homepage: control.homepage,
			built_using: control.built_using,
			package_type: control.package_type,
			tags: control.tags,
			filename,
			size,
			md5sum: map.get("MD5Sum").cloned(),
			sha1sum: map.get("SHA1").cloned(),
			sha256sum: map.get("SHA256").cloned(),
			sha512sum: map.get("SHA512").cloned(),
			description_md5sum: map.get("Description-md5").cloned(),
		})
	}

	pub fn get(&self, key: &str) -> Option<&str> {
		self.map.get(key).map(|x| &**x)
	}
}

pub struct Packages {
	pub(crate) packages: Vec<Package>,
}

impl Packages {
	pub fn from(data: &str) -> Result<Packages, PackagesError> {
		let binding = data.replace("\r\n", "\n").replace('\0', "");
		let iter = binding.trim().split("\n\n").par_bridge().into_par_iter();

		let values = iter
			.map(|package| match Package::from(&package) {
				Ok(package) => Ok(package),
				Err(err) => Err(err.to_string()),
			})
			.collect::<Vec<Result<Package, String>>>();

		let mut packages = Vec::new();
		let mut errors = Vec::new();

		for value in values {
			match value {
				Ok(package) => packages.push(package),
				Err(err) => errors.push(err),
			}
		}

		if !errors.is_empty() {
			return Err(PackagesError::new(data, errors));
		}

		Ok(Packages { packages })
	}

	pub fn len(&self) -> usize {
		self.packages.len()
	}
}

impl Iterator for Packages {
	type Item = Package;

	fn next(&mut self) -> Option<Self::Item> {
		self.packages.pop()
	}
}

impl Index<usize> for Packages {
	type Output = Package;

	fn index(&self, index: usize) -> &Self::Output {
		&self.packages[index]
	}
}

#[cfg(test)]
mod tests {
	use super::Packages;
	use std::fs::read_to_string;

	#[test]
	fn packages_chariz() {
		let file = "./test/chariz.packages";
		let data = match read_to_string(file) {
			Ok(data) => data,
			Err(err) => panic!("Failed to read file: {}", err),
		};

		let packages = match Packages::from(&data) {
			Ok(packages) => packages,
			Err(err) => panic!("Failed to parse packages: {}", err),
		};

		let control = &packages[0];
		assert_eq!(packages.len(), 415);

		assert_eq!(control.package, "arpoison");
		assert_eq!(control.source, None);
		assert_eq!(control.version, "0.7");
		assert_eq!(control.section, Some("System".to_owned()));
		assert_eq!(control.priority, None);
		assert_eq!(control.architecture, "iphoneos-arm");
		assert_eq!(control.is_essential, None);

		assert_eq!(control.depends, Some(vec!["libnet9".to_owned()]));
		assert_eq!(control.pre_depends, None);
		assert_eq!(control.recommends, None);
		assert_eq!(control.suggests, None);
		assert_eq!(control.replaces, None);
		assert_eq!(control.enhances, None);
		assert_eq!(control.breaks, None);
		assert_eq!(control.conflicts, None);

		assert_eq!(control.installed_size, Some(88));
		assert_eq!(
			control.maintainer,
			Some("MidnightChips <midnightchips@gmail.com>".to_owned())
		);
		assert_eq!(
			control.description,
			Some("Generates user-defined ARP packets".to_owned())
		);
		assert_eq!(
			control.homepage,
			Some("http://www.arpoison.net/".to_owned())
		);
		assert_eq!(control.built_using, None);
		assert_eq!(control.package_type, None);
		assert_eq!(
			control.tags,
			Some(vec![
				"role::developer".to_owned(),
				"compatible_min::ios14.0".to_owned(),
			])
		);

		assert_eq!(control.filename, "debs/arpoison_0.7_iphoneos-arm.deb");
		assert_eq!(control.size, 9618);
		assert_eq!(
			control.md5sum,
			Some("e0be09b9f6d1c17371701d0ed6f625bf".to_owned())
		);
		assert_eq!(control.sha1sum, None);
		assert_eq!(
			control.sha256sum,
			Some("9f9f615c50e917e0ce629966899ed28ba78fa637c5de5476aac34f630ab18dd5".to_owned())
		);
		assert_eq!(control.sha512sum, None);
		assert_eq!(control.description_md5sum, None);

		assert_eq!(
			control.get("Depiction"),
			Some("https://chariz.com/get/arpoison")
		);

		assert_eq!(
			control.get("SileoDepiction"),
			Some("https://repo.chariz.com/api/sileo/package/arpoison/depiction.json")
		);

		assert_eq!(
			control.get("Author"),
			Some("MidnightChips <midnightchips@gmail.com>")
		);
	}

	#[test]
	fn packages_jammy() {
		let file = "./test/jammy.packages";
		let data = match read_to_string(file) {
			Ok(data) => data,
			Err(err) => panic!("Failed to read file: {}", err),
		};

		let packages = match Packages::from(&data) {
			Ok(packages) => packages,
			Err(err) => panic!("Failed to parse packages: {}", err),
		};

		let control = &packages[0];
		assert_eq!(packages.len(), 6132);

		assert_eq!(control.package, "accountsservice");
		assert_eq!(control.source, None);
		assert_eq!(control.version, "0.6.55-3ubuntu2");
		assert_eq!(control.section, Some("gnome".to_owned()));
		assert_eq!(control.priority, Some("optional".to_owned()));
		assert_eq!(control.architecture, "amd64");
		assert_eq!(control.is_essential, None);

		assert_eq!(
			control.depends,
			Some(vec![
				"dbus (>= 1.9.18)".to_owned(),
				"libaccountsservice0 (= 0.6.55-3ubuntu2)".to_owned(),
				"libc6 (>= 2.34)".to_owned(),
				"libglib2.0-0 (>= 2.44)".to_owned(),
				"libpolkit-gobject-1-0 (>= 0.99)".to_owned(),
			])
		);
		assert_eq!(control.pre_depends, None);
		assert_eq!(
			control.recommends,
			Some(vec!["default-logind | logind".to_owned()])
		);
		assert_eq!(
			control.suggests,
			Some(vec!["gnome-control-center".to_owned()])
		);
		assert_eq!(control.replaces, None);
		assert_eq!(control.enhances, None);
		assert_eq!(control.breaks, None);
		assert_eq!(control.conflicts, None);

		assert_eq!(control.installed_size, Some(484));
		assert_eq!(
			control.maintainer,
			Some("Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>".to_owned())
		);
		assert_eq!(
			control.description,
			Some("query and manipulate user account information".to_owned())
		);
		assert_eq!(
			control.homepage,
			Some("https://www.freedesktop.org/wiki/Software/AccountsService/".to_owned())
		);
		assert_eq!(control.built_using, None);
		assert_eq!(control.package_type, None);
		assert_eq!(control.tags, None);

		assert_eq!(
			control.filename,
			"pool/main/a/accountsservice/accountsservice_0.6.55-3ubuntu2_amd64.deb"
		);
		assert_eq!(control.size, 66304);
		assert_eq!(
			control.md5sum,
			Some("d1dc884f3b039c09d9aaa317d6614582".to_owned())
		);
		assert_eq!(
			control.sha1sum,
			Some("f0c2c870146d05b8d53cd805527e942ca793ce38".to_owned())
		);
		assert_eq!(
			control.sha256sum,
			Some("9823e2e330e3ca986440eb5117574c29c1247efc4e8e23cd3b936013dff493b1".to_owned())
		);
		assert_eq!(control.sha512sum, Some("9d816378feaa1cb1135212b416321059b86ee622eccfd3e395b863e5b2ea976244c2b2c016b44f5bf6a30f18cd04406c0193f0da13ca296aac0212975f763bd7".to_owned()));
		assert_eq!(
			control.description_md5sum,
			Some("8aeed0a03c7cd494f0c4b8d977483d7e".to_owned())
		);

		assert_eq!(control.get("Origin"), Some("Ubuntu"));

		assert_eq!(
			control.get("Original-Maintainer"),
			Some("Debian freedesktop.org maintainers <pkg-freedesktop-maintainers@lists.alioth.debian.org>")
		);

		assert_eq!(
			control.get("Bugs"),
			Some("https://bugs.launchpad.net/ubuntu/+filebug")
		);

		assert_eq!(
			control.get("Task"),
			Some("ubuntu-desktop-minimal, ubuntu-desktop, ubuntu-desktop-raspi, kubuntu-desktop, xubuntu-core, xubuntu-desktop, lubuntu-desktop, ubuntustudio-desktop-core, ubuntustudio-desktop, ubuntukylin-desktop, ubuntu-mate-core, ubuntu-mate-desktop, ubuntu-budgie-desktop, ubuntu-budgie-desktop-raspi")
		);
	}
}
