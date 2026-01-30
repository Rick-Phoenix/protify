# Usage With Tonic

The recommended workflow with this library is to define the proto items in a separate workspace crate (which I will refer to as the "models" crate) using the workflow described in the [package setup](crate::guide::package_setup) section, and export the package handle, so that the consuming crate (like a tonic server) can use the handle to generate the files and to generate the services from those files, while importing the pre-built messages from the models crate.

This is how to set up the `build.rs` file in the consuming crate, which is this case will be a tonic server.

```rust,ignore
use std::env;

use tonic_prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../models/src/");

    // We import the package handle from the models crate.
    // (Special considerations needed for no_std crates can be found in the docs)
    let pkg = models::PKG.get_package();

    // Create the proto files, which we need
    // to generate the services
    pkg
        .render_files(concat!(env!("CARGO_MANIFEST_DIR"), "/proto"))
        .unwrap();

    let include_paths = &["proto", "proto_deps"];
    
    let files = &[ "file1.proto", "and_so_on_and_so_forth.proto" ];

    let mut config = Config::new();

    config
        // You can use `proto_types` directly or its re-export
        .extern_path(".google.protobuf", "::prelude::proto_types")
        // If we are using validators
        .extern_path(".buf.validate", "::prelude::proto_types::protovalidate")
        // Required, if bytes fields are used
        .bytes(["."])
        .compile_well_known_types();

    // We only need to build the services, and we will import
    // the pre-built messages directly from our models crate
    // 
    // We use the `extern_paths` helper from the package so that
    // each message is automatically mapped to its Rust path
    for (name, path) in pkg.extern_paths() {
        config.extern_path(name, path);
    }

    config.compile_protos(files, include_paths)?;

    tonic_prost_build::configure()
        .compile_with_config(config, files, include_paths)?;

    Ok(())
}
```

Then, we can just use our services and messages like in any normal tonic app. The only difference is that the services will be in the generated code, but the messages will be directly imported by the models crate. 

You can take a look at the `test-server` crate in the repo for a full example of this which also includes working with `diesel` and an sqlite database with the same models.
