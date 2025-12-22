use testing::collect_package;

fn main() {
  let pkg = collect_package();

  for file in pkg.files {
    println!("{:#?}", file.options);
  }
}
