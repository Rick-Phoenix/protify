use crate::*;

pub fn impl_oneof_consistency_checks<T>(
  oneof_ident: &Ident,
  variants: &[T],
  no_auto_test: bool,
) -> TokenStream2
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

  let auto_test_fn = (!no_auto_test).then(|| {
    let test_fn_ident = format_ident!(
      "{}_validators_consistency",
      ccase!(snake, oneof_ident.to_string())
    );

    quote! {
      #[cfg(test)]
      #[test]
      fn #test_fn_ident() {
        if let Err(e) = #oneof_ident::check_validators_consistency() {
          panic!("{e}")
        }
      }
    }
  });

  quote! {
    #auto_test_fn

    #[cfg(test)]
    impl #oneof_ident {
      pub fn check_validators_consistency() -> Result<(), ::prelude::test_utils::OneofErrors> {
        let mut errors: Vec<(&'static str, Vec<::prelude::test_utils::ConsistencyError>)> = Vec::new();

        #(
          let (field_name, check) = #consistency_checks;

          if let Err(errs) = check {
            errors.push((field_name, errs));
          }
        )*

        if errors.is_empty() {
          Ok(())
        } else {
          Err(
            ::prelude::test_utils::OneofErrors {
              oneof_name: stringify!(#oneof_ident),
              errors
            }
          )
        }
      }
    }
  }
}
