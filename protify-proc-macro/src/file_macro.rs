use crate::*;
use syn::{braced, custom_keyword, parse::ParseStream};

enum MessageExpr {
	Single(Path),
	Nested {
		path: Path,
		nested_messages: Vec<Self>,
		nested_enums: Vec<Path>,
	},
}

impl MessageExpr {
	const fn path(&self) -> &Path {
		match self {
			Self::Single(path) | Self::Nested { path, .. } => path,
		}
	}
}

impl ToTokens for MessageExpr {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let path = self.path();

		tokens.extend(quote! { <#path as ::protify::ProtoMessage>::proto_schema() });

		if let Self::Nested {
			nested_messages,
			nested_enums,
			..
		} = self
		{
			if !nested_messages.is_empty() {
				tokens.extend(quote! {
				  .with_nested_messages([ #(#nested_messages),* ])
				});
			}

			if !nested_enums.is_empty() {
				tokens.extend(quote! {
				  .with_nested_enums([ #(<#nested_enums as ::protify::ProtoEnumSchema>::proto_schema()),* ])
				});
			}
		}
	}
}

impl Parse for MessageExpr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let path: Path = input.parse()?;

		if input.peek(Token![=]) {
			input.parse::<Token![=]>()?;

			let content;
			braced!(content in input);

			let mut nested_messages = Vec::new();
			let mut nested_enums = Vec::new();

			while !content.is_empty() {
				let lookahead = content.lookahead1();

				if lookahead.peek(messages) {
					content.parse::<messages>()?;
					content.parse::<Token![=]>()?;

					nested_messages = parse_bracketed::<PunctuatedItems<Self>>(&content)?.list;
				} else if lookahead.peek(enums) {
					content.parse::<enums>()?;
					content.parse::<Token![=]>()?;

					nested_enums = parse_bracketed::<PathList>(&content)?.list;
				} else if lookahead.peek(Token![,]) {
					content.parse::<Token![,]>()?;
				} else {
					return Err(lookahead.error());
				}
			}

			Ok(Self::Nested {
				path,
				nested_messages,
				nested_enums,
			})
		} else {
			Ok(Self::Single(path))
		}
	}
}

custom_keyword!(messages);
custom_keyword!(enums);

pub fn process_file_macro(input: TokenStream2) -> syn::Result<TokenStream2> {
	let mut file_ident: Option<Ident> = None;
	let mut name: Option<String> = None;
	let mut package: Option<Path> = None;
	let mut options = TokenStreamOr::new(|_| quote! { ::protify::vec![] });
	let mut extern_path = TokensOr::<LitStr>::new(|_| quote! { ::core::module_path!() });
	let mut imports = TokenStreamOr::new(|_| quote! { ::protify::Vec::<&'static str>::new() });
	let mut extensions: Vec<Path> = Vec::new();
	let mut edition = TokenStreamOr::new(|_| quote! { ::protify::Edition::Proto3 });
	let mut messages: Vec<MessageExpr> = Vec::new();
	let mut enums: Vec<Path> = Vec::new();
	let mut services: Vec<Path> = Vec::new();
	let mut no_emit = false;

	let parser = syn::meta::parser(|meta| {
		let ident_str = meta.ident_str()?;

		match ident_str.as_str() {
			"no_emit" => {
				no_emit = true;
			}
			"messages" => {
				messages = parse_bracketed::<PunctuatedItems<MessageExpr>>(meta.value()?)?.list;
			}
			"enums" => {
				enums = parse_bracketed::<PathList>(meta.value()?)?.list;
			}
			"services" => {
				services = parse_bracketed::<PathList>(meta.value()?)?.list;
			}
			"name" => {
				name = Some(meta.parse_value::<LitStr>()?.value());
			}
			"package" => {
				package = Some(meta.parse_value::<Path>()?);
			}
			"options" => {
				options.span = meta.input.span();
				options.set(meta.expr_value()?.into_token_stream());
			}
			"extern_path" => {
				extern_path.set(meta.parse_value::<LitStr>()?);
			}
			"imports" => {
				imports.set(meta.parse_value::<Expr>()?.into_token_stream());
			}
			"extensions" => {
				extensions = parse_bracketed::<PathList>(meta.value()?)?.list;
			}
			"edition" => {
				edition.set(meta.parse_value::<Path>()?.into_token_stream());
			}
			_ => {
				file_ident = Some(meta.ident()?.clone());
			}
		};

		Ok(())
	});

	parser.parse2(input)?;

	let file_ident = file_ident
		.ok_or_else(|| error_call_site!("Missing file ident (must be the first argument)"))?;
	let file = name.unwrap_or_else(|| {
		let ident_str = file_ident.to_string();
		let cleaned_name = ident_str
			.strip_suffix("_FILE")
			.unwrap_or(&ident_str);

		format!("{}.proto", to_snake_case(cleaned_name))
	});
	let package = package.ok_or_else(|| error_call_site!("Missing `package` attribute"))?;

	let inventory_call = (!no_emit).then(|| {
		quote! {
		  ::protify::register_proto_data! {
				::protify::RegistryFile {
					file: || <#file_ident as ::protify::FileSchema>::file_schema(),
					package: <#package as ::protify::PackageSchema>::NAME
				}
		  }
		}
	});

	Ok(quote! {
	  #[doc(hidden)]
	  #[allow(unused)]
	  const __PROTO_FILE: ::protify::FileReference = ::protify::FileReference {
			name: #file,
			package: <#package as ::protify::PackageSchema>::NAME,
			extern_path: #extern_path,
	  };

	  #[allow(non_camel_case_types)]
	  pub(crate) struct #file_ident;

	  impl ::protify::FileSchema for #file_ident {
			const NAME: &str = #file;
			const PACKAGE: &str = <#package as ::protify::PackageSchema>::NAME;
			const EXTERN_PATH: &str = #extern_path;

			fn file_schema() -> ::protify::ProtoFile {
				let mut file = ::protify::ProtoFile::new(#file, <#package as ::protify::PackageSchema>::NAME);

				file
				.with_edition(#edition)
				.with_messages([ #(#messages),* ])
				.with_services([ #(#services::proto_schema()),* ])
				.with_enums([ #(#enums::proto_schema()),* ])
				.with_extensions([ #(#extensions::proto_schema()),* ])
				.with_imports(#imports)
				.with_options(#options);

				file
			}
	  }

	  #inventory_call
	})
}
