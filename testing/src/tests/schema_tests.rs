use super::*;
mod deprecated_tests;
mod nested_items_tests;
mod rendering_tests;

#[proto_message(no_auto_test)]
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
