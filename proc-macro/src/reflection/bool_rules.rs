use ::proto_types::protovalidate::BoolRules;

use super::*;

pub fn get_bool_validator(rules: &BoolRules, ctx: &super::RulesCtx) -> TokenStream2 {
  let mut validator = quote! { ::prelude::BoolValidator::builder() };

  ctx.ignore.tokenize(&mut validator);
  ctx.tokenize_required(&mut validator);

  if let Some(val) = rules.r#const {
    validator.extend(quote! { .const_(#val) });
  }

  quote! { #validator.build() }
}
