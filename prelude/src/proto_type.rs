use crate::*;

pub trait AsProtoType {
  fn proto_type() -> ProtoType;
}

#[derive(Debug, Clone)]
pub enum ProtoType {
  Single(TypeInfo),
  Repeated(TypeInfo),
  Optional(TypeInfo),
  Map { keys: TypeInfo, values: TypeInfo },
}

impl TypeInfo {
  pub fn register_import(&self, imports: &mut HashSet<&'static str>) {
    self.path.as_ref().map(|path| imports.insert(path.file));
  }
}

pub(crate) fn invalid_type_output(msg: &'static str) -> TypeInfo {
  TypeInfo {
    name: msg,
    path: None,
  }
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
  pub name: &'static str,
  pub path: Option<ProtoPath>,
}

#[derive(Debug, Clone)]
pub struct ProtoPath {
  pub package: &'static str,
  pub file: &'static str,
}
