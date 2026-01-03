use crate::*;

#[derive(Clone, Debug)]
pub struct MessageInfo {
  pub path: Path,
  pub boxed: bool,
}

impl MessageInfo {
  pub fn parse(meta: &ParseNestedMeta, fallback: Option<&Path>) -> syn::Result<Self> {
    let mut item_path = ItemPathEntry::default();
    let mut boxed = false;

    meta.parse_nested_meta(|meta| {
      if let Ok(ident_str) = meta.ident_str() {
        match ident_str.as_str() {
          "proxied" => {
            item_path = ItemPathEntry::Proxied;
          }
          "boxed" => boxed = true,
          _ => item_path = ItemPathEntry::Path(meta.path),
        };
      } else {
        item_path = ItemPathEntry::Path(meta.path);
      }

      Ok(())
    })?;

    let path = item_path
      .get_path_or_fallback(fallback)
      .ok_or(meta.error("Failed to infer the message path. Please set it manually"))?;

    Ok(Self { path, boxed })
  }
}
