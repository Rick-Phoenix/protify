use crate::*;

pub struct EnumVariantAttrs {
  pub name: String,
  pub tag: Option<i32>,
  pub options: ProtoOptions,
}

pub fn process_enum_variants_attrs(
  enum_name: &str,
  rust_variant_name: &Ident,
  attrs: &Vec<Attribute>,
  is_in_module_macro: bool,
) -> EnumVariantAttrs {
  let mut tag: Option<i32> = None;
  let mut options: Option<TokenStream2> = None;
  let mut name: Option<String> = None;

  for attr in attrs {
    if !attr.path().is_ident("proto") {
      continue;
    }

    let args = attr.parse_args::<PunctuatedParser<Meta>>().unwrap();

    for meta in args.inner {
      match meta {
        Meta::NameValue(nameval) => {
          if nameval.path.is_ident("tag") {
            tag = Some(extract_i32(&nameval.value).unwrap());
          } else if !is_in_module_macro && nameval.path.is_ident("options") {
            let func_call = nameval.value;

            options = Some(quote! { #func_call });
          } else if nameval.path.is_ident("name") {
            name = Some(extract_string_lit(&nameval.value).unwrap());
          }
        }
        Meta::List(list) => {
          if !is_in_module_macro && list.path.is_ident("options") {
            let exprs = list.parse_args::<PunctuatedParser<Expr>>().unwrap().inner;

            options = Some(quote! { vec! [ #exprs ] });
          }
        }
        Meta::Path(_) => {}
      };
    }
  }

  let name = format!(
    "{}_{}",
    ccase!(constant, enum_name),
    name.unwrap_or_else(|| ccase!(constant, rust_variant_name.to_string()))
  );

  EnumVariantAttrs {
    tag,
    options: attributes::ProtoOptions(options),
    name,
  }
}
