use crate::*;

pub fn process_oneof_derive2(
  oneof: &mut OneofData,
  tag_allocator: &mut TagAllocator,
) -> Result<TokenStream2, Error> {
  let mut variants_tokens: Vec<TokenStream2> = Vec::new();

  for variant in &oneof.variants {
    let field_name = &variant.data.name;
    let field_options = &variant.data.options;
    let proto_type = &variant.type_;
    let tag = tag_allocator.get_or_next(variant.data.tag);

    let validator_tokens = if let Some(validator) = &variant.data.validator {
      match validator {
        ValidatorExpr::Call(call) => {
          quote! { Some(<ValidatorMap as ProtoValidator<#proto_type>>::from_builder(#call)) }
        }
        ValidatorExpr::Closure(closure) => {
          quote! { Some(<ValidatorMap as ProtoValidator<#proto_type>>::build_rules(#closure)) }
        }
      }
    } else {
      quote! { None }
    };

    variants_tokens.push(quote! {
      ProtoField {
        name: #field_name.to_string(),
        options: #field_options,
        type_: <#proto_type as AsProtoType>::proto_type(),
        validator: #validator_tokens,
        tag: #tag,
      }
    });
  }

  let enum_name = &oneof.tokens.ident;
  let proto_name = &oneof.data.name;
  let oneof_options = &oneof.data.options;
  let required_option_tokens = oneof
    .data
    .required
    .then(|| quote! { options.push(oneof_required()); });

  let output_tokens = quote! {
    impl ProtoOneof for #enum_name {
      fn fields(tag_allocator: &mut TagAllocator) -> Vec<ProtoField> {
        vec![ #(#variants_tokens,)* ]
      }
    }

    impl #enum_name {
      #[track_caller]
      pub fn to_oneof(tag_allocator: &mut TagAllocator) -> Oneof {
        let mut options: Vec<ProtoOption> = #oneof_options;

        #required_option_tokens

        Oneof {
          name: #proto_name.into(),
          fields: vec![ #(#variants_tokens,)* ],
          options,
        }
      }
    }
  };

  Ok(output_tokens)
}
