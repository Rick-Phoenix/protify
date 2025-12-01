use convert_case::ccase;
use syn::{
  parse::{ParseStream, Parser},
  spanned::Spanned,
  ExprCall,
};

use crate::*;

#[derive(Default, Debug, Clone)]
pub enum ItemPath {
  Path(Path),
  Suffixed,
  #[default]
  None,
}

impl ItemPath {
  pub fn is_proxied(&self) -> bool {
    !self.is_none()
  }

  pub fn get_path_or_fallback(&self, fallback: Option<&Path>) -> Option<Path> {
    let output = if let Self::Path(path) = self {
      path.clone()
    } else if let Some(fallback) = fallback {
      let fallback = fallback.clone();

      if self.is_suffixed() {
        append_proto_ident(fallback)
      } else {
        fallback
      }
    } else {
      return None;
    };

    Some(output)
  }

  pub fn is_suffixed(&self) -> bool {
    matches!(self, Self::Suffixed)
  }

  pub fn is_none(&self) -> bool {
    matches!(self, Self::None)
  }
}

impl ToTokens for ItemPath {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match self {
      Self::Path(path) => tokens.extend(path.to_token_stream()),
      _ => {}
    };
  }
}

#[derive(Clone)]
pub struct FieldAttrs {
  pub tag: i32,
  pub validator: Option<ValidatorExpr>,
  pub options: ProtoOptions,
  pub name: String,
  pub proto_field: Option<ProtoField>,
  pub from_proto: Option<PathOrClosure>,
  pub into_proto: Option<PathOrClosure>,
  pub is_ignored: bool,
}

#[derive(Clone)]
pub enum ValidatorExpr {
  Closure(ExprClosure),
  Call(ExprCall),
}

pub fn process_derive_field_attrs(
  original_name: &Ident,
  rust_type: &RustType,
  attrs: &Vec<Attribute>,
) -> Result<FieldAttrs, Error> {
  let mut validator: Option<ValidatorExpr> = None;
  let mut tag: Option<i32> = None;
  let mut options: Option<TokenStream2> = None;
  let mut name: Option<String> = None;
  let mut proto_field: Option<ProtoField> = None;
  let mut is_ignored = false;
  let mut from_proto: Option<PathOrClosure> = None;
  let mut into_proto: Option<PathOrClosure> = None;

  let mut oneof_attrs: Vec<MetaList> = Vec::new();

  for attr in attrs {
    if !attr.path().is_ident("proto") {
      continue;
    }

    let args = attr.parse_args::<PunctuatedParser<Meta>>()?;

    for meta in args.inner {
      match meta {
        Meta::NameValue(nv) => {
          let ident = get_ident_or_continue!(nv.path);

          match ident.as_str() {
            "validate" => {
              validator = match nv.value {
                Expr::Closure(closure) => Some(ValidatorExpr::Closure(closure)),
                Expr::Call(call) => Some(ValidatorExpr::Call(call)),
                _ => panic!("Invalid validator"),
              };
            }
            "from_proto" => {
              let value = parse_path_or_closure(nv.value)?;

              from_proto = Some(value);
            }
            "into_proto" => {
              let value = parse_path_or_closure(nv.value)?;

              into_proto = Some(value);
            }
            "tag" => {
              tag = Some(extract_i32(&nv.value).unwrap());
            }
            "options" => {
              let func_call = nv.value;

              options = Some(quote! { #func_call });
            }
            "name" => {
              name = Some(extract_string_lit(&nv.value).unwrap());
            }
            _ => {}
          };
        }
        Meta::List(list) => {
          let ident = get_ident_or_continue!(list.path);

          match ident.as_str() {
            "options" => {
              let exprs = list.parse_args::<PunctuatedParser<Expr>>().unwrap().inner;

              options = Some(quote! { vec! [ #exprs ] });
            }

            "oneof" => {
              oneof_attrs.push(list);
            }

            "repeated" => {
              let args = list.parse_args::<Meta>()?;

              let fallback = if let RustType::Vec(path) = rust_type {
                path
              } else {
                panic!("This field was marked as repeated but it is not using Vec. Please set the output type manually");
              };

              let inner = ProtoType::from_meta(args, Some(fallback))?.unwrap();

              proto_field = Some(ProtoField::Repeated(inner));
            }

            "optional" => {
              let args = list.parse_args::<Meta>()?;

              let fallback = if let RustType::Option(path) = rust_type {
                path
              } else {
                panic!("Could not parse the option type");
              };

              let inner = ProtoType::from_meta(args, Some(fallback))?.unwrap();

              proto_field = Some(ProtoField::Optional(inner));
            }

            "map" => {
              let parser = |input: ParseStream| parse_map_with_context(input, rust_type);

              let map = parser.parse2(list.tokens)?;

              proto_field = Some(ProtoField::Map(map));
            }

            _ => {
              let fallback = rust_type.inner_path();

              if let Some(field_info) = ProtoType::from_meta_list(&ident, list, fallback)? {
                proto_field = Some(ProtoField::Single(field_info));
              }
            }
          };
        }
        Meta::Path(path) => {
          let ident = get_ident_or_continue!(path);

          match ident.as_str() {
            "ignore" => is_ignored = true,

            _ => {
              let fallback = rust_type.inner_path();

              let span = path.span();

              if let Some(parsed_kind) = ProtoType::from_ident(&ident, span, fallback)? {
                proto_field = Some(ProtoField::Single(parsed_kind));
              }
            }
          };
        }
      };
    }
  }

  if !oneof_attrs.is_empty() {
    let mut oneof_path: Option<Path> = None;
    let mut oneof_tags: Vec<i32> = Vec::new();
    let mut use_default = false;
    let mut is_proxied = false;

    let fallback = rust_type.inner_path();

    for attr in oneof_attrs {
      let OneofInfo {
        path,
        tags,
        default,
      } = attr.parse_args::<OneofInfo>()?;

      if path.is_proxied() {
        is_proxied = true;
      }

      if let Some(path) = path.get_path_or_fallback(fallback) {
        oneof_path = Some(path);
      }

      if !tags.is_empty() {
        oneof_tags = tags;
      }

      if default {
        use_default = true;
      }
    }

    proto_field = Some(ProtoField::Oneof {
      path: oneof_path.ok_or(error!(
        Span::call_site(),
        "Failed to infer the path to the oneof"
      ))?,
      tags: oneof_tags,
      default: use_default,
      is_proxied,
    })
  }

  let tag = if let Some(tag) = tag {
    tag
  } else if is_ignored || proto_field.as_ref().is_some_and(|f| f.is_oneof()) {
    0
  } else {
    return Err(spanned_error!(original_name, "Field tag is missing"));
  };

  Ok(FieldAttrs {
    validator,
    tag,
    options: attributes::ProtoOptions(options),
    name: name.unwrap_or_else(|| ccase!(snake, original_name.to_string())),
    proto_field,
    from_proto,
    into_proto,
    is_ignored,
  })
}
