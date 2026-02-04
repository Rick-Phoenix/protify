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
<img src="https://github.com/Rick-Phoenix/protify/blob/main/assets/logo.jpg?raw=true">
</p>

"#
	)?;

	write!(file, "{}", readme_content.trim())?;

	println!("README.md updated successfully!");

	Ok(())
}
