use crate::*;

mod fallback_impls;
mod message_schema_impl;
mod oneof_schema_impl;
pub use fallback_impls::*;
mod conversions;
pub use oneof_validator_impl::*;
mod oneof_validator_impl;
pub use conversions::*;
mod consistency_checks;
pub use consistency_checks::*;
mod message_validator_impl;
pub use message_validator_impl::*;

pub fn wrap_with_imports(tokens: &TokenStream2) -> TokenStream2 {
  quote! {
    const _: () = {
      use ::protify::{Validator as ___Validator};
      use ::protify::alloc::{vec, vec::Vec, format, string::String};

      #tokens
    };
  }
}

pub fn wrap_multiple_with_imports(tokens: &[TokenStream2]) -> TokenStream2 {
  wrap_with_imports(&quote! { #(#tokens)* })
}
