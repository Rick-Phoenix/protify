use super::*;

pub fn get_repeated_validator(ctx: &RulesCtx, inner: &ProtoType) -> BuilderTokens {
  let inner_validator_type = inner.validator_target_type();
  let mut builder =
    BuilderTokens::new(quote! { RepeatedValidator::<#inner_validator_type>::builder() });

  ctx.tokenize_ignore(&mut builder);
  ctx.tokenize_cel_rules(&mut builder);

  if let Some(RulesType::Repeated(rules)) = &ctx.rules.r#type {
    if let Some(val) = rules.min_items {
      #[allow(clippy::cast_possible_truncation)]
      let val = val as usize;

      builder.extend(quote! { .min_items(#val) });
    }

    if let Some(val) = rules.max_items {
      #[allow(clippy::cast_possible_truncation)]
      let val = val as usize;

      builder.extend(quote! { .max_items(#val) });
    }

    if rules.unique() {
      builder.extend(quote! { .unique() });
    }

    if let Some(items_rules) = rules
      .items
      .as_ref()
      .and_then(|r| RulesCtx::from_non_empty_rules(r, ctx.field_span))
    {
      let items_validator = get_field_validator(&items_rules, inner)
        .unwrap()
        .into_builder();

      builder.extend(quote! { .items(|_| #items_validator) });
    }
  }

  builder
}
