use ::proto_types::protovalidate::bytes_rules::WellKnown;
use bytes::Bytes;
use syn::LitByteStr;

use super::*;

fn tokenize_bytes(bytes: &Bytes) -> LitByteStr {
  LitByteStr::new(bytes, Span::call_site())
}

pub fn get_bytes_validator(ctx: &RulesCtx) -> BuilderTokens {
  let mut builder = BuilderTokens::new(quote! { BytesValidator::builder() });

  ctx.tokenize_ignore(&mut builder);
  ctx.tokenize_required(&mut builder);
  ctx.tokenize_cel_rules(&mut builder);

  if let Some(RulesType::Bytes(rules)) = &ctx.rules.r#type {
    if let Some(val) = &rules.r#const {
      let tokens = tokenize_bytes(val);

      builder.extend(quote! { .const_(#tokens) });
    }

    macro_rules! len_rule {
      ($name:ident) => {
        if let Some(val) = rules.$name {
          #[allow(clippy::cast_possible_truncation)]
          let val = val as usize;

          builder.extend(quote! { .$name(#val) });
        }
      };
    }

    macro_rules! str_rule {
      ($name:ident) => {
        if let Some(val) = &rules.$name {
          let tokens = tokenize_bytes(val);
          builder.extend(quote! { .$name(#tokens) });
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
      builder.extend(quote! { .pattern(#val) });
    }

    if !rules.r#in.is_empty() {
      let list = rules.r#in.iter().map(|b| tokenize_bytes(b));
      builder.extend(quote! { .in_([ #(#list),* ]) });
    }

    if !rules.not_in.is_empty() {
      let list = rules.not_in.iter().map(|b| tokenize_bytes(b));
      builder.extend(quote! { .not_in([ #(#list),* ]) });
    }

    if let Some(well_known) = rules.well_known {
      macro_rules! match_well_known {
        ($($name:ident),*) => {
          paste::paste! {
            match well_known {
              $(
                #[allow(unreachable_patterns)]
                WellKnown::$name(true) => {
                  builder.extend(quote! { .[< $name:snake >]() });
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

  builder
}
