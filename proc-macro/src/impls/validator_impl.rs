use crate::*;

pub struct ValidatorImplCtx<'a> {
  pub target_ident: &'a Ident,
  pub validators_tokens: TokenStream2,
  pub top_level_programs_ident: Option<&'a Ident>,
}

pub fn impl_validator(ctx: ValidatorImplCtx) -> TokenStream2 {
  let ValidatorImplCtx {
    target_ident,
    validators_tokens,
    top_level_programs_ident,
  } = ctx;

  let top_level_programs_expr = tokens_or_default!(top_level_programs_ident, quote! { vec![] });

  quote! {
    impl #target_ident {
      #[doc(hidden)]
      fn __validate_internal(&self, field_context: Option<&FieldContext>, parent_elements: &mut Vec<FieldPathElement>) -> Result<(), Violations> {
        let mut violations = Violations::new();

        if let Some(field_context) = field_context {
          parent_elements.push(FieldPathElement {
            field_number: Some(field_context.tag),
            field_name: Some(field_context.name.to_string()),
            field_type: Some(Type::Message as i32),
            key_type: field_context.key_type.map(|t| t as i32),
            value_type: field_context.value_type.map(|t| t as i32),
            subscript: field_context.subscript.clone(),
          });
        }

        let top_level_programs: &Vec<&CelProgram> = &#top_level_programs_expr;

        if !top_level_programs.is_empty() {
          ::prelude::execute_cel_programs(::prelude::ProgramsExecutionCtx {
            programs: top_level_programs,
            value: self.clone(),
            violations: &mut violations,
            field_context,
            parent_elements,
          });
        }

        #validators_tokens

        if field_context.is_some() {
          parent_elements.pop();
        }

        if violations.is_empty() {
          Ok(())
        } else {
          Err(violations)
        }
      }

      pub fn validate(&self) -> Result<(), Violations> {
        self.__validate_internal(None, &mut vec![])
      }

      pub fn nested_validate(&self, field_context: &FieldContext, parent_elements: &mut Vec<FieldPathElement>) -> Result<(), Violations> {
        self.__validate_internal(Some(field_context), parent_elements)
      }
    }

    impl ::prelude::ProtoValidator<#target_ident> for #target_ident {
      type Target = Self;
      type Validator = ::prelude::MessageValidator<Self>;
      type Builder = ::prelude::MessageValidatorBuilder<Self>;

      fn builder() -> Self::Builder {
        ::prelude::MessageValidator::builder()
      }
    }
  }
}
