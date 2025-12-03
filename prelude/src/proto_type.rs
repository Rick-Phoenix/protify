use crate::*;

pub trait AsProtoType {
  fn proto_type() -> ProtoType;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProtoType {
  Single(TypeInfo),
  Repeated(TypeInfo),
  Optional(TypeInfo),
  Map { keys: TypeInfo, values: TypeInfo },
}

impl TypeInfo {
  pub(crate) fn register_import(&self, imports: &mut FileImports) {
    if let Some(path) = self.path.as_ref() {
      imports.insert(path)
    }
  }
}

pub(crate) fn invalid_type_output(msg: &'static str) -> TypeInfo {
  TypeInfo {
    name: msg,
    path: None,
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
  pub name: &'static str,
  pub path: Option<ProtoPath>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProtoPath {
  pub package: &'static str,
  pub file: &'static str,
}
