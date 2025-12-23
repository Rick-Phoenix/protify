use crate::*;

pub fn impl_oneof_consistency_checks(
  oneof_ident: &Ident,
  consistency_checks: Vec<TokenStream2>,
) -> TokenStream2 {
  quote! {
    #[cfg(test)]
    impl #oneof_ident {
      pub fn check_validators_consistency() -> Result<(), Vec<String>> {
        let mut errors: Vec<(&'static str, Vec<String>)> = Vec::new();

        #(
          let (field_name, check) = #consistency_checks;

          if let Err(errs) = check {
            errors.push((field_name, errs));
          }
        )*

        if errors.is_empty() {
          Ok(())
        } else {
          Err(::prelude::test_utils::format_oneof_errors(stringify!(#oneof_ident), errors))
        }
      }
    }
  }
}
