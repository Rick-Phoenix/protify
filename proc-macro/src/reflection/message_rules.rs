use super::*;

pub fn get_message_field_validator(ctx: &RulesCtx, msg_path: &Path) -> BuilderTokens {
  let mut builder = BuilderTokens::new(quote! { MessageValidator::<#msg_path>::builder() });

  ctx.tokenize_ignore(&mut builder);
  ctx.tokenize_cel_rules(&mut builder);
  ctx.tokenize_required(&mut builder);

  builder
}
