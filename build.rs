use std::fs;

use regex::Regex;

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=README.md");
	println!("cargo:rerun-if-changed=Cargo.toml");

	// Read Cargo.toml
	let cargo_toml = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");

	// Extract rust-version using regex with find_map
	let rust_version_regex =
		Regex::new(r#"rust-version\s*=\s*"([^"]+)""#).expect("Failed to compile regex");

	let msrv = cargo_toml
		.lines()
		.find_map(|line| {
			rust_version_regex
				.captures(line)
				.and_then(|caps| caps.get(1))
				.map(|m| m.as_str())
		})
		.expect("Could not find rust-version in Cargo.toml");

	println!("cargo:rustc-env=MSRV={}", msrv);

	// Process README.md
	if let Ok(readme) = fs::read_to_string("README.md") {
		let updated = update_msrv_markers(&readme, msrv);

		if updated != readme {
			fs::write("README.md", updated).expect("Failed to write README.md");
		}
	}
}

fn update_msrv_markers(content: &str, msrv: &str) -> String {
	let marker_regex =
		Regex::new(r"<!-- MSRV_START -->.*?<!-- MSRV_END -->").expect("Failed to compile marker regex");

	marker_regex
		.replace_all(
			content,
			format!("<!-- MSRV_START -->{msrv}<!-- MSRV_END -->"),
		)
		.into_owned()
}
