use crate::*;

mod fallback_impls;
pub use fallback_impls::*;

pub fn wrap_with_imports(tokens: &[TokenStream2]) -> TokenStream2 {
  quote! {
    const _: () = {
      use std::sync::LazyLock;
      use ::prelude::*;
      use ::prelude::proto_types::{
        protovalidate::{Violations, FieldPathElement},
        field_descriptor_proto::Type,
      };

      #(#tokens)*
    };
  }
}

pub fn options_tokens(options: &TokensOr<TokenStream2>, deprecated: bool) -> TokenStream2 {
  if deprecated {
    quote! {
      {
        let mut options: Vec<::prelude::ProtoOption> = #options;
        options.push(::prelude::proto_deprecated());
        options
      }
    }
  } else {
    options.to_token_stream()
  }
}
