use tonic_prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=../testing/proto_test/");

  let mut config = Config::new();
  config
    .extern_path(".myapp.v1.Abc", "::testing::AbcProto")
    .extern_path(".myapp.v1.Abc.Nested", "::testing::NestedProto")
    .extern_path(".google.protobuf", "::proto_types")
    .compile_well_known_types()
    .bytes(["."]);

  let proto_include_paths = &["../testing/proto_test/", "proto_deps"];

  let proto_files = &["../testing/proto_test/abc.proto"];

  tonic_prost_build::configure()
    .build_client(false)
    .compile_with_config(config, proto_files, proto_include_paths)?;

  Ok(())
}
