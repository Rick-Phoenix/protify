use crate::*;

pub fn impl_oneof_validator2<T>(oneof_ident: &Ident, variants: &[T]) -> TokenStream2
where
  T: Borrow<FieldData>,
{
  let validators_tokens = variants.iter().flat_map(|data| {
    let FieldData {
      ident,
      type_info,
      ident_str,
      is_variant,
      tag,
      validator,
      proto_name,
      proto_field,
      ..
    } = data.borrow();

    if let Some(ValidatorTokens {
      expr: validator_expr,
      ..
    }) = validator.as_ref()
    {
      let validator_static_ident = format_ident!("{}_VALIDATOR", ccase!(constant, &ident_str));

      let validator_name = proto_field.validator_name();

      let validator_static = quote! {
        static #validator_static_ident: LazyLock<#validator_name> = LazyLock::new(|| {
          #validator_expr
        });
      };

      let field_type = proto_field.descriptor_type_tokens();

      let field_context_tokens = quote! {
        ::prelude::FieldContext {
          proto_name: #proto_name,
          tag: #tag,
          field_type: #field_type,
          key_type: None,
          value_type: None,
          subscript: None,
          field_kind: Default::default(),
        }
      };

      let validator_tokens = generate_validator_tokens(
        &type_info.type_,
        *is_variant,
        ident,
        field_context_tokens,
        &validator_static_ident,
        validator_static,
      );

      Some(validator_tokens)
    } else {
      None
    }
  });

  quote! {
    #[allow(clippy::ptr_arg)]
    impl #oneof_ident {
      pub fn validate(&self, parent_elements: &mut Vec<FieldPathElement>, violations: &mut ViolationsAcc) {
        match self {
          #(#validators_tokens,)*
          _ => {}
        }
      }
    }
  }
}
