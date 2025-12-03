use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ProtoField {
  pub name: String,
  pub tag: i32,
  pub type_: ProtoType,
  pub options: Vec<ProtoOption>,
  pub validator: Option<ProtoOption>,
}

impl ProtoField {
  pub(crate) fn register_type_import_path(&self, imports: &mut FileImports) {
    match &self.type_ {
      ProtoType::Single(ty) => ty.register_import(imports),
      ProtoType::Repeated(ty) => ty.register_import(imports),
      ProtoType::Optional(ty) => ty.register_import(imports),
      ProtoType::Map { keys, values } => {
        keys.register_import(imports);
        values.register_import(imports);
      }
    };
  }
}
