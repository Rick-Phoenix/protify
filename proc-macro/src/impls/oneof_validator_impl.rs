use crate::*;

pub fn generate_oneof_validator(
  use_fallback: UseFallback,
  oneof_ident: &Ident,
  variants: &[FieldDataKind],
) -> TokenStream2 {
  let mut maybe_default_validators = 0;
  let mut non_default_validators = 0;

  let validators_tokens = if *use_fallback {
    quote! { unimplemented!() }
  } else {
    let tokens = variants
      .iter()
      .filter_map(|d| d.as_normal())
      .filter_map(|d| {
        let tokens = field_validator_tokens(d, ItemKind::Oneof);

        (!tokens.is_empty()).then_some((d, tokens))
      })
      .map(|(data, validators)| {
        let ident = &data.ident;

        quote_spanned! {data.span=>
          Self::#ident(v) => {
            #(#validators)*
          }
        }
      });

    quote! { #(#tokens,)* }
  };

  for v in variants
    .iter()
    .filter_map(|f| f.as_normal())
    .flat_map(|d| &d.validators)
  {
    if v.kind.is_default() {
      maybe_default_validators += 1;
    } else {
      non_default_validators += 1;
    }
  }

  let has_validators = maybe_default_validators + non_default_validators != 0;

  let inline_if_empty = (!has_validators).then(|| quote! { #[inline(always)] });

  let has_default_validator_tokens = if !has_validators {
    quote! { false }
  } else if non_default_validators > 0 {
    quote! { true }
  } else {
    let message_paths = variants
      .iter()
      .filter_map(|f| f.as_normal())
      .filter_map(|f| f.message_info())
      .filter_map(|m| (!m.boxed).then_some(&m.path));

    let mut has_default_validator_tokens = TokenStream2::new();

    for (i, path) in message_paths.enumerate() {
      if i != 0 {
        has_default_validator_tokens.extend(quote! { && });
      }

      has_default_validator_tokens
        .extend(quote! { <#path as ::prelude::ProtoValidator>::HAS_DEFAULT_VALIDATOR });
    }

    // If we got this far, we only met Boxed messages which we cannot check
    // without causing an infinite loop, so we are forced to fall back to `true`
    if has_default_validator_tokens.is_empty() {
      has_default_validator_tokens = quote! { true };
    }

    has_default_validator_tokens
  };

  quote! {
    impl ::prelude::ValidatedOneof for #oneof_ident {
      #inline_if_empty
      fn validate(&self, ctx: &mut ::prelude::ValidationCtx) -> ::prelude::ValidatorResult {
        let mut is_valid = ::prelude::IsValid::Yes;

        match self {
          #validators_tokens
          _ => {}
        };

        Ok(is_valid)
      }
    }

    impl ::prelude::ProtoValidator for #oneof_ident {
      #[doc(hidden)]
      type Target = Self;
      #[doc(hidden)]
      type Validator = ::prelude::OneofValidator;
      #[doc(hidden)]
      type Builder = ::prelude::OneofValidator;

      type UniqueStore<'a>
      = ::prelude::LinearRefStore<'a, Self>
    where
      Self: 'a;

      const HAS_DEFAULT_VALIDATOR: bool = #has_default_validator_tokens;
    }
  }
}

impl OneofCtx<'_> {
  pub fn generate_validator(&self) -> TokenStream2 {
    let oneof_ident = self.proto_enum_ident();

    // For non-reflection implementations we don't skip fields if they don't have
    // validators, so empty fields = an error occurred
    generate_oneof_validator(self.variants.is_empty().into(), oneof_ident, &self.variants)
  }
}
