#![allow(
  clippy::derive_partial_eq_without_eq,
  clippy::duplicated_attributes,
  clippy::struct_field_names
)]

use std::collections::HashMap;

use super::*;

proto_package!(RENDERING_PKG, name = "rendering");

define_proto_file!(
  RENDERING,
  file = "rendering.proto",
  package = TESTING_PKG,
  options = test_options(),
  extensions(SomeExt),
);

#[test]
fn test_renders() {
  let pkg = RENDERING_PKG.get_package();

  pkg.render_files("proto_test").unwrap();
}

fn list_option() -> ProtoOption {
  proto_option!("list_option" => [1i32, 2i32, 3i32, 4i32])
}

fn simple_option() -> ProtoOption {
  proto_option!("is_cool" => true)
}

fn message_option() -> ProtoOption {
  proto_option!("message_opt" => {
    "thing1" => 15,
    "thing2" => true,
    "thing3" => list_option().value
  })
}

fn nested_message_option() -> ProtoOption {
  let innermost = option_message!("thing1" => 15,
    "thing2" => true,
    "thing3" => list_option().value
  );

  let inner = option_message!("very_nested" => innermost);
  let outer = option_message!("nested" => inner);

  proto_option!("nested_opt" => outer)
}

fn test_options() -> Vec<ProtoOption> {
  vec![
    simple_option(),
    list_option(),
    message_option(),
    nested_message_option(),
  ]
}

#[proto_extension(target = MessageOptions)]
pub struct SomeExt {
  #[proto(options = test_options())]
  #[proto(tag = 5000)]
  name: String,

  #[proto(tag = 5001)]
  name2: String,
}

#[proto_service]
#[proto(options = test_options())]
pub enum FrodoService {
  #[proto(options = test_options())]
  GetRing {
    request: Abc,
    response: Nested,
  },
  DestroyRing {
    request: Abc,
    response: Nested,
  },
}

#[proto_enum]
#[proto(reserved_numbers(1, 2, 10..MAX))]
#[proto(reserved_names("abc", "bcd"))]
#[proto(options = test_options())]
pub enum TestEnum {
  #[proto(options = test_options())]
  AbcDeg,
  B,
}

#[proto_oneof(no_auto_test)]
#[proto(options = test_options())]
pub enum OneofA {
  #[proto(tag = 200, validate = |v| v.min_len(10).max_len(50))]
  A(String),
  #[proto(tag = 201)]
  B(i32),
}

#[proto_oneof(no_auto_test)]
pub enum OneofB {
  #[proto(tag = 1502)]
  A(String),
  #[proto(tag = 1503)]
  B(String),
}

fn msg_rule() -> CelProgram {
  cel_program!(id = "abc", msg = "abc", expr = "true == true")
}

#[proto_message(no_auto_test)]
#[proto(reserved_numbers(1, 2, 3..9))]
#[proto(reserved_names("abc", "bcd"))]
#[proto(options = test_options())]
#[proto(cel_rules(msg_rule(), msg_rule()))]
pub struct Abc {
  #[proto(repeated(int32), validate = |v| v.min_items(15).items(|it| it.gt(0).lt(50)))]
  pub repeated_field: Vec<i32>,

  #[proto(bytes, validate = |v| v.min_len(45).max_len(128))]
  pub optional_field: Option<Bytes>,

  #[proto(message, validate = |v| v.cel(cel_program!(id = "abc", msg = "abc", expr = "true == true")))]
  pub msg_field: Option<Nested>,

  #[proto(map(sint32, sint32), validate = |v| v.min_pairs(5).max_pairs(15).keys(|k| k.gt(15)).values(|vals| vals.gt(56)))]
  pub map_field: HashMap<i32, i32>,

  #[proto(oneof(required, tags(200, 201)))]
  pub oneof_field: Option<OneofA>,
}

#[proto_message(no_auto_test)]
#[proto(parent_message = Abc)]
pub struct Nested {
  #[proto(validate = |v| v.len_bytes(68))]
  name: String,
}

#[proto_message(no_auto_test)]
#[proto(parent_message = Nested)]
pub struct Nested2 {
  name: String,

  #[proto(map(sint32, sint32), validate = |v| v.min_pairs(5).max_pairs(15).keys(|k| k.gt(15)).values(|vals| vals.gt(56)))]
  pub map_field: HashMap<i32, i32>,

  #[proto(oneof(tags(200, 201)))]
  reused_oneof: Option<OneofA>,

  #[proto(oneof(tags(1502, 1503)))]
  oneof_b: Option<OneofB>,
}
