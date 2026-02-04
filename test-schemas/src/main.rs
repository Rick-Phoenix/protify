use test_schemas::TEST_SCHEMAS;

fn main() {
	TEST_SCHEMAS::get_package()
		.render_files(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/../test-reflection/proto"
		))
		.unwrap()
}
