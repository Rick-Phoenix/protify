use crate::*;

pub fn wrap_with_imports(item_ident: &Ident, tokens: TokenStream2) -> TokenStream2 {
  let module_ident = format_ident!("__proto_{}", ccase!(snake, item_ident.to_string()));
  quote! {
    #[doc(inline)]
    pub use #module_ident::*;

    #[doc(hidden)]
    mod #module_ident {
      use super::*;
      use std::sync::LazyLock;
      use ::prelude::{*, field_context::Violations};

      #tokens
    }
  }
}
