use crate::*;

pub(crate) fn process_enum_derive(tokens: DeriveInput) -> Result<TokenStream2, Error> {
  let DeriveInput {
    attrs,
    ident: enum_name,
    data,
    ..
  } = tokens;

  let EnumAttrs {
    reserved_names,
    reserved_numbers,
    options,
    name: proto_name,
    file,
    package,
    full_name,
  } = process_enum_attrs(&enum_name, &attrs).unwrap();

  let reserved_numbers_tokens = reserved_numbers.to_token_stream();
  let mut manually_set_tags: Vec<i32> = Vec::new();

  let data = if let Data::Enum(enum_data) = data {
    enum_data
  } else {
    panic!("The enum derive can only be used on enums");
  };

  let mut output_tokens = TokenStream2::new();

  let mut variants_tokens: Vec<TokenStream2> = Vec::new();

  for variant in data.variants {
    if !variant.fields.is_empty() {
      panic!("Must be a unit variant");
    }

    let variant_name = variant.ident;

    let EnumVariantAttrs { tag, options, name } =
      process_enum_variants_attrs(&proto_name, &variant_name, &variant.attrs);

    if let Some(tag) = tag {
      manually_set_tags.push(tag);
    }

    let tag_tokens = OptionTokens::new(tag.as_ref());

    variants_tokens.push(quote! {
      EnumVariant { name: #name.to_string(), options: #options, tag: tag_allocator.get_or_next(#tag_tokens), }
    });
  }

  let occupied_ranges = reserved_numbers.build_unavailable_ranges(manually_set_tags);

  output_tokens.extend(quote! {
    impl ProtoEnumTrait for #enum_name {}

    impl ProtoValidator<#enum_name> for ValidatorMap {
      type Builder = EnumValidatorBuilder;

      fn builder() -> Self::Builder {
        EnumValidator::builder()
      }
    }

    impl AsProtoType for #enum_name {
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

    impl #enum_name {
      #[track_caller]
      pub fn to_enum() -> ProtoEnum {
        const UNAVAILABLE_TAGS: &'static [std::ops::Range<i32>] = &[#occupied_ranges];

        let mut tag_allocator = TagAllocator::new(UNAVAILABLE_TAGS);

        ProtoEnum {
          name: #proto_name.into(),
          full_name: #full_name,
          package: #package.into(),
          file: #file.into(),
          variants: vec! [ #(#variants_tokens,)* ],
          reserved_names: #reserved_names,
          reserved_numbers: vec![ #reserved_numbers_tokens ],
          options: #options,
        }
      }
    }
  });

  Ok(output_tokens)
}
