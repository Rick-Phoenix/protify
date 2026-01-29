use crate::*;

enum StoreType {
  Linear,
  Copy,
  Hybrid,
}

impl Parse for StoreType {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let ident = input.parse::<Ident>()?.to_string();

    if ident == "linear" {
      Ok(Self::Linear)
    } else if ident == "copy" {
      Ok(Self::Copy)
    } else if ident == "hybrid" {
      Ok(Self::Hybrid)
    } else {
      bail_call_site!("Unknown type")
    }
  }
}

#[derive(Default)]
enum ItemType {
  #[default]
  Message,
  Enum,
}

impl ItemType {
  /// Returns `true` if the item type is [`Message`].
  ///
  /// [`Message`]: ItemType::Message
  #[must_use]
  const fn is_message(&self) -> bool {
    matches!(self, Self::Message)
  }
}

impl Parse for ItemType {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let ident = input.parse::<Ident>()?.to_string();

    if ident == "Message" {
      Ok(Self::Message)
    } else if ident == "Enum" {
      Ok(Self::Enum)
    } else {
      bail_call_site!("Unknown type")
    }
  }
}

pub fn well_known_type_impl_macro(input: TokenStream2) -> syn::Result<TokenStream2> {
  let mut target: Option<Ident> = None;
  let mut file: Option<String> = None;
  let mut store = StoreType::Linear;
  let mut package: Option<String> = None;
  let mut name: Option<String> = None;
  let mut type_: Option<ItemType> = None;
  let mut impl_validator = true;

  let parser = syn::meta::parser(|meta| {
    let ident_str = meta.ident_str()?;

    match ident_str.as_str() {
      "impl_validator" => {
        impl_validator = meta.parse_value::<LitBool>()?.value();
      }
      "type_" => {
        type_ = Some(meta.parse_value()?);
      }
      "package" => {
        package = Some(meta.parse_value::<LitStr>()?.value());
      }
      "name" => {
        name = Some(meta.parse_value::<LitStr>()?.value());
      }
      "target" => {
        target = Some(meta.parse_value()?);
      }
      "file" => {
        file = Some(meta.parse_value::<LitStr>()?.value());
      }
      "store" => {
        store = meta.parse_value()?;
      }
      _ => return Err(meta.error("Unknown attribute")),
    };

    Ok(())
  });

  parser.parse2(input)?;

  let target = target.expect("Missing target");
  let package = package.expect("Missing package");
  let type_ = type_.unwrap_or_default();

  let name = name.unwrap_or_else(|| target.to_string());

  let file = file.unwrap_or_else(|| {
    format!(
      "{}/{}.proto",
      package.replace(".", "/"),
      to_snake_case(&name)
    )
  });

  let proto_validation_impl = impl_validator.then(|| {
    if !type_.is_message() {
      store = StoreType::Copy;
    }

    let unique_store_tokens = match store {
      StoreType::Linear => quote! { LinearRefStore<'a, Self> },
      StoreType::Copy => quote! { CopyHybridStore<Self> },
      StoreType::Hybrid => quote! { RefHybridStore<'a, Self> },
    };

    let message_impls = type_.is_message().then(|| {
      quote! {
        impl ValidatedMessage for #target {
          #[inline(always)]
          #[doc(hidden)]
          fn validate_with_ctx(&self, _: &mut ValidationCtx) -> ValidationResult {
            Ok(IsValid::Yes)
          }
        }
      }
    });

    quote! {
      impl ProtoValidation for #target {
        #[doc(hidden)]
        type ValidatorBuilder = NoOpValidatorBuilder<Self>;
        #[doc(hidden)]
        type Stored = Self;
        #[doc(hidden)]
        type Target = Self;
        #[doc(hidden)]
        type Validator = NoOpValidator<Self>;

        type UniqueStore<'a>
        = #unique_store_tokens
      where
        Self: 'a;

        #[doc(hidden)]
        const HAS_DEFAULT_VALIDATOR: bool = false;
      }

      #message_impls
    }
  });

  let proto_type_impl = if type_.is_message() {
    quote! {
      impl MessagePath for #target {
        fn proto_path() -> ProtoPath {
          ProtoPath {
            name: #name.into(),
            package: #package.into(),
            file: #file.into(),
          }
        }
      }

      impl AsProtoType for #target {
        fn proto_type() -> ProtoType {
          ProtoType::Message(Self::proto_path())
        }
      }
    }
  } else {
    quote! {
      impl AsProtoType for #target {
        fn proto_type() -> ProtoType {
          ProtoType::Enum(ProtoPath {
            name: #name.into(),
            package: #package.into(),
            file: #file.into(),
          })
        }
      }
    }
  };

  Ok(quote! {
    #proto_validation_impl
    #proto_type_impl
  })
}
