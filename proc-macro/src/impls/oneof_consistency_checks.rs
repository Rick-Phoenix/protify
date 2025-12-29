use crate::*;

pub fn impl_oneof_consistency_checks<T>(oneof_ident: &Ident, variants: &[T]) -> TokenStream2
where
  T: Borrow<FieldData>,
{
  let consistency_checks = variants.iter().filter_map(|data| {
    let FieldData {
      ident_str,
      validator,
      ..
    } = data.borrow();

    validator
      .as_ref()
      // Useless to check consistency for default validators
      .filter(|v| !v.is_fallback)
      .map(|validator| {
        quote! {
          (#ident_str, #validator.check_consistency())
        }
      })
  });

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
