use crate::*;

pub struct EnumAttrs {
  pub reserved_names: Vec<String>,
  pub reserved_numbers: ReservedNumbers,
  pub options: TokensOr<TokenStream2>,
  pub parent_message: Option<Ident>,
  pub name: String,
  pub no_prefix: bool,
  pub extern_path: Option<String>,
  pub deprecated: bool,
}

pub fn process_derive_enum_attrs(
  enum_ident: &Ident,
  attrs: &[Attribute],
) -> Result<EnumAttrs, Error> {
  let mut reserved_names: Vec<String> = Vec::new();
  let mut reserved_numbers = ReservedNumbers::default();
  let mut options = TokensOr::<TokenStream2>::new(|| quote! { vec![] });
  let mut proto_name: Option<String> = None;
  let mut no_prefix = false;
  let mut parent_message: Option<Ident> = None;
  let mut extern_path: Option<String> = None;
  let mut deprecated = false;

  for attr in attrs {
    let ident = if let Some(ident) = attr.path().get_ident() {
      ident.to_string()
    } else {
      continue;
    };

    match ident.as_str() {
      "deprecated" => {
        deprecated = true;
      }
      "proto" => {
        attr.parse_nested_meta(|meta| {
          let ident = meta.path.require_ident()?.to_string();

          match ident.as_str() {
            "deprecated" => {
              let boolean = meta.parse_value::<LitBool>()?;

              deprecated = boolean.value();
            }
            "reserved_names" => {
              let names = meta.parse_list::<StringList>()?;

              reserved_names = names.list;
            }
            "reserved_numbers" => {
              let numbers = meta.parse_list::<ReservedNumbers>()?;

              reserved_numbers = numbers;
            }
            "extern_path" => {
              extern_path = Some(meta.expr_value()?.as_string()?);
            }
            "parent_message" => {
              parent_message = Some(
                meta
                  .expr_value()?
                  .as_path()?
                  .require_ident()?
                  .clone(),
              );
            }
            "options" => {
              options.set(meta.expr_value()?.into_token_stream());
            }
            "name" => {
              proto_name = Some(meta.expr_value()?.as_string()?);
            }
            "no_prefix" => no_prefix = true,
            _ => return Err(meta.error("Unknown attribute")),
          };

          Ok(())
        })?;
      }
      _ => {}
    }
  }

  let name = proto_name.unwrap_or_else(|| to_pascal_case(&enum_ident.to_string()));

  Ok(EnumAttrs {
    reserved_names,
    reserved_numbers,
    options,
    parent_message,
    name,
    no_prefix,
    extern_path,
    deprecated,
  })
}
