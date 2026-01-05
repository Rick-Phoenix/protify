use prelude::{
  StringValidator, ValidatorBuilderFor, cel_program, define_proto_file, proto_message,
  proto_package,
};

fn string_validator() -> StringValidator {
  StringValidator::builder()
    .len(5)
    .min_len(5)
    .max_len(5)
    .len_bytes(5)
    .min_bytes(5)
    .max_bytes(6)
    .pattern("abc")
    .prefix("abc")
    .suffix("abc")
    .contains("abc")
    .not_contains("abc")
    .in_(["abc", "cde"])
    .not_in(["abc", "cde"])
    .const_("abc")
    .ignore_if_zero_value()
    .cel(cel_program!(id = "abc", msg = "abc", expr = "true == true"))
    .build()
}

#[proto_message(no_auto_test)]
struct StringRules {
  #[proto(validate = string_validator())]
  pub test: String,
}

proto_package!(REFLECTION, name = "reflection.v1");
define_proto_file!(
  REFLECTION_FILE,
  file = "reflection.proto",
  package = REFLECTION
);

fn main() {
  REFLECTION
    .get_package()
    .render_files(concat!(
      env!("CARGO_MANIFEST_DIR"),
      "/../test-reflection/proto"
    ))
    .unwrap()
}
