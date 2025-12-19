use crate::*;

pub fn impl_message_cel_checks(
  item_ident: &Ident,
  static_ident: &Ident,
  programs_paths: &[Path],
  field_cel_checks: Vec<TokenStream2>,
) -> TokenStream2 {
  let top_level_programs = quote! { #(&*#programs_paths),* };

  let test_module_ident = format_ident!("__{}_cel_test", ccase!(snake, item_ident.to_string()));

  quote! {
    static #static_ident: LazyLock<Vec<&'static CelProgram>> = LazyLock::new(|| {
      vec![ #top_level_programs ]
    });

    #[cfg(test)]
    mod #test_module_ident {
      use super::*;

      #[test]
      fn test() {
        #item_ident::check_cel_programs()
      }

      impl #item_ident {
        #[track_caller]
        fn check_cel_programs() {
          let mut errors: Vec<::prelude::CelError> = Vec::new();

          #(
            if let Err(errs) = #field_cel_checks {
              errors.extend(errs);
            }
          )*

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

            panic!("Failed CEL program test");
          }
        }
      }
    }
  }
}
