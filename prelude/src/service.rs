use crate::*;

#[derive(Debug, PartialEq, Template)]
#[template(path = "service.proto.j2")]
pub struct Service {
  pub name: &'static str,
  pub options: Vec<ProtoOption>,
  pub handlers: Vec<ServiceHandler>,
  pub package: &'static str,
}

impl Service {
  pub(crate) fn render_options(&self) -> Option<String> {
    if self.options.is_empty() {
      return None;
    }

    Some(render_normal_options(&self.options))
  }
}

#[derive(Debug, PartialEq)]
pub struct ServiceHandler {
  pub name: &'static str,
  pub options: Vec<ProtoOption>,
  pub request: ProtoPath,
  pub response: ProtoPath,
}

impl ServiceHandler {
  pub(crate) fn render_options(&self) -> Option<String> {
    if self.options.is_empty() {
      return None;
    }

    Some(render_normal_options(&self.options))
  }
}
