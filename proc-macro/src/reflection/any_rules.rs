use ::proto_types::Any;
use ::proto_types::protovalidate::AnyRules;

use super::*;

pub fn get_any_validator(ctx: &super::RulesCtx) -> TokenStream2 {
  let mut validator = quote! { ::prelude::AnyValidator::builder() };

  ctx.tokenize_ignore(&mut validator);
  ctx.tokenize_required(&mut validator);
  ctx.tokenize_cel_rules(&mut validator);

  if let Some(RulesType::Any(rules)) = &ctx.rules.r#type {
    let in_list = &rules.r#in;
    if !in_list.is_empty() {
      validator.extend(quote! { .in_([ #(#in_list),* ]) });
    }

    let not_in_list = &rules.not_in;
    if !not_in_list.is_empty() {
      validator.extend(quote! { .not_in([ #(#not_in_list),* ]) });
    }
  }

  quote! { #validator.build() }
}
