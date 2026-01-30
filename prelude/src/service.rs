use crate::*;

pub trait ProtoService {
  fn proto_schema() -> Service;
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(Template))]
#[cfg_attr(feature = "std", template(path = "service.proto.j2"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Service {
  pub name: FixedStr,
  pub file: FixedStr,
  pub options: Vec<ProtoOption>,
  pub handlers: Vec<ServiceHandler>,
  pub package: FixedStr,
}

impl Service {
  /// Renders the schema representation.
  #[cfg(feature = "std")]
  pub fn render_schema(&self) -> Result<String, askama::Error> {
    self.render()
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ServiceHandler {
  pub name: FixedStr,
  pub options: Vec<ProtoOption>,
  pub request: ProtoPath,
  pub response: ProtoPath,
}
