use crate::*;

impl OneofCtx<'_> {
  pub fn generate_schema_impl(&self) -> TokenStream2 {
    let enum_ident = self.proto_enum_ident();

    let variants_tokens = if self.variants.is_empty() {
      quote! { unimplemented!() }
    } else {
      let tokens = self
        .variants
        .iter()
        .filter_map(|v| v.as_normal())
        .map(|data| {
          let FieldData {
            tag,
            validator,
            options,
            proto_name,
            proto_field,
            deprecated,
            span,
            ..
          } = data;

          let field_type_tokens = proto_field.field_proto_type_tokens(*span);

          let validator_schema_tokens = validator
            .as_ref()
            // For default validators (messages only) we skip the schema generation
            .filter(|v| !v.is_fallback)
            .map_or_else(
              || quote_spanned! {*span=> None },
              |e| quote_spanned! {*span=> Some(#e.into_schema()) },
            );

          let options_tokens = options_tokens(*span, options, *deprecated);

          quote_spanned! {*span=>
            ::prelude::Field {
              name: #proto_name,
              tag: #tag,
              options: #options_tokens.into_iter().collect(),
              type_: #field_type_tokens,
              validator: #validator_schema_tokens,
            }
          }
        });

      quote! { #(#tokens),* }
    };

    let OneofAttrs {
      options: options_tokens,
      name: proto_name,
      ..
    } = &self.oneof_attrs;
    let tags = &self.tags;

    quote! {
      impl ::prelude::ProtoOneof for #enum_ident {
        #[doc(hidden)]
        const NAME: &str = #proto_name;
        #[doc(hidden)]
        const TAGS: &[i32] = &[ #(#tags),* ];

        fn proto_schema() -> ::prelude::Oneof {
          ::prelude::Oneof {
            name: #proto_name,
            fields: vec![ #variants_tokens ],
            options: #options_tokens.into_iter().collect(),
          }
        }
      }
    }
  }
}
