use crate::*;

pub fn package_macro_impl(input: TokenStream2) -> syn::Result<TokenStream2> {
  let input_span = input.span();

  let mut pkg_name: Option<String> = None;
  let mut include_cel_test = true;
  let mut fn_ident: Option<Ident> = None;

  let parser = syn::meta::parser(|meta| {
    let ident = meta.ident_str()?;

    match ident.as_str() {
      "name" => {
        pkg_name = Some(meta.parse_value::<LitStr>()?.value());
      }
      "no_cel_test" => {
        include_cel_test = false;
      }
      "fn_name" => {
        fn_ident = Some(meta.parse_value::<Ident>()?);
      }
      _ => return Err(meta.error("Unknown attribute")),
    };

    Ok(())
  });

  parser.parse2(input)?;

  let pkg_name = pkg_name.ok_or_else(|| error_with_span!(input_span, "package name is missing"))?;
  let converted_name = ccase!(snake, pkg_name.replace(".", "_"));
  let fn_ident = fn_ident.unwrap_or_else(|| format_ident!("{converted_name}_package"));

  let test_impl = include_cel_test.then(|| {
    let test_fn_ident = format_ident!("unique_cel_rules_{converted_name}");

    quote! {
      #[cfg(test)]
      #[test]
      fn #test_fn_ident() {
        let pkg = #fn_ident();

        if let Err(e) = pkg.check_unique_cel_rules() {
          panic!("{e}");
        }
      }
    }
  });

  Ok(quote! {
    pub fn #fn_ident() -> ::prelude::Package {
      ::prelude::collect_package(#pkg_name)
    }

    #test_impl
  })
}
