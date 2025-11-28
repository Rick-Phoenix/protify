use crate::*;

#[derive(Debug, Clone, Default)]
pub struct OneofInfo {
  pub path: ItemPath,
  pub tags: Vec<i32>,
  pub default: bool,
}

#[allow(clippy::single_match)]
impl Parse for OneofInfo {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

    let mut oneof_path = ItemPath::default();
    let mut tags: Vec<i32> = Vec::new();
    let mut default = false;

    for meta in metas {
      match meta {
        Meta::Path(path) => {
          let ident = get_ident_or_continue!(path);

          match ident.as_str() {
            "default" => default = true,
            "suffixed" => oneof_path = ItemPath::Suffixed,
            _ => oneof_path = ItemPath::Path(path),
          };
        }
        Meta::List(list) => {
          let ident = get_ident_or_continue!(list.path);

          match ident.as_str() {
            "tags" => {
              tags = list.parse_args::<NumList>()?.list;
            }
            _ => {}
          };
        }
        Meta::NameValue(_) => {}
      }
    }

    Ok(Self {
      path: oneof_path,
      tags,
      default,
    })
  }
}
