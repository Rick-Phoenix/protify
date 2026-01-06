use ::proto_types::protovalidate::MapRules;

use super::*;

pub fn get_map_validator(ctx: &RulesCtx, map_data: &ProtoMap) -> TokenStream2 {
  let ProtoMap { keys, values } = map_data;

  let keys_validator_type = keys.validator_target_type();
  let values_validator_type = values.validator_target_type();
  let mut validator =
    quote! { ::prelude::MapValidator::<#keys_validator_type, #values_validator_type>::builder() };

  ctx.tokenize_ignore(&mut validator);
  ctx.tokenize_cel_rules(&mut validator);

  if let Some(RulesType::Map(rules)) = &ctx.rules.r#type {
    if let Some(val) = rules.min_pairs {
      #[allow(clippy::cast_possible_truncation)]
      let val = val as usize;

      validator.extend(quote! { .min_pairs(#val) });
    }

    if let Some(val) = rules.max_pairs {
      #[allow(clippy::cast_possible_truncation)]
      let val = val as usize;

      validator.extend(quote! { .max_pairs(#val) });
    }

    if let Some(keys_rules) = rules
      .keys
      .as_ref()
      .and_then(|r| RulesCtx::from_non_empty_rules(r, ctx.field_span))
    {
      let keys_validator = get_field_validator(&keys_rules, &((*keys).into())).unwrap();

      validator.extend(quote! { .keys(|_| #keys_validator) });
    }

    if let Some(values_rules) = rules
      .values
      .as_ref()
      .and_then(|r| RulesCtx::from_non_empty_rules(r, ctx.field_span))
    {
      let values_validator = get_field_validator(&values_rules, &values).unwrap();

      validator.extend(quote! { .values(|_| #values_validator) });
    }
  }

  quote! { #validator.build() }
}
