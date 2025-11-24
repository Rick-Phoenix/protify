use crate::*;

pub fn process_enum_derive2(
  enum_data: &mut EnumData,
  module_attrs: &ModuleAttrs,
) -> Result<TokenStream2, Error> {
  let EnumData {
    name: proto_name,
    reserved_names,
    reserved_numbers,
    options,
    variants,
    used_tags,
    tokens,
  } = enum_data;

  let mut variants_tokens: Vec<TokenStream2> = Vec::new();
  let taken_tags = reserved_numbers
    .clone()
    .build_unavailable_ranges(used_tags.clone());

  let mut tag_allocator = TagAllocator::new(&taken_tags.0);

  for variant in variants {
    let tag = tag_allocator.get_or_next(variant.tag);
    let name = &variant.name;
    let options = &variant.options;

    variants_tokens.push(quote! {
      EnumVariant { name: #name.to_string(), options: #options, tag: #tag, }
    });
  }

  let enum_name = &tokens.ident;
  let full_name = "todo";

  let ModuleAttrs { file, package } = module_attrs;

  let output_tokens = quote! {
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
        ProtoEnum {
          name: #proto_name.into(),
          full_name: #full_name,
          package: #package.into(),
          file: #file.into(),
          variants: vec! [ #(#variants_tokens,)* ],
          reserved_names: #reserved_names,
          reserved_numbers: vec![ #reserved_numbers ],
          options: #options,
        }
      }
    }
  };

  Ok(output_tokens)
}
