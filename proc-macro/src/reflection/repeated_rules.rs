use ::proto_types::protovalidate::RepeatedRules;

use super::*;

pub fn get_repeated_validator(ctx: &RulesCtx, inner: &ProtoType) -> TokenStream2 {
  let inner_validator_type = inner.validator_target_type();
  let mut validator = quote! { ::prelude::RepeatedValidator::<#inner_validator_type>::builder() };

  ctx.tokenize_ignore(&mut validator);
  ctx.tokenize_cel_rules(&mut validator);

  if let Some(RulesType::Repeated(rules)) = &ctx.rules.r#type {
    if let Some(val) = rules.min_items {
      #[allow(clippy::cast_possible_truncation)]
      let val = val as usize;

      validator.extend(quote! { .min_items(#val) });
    }

    if let Some(val) = rules.max_items {
      #[allow(clippy::cast_possible_truncation)]
      let val = val as usize;

      validator.extend(quote! { .max_items(#val) });
    }

    if rules.unique() {
      validator.extend(quote! { .unique() });
    }

    if let Some(items_rules) = rules
      .items
      .as_ref()
      .and_then(|r| RulesCtx::from_non_empty_rules(r, ctx.field_span))
    {
      let items_validator = get_field_validator(&items_rules, inner).unwrap();

      validator.extend(quote! { .items(|_| #items_validator) });
    }
  }

  quote! { #validator.build() }
}
