use ::proto_types::protovalidate::StringRules;

use crate::*;

pub fn get_string_validator(rules: &StringRules, ctx: &super::RulesCtx) -> TokenStream2 {
  let mut validator = quote! { ::prelude::StringValidator::builder() };

  ctx.ignore.tokenize(&mut validator);
  ctx.tokenize_required(&mut validator);

  if let Some(val) = &rules.r#const {
    validator.extend(quote! { .const_(#val) });
    // return quote! { #validator.build() };
  }

  macro_rules! len_rule {
    ($name:ident) => {
      if let Some($name) = rules.$name {
        let $name = $name as usize;

        validator.extend(quote! { .$name(#$name) });
      }
    };
  }

  macro_rules! str_rule {
    ($name:ident) => {
      if let Some($name) = &rules.$name {
        validator.extend(quote! { .$name(#$name) });
      }
    };
  }

  len_rule!(len);
  len_rule!(min_len);
  len_rule!(max_len);
  len_rule!(len_bytes);
  len_rule!(min_bytes);
  len_rule!(max_bytes);

  str_rule!(pattern);
  str_rule!(prefix);
  str_rule!(suffix);
  str_rule!(contains);
  str_rule!(not_contains);

  if !rules.r#in.is_empty() {
    let list = &rules.r#in;
    validator.extend(quote! { .in_([ #(#list),* ]) });
  }

  if !rules.not_in.is_empty() {
    let list = &rules.not_in;
    validator.extend(quote! { .not_in([ #(#list),* ]) });
  }

  ctx.tokenize_cel_rules(&mut validator);

  quote! { #validator.build() }
}
