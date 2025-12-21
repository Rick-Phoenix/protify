use syn_utils::filter_attributes;

use crate::*;

pub struct ServiceOrHandlerAttrs {
  pub options: Option<Expr>,
}

pub fn process_service_or_handler_attrs(
  attrs: &[Attribute],
) -> Result<ServiceOrHandlerAttrs, Error> {
  let mut options: Option<Expr> = None;

  for arg in filter_attributes(attrs, &["proto"])? {
    match arg {
      Meta::NameValue(nv) => {
        let ident = nv.path.require_ident()?.to_string();

        match ident.as_str() {
          "options" => {
            options = Some(nv.value);
          }
          _ => bail!(nv.path, "Unknown attribute `{ident}`"),
        };
      }
      Meta::List(_) => {}
      Meta::Path(_) => {}
    }
  }

  Ok(ServiceOrHandlerAttrs { options })
}
