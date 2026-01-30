use std::env;

use tonic_prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=../test-schemas/src/server_models.rs");

  let pkg = test_schemas::server_models::DB_TEST::get_package();

  pkg
    .render_files(concat!(env!("CARGO_MANIFEST_DIR"), "/proto"))
    .unwrap();

  let include_paths = &["proto", "proto_deps"];

  let files = &["proto/db_test.proto"];

  let mut config = Config::new();

  config
    .extern_path(".google.protobuf", "::proto_types")
    .extern_path(".buf.validate", "::proto_types::protovalidate")
    .compile_well_known_types();

  for (name, path) in pkg.extern_paths() {
    config.extern_path(name, path);
  }

  config.compile_protos(files, include_paths)?;

  tonic_prost_build::configure()
    .build_client(true)
    .compile_with_config(config, files, include_paths)?;

  Ok(())
}
