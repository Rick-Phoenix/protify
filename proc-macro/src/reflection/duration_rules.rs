use ::proto_types::Duration;
use ::proto_types::protovalidate::duration_rules::{GreaterThan, LessThan};

use super::*;

pub(super) fn tokenize_duration(duration: Duration) -> TokenStream2 {
  let Duration { seconds, nanos } = duration;

  quote! { ::prelude::proto_types::Duration { seconds: #seconds, nanos: #nanos } }
}

pub fn get_duration_validator(ctx: &RulesCtx) -> BuilderTokens {
  let mut builder = BuilderTokens::new(quote! { DurationValidator::builder() });

  ctx.tokenize_ignore(&mut builder);
  ctx.tokenize_required(&mut builder);
  ctx.tokenize_cel_rules(&mut builder);

  if let Some(RulesType::Duration(rules)) = &ctx.rules.r#type {
    if let Some(val) = rules.r#const {
      let val = tokenize_duration(val);

      builder.extend(quote! { .const_(#val) });
    }

    if let Some(less_than) = &rules.less_than {
      match less_than {
        LessThan::Lt(val) => {
          let val = tokenize_duration(*val);
          builder.extend(quote! { .lt(#val) });
        }
        LessThan::Lte(val) => {
          let val = tokenize_duration(*val);
          builder.extend(quote! { .lte(#val) });
        }
      };
    }

    if let Some(greater_than) = &rules.greater_than {
      match greater_than {
        GreaterThan::Gt(val) => {
          let val = tokenize_duration(*val);
          builder.extend(quote! { .gt(#val) });
        }
        GreaterThan::Gte(val) => {
          let val = tokenize_duration(*val);
          builder.extend(quote! { .gte(#val) });
        }
      };
    }

    let in_list = &rules.r#in;
    if !in_list.is_empty() {
      let in_list = in_list.iter().map(|d| tokenize_duration(*d));
      builder.extend(quote! { .in_([ #(#in_list),* ]) });
    }

    let not_in_list = &rules.not_in;
    if !not_in_list.is_empty() {
      let not_in_list = not_in_list.iter().map(|d| tokenize_duration(*d));
      builder.extend(quote! { .not_in([ #(#not_in_list),* ]) });
    }
  }

  builder
}
