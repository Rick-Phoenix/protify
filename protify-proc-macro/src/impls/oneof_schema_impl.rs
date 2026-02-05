use crate::*;

impl OneofCtx<'_> {
	pub fn generate_schema_impl(&self) -> TokenStream2 {
		let enum_ident = self.proto_enum_ident;

		let variants_tokens = if self.variants.is_empty() {
			quote! { unimplemented!() }
		} else {
			let tokens = self
				.variants
				.iter()
				.filter_map(|v| v.as_normal())
				.map(|data| data.field_schema_tokens());

			quote! { #(#tokens),* }
		};

		let OneofAttrs {
			options: options_tokens,
			validators,
			..
		} = &self.oneof_attrs;
		let tags = &self.tags;

		quote! {
		  impl ::protify::ProtoOneof for #enum_ident {
				#[doc(hidden)]
				const TAGS: &[i32] = &[ #(#tags),* ];

				#[inline(never)]
				#[cold]
				fn proto_schema() -> ::protify::Oneof {
					::protify::Oneof::builder()
						.name(stringify!(#enum_ident).into())
						.fields(vec![ #variants_tokens ])
						.options(#options_tokens.into_iter().collect())
						.validators(::protify::collect_validators([ #(::protify::Validator::<#enum_ident>::schema(&#validators)),* ]))
						.build()
				}
		  }
		}
	}
}
