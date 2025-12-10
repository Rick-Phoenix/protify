use quote::format_ident;

use crate::*;

#[derive(Default, Debug, Clone)]
pub enum ItemPathEntry {
  Path(Path),
  Proxied,
  #[default]
  None,
}

#[derive(Debug, Clone)]
pub enum ItemPath {
  Path(Path),
  Proxied(Path),
}

impl ItemPath {}

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

  pub fn is_none(&self) -> bool {
    matches!(self, Self::None)
  }
}

impl ToTokens for ItemPathEntry {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match self {
      Self::Path(path) => tokens.extend(path.to_token_stream()),
      _ => {}
    };
  }
}

pub fn ident_with_proto_suffix(mut path: Path) -> Path {
  let last_segment = path.segments.last_mut().unwrap();

  last_segment.ident = format_ident!("{}Proto", last_segment.ident);

  path
}

pub struct PathWrapper<'a> {
  pub inner: Cow<'a, Path>,
}

impl<'a> PathWrapper<'a> {
  pub fn new(path: Cow<'a, Path>) -> Self {
    Self { inner: path }
  }

  pub fn last_segment(&'_ self) -> PathSegmentWrapper<'_> {
    PathSegmentWrapper::new(Cow::Borrowed(self.inner.segments.last().unwrap()))
  }
}

pub struct PathSegmentWrapper<'a> {
  pub inner: Cow<'a, PathSegment>,
}

impl<'a> PathSegmentWrapper<'a> {
  pub fn new(segment: Cow<'a, PathSegment>) -> Self {
    Self { inner: segment }
  }

  pub fn ident(&self) -> &Ident {
    &self.inner.ident
  }

  pub fn get_arguments(&self) -> Option<impl Iterator<Item = &Path>> {
    if let PathArguments::AngleBracketed(args) = &self.inner.arguments {
      Some(args.args.iter().filter_map(|arg| {
        if let GenericArgument::Type(typ) = arg && let Type::Path(path) = typ {
        Some(&path.path)
      } else { None }
      }))
    } else {
      None
    }
  }

  pub fn first_argument(&self) -> Option<&Path> {
    self
      .get_arguments()
      .and_then(|args| args.find_or_first(|_| true))
  }

  pub fn first_two_arguments(&self) -> Option<(&Path, &Path)> {
    self.get_arguments().and_then(|args| {
      let mut first_arg: Option<&Path> = None;
      let mut second_arg: Option<&Path> = None;
      for (i, arg) in args.enumerate() {
        if i == 0 {
          first_arg = Some(arg);
        } else if i == 1 {
          second_arg = Some(arg);
          break;
        }
      }

      if let Some(first) = first_arg && let Some(second) = second_arg {
          Some((first, second))
        } else {
          None
        }
    })
  }
}

pub fn extract_type_path(ty: &Type) -> Result<&Path, Error> {
  match ty {
    Type::Path(type_path) => Ok(&type_path.path),

    _ => Err(spanned_error!(ty, "Expected be a type path")),
  }
}
