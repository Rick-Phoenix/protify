use crate::*;

pub fn generate_oneof_validator(
  use_fallback: UseFallback,
  oneof_ident: &Ident,
  variants: &[FieldDataKind],
) -> TokenStream2 {
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
            #(
              is_valid &= #validators?;
            )*
          }
        }
      });

    quote! {
      let mut is_valid = ::prelude::IsValid::Yes;
      match self {
        #(#tokens,)*
        _ => {}
      };

      Ok(is_valid)
    }
  };

  // Validators will always be populated if at least one field
  // is a message, because we cannot know if it has validators
  // of its own
  if validators_tokens.is_empty() {
    quote! {
      impl ::prelude::ValidatedOneof for #oneof_ident {
        #[inline(always)]
        fn validate(&self, _: &mut ::prelude::ValidationCtx) -> ::prelude::ValidatorResult {
          Ok(::prelude::IsValid::Yes)
        }
      }
    }
  } else {
    quote! {
      impl ::prelude::ValidatedOneof for #oneof_ident {
        fn validate(&self, ctx: &mut ::prelude::ValidationCtx) -> ::prelude::ValidatorResult {
          #validators_tokens
        }
      }
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
