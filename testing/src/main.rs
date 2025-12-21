use prelude::Package;

use testing::proto_file;

fn main() {
  env_logger::init();

  let package = Package {
    name: "mypkg",
    files: vec![proto_file()],
  };

  package.render_files("proto_test").unwrap();
}
