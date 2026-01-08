use crate::*;

reusable_string!(PROTO_DEPRECATED, "deprecated");

pub fn proto_deprecated() -> ProtoOption {
  ProtoOption {
    name: PROTO_DEPRECATED.clone(),
    value: OptionValue::Bool(true),
  }
}
