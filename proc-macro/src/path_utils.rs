use quote::format_ident;

use crate::*;

#[derive(Default, Debug, Clone)]
pub enum ItemPathEntry {
  Path(Path),
  Proxied,
  #[default]
  None,
}

impl ItemPathEntry {
  pub fn get_path_or_fallback(&self, fallback: Option<&Path>) -> Option<Path> {
    let output = if let Self::Path(path) = self {
      path.clone()
    } else if let Some(fallback) = fallback {
      let fallback = fallback.clone();

      if matches!(self, Self::Proxied) {
        ident_with_proto_suffix(fallback)
      } else {
        fallback
      }
    } else {
      return None;
    };

    Some(output)
  }
}

impl ToTokens for ItemPathEntry {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match self {
      Self::Path(path) => path.to_tokens(tokens),
      _ => {}
    };
  }
}

pub fn ident_with_proto_suffix(mut path: Path) -> Path {
  let last_segment = path.segments.last_mut().unwrap();

  last_segment.ident = format_ident!("{}Proto", last_segment.ident);

  path
}
