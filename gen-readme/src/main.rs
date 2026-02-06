use std::fs::File;
use std::io::Write;
use std::process::Command;
use syn::{Expr, Lit, Meta};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Expanding crate...");
	let output = Command::new("cargo")
		.args([
			"expand",
			"-p",
			"protify",
			"--no-default-features",
			"--features",
			"document-features",
		])
		.output()?;

	if !output.status.success() {
		eprintln!("{}", str::from_utf8(&output.stderr)?);
		return Err("Failed to expand crate".into());
	}

	let expanded_src = str::from_utf8(&output.stdout)?;

	let syntax = syn::parse_file(expanded_src)?;

	let mut readme_content = String::new();

	for attr in syntax.attrs {
		if attr.path().is_ident("doc")
			&& let Meta::NameValue(nv) = attr.meta
			&& let Expr::Lit(expr_lit) = nv.value
			&& let Lit::Str(lit_str) = expr_lit.lit
		{
			let line = lit_str.value();
			readme_content.push_str(&line);
			readme_content.push('\n');
		}
	}

	let mut file = File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/../README.md"))?;

	write!(
		file,
		r#"
<p align="center">
<img alt="protify logo" src="https://github.com/Rick-Phoenix/protify/blob/main/assets/logo.jpg?raw=true">
</p>

<div align="center">
<div>

[![Crates.io Version](https://img.shields.io/crates/v/protify)](https://crates.io/crates/protify) [![License](https://img.shields.io/github/license/Rick-Phoenix/protify)](https://www.mozilla.org/en-US/MPL/2.0/)

[![Sponsor](https://img.shields.io/badge/GitHub%20Sponsors-30363D?&logo=GitHub-Sponsors&logoColor=EA4AAA)](https://github.com/sponsors/Rick-Phoenix)

<a href="https://docs.rs/protify/latest/protify/">Docs</a> &emsp; <a href="https://docs.rs/protify/latest/protify/guide/index.html">Guide</a>
</div>
</div>

"#
	)?;

	write!(file, "{}", readme_content.trim())?;

	write!(
		file,
		"
# License
This repository is licensed under the MPL-2.0 license.
The file `CREDITS.md` contains the licensing details for the external code used in this project."
	)?;

	println!("README.md updated successfully!");

	Ok(())
}
