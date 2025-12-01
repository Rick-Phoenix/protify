use crate::*;

#[derive(Clone, Debug, Default)]
pub struct MessageInfo {
  pub path: ItemPath,
  pub boxed: bool,
}

impl MessageInfo {
  pub fn with_path(&mut self, fallback: Option<&Path>) -> Result<(), Error> {
    self.path = ItemPath::Path(
      self
        .path
        .get_path_or_fallback(fallback)
        .expect("Failed to infer the path to the message, please set it manually"),
    );

    Ok(())
  }
}

impl Parse for MessageInfo {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

    let mut item_path = ItemPath::default();
    let mut boxed = false;

    for meta in metas {
      match meta {
        Meta::Path(path) => {
          let ident = get_ident_or_continue!(path);

          match ident.as_str() {
            "suffixed" => {
              item_path = ItemPath::Suffixed;
            }
            "boxed" => boxed = true,
            _ => item_path = ItemPath::Path(path),
          };
        }
        _ => {}
      }
    }

    Ok(Self {
      path: item_path,
      boxed,
    })
  }
}
