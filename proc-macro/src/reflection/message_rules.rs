use super::*;

pub fn get_message_field_validator(ctx: &RulesCtx, msg_path: &Path) -> TokenStream2 {
  let mut validator = quote! { ::prelude::MessageValidator::<#msg_path>::builder() };

  ctx.tokenize_ignore(&mut validator);
  ctx.tokenize_cel_rules(&mut validator);
  ctx.tokenize_required(&mut validator);

  quote! { #validator.build() }
}
