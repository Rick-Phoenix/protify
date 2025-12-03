use std::sync::LazyLock;

use super::*;

#[allow(clippy::type_complexity)]
static REQUIRED_OPTION: LazyLock<Arc<[(Arc<str>, OptionValue)]>> =
  LazyLock::new(|| [(REQUIRED.clone(), OptionValue::Bool(true))].into());

/// Specifies that at least one of the variants of the oneof must be set in order to pass validation.
pub fn oneof_required() -> ProtoOption {
  ProtoOption {
    name: BUF_VALIDATE_ONEOF.clone(),
    value: OptionValue::Message(REQUIRED_OPTION.clone()),
  }
}
