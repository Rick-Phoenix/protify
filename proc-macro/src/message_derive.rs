use crate::*;

pub(crate) fn process_message_derive2(
  msg: &mut MessageData,
  oneofs_map: &mut HashMap<Ident, OneofData>,
  module_attrs: &ModuleAttrs,
) -> Result<TokenStream2, Error> {
  let MessageData {
    fields,
    reserved_names,
    reserved_numbers,
    options,
    name: proto_name,
    oneofs,
    used_tags,
    ..
  } = msg;

  let mut fields_tokens: Vec<TokenStream2> = Vec::new();

  for oneof in oneofs {
    let oneof_data = oneofs_map.get_mut(oneof).expect("Failed to find oneof");

    let taken_tags = std::mem::take(&mut oneof_data.used_tags);
    used_tags.extend(taken_tags);
  }

  let unavailable_tags = reserved_numbers
    .clone()
    .build_unavailable_ranges(used_tags.clone());

  let mut tag_allocator = TagAllocator::new(&unavailable_tags.0);

  for field in fields {
    if field.data.is_oneof {
      let oneof = oneofs_map
        .get_mut(field.type_.require_ident()?)
        .expect("Failed to find oneof");

      for variant in &mut oneof.variants {
        variant.data.tag = Some(tag_allocator.get_or_next(variant.data.tag));
      }

      continue;
    }

    let name = &field.data.name;
    let proto_type = &field.type_;
    let field_options = &field.data.options;
    let tag = tag_allocator.get_or_next(field.data.tag);

    let validator_tokens = if let Some(validator) = &field.data.validator {
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

    let field_type_tokens = quote! { <#proto_type as AsProtoType>::proto_type() };

    fields_tokens.push(quote! {
      MessageEntry::Field(
        ProtoField {
          name: #name.to_string(),
          tag: #tag,
          options: #field_options,
          type_: #field_type_tokens,
          validator: #validator_tokens,
        }
      )
    });
  }

  let struct_name = &msg.tokens.ident;
  let full_name = msg.full_name.get().unwrap_or_else(|| proto_name);
  let file = &module_attrs.file;
  let package = &module_attrs.package;

  let output_tokens = quote! {
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
        let mut new_msg = Message {
          name: #proto_name,
          full_name: #full_name,
          package: #package.into(),
          file: #file.into(),
          reserved_names: #reserved_names,
          reserved_numbers: vec![ #reserved_numbers ],
          options: #options,
          entries: vec![ #(#fields_tokens,)* ],
          ..Default::default()
        };

        new_msg
      }
    }
  };

  Ok(output_tokens)
}
