use super::*;

pub fn get_field_mask_validator(ctx: &super::RulesCtx) -> BuilderTokens {
  let mut builder = BuilderTokens::new(quote! { FieldMaskValidator::builder() });

  ctx.tokenize_ignore(&mut builder);
  ctx.tokenize_required(&mut builder);
  ctx.tokenize_cel_rules(&mut builder);

  if let Some(RulesType::FieldMask(rules)) = &ctx.rules.r#type {
    if let Some(val) = &rules.r#const {
      let paths = &val.paths;

      builder.extend(quote! { .const_([ #(#paths),* ]) });
    }

    let in_list = &rules.r#in;
    if !in_list.is_empty() {
      builder.extend(quote! { .in_([ #(#in_list),* ]) });
    }

    let not_in_list = &rules.not_in;
    if !not_in_list.is_empty() {
      builder.extend(quote! { .not_in([ #(#not_in_list),* ]) });
    }
  }

  builder
}
