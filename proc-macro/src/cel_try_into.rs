use crate::*;

pub fn get_conversion_tokens(
  type_info: &TypeInfo,
  val_tokens: &TokenStream2,
  proto_types_path: &TokensOr<TokenStream2>,
) -> TokenStream2 {
  match type_info.type_.as_ref() {
    RustType::Box(_) => quote! { (*#val_tokens).try_into_cel_recursive(depth + 1)? },
    RustType::Bytes => quote! { #val_tokens.to_vec().into() },
    RustType::Float(_) | RustType::Uint(_) | RustType::Int(_) | RustType::Bool => {
      quote! { #val_tokens.into() }
    }
    _ => {
      quote! { #val_tokens.try_into().map_err(#proto_types_path::cel::CelConversionError::from)? }
    }
  }
}

pub fn derive_cel_value_oneof(
  item: &ItemEnum,
  cel_crate_path: &TokensOr<TokenStream2>,
  proto_types_path: &TokensOr<TokenStream2>,
) -> Result<TokenStream2, Error> {
  let enum_name = &item.ident;

  let variants = &item.variants;

  let mut match_arms = Vec::<TokenStream2>::new();

  for variant in variants {
    let variant_ident = &variant.ident;
    let proto_name = to_snake_case(&variant_ident.to_string());

    if let syn::Fields::Unnamed(fields) = &variant.fields
      && let Some(variant_type) = &fields.unnamed.get(0)
    {
      let type_ident = &variant_type.ty;

      let type_info = TypeInfo::from_type(type_ident)?;

      let into_expression = get_conversion_tokens(&type_info, &quote! { val }, proto_types_path);

      match_arms.push(quote! {
        #enum_name::#variant_ident(val) => {
          if depth >= 16 {
            Ok((#proto_name.to_string(), #cel_crate_path::Value::Null))
          } else {
            Ok((#proto_name.to_string(), #into_expression))
          }
        }
      });
    }
  }

  // We cannot rely on the try_into impl as is here, because we need to know
  // the name of the specific oneof variant being used, so we need this helper here.
  // In the future we might skip this and just use the name of the oneof, to mirror
  // the rust side of things more accurately
  Ok(quote! {
    impl ::prelude::CelOneof for #enum_name {
      #[doc(hidden)]
      fn try_into_cel_recursive(self, depth: usize) -> Result<(String, #cel_crate_path::Value), #proto_types_path::cel::CelConversionError> {
        use ::prelude::{CelOneof, CelValue};

        match self {
          #(#match_arms),*
        }
      }
    }

    impl TryFrom<#enum_name> for #cel_crate_path::Value {
      type Error = #proto_types_path::cel::CelConversionError;

      #[inline]
      fn try_from(value: #enum_name) -> Result<Self, Self::Error> {
        Ok(<#enum_name as ::prelude::CelOneof>::try_into_cel_recursive(value, 0)?.1)
      }
    }
  })
}

pub(crate) fn derive_cel_value_struct(
  item: &ItemStruct,
  cel_crate_path: &TokensOr<TokenStream2>,
  proto_types_path: &TokensOr<TokenStream2>,
) -> Result<TokenStream2, Error> {
  let struct_name = &item.ident;

  let fields = if let syn::Fields::Named(fields) = &item.fields {
    &fields.named
  } else {
    bail!(
      item,
      "This derive macro only works on structs with named fields"
    );
  };

  let mut tokens = TokenStream2::new();

  for field in fields {
    let field_ident = field.ident.as_ref().unwrap();
    let field_name = field_ident.to_string();
    let field_type = &field.ty;
    let mut is_oneof = false;

    for attr in &field.attrs {
      if attr.path().is_ident("prost") {
        let _ = attr.parse_nested_meta(|meta| {
          if meta.path.is_ident("oneof") {
            is_oneof = true;
          }
          Ok(())
        });
      }
    }

    if is_oneof {
      tokens.extend(quote! {
        if let Some(oneof) = value.#field_ident {
          let (oneof_field_name, cel_val) = ::prelude::CelOneof::try_into_cel_recursive(oneof, depth + 1)?;
          fields.insert(oneof_field_name.into(), cel_val);
        }
      });
    } else {
      let outer_type = TypeInfo::from_type(field_type)?;

      let val_tokens = quote! { val };

      match outer_type.type_.as_ref() {
        RustType::Option(inner) => {
          let conversion_tokens = get_conversion_tokens(inner, &val_tokens, proto_types_path);

          tokens.extend(quote! {
            if let Some(val) = value.#field_ident {
              fields.insert(#field_name.into(), #conversion_tokens);
            } else {
              fields.insert(#field_name.into(), #cel_crate_path::Value::Null);
            }
          });
        }
        RustType::Vec(inner) => {
          let conversion_tokens = get_conversion_tokens(inner, &val_tokens, proto_types_path);

          tokens.extend(quote! {
            let mut converted: Vec<#cel_crate_path::Value> = Vec::new();
            for val in value.#field_ident {
              converted.push(#conversion_tokens);
            }

            fields.insert(#field_name.into(), #cel_crate_path::Value::List(converted.into()));
          });
        }

        RustType::HashMap((k, v)) => {
          let keys_conversion_tokens = get_conversion_tokens(k, &quote! { key }, proto_types_path);
          let values_conversion_tokens = get_conversion_tokens(v, &val_tokens, proto_types_path);

          tokens.extend(quote! {
            let mut field_map: ::std::collections::HashMap<#cel_crate_path::objects::Key, #cel_crate_path::Value> = ::std::collections::HashMap::new();

            for (key, val) in value.#field_ident {
              field_map.insert(#keys_conversion_tokens, #values_conversion_tokens);
            }

            fields.insert(#field_name.into(), #cel_crate_path::Value::Map(field_map.into()));
          });
        }
        _ => {
          let val_tokens = quote! { value.#field_ident };
          let conversion_tokens = get_conversion_tokens(&outer_type, &val_tokens, proto_types_path);

          tokens.extend(quote! {
            fields.insert(#field_name.into(), #conversion_tokens);
          });
        }
      };
    }
  }

  Ok(quote! {
    impl ::prelude::CelValue for #struct_name {
      fn try_into_cel_recursive(self, depth: usize) -> Result<#cel_crate_path::Value, #proto_types_path::cel::CelConversionError> {
        use ::prelude::{CelOneof, CelValue};
        if depth >= 16 {
          return Ok(#cel_crate_path::Value::Null);
        }

        let mut fields: ::std::collections::HashMap<#cel_crate_path::objects::Key, #cel_crate_path::Value> = std::collections::HashMap::new();
        let value = self;

        #tokens

        Ok(#cel_crate_path::Value::Map(fields.into()))
      }
    }

    impl TryFrom<#struct_name> for #cel_crate_path::Value {
      type Error = #proto_types_path::cel::CelConversionError;

      #[inline]
      fn try_from(value: #struct_name) -> Result<Self, Self::Error> {
        use ::prelude::{CelOneof, CelValue};

        value.try_into_cel_recursive(0)
      }
    }
  })
}
