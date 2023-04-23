use crate::{
	case_map::CaseMap,
	errors::{APTError, MissingKeyError, ParseError},
	make_array, parse_kv,
};

pub struct Control {
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
	pub installed_size: Option<i64>,
	pub maintainer: Option<String>,
	pub description: Option<String>,
	pub homepage: Option<String>,
	pub built_using: Option<String>,
	pub package_type: Option<String>,
	pub tags: Option<Vec<String>>,
}

impl Control {
	pub fn from(data: &str) -> Result<Control, APTError> {
		let map = match parse_kv(data) {
			Ok(map) => map,
			Err(err) => return Err(APTError::KVError(err)),
		};

		let package = match map.get("Package") {
			Some(package) => package,
			None => {
				return Err(APTError::MissingKeyError(MissingKeyError::new(
					"Package", data,
				)))
			}
		};

		let version = match map.get("Version") {
			Some(version) => version,
			None => {
				return Err(APTError::MissingKeyError(MissingKeyError::new(
					"Version", data,
				)))
			}
		};

		let architecture = match map.get("Architecture") {
			Some(architecture) => architecture,
			None => {
				return Err(APTError::MissingKeyError(MissingKeyError::new(
					"Architecture",
					data,
				)))
			}
		};

		let installed_size = match map.get("Installed-Size") {
			Some(size) => Some(match size.parse::<i64>() {
				Ok(size) => size,
				Err(_) => return Err(APTError::ParseError(ParseError)),
			}),
			None => None,
		};

		Ok(Control {
			map: map.clone(),
			package: package.to_string(),
			source: map.get("Source").cloned(),
			version: version.to_string(),
			section: map.get("Section").cloned(),
			priority: map.get("Priority").cloned(),
			architecture: architecture.to_string(),
			is_essential: map.get("Essential").map(|x| x == "yes"),
			depends: make_array(map.get("Depends")),
			pre_depends: make_array(map.get("Pre-Depends")),
			recommends: make_array(map.get("Recommends")),
			suggests: make_array(map.get("Suggests")),
			replaces: make_array(map.get("Replaces")),
			enhances: make_array(map.get("Enhances")),
			breaks: make_array(map.get("Breaks")),
			conflicts: make_array(map.get("Conflicts")),
			installed_size,
			maintainer: map.get("Maintainer").cloned(),
			description: map.get("Description").cloned(),
			homepage: map.get("Homepage").cloned(),
			built_using: map.get("Built-Using").cloned(),
			package_type: map.get("Package-Type").cloned(),
			tags: make_array(map.get("Tag")),
		})
	}

	pub fn get(&self, key: &str) -> Option<&str> {
		self.map.get(key).map(|x| &**x)
	}
}

#[cfg(test)]
mod tests {
	use super::Control;
	use std::fs::read_to_string;

	#[test]
	fn control_clang() {
		let file = "./test/clang.control";
		let data = match read_to_string(file) {
			Ok(data) => data,
			Err(err) => panic!("Failed to read file: {}", err),
		};

		let control = match Control::from(&data) {
			Ok(control) => control,
			Err(err) => panic!("Failed to parse control: {}", err),
		};

		assert_eq!(control.package, "clang");
		assert_eq!(control.source, Some("llvm-defaults (0.54)".to_owned()));
		assert_eq!(control.version, "1:13.0-54");
		assert_eq!(control.section, Some("devel".to_owned()));
		assert_eq!(control.priority, Some("optional".to_owned()));
		assert_eq!(control.architecture, "amd64");
		assert_eq!(control.is_essential, None);

		assert_eq!(control.depends, Some(vec!["clang-13 (>= 13~)".to_owned()]));
		assert_eq!(control.pre_depends, None);
		assert_eq!(control.recommends, None);
		assert_eq!(control.suggests, None);
		assert_eq!(
			control.replaces,
			Some(vec![
				"clang (<< 3.2-1~exp2)".to_owned(),
				"clang-3.2".to_owned(),
				"clang-3.3".to_owned(),
				"clang-3.4 (<< 1:3.4.2-7~exp1)".to_owned(),
				"clang-3.5 (<< 1:3.5~+rc1-3~exp1)".to_owned(),
			])
		);
		assert_eq!(control.enhances, None);
		assert_eq!(
			control.breaks,
			Some(vec![
				"clang-3.2".to_owned(),
				"clang-3.3".to_owned(),
				"clang-3.4 (<< 1:3.4.2-7~exp1)".to_owned(),
				"clang-3.5 (<< 1:3.5~+rc1-3~exp1)".to_owned(),
			])
		);
		assert_eq!(control.conflicts, None);

		assert_eq!(control.installed_size, Some(24));
		assert_eq!(
			control.maintainer,
			Some("Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>".to_owned())
		);
		assert_eq!(control.description, Some("C, C++ and Objective-C compiler (LLVM based), clang binary\nClang project is a C, C++, Objective C and Objective C++ front-end for the LLVM compiler. Its goal is to offer a replacement to the GNU Compiler Collection (GCC).\n\nClang implements all of the ISO C++ 1998, 11 and 14 standards and also provides most of the support of C++17.\n\nThis is a dependency package providing the default clang compiler.".to_owned()));
		assert_eq!(control.homepage, None);
		assert_eq!(control.built_using, None);
		assert_eq!(control.package_type, None);
		assert_eq!(control.tags, None);

		assert_eq!(
			control.get("Original-Maintainer"),
			Some("LLVM Packaging Team <pkg-llvm-team@lists.alioth.debian.org>")
		);
	}

	#[test]
	fn control_com_amywhile_signalreborn() {
		let file = "./test/com.amywhile.signalreborn.control";
		let data = match read_to_string(file) {
			Ok(data) => data,
			Err(err) => panic!("Failed to read file: {}", err),
		};

		let control = match Control::from(&data) {
			Ok(control) => control,
			Err(err) => panic!("Failed to parse control: {}", err),
		};

		assert_eq!(control.package, "com.amywhile.signalreborn");
		assert_eq!(control.source, None);
		assert_eq!(control.version, "2.2.1-2");
		assert_eq!(control.section, Some("Applications".to_owned()));
		assert_eq!(control.priority, None);
		assert_eq!(control.architecture, "iphoneos-arm");
		assert_eq!(control.is_essential, None);

		assert_eq!(
			control.depends,
			Some(vec!["firmware (>= 12.2) | org.swift.libswift".to_owned()])
		);
		assert_eq!(control.pre_depends, None);
		assert_eq!(control.recommends, None);
		assert_eq!(control.suggests, None);
		assert_eq!(
			control.replaces,
			Some(vec!["com.charliewhile.signalreborn".to_owned()])
		);
		assert_eq!(control.enhances, None);
		assert_eq!(
			control.breaks,
			Some(vec!["com.charliewhile.signalreborn".to_owned()])
		);
		assert_eq!(
			control.conflicts,
			Some(vec!["com.charliewhile.signalreborn".to_owned()])
		);

		assert_eq!(control.installed_size, Some(1536));
		assert_eq!(
			control.maintainer,
			Some("Amy While <support@anamy.gay>".to_owned())
		);
		assert_eq!(
			control.description,
			Some("Visualise your nearby cell towers".to_owned())
		);
		assert_eq!(control.homepage, None);
		assert_eq!(control.built_using, None);
		assert_eq!(control.package_type, None);
		assert_eq!(
			control.tags,
			Some(vec!["compatible_min::ios11.0".to_owned()])
		);

		assert_eq!(control.get("Name"), Some("SignalReborn"));
		assert_eq!(control.get("Author"), Some("Amy While <support@anamy.gay>"));
		assert_eq!(
			control.get("Icon"),
			Some("https://img.chariz.cloud/icon/signal/icon@3x.png")
		);
		assert_eq!(
			control.get("Depiction"),
			Some("https://chariz.com/get/signal")
		);
	}
}
