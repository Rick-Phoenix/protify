use crate::*;

pub fn named_enum_derive(item: &ItemEnum) -> syn::Result<TokenStream2> {
  let mut name: Option<String> = None;

  for attr in &item.attrs {
    if attr.path().is_ident("proto") {
      attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("name") {
          name = Some(meta.parse_value::<LitStr>()?.value());
        } else {
          return Err(meta.error("Unknown attribute"));
        }

        Ok(())
      })?;
    }
  }

  let ident = &item.ident;
  let name = name.unwrap_or_else(|| to_pascal_case(&item.ident.to_string()));

  Ok(quote! {
    impl ::prelude::ProtoEnum for #ident {
      fn proto_name() -> &'static str {
        #name
      }
    }
  })
}
