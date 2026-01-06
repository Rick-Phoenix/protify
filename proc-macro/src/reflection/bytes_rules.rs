use ::proto_types::protovalidate::{BytesRules, bytes_rules::WellKnown};
use bytes::Bytes;
use syn::LitByteStr;

use super::*;

fn tokenize_bytes(bytes: &Bytes) -> LitByteStr {
  LitByteStr::new(bytes, Span::call_site())
}

pub fn get_bytes_validator(ctx: &RulesCtx) -> TokenStream2 {
  let mut validator = quote! { ::prelude::BytesValidator::builder() };

  ctx.tokenize_ignore(&mut validator);
  ctx.tokenize_required(&mut validator);
  ctx.tokenize_cel_rules(&mut validator);

  if let Some(RulesType::Bytes(rules)) = &ctx.rules.r#type {
    if let Some(val) = &rules.r#const {
      let tokens = tokenize_bytes(val);

      validator.extend(quote! { .const_(#tokens) });
    }

    macro_rules! len_rule {
      ($name:ident) => {
        if let Some(val) = rules.$name {
          #[allow(clippy::cast_possible_truncation)]
          let val = val as usize;

          validator.extend(quote! { .$name(#val) });
        }
      };
    }

    macro_rules! str_rule {
      ($name:ident) => {
        if let Some(val) = &rules.$name {
          let tokens = tokenize_bytes(val);
          validator.extend(quote! { .$name(#tokens) });
        }
      };
    }

    len_rule!(len);
    len_rule!(min_len);
    len_rule!(max_len);

    str_rule!(prefix);
    str_rule!(suffix);
    str_rule!(contains);

    if let Some(val) = &rules.pattern {
      validator.extend(quote! { .pattern(#val) });
    }

    if !rules.r#in.is_empty() {
      let list = rules.r#in.iter().map(|b| tokenize_bytes(b));
      validator.extend(quote! { .in_([ #(#list),* ]) });
    }

    if !rules.not_in.is_empty() {
      let list = rules.not_in.iter().map(|b| tokenize_bytes(b));
      validator.extend(quote! { .not_in([ #(#list),* ]) });
    }

    if let Some(well_known) = rules.well_known {
      macro_rules! match_well_known {
        ($($name:ident),*) => {
          paste::paste! {
            match well_known {
              $(
                WellKnown::$name(true) => {
                  validator.extend(quote! { .[< $name:snake >]() });
                }
              )*
              _ => {}
            }
          }
        };
      }

      match_well_known!(Ip, Ip, Ipv4, Ipv6, Uuid);
    }
  }

  quote! { #validator.build() }
}
