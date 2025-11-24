use crate::*;

pub trait ProtoOneof {
  fn fields() -> Vec<ProtoField>;
}

#[derive(Debug, Default, Clone)]
pub struct Oneof {
  pub name: Arc<str>,
  pub fields: Vec<ProtoField>,
  pub options: Vec<ProtoOption>,
}
