use std::borrow::Cow;

use crate::*;

pub trait AsProtoType {
  fn proto_type() -> ProtoType;
}

pub trait AsProtoField {
  fn as_proto_field() -> ProtoFieldInfo;
}

impl<T: AsProtoType> AsProtoField for T {
  fn as_proto_field() -> ProtoFieldInfo {
    ProtoFieldInfo::Single(T::proto_type())
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProtoFieldInfo {
  Single(ProtoType),
  Map { keys: ProtoType, values: ProtoType },
  Repeated(ProtoType),
  Optional(ProtoType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProtoType {
  Scalar { name: &'static str },
  Message(ProtoPath),
  Enum(ProtoPath),
}

impl ProtoType {
  /// Returns `true` if the proto type2 is [`Message`].
  ///
  /// [`Message`]: ProtoType::Message
  #[must_use]
  pub fn is_message(&self) -> bool {
    matches!(self, Self::Message { .. })
  }
}

impl ProtoFieldInfo {
  pub(crate) fn render(&self, current_package: &'static str) -> Cow<'static, str> {
    let name = self.render_name(current_package);

    match self {
      ProtoFieldInfo::Single(_) | ProtoFieldInfo::Map { .. } => name,
      ProtoFieldInfo::Repeated(_) => format!("repeated {name}").into(),
      ProtoFieldInfo::Optional(inner) => {
        if inner.is_message() {
          name
        } else {
          format!("optional {name}").into()
        }
      }
    }
  }

  pub(crate) fn render_name(&self, current_package: &'static str) -> Cow<'static, str> {
    match self {
      ProtoFieldInfo::Single(type_info) => type_info.render_name(current_package),
      ProtoFieldInfo::Repeated(type_info) => type_info.render_name(current_package),
      ProtoFieldInfo::Optional(type_info) => type_info.render_name(current_package),
      ProtoFieldInfo::Map { keys, values } => format!(
        "map<{}, {}>",
        keys.render_name(current_package),
        values.render_name(current_package)
      )
      .into(),
    }
  }
}

impl ProtoType {
  pub(crate) fn render_name(&self, current_package: &'static str) -> Cow<'static, str> {
    match self {
      ProtoType::Scalar { name } => (*name).into(),
      ProtoType::Message(path) => {
        if path.package != current_package {
          format!("{}.{}", path.package, path.name).into()
        } else {
          path.name.into()
        }
      }
      ProtoType::Enum(path) => {
        if path.package != current_package {
          format!("{}.{}", path.package, path.name).into()
        } else {
          path.name.into()
        }
      }
    }
  }

  pub(crate) fn register_import(&self, imports: &mut FileImports) {
    match self {
      ProtoType::Scalar { .. } => {}
      ProtoType::Message(path) => imports.insert_path(path),
      ProtoType::Enum(path) => imports.insert_path(path),
    }
  }
}

pub(crate) fn invalid_type_output(msg: &'static str) -> ProtoType {
  ProtoType::Scalar { name: msg }
}

impl ProtoPath {
  pub(crate) fn render_name(&self, current_package: &'static str) -> Cow<'static, str> {
    if self.package != current_package {
      format!("{}.{}", self.package, self.name).into()
    } else {
      self.name.into()
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProtoPath {
  pub name: &'static str,
  pub package: &'static str,
  pub file: &'static str,
}
