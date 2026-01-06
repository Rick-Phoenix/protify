use ::proto_types::protovalidate::EnumRules;

use super::*;

pub fn get_enum_validator(ctx: &RulesCtx, enum_path: &Path) -> TokenStream2 {
  let mut validator = quote! { ::prelude::EnumValidator::<#enum_path>::builder() };

  ctx.tokenize_ignore(&mut validator);
  ctx.tokenize_required(&mut validator);
  ctx.tokenize_cel_rules(&mut validator);

  if let Some(RulesType::Enum(rules)) = &ctx.rules.r#type {
    if let Some(val) = rules.r#const {
      validator.extend(quote! { .const_(#val) });
    }

    if rules.defined_only() {
      validator.extend(quote! { .defined_only() });
    }

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
