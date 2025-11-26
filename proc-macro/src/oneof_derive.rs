use crate::*;

pub(crate) fn process_oneof_derive(item: &mut ItemEnum) -> Result<TokenStream2, Error> {
  let ItemEnum {
    attrs,
    ident: enum_name,
    variants,
    ..
  } = item;

  let OneofAttrs {
    options,
    name: proto_name,
    required,
  } = process_oneof_attrs(enum_name, attrs, false);

  let mut variants_tokens: Vec<TokenStream2> = Vec::new();

  for variant in variants {
    let field_attrs =
      if let Some(attrs) = process_derive_field_attrs(&variant.ident, &variant.attrs)? {
        attrs
      } else {
        continue;
      };

    let FieldAttrs {
      tag,
      validator,
      options,
      name,
      ..
    } = field_attrs;

    let variant_type = if let Fields::Unnamed(variant_fields) = &variant.fields {
      if variant_fields.unnamed.len() != 1 {
        panic!("Oneof variants must contain a single value");
      }

      let type_ = &variant_fields.unnamed.first().unwrap().ty;

      TypeInfo::from_type(type_)?
    } else {
      panic!("Enum can only have one unnamed field")
    };

    let prost_attr_tokens = variant_type.to_prost_attr(tag);

    let prost_attr: Attribute = parse_quote!(#prost_attr_tokens);

    variant.attrs.push(prost_attr);

    let validator_tokens = if let Some(validator) = validator {
      variant_type.validator_tokens(&validator)
    } else {
      quote! { None }
    };

    let full_type_path = &variant_type.full_type;

    variants_tokens.push(quote! {
      ProtoField {
        name: #name.to_string(),
        options: #options,
        type_: <#full_type_path as AsProtoType>::proto_type(),
        validator: #validator_tokens,
        tag: #tag,
      }
    });
  }

  let required_option_tokens = required.then(|| quote! { options.push(oneof_required()); });

  let output_tokens = quote! {
    impl ProtoOneof for #enum_name {
      fn fields() -> Vec<ProtoField> {
        vec![ #(#variants_tokens,)* ]
      }
    }

    impl #enum_name {
      #[track_caller]
      pub fn to_oneof() -> Oneof {
        let mut options: Vec<ProtoOption> = #options;

        #required_option_tokens

        Oneof {
          name: #proto_name.into(),
          fields: Self::fields(),
          options,
        }
      }
    }
  };

  Ok(output_tokens)
}
