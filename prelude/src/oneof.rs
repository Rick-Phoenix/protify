use crate::*;

pub trait ProtoOneof {
  fn fields() -> Vec<ProtoField>;
}

#[derive(Debug, Default, Clone)]
pub struct Oneof {
  pub name: &'static str,
  pub fields: Vec<ProtoField>,
  pub options: Vec<ProtoOption>,
}
