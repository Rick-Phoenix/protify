use super::*;

pub fn get_enum_validator(ctx: &RulesCtx, enum_path: &Path) -> BuilderTokens {
  let mut builder = BuilderTokens::new(quote! { EnumValidator::<#enum_path>::builder() });

  ctx.tokenize_ignore(&mut builder);
  ctx.tokenize_required(&mut builder);
  ctx.tokenize_cel_rules(&mut builder);

  if let Some(RulesType::Enum(rules)) = &ctx.rules.r#type {
    if let Some(val) = rules.r#const {
      builder.extend(quote! { .const_(#val) });
    }

    if rules.defined_only() {
      builder.extend(quote! { .defined_only() });
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
