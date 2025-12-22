use testing::collect_package;

fn main() {
  let pkg = collect_package();

  eprintln!("{pkg:#?}");
}
