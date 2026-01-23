use super::*;

use rendering_tests::{Nested1, TestEnum};

#[proto_message]
#[proto(skip_checks(all))]
struct IntWrappers {
  #[proto(sint32)]
  sint32: i32,
  #[proto(sint64)]
  sint64: i64,
  #[proto(sfixed32)]
  sfixed32: i32,
  #[proto(sfixed64)]
  sfixed64: i64,
  #[proto(fixed32)]
  fixed32: u32,
  #[proto(fixed64)]
  fixed64: u64,
}

#[test]
fn int_wrappers_resolution() {
  let schema = IntWrappers::proto_schema();

  let outputs = [
    ProtoScalar::Sint32,
    ProtoScalar::Sint64,
    ProtoScalar::Sfixed32,
    ProtoScalar::Sfixed64,
    ProtoScalar::Fixed32,
    ProtoScalar::Fixed64,
  ];

  for (field, exp_type) in schema.fields().zip(outputs) {
    assert_eq_pretty!(
      field.type_,
      FieldType::Normal(ProtoType::Scalar(exp_type)),
      "expected field {} to have the type {exp_type}",
      field.name
    );
  }
}

#[proto_message]
#[proto(skip_checks(all))]
struct Maps {
  #[proto(map(int32, enum_(TestEnum)))]
  enum_map: HashMap<i32, i32>,
  #[proto(map(sfixed32, message(Nested1)))]
  msg_map: HashMap<i32, Nested1>,
}

#[test]
fn map_type_resolution() {
  let schema = Maps::proto_schema();

  let exp_types = [
    FieldType::Map {
      keys: ProtoMapKey::Int32,
      values: ProtoType::Enum(TestEnum::proto_path()),
    },
    FieldType::Map {
      keys: ProtoMapKey::Sfixed32,
      values: ProtoType::Message(Nested1::proto_path()),
    },
  ];

  for (field, exp_type) in schema.fields().zip(exp_types) {
    assert_eq_pretty!(field.type_, exp_type)
  }
}
