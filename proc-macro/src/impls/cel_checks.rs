use crate::*;

pub struct CelChecksImplOutput {
  pub static_ident: Ident,
  pub cel_check_impl: TokenStream2,
}

pub fn impl_cel_checks(
  item_ident: &Ident,
  programs_paths: &[Path],
  cel_checks_tokens: TokenStream2,
) -> CelChecksImplOutput {
  let static_ident = format_ident!("{}_CEL_RULES", ccase!(constant, item_ident.to_string()));

  let top_level_programs = quote! { #(&*#programs_paths),* };

  let test_module_ident = format_ident!("__{}_cel_test", ccase!(snake, item_ident.to_string()));

  CelChecksImplOutput {
    cel_check_impl: quote! {
      static #static_ident: LazyLock<Vec<&'static CelProgram>> = LazyLock::new(|| {
        vec![ #top_level_programs ]
      });

      #[cfg(test)]
      mod #test_module_ident {
        use super::*;

        #[test]
        fn test() {
          #item_ident::validate_cel()
        }

        impl #item_ident {
          #[track_caller]
          fn validate_cel() {
            let mut errors: Vec<::prelude::CelError> = Vec::new();

            #cel_checks_tokens

            let top_level_programs = &#static_ident;

            if !top_level_programs.is_empty() {
              if let Err(errs) = ::prelude::test_programs(&top_level_programs, Self::default()) {
                errors.extend(errs);
              }
            }

            if !errors.is_empty() {
              for error in errors {
                eprintln!("{error}");
              }

              panic!();
            }
          }
        }
      }
    },
    static_ident,
  }
}
