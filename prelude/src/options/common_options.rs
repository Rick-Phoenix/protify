use crate::*;

#[must_use]
pub fn proto_deprecated() -> ProtoOption {
  ProtoOption {
    name: "deprecated".into(),
    value: OptionValue::Bool(true),
  }
}
