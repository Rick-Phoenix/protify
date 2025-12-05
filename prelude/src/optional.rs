use crate::*;

impl<T> AsProtoType for Box<T>
where
  T: AsProtoType,
{
  fn proto_type() -> ProtoType {
    T::proto_type()
  }
}

impl<T> AsProtoField for Option<T>
where
  T: AsProtoField,
{
  fn as_proto_field() -> ProtoFieldInfo {
    match T::as_proto_field() {
      ProtoFieldInfo::Single(typ) => {
        if typ.is_message() {
          ProtoFieldInfo::Single(typ)
        } else {
          ProtoFieldInfo::Optional(typ)
        }
      }
      ProtoFieldInfo::Map { .. } => {
        ProtoFieldInfo::Single(invalid_type_output("Optional fields cannot be maps"))
      }
      ProtoFieldInfo::Repeated(_) => {
        ProtoFieldInfo::Single(invalid_type_output("Optional fields cannot be repeated"))
      }
      ProtoFieldInfo::Optional(_) => {
        ProtoFieldInfo::Single(invalid_type_output("Optional fields cannot be nested"))
      }
    }
  }
}
