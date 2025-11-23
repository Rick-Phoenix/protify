use crate::*;

pub(crate) fn process_message_derive(tokens: DeriveInput) -> Result<TokenStream2, Error> {
  let DeriveInput {
    attrs,
    ident: struct_name,
    data,
    ..
  } = tokens;

  let MessageAttrs {
    reserved_names,
    reserved_numbers,
    options,
    name: proto_name,
    file,
    package,
    nested_messages,
    full_name,
    nested_enums,
    oneofs,
  } = process_message_attrs(&struct_name, &attrs).unwrap();

  let reserved_numbers_tokens = reserved_numbers.to_token_stream();

  let data = if let Data::Struct(struct_data) = data {
    struct_data
  } else {
    panic!()
  };

  let fields = if let Fields::Named(fields) = data.fields {
    fields.named
  } else {
    panic!()
  };

  let mut output_tokens = TokenStream2::new();

  let mut fields_data: Vec<TokenStream2> = Vec::new();
  let mut manually_set_tags: Vec<i32> = Vec::new();

  for field in fields {
    let field_name = field.ident.as_ref().expect("Expected named field");

    let field_attrs = if let Some(attrs) = process_field_attrs(field_name, &field.attrs)? {
      attrs
    } else {
      continue;
    };

    let FieldAttrs {
      tag,
      validator,
      options,
      name,
      type_,
    } = field_attrs;

    let mut field_type = match &field.ty {
      Type::Path(type_path) => &type_path.path,

      _ => panic!("Must be a path type"),
    };

    if let Some(oneofs) = &oneofs && oneofs.contains(field_type) {
      fields_data.push(quote! {
        MessageEntry::Oneof(#field_type::to_oneof(&mut tag_allocator))
      });

      continue;
    }

    if let Some(tag) = tag {
      manually_set_tags.push(tag);
    }

    let processed_type = extract_type_from_path(field_type);

    field_type = processed_type.path();

    let is_optional = processed_type.is_option();

    let proto_type = if let Some(type_data) = &type_ {
      type_data
    } else {
      field_type
    };

    let validator_tokens = if let Some(validator) = validator {
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

    let field_type_tokens = if is_optional {
      quote! { <Option<#proto_type> as AsProtoType>::proto_type() }
    } else {
      quote! { <#proto_type as AsProtoType>::proto_type() }
    };

    let tag_tokens = OptionTokens::new(tag.as_ref());

    fields_data.push(quote! {
      MessageEntry::Field(
        ProtoField {
          name: #name.to_string(),
          tag: tag_allocator.get_or_next(#tag_tokens),
          options: #options,
          type_: #field_type_tokens,
          validator: #validator_tokens,
        }
      )
    });
  }

  let occupied_ranges = reserved_numbers.build_unavailable_ranges(manually_set_tags);

  output_tokens.extend(quote! {
    impl ProtoMessage for #struct_name {}

    impl ProtoValidator<#struct_name> for ValidatorMap {
      type Builder = MessageValidatorBuilder;

      fn builder() -> Self::Builder {
        MessageValidator::builder()
      }
    }

    impl AsProtoType for #struct_name {
      fn proto_type() -> ProtoType {
        ProtoType::Single(TypeInfo {
          name: #full_name,
          path: Some(ProtoPath {
            file: #file.into(),
            package: #package.into()
          })
        })
      }
    }

    impl #struct_name {
      #[track_caller]
      pub fn to_message() -> Message {
        const UNAVAILABLE_TAGS: &'static [std::ops::Range<i32>] = &[#occupied_ranges];

        let mut tag_allocator = TagAllocator::new(UNAVAILABLE_TAGS);

        let mut new_msg = Message {
          name: #proto_name,
          full_name: #full_name,
          package: #package.into(),
          file: #file.into(),
          reserved_names: #reserved_names,
          reserved_numbers: vec![ #reserved_numbers_tokens ],
          options: #options,
          messages: vec![ #nested_messages ],
          enums: vec![ #nested_enums ],
          entries: vec![ #(#fields_data,)* ],
        };

        new_msg
      }
    }
  });

  Ok(output_tokens)
}
