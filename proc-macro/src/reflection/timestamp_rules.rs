use ::proto_types::Timestamp;
use ::proto_types::protovalidate::timestamp_rules::{GreaterThan, LessThan};

use super::*;

fn tokenize_timestamp(timestamp: Timestamp) -> TokenStream2 {
  let Timestamp { seconds, nanos } = timestamp;

  quote! { ::prelude::proto_types::Timestamp { seconds: #seconds, nanos: #nanos } }
}

pub fn get_timestamp_validator(ctx: &RulesCtx) -> BuilderTokens {
  let mut builder = BuilderTokens::new(quote! { TimestampValidator::builder() });

  ctx.tokenize_ignore(&mut builder);
  ctx.tokenize_required(&mut builder);
  ctx.tokenize_cel_rules(&mut builder);

  if let Some(RulesType::Timestamp(rules)) = &ctx.rules.r#type {
    if let Some(val) = rules.r#const {
      let val = tokenize_timestamp(val);

      builder.extend(quote! { .const_(#val) });
    }

    if let Some(less_than) = &rules.less_than {
      match less_than {
        LessThan::Lt(val) => {
          let val = tokenize_timestamp(*val);
          builder.extend(quote! { .lt(#val) });
        }
        LessThan::Lte(val) => {
          let val = tokenize_timestamp(*val);
          builder.extend(quote! { .lte(#val) });
        }
        LessThan::LtNow(true) => {
          builder.extend(quote! { .lt_now() });
        }
        _ => {}
      };
    }

    if let Some(greater_than) = &rules.greater_than {
      match greater_than {
        GreaterThan::Gt(val) => {
          let val = tokenize_timestamp(*val);
          builder.extend(quote! { .gt(#val) });
        }
        GreaterThan::Gte(val) => {
          let val = tokenize_timestamp(*val);
          builder.extend(quote! { .gte(#val) });
        }
        GreaterThan::GtNow(true) => {
          builder.extend(quote! { .gt_now() });
        }
        _ => {}
      };
    }

    if let Some(val) = &rules.within {
      let val = tokenize_duration(*val);

      builder.extend(quote! { .within(#val) });
    }
  }

  builder
}
