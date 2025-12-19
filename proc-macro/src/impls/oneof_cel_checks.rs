use crate::*;

pub fn impl_oneof_cel_checks(
  oneof_ident: &Ident,
  field_cel_checks: Vec<TokenStream2>,
) -> TokenStream2 {
  quote! {
    #[cfg(test)]
    impl #oneof_ident {
      #[track_caller]
      pub fn check_cel_programs() -> Result<(), Vec<CelError>> {
        let mut errors: Vec<::prelude::CelError> = Vec::new();

        #(
          if let Err(errs) = #field_cel_checks {
            errors.extend(errs);
          }
        )*

        if errors.is_empty() {
          Ok(())
        } else {
          Err(errors)
        }
      }
    }
  }
}
