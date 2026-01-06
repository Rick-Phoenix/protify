use ::proto_types::protovalidate::BoolRules;

use super::*;

pub fn get_bool_validator(ctx: &RulesCtx) -> TokenStream2 {
  let mut validator = quote! { ::prelude::BoolValidator::builder() };

  ctx.tokenize_ignore(&mut validator);
  ctx.tokenize_required(&mut validator);

  if let Some(RulesType::Bool(rules)) = &ctx.rules.r#type
    && let Some(val) = rules.r#const
  {
    validator.extend(quote! { .const_(#val) });
  }

  quote! { #validator.build() }
}
