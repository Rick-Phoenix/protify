use crate::*;

impl<T> AsProtoType for Box<T>
where
  T: AsProtoType,
{
  fn proto_type() -> ProtoType {
    T::proto_type()
  }
}

impl<T> AsProtoType for Option<T>
where
  T: AsProtoType,
{
  fn proto_type() -> ProtoType {
    match T::proto_type() {
      ProtoType::Single(type_info) => ProtoType::Optional(type_info),
      ProtoType::Optional(_) => {
        ProtoType::Single(invalid_type_output("Optional fields cannot be nested"))
      }
      ProtoType::Repeated(_) => {
        ProtoType::Single(invalid_type_output("Optional fields cannot be repeated"))
      }
      ProtoType::Map { .. } => {
        ProtoType::Single(invalid_type_output("Optional fields cannot be maps"))
      }
    }
  }
}
