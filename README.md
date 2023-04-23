# APT Parser
>
> Looking for the Typescript Version? [It's been moved here...](https://github.com/cnstr/apt-parser-ts)

`apt-parser` is a library for parsing [APT](https://en.wikipedia.org/wiki/APT_(software)) list files.<br>
An APT repository normally consists of a Release file, Packages file, and compressed binary packages.<br>
The library is able to parse these files and return them as [`serde`](https://serde.rs) serialized structs.<br>

### Installation

Make sure you have a modern version of Rust (1.56+) using `rustup`.<br>
Then, add the following to your `Cargo.toml`:<br>

```toml
[dependencies]
# You can also use the latest version
apt-parser = "1.0.0"
```

### Release Parsing

Release files are the main entry point for an APT repository.<br>
The `Release` struct has strict types for all documented fields in the [`Release` file](https://wiki.debian.org/DebianRepository/Format#A.22Release.22_files).<br>
If you need to access a field that isn't defined, you can use the `get` method.<br>
Here's a simple example:<br>

```rust
use apt_parser::Release;
use surf::get;

let data = get("http://archive.ubuntu.com/ubuntu/dists/jammy/Release")
    .await?
    .body_string()
    .await?;

let release = match Release::from(data) {
    Ok(release) => release,
    Err(err) => panic!("Failed to parse Release file: {}", err),
}

assert_eq!(release.origin, "Ubuntu");
assert_eq!(release.version, "22.04");
assert_eq!(release.get("InvalidKey"), None);
```

```rust
struct Release {
    architectures: Vec<String>, // => Architectures
    no_support_for_architecture_all: Option<bool>, // => No-Support-For-Architecture-All
    description: Option<String>, // => Description
    origin: Option<String>, // => Origin
    label: Option<String>, // => Label
    suite: Option<String>, // => Suite
    version: Option<String>, // => Version
    codename: Option<String>, // => Codename
    date: Option<String>, // => Date
    valid_until: Option<String>, // => Valid-Until
    components: Vec<String>, // => Components
    md5sum: Option<Vec<ReleaseHash>>, // => MD5Sum
    sha1sum: Option<Vec<ReleaseHash>>, // => SHA1
    sha256sum: Option<Vec<ReleaseHash>>, // => SHA256
    sha512sum: Option<Vec<ReleaseHash>>, // => SHA512
    not_automatic: Option<bool>, // => NotAutomatic
    but_automatic_upgrades: Option<bool>, // => ButAutomaticUpgrades
    acquire_by_hash: Option<bool>, // => Acquire-By-Hash
    signed_by: Option<String>, // => Signed-By
    packages_require_authorization: Option<bool>, // => Packages-Require-Authorization

    fn from(data: &str) -> Result<Self, APTError>; // => Parse a Release file
    fn get(&self, key: &str) -> Option<&str>; // => Retrieve a raw field value
}

// A struct for holding the hash information for a Release file
struct ReleaseHash {
    filename: String,
    hash: String,
    size: u64,
}
```

### Control File Parsing

Control files are used to describe the contents of a binary package.<br>
The `Control` struct has strict types for all documented fields in the [`control` file](https://www.debian.org/doc/debian-policy/ch-controlfields.html).<br>
If you need to access a field that isn't defined, you can use the `get` method.<br>
Here's a simple example:<br>

```rust
use apt_parser::Control;

let data = "
Package: com.amywhile.signalreborn
Architecture: iphoneos-arm
Description: Visualise your nearby cell towers
Depends: firmware (>= 12.2) | org.swift.libswift
Maintainer: Amy While <support@anamy.gay>
Section: Applications
Version: 2.2.1-2
Installed-Size: 1536
Custom-Key: cool-value
";

let control = match Control::from(data) {
    Ok(control) => control,
    Err(err) => panic!("Failed to parse Control file: {}", err),
}

assert_eq!(control.version, "2.2.1-2");
assert_eq!(control.package, "com.amywhile.signalreborn");
assert_eq!(control.get("Custom-Key"), Some("cool-value"));
assert_eq!(control.get("Invalid-Key"), None);
```

```rust
struct Control {
    package: String, // => Package
    source: Option<String>, // => Source
    version: String, // => Version
    section: Option<String>, // => Section
    priority: Option<String>, // => Priority
    architecture: String, // => Architecture
    is_essential: Option<bool>, // => Essential
    depends: Option<Vec<String>>, // => Depends
    pre_depends: Option<Vec<String>>, // => Pre-Depends
    recommends: Option<Vec<String>>, // => Recommends
    suggests: Option<Vec<String>>, // => Suggests
    replaces: Option<Vec<String>>, // => Replaces
    enhances: Option<Vec<String>>, // => Enhances
    breaks: Option<Vec<String>>, // => Breaks
    conflicts: Option<Vec<String>>, // => Conflicts
    installed_size: Option<i64>, // => Installed-Size
    maintainer: Option<String>, // => Maintainer
    description: Option<String>, // => Description
    homepage: Option<String>, // => Homepage
    built_using: Option<String>, // => Built-Using
    package_type: Option<String>, // => Package-Type
    tags: Option<Vec<String>>, // => Tags

    fn from(data: &str) -> Result<Self, APTError>; // => Parse a Control file
    fn get(&self, key: &str) -> Option<&str>; // => Retrieve a raw field value
}
```

### Packages Parsing

Packages files are used to describe the contents of a repository.<br>
The `Packages` struct implements an iterator and has methods for accessing the packages.<br>
The `Package` struct has strict types for all documented fields in the [`Packages` file](https://wiki.debian.org/DebianRepository/Format#A.22Packages.22_Indices).<br>
If you need to access a field that isn't defined, you can use the `get` method.<br>
Here's a simple example:<br>

```rust
use apt_parser::Packages;
use surf::get;

let data = get("https://repo.chariz.com/Packages")
    .await?
    .body_string()
    .await?;

let packages = match Packages::from(&data) {
    Ok(packages) => packages,
    Err(err) => panic!("Failed to parse Packages file: {}", err),
}

assert_eq!(packages.len(), 419);

for package in packages {
    println!("{}: {}", package.package, package.version);
}
```

```rust
struct Packages {
    packages: Vec<Package>,

    fn from(data: &str) -> Result<Self, APTError>; // => Parse a Packages file
    fn len(&self) -> usize; // => Get the number of packages
}

impl Iterator for Packages;
impl Index for Packages;

struct Package {
    package: String, // => Package
    source: Option<String>, // => Source
    version: String, // => Version
    section: Option<String>, // => Section
    priority: Option<String>, // => Priority
    architecture: String, // => Architecture
    is_essential: Option<bool>, // => Essential
    depends: Option<Vec<String>>, // => Depends
    pre_depends: Option<Vec<String>>, // => Pre-Depends
    recommends: Option<Vec<String>>, // => Recommends
    suggests: Option<Vec<String>>, // => Suggests
    replaces: Option<Vec<String>>, // => Replaces
    enhances: Option<Vec<String>>, // => Enhances
    breaks: Option<Vec<String>>, // => Breaks
    conflicts: Option<Vec<String>>, // => Conflicts
    installed_size: Option<i64>, // => Installed-Size
    maintainer: Option<String>, // => Maintainer
    description: Option<String>, // => Description
    homepage: Option<String>, // => Homepage
    built_using: Option<String>, // => Built-Using
    package_type: Option<String>, // => Package-Type
    tags: Option<Vec<String>>, // => Tags
    filename: String, // => Filename
    size: i64, // => Size
    md5sum: Option<String>, // => MD5sum
    sha1sum: Option<String>, // => SHA1
    sha256sum: Option<String>, // => SHA256
    sha512sum: Option<String>, // => SHA512
    description_md5sum: Option<String>, // => Description-md5

    fn get(&self, key: &str) -> Option<&str>; // => Retrieve a raw field value
}
```

> Copyright (c) 2023 Aarnav Tale
