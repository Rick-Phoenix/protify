use crate::*;

pub fn package_macro_impl(input: TokenStream2) -> syn::Result<TokenStream2> {
	let mut pkg_ident: Option<Ident> = None;
	let mut pkg_name: Option<String> = None;
	let mut include_cel_test = cfg!(feature = "cel");
	let mut files: Vec<Path> = Vec::new();

	let parser = syn::meta::parser(|meta| {
		let ident = meta.ident_str()?;

		match ident.as_str() {
			"files" => {
				let _ = meta.value()?;
				files = parse_bracketed::<PathList>(meta.input)?.list;
			}
			"name" => {
				pkg_name = Some(meta.parse_value::<LitStr>()?.value());
			}
			"no_cel_test" => {
				include_cel_test = false;
			}
			_ => pkg_ident = Some(meta.ident()?.clone()),
		};

		Ok(())
	});

	parser.parse2(input)?;

	let pkg_ident = pkg_ident
		.ok_or_else(|| error_call_site!("Missing package ident (must be the first argument)"))?;

	let pkg_name = pkg_name.ok_or_else(|| error_call_site!("package name is missing"))?;
	let converted_name = to_snake_case(&pkg_name.replace(".", "_"));

	let test_impl = include_cel_test.then(|| {
		let test_fn_ident = format_ident!("unique_cel_rules_{converted_name}");

		quote! {
		  #[cfg(test)]
		  #[test]
		  fn #test_fn_ident() {
				let pkg = <#pkg_ident as ::protify::PackageSchema>::get_package();

				if let Err(e) = pkg.check_unique_cel_rules() {
					panic!("{e}");
				}
		  }
		}
	});

	Ok(quote! {
	  #[allow(non_camel_case_types)]
	  pub struct #pkg_ident;

	  impl ::protify::PackageSchema for #pkg_ident {
			const NAME: &str = #pkg_name;

			#[inline(never)]
			#[cold]
			fn files() -> ::protify::Vec<::protify::ProtoFile> {
				::protify::vec![ #(<#files as ::protify::FileSchema>::file_schema()),* ]
			}
	  }

	  impl #pkg_ident {
			#[doc = concat!("Returns the fully built ", #pkg_name, " protobuf package.")]
			#[track_caller]
			pub fn get_package() -> ::protify::Package {
				<Self as ::protify::PackageSchema>::get_package()
			}
	  }

	  #test_impl
	})
}
