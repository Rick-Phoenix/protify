use super::*;

pub fn get_bool_validator(ctx: &RulesCtx) -> BuilderTokens {
  let mut builder = BuilderTokens::new(quote! { BoolValidator::builder() });

  ctx.tokenize_ignore(&mut builder);
  ctx.tokenize_required(&mut builder);

  if let Some(RulesType::Bool(rules)) = &ctx.rules.r#type
    && let Some(val) = rules.r#const
  {
    builder.extend(quote! { .const_(#val) });
  }

  builder
}
