# From Proto

The preferred usage for this crate is to define the items in rust, and to generate the `.proto` files from it rather than the other way around, but it is also possible to use it purely for the protovalidate support, which it picks up from its predecessor `protocheck`.

To do that, you must first add the builder to the build-dependencies.

For convenience, the builder exports the [`DescriptorDataConfig`](builder::DescriptorDataConfig) struct, which you can use to gather information about the elements of a package while the validators are being set (which can often be useful to handle things like oneof attributes which aren't very ergonomic to set up in prost in a programmatic way).

The [`DescriptorDataConfig::set_up_validators`](builder::DescriptorDataConfig::set_up_validators) method can then be used to set up the validators for the target packages. If you desire to just set up the validators without gathering any other data, you can just call the omonimous [`set_up_validators`](builder::set_up_validators) function exported from the root of the crate.

```rust,ignore
use std::{env, path::PathBuf};

use builder::DescriptorDataConfig;
use prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto");

    let out_dir = env::var("OUT_DIR")
        .map(PathBuf::from)
        .unwrap_or(env::temp_dir());

    // Set path for descriptor output
    let descriptor_path = out_dir.join("file_descriptor_set.bin");

    // Your proto files and dependencies
    let include_paths = &["proto", "proto_deps"];

    let files = &["proto/test.proto"];

    let mut config = Config::new();
    config
        .file_descriptor_set_path(&descriptor_path)
        // Required, if bytes fields are used
        .bytes(["."])
        .out_dir(&out_dir);

    let _ = builder::set_up_validators(
        &mut config,
        files,
        include_paths,
        // The packages for which you want to apply the validators
        &["test_schemas.v1"]
    )?;

    // Compile the protos
    config.compile_protos(files, include_paths)?;

    // Emit env for descriptor location
    println!(
        "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
        descriptor_path.display()
    );

    Ok(())
}
```

After doing this, all of the selected messages will implement [`ValidatedMessage`](crate::ValidatedMessage), the selected enums will impement [`ProtoEnum`](crate::ProtoEnum), and the selected oneofs will implement [`ValidatedOneof`](crate::ValidatedOneof), and all of them will impement [`ProtoValidation`](crate::ProtoValidation).

If the `cel` feature is enabled, [`CelValue`](crate::CelValue) and [`CelOneof`](crate::CelOneof) will also be implemented for messages and oneofs.

Just like the non-reflection-based version of this crate, this will also automatically generate a `check_validators` method on each message and oneof, as well as a test that automatically calls this method and panics on failure, in order to ensure that validators represent valid configurations.

Should you wish to disable these checks, you can disable them with the `skip_checks(validators)` attribute, which you can easily set up programmatically by taking advantage of the data collection performed by the [`DescriptorDataConfig`] struct. For more information about this, visit the [`correctness`](crate::guide::correctness) section.
