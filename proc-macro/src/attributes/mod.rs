use crate::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct AutoTests {
  pub skip_consistency_checks: bool,
  pub skip_oneof_tags_check: bool,
}

impl AutoTests {
  pub fn parse(nested_meta: &ParseNestedMeta) -> syn::Result<Self> {
    let mut skip_consistency_checks = false;
    let mut skip_oneof_tags_check = false;

    nested_meta.parse_nested_meta(|meta| {
      let ident_str = meta.ident_str()?;

      match ident_str.as_str() {
        "all" => {
          skip_consistency_checks = true;
          skip_oneof_tags_check = true;
        }
        "validators" => {
          skip_consistency_checks = true;
        }
        "oneof_tags" => {
          skip_oneof_tags_check = true;
        }
        _ => return Err(meta.error("Unknown attribute")),
      };

      Ok(())
    })?;

    Ok(Self {
      skip_consistency_checks,
      skip_oneof_tags_check,
    })
  }
}

mod validator_tokens;
pub use validator_tokens::*;
mod extension_field_attributes;
pub use extension_field_attributes::*;
mod enum_attributes;
mod enum_variant_attributes;
mod field_attributes;
mod message_attributes;
mod message_info;
mod oneof_attributes;
mod oneof_info;
mod reserved_numbers;
mod service_attributes;
mod tag_allocator;

pub use enum_attributes::*;
pub use enum_variant_attributes::*;
pub use field_attributes::*;
pub use message_attributes::*;
pub use message_info::*;
pub use oneof_attributes::*;
pub use oneof_info::*;
pub use reserved_numbers::*;
pub use service_attributes::*;
pub use tag_allocator::*;
