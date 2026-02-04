use std::{env, path::PathBuf};

use prost_build::Config;
use protify_build::{DescriptorDataConfig, set_up_validators};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("cargo:rerun-if-changed=../no-std-models/src/lib.rs");

	// Because we are overriding the `protify` dependency by activating the
	// inventory feature, the file collection is done automatically,
	// but it comes at the expense of annoying behaviour with rust-analyzer
	no_std_models::NO_STD_PKG::get_package()
		.render_files(concat!(env!("CARGO_MANIFEST_DIR"), "/proto"))
		.unwrap();

	let out_dir = env::var("OUT_DIR")
		.map(PathBuf::from)
		.unwrap_or(env::temp_dir());
	let descriptor_path = out_dir.join("file_descriptor_set.bin");

	let include_paths = &["proto", "proto_deps"];

	let files = &["proto/no_std_models.proto"];

	let mut config = Config::new();
	config
		.file_descriptor_set_path(&descriptor_path)
		.bytes(["."])
		// NOTE: Due to a bug in prost's macro output, if you need to use btree maps
		// you need to add `prost` as a runtime dependency too and cannot rely solely on the
		// re-export from `protify`.
		.btree_map(["."])
		.out_dir(&out_dir);

	// This last part is not required for basic no_std usage when the messages
	// are imported from the models crate. I am doing this so that I can use this crate
	// both for testing models compilation and also validators injected via a
	// reflection-based impl.

	let desc_data = DescriptorDataConfig::new()
		.collect_oneofs_data()
		.set_up_validators(&mut config, files, include_paths, &["no_std_models"])?;

	let skip_test_attr = "#[proto(skip_checks(all))]";

	for oneof in desc_data.oneofs {
		config.enum_attribute(oneof.full_name(), skip_test_attr);
	}

	config.message_attribute(".no_std_models", skip_test_attr);

	config.compile_protos(files, include_paths)?;

	println!(
		"cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
		descriptor_path.display()
	);

	Ok(())
}
