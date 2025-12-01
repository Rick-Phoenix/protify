use crate::*;

#[derive(Clone, Debug, Default)]
pub struct MessageInfo {
  pub path: ItemPath,
  pub boxed: bool,
}

impl MessageInfo {
  pub fn to_proto_type(&self, rust_type: &RustType) -> Result<ProtoType, Error> {
    let Self { path, boxed } = self;

    let msg_path = if let ItemPath::Path(path) = path {
      path.clone()
    } else {
      let inner_type = rust_type
        .inner_path()
        .ok_or(error!(
          Span::call_site(),
          "Failed to extract the inner type. Expected a type, or a type wrapped in Option or Vec"
        ))?
        .clone();

      if path.is_suffixed() {
        append_proto_ident(inner_type)
      } else {
        inner_type
      }
    };

    Ok(ProtoType::Message {
      path: msg_path,
      boxed: *boxed,
    })
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
