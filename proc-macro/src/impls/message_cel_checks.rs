use crate::*;

pub fn impl_message_cel_checks(
  item_ident: &Ident,
  programs_paths: Option<&Vec<Path>>,
  field_cel_checks: Vec<TokenStream2>,
  no_auto_test: bool,
  message_name: &str,
) -> (TokenStream2, Option<Ident>) {
  let (static_ident, top_level_programs) = if let Some(paths) = programs_paths {
    let static_ident = format_ident!("{}_CEL_RULES", ccase!(constant, item_ident.to_string()));

    let programs = quote! {
      static #static_ident: LazyLock<Vec<&'static CelProgram>> = LazyLock::new(|| {
        vec![ #(&*#paths),* ]
      });
    };

    (Some(static_ident), Some(programs))
  } else {
    (None, None)
  };

  let top_level_programs_check = static_ident.as_ref().map(|ident| {
    quote! {
      let top_level_programs = &#ident;

      if !top_level_programs.is_empty() {
        if let Err(errs) = ::prelude::test_programs(top_level_programs.as_slice(), Self::default()) {
          errors.extend(errs);
        }
      }
    }
  });

  let test_module_ident = format_ident!("__{}_cel_test", ccase!(snake, item_ident.to_string()));

  let auto_test_fn = if !no_auto_test {
    Some(quote! {
      #[test]
      fn test() {
        #item_ident::check_cel_programs()
      }
    })
  } else {
    None
  };

  (
    quote! {
      #top_level_programs

      #[cfg(test)]
      mod #test_module_ident {
        use super::*;

        #auto_test_fn

        impl #item_ident {
          #[track_caller]
          pub(crate) fn check_cel_programs() {
            let mut errors: Vec<::prelude::CelError> = Vec::new();

            #(
              if let Err(errs) = #field_cel_checks {
                errors.extend(errs);
              }
            )*

            #top_level_programs_check

            if !errors.is_empty() {
              ::prelude::test_utils::check_cel_programs(#message_name, errors)
            }
          }
        }
      }
    },
    static_ident,
  )
}
