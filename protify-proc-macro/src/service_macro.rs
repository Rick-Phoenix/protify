use crate::*;

struct HandlerCtx {
	name: String,
	span: Span,
	request: TokenStream2,
	request_is_stream: bool,
	response: TokenStream2,
	response_is_stream: bool,
	options: TokensOr<TokenStream2>,
	deprecated: bool,
}

pub fn process_service_macro(item: &ItemEnum) -> Result<TokenStream2, Error> {
	let ItemEnum {
		attrs,
		ident,
		variants,
		vis,
		..
	} = item;

	let mut handlers_data: Vec<HandlerCtx> = Vec::new();

	let ServiceOrHandlerAttrs {
		options: service_options,
		deprecated,
	} = process_service_or_handler_attrs(attrs)?;

	let service_name = to_pascal_case(&ident.to_string());

	for variant in variants {
		let ServiceOrHandlerAttrs {
			options: handler_options,
			deprecated,
		} = process_service_or_handler_attrs(&variant.attrs)?;

		let handler_name = variant.ident.to_string();

		let mut request_type: Option<&Path> = None;
		let mut response_type: Option<&Path> = None;
		let mut request_is_stream = false;
		let mut response_is_stream = false;

		let fields = if let Fields::Named(named) = &variant.fields {
			&named.named
		} else {
			bail!(
				variant,
				"Service variants must have 2 named fields, `request` and `response`"
			);
		};

		for field in fields {
			let field_ident = field.require_ident()?.to_string();
			let mut found_stream_attr = false;

			let field_type = match &field.ty {
				Type::Path(type_path) => &type_path.path,
				_ => bail!(&field.ty, "Expected a type path"),
			};

			for attr in &field.attrs {
				if attr.path().is_ident("stream") {
					found_stream_attr = true;
				}
			}

			match field_ident.as_str() {
				"request" => {
					request_type = Some(field_type);
					request_is_stream = found_stream_attr;
				}
				"response" => {
					response_type = Some(field_type);
					response_is_stream = found_stream_attr;
				}
				_ => bail!(
					variant,
					"Service variants must have 2 named fields, `request` and `response`"
				),
			};
		}

		let request = request_type
			.ok_or_else(|| error!(&variant, "Missing request type"))?
			.to_token_stream();
		let response = response_type
			.ok_or_else(|| error!(&variant, "Missing response type"))?
			.to_token_stream();

		handlers_data.push(HandlerCtx {
			name: handler_name,
			request,
			request_is_stream,
			response,
			response_is_stream,
			options: handler_options,
			deprecated,
			span: variant.ident.span(),
		});
	}

	// Using a struct + `iter()` rather than expanding a TokenStream because the output of
	// `cargo expand` is far worse with the latter

	let handlers_tokens = handlers_data.iter().map(|data| {
		let HandlerCtx {
			name,
			request,
			request_is_stream,
			response,
			response_is_stream,
			options,
			deprecated,
			span,
		} = data;

		quote_spanned! {*span=>
		  ::protify::ServiceHandler::builder()
				.name(#name.into())
				.request(
					::protify::HandlerTarget::builder()
						.message(<#request as ::protify::MessagePath>::proto_path())
						.is_stream(#request_is_stream)
						.build()
				)
				.response(
					::protify::HandlerTarget::builder()
						.message(<#response as ::protify::MessagePath>::proto_path())
						.is_stream(#response_is_stream)
						.build()
				)
				.options(::protify::__collect_options(#options, #deprecated))
				.build()
		}
	});

	Ok(quote! {
	  #[derive(::protify::macros::__Service)]
	  #vis struct #ident;

	  ::protify::register_proto_data! {
			::protify::RegistryService {
				package: __PROTO_FILE.package,
				service: || <#ident as ::protify::ProtoService>::proto_schema()
			}
	  }

	  impl ::protify::ProtoService for #ident {
			#[inline(never)]
			#[cold]
			fn proto_schema() -> ::protify::Service {
				::protify::Service::builder()
					.name(#service_name.into())
					.file(__PROTO_FILE.name.into())
					.package(__PROTO_FILE.package.into())
					.handlers(::protify::vec![ #(#handlers_tokens),* ])
					.options(::protify::__collect_options(#service_options, #deprecated))
					.build()
			}
	  }
	})
}
