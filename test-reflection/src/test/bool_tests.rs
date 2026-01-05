use crate::proto::BoolRules;

use super::*;

#[test]
fn bool_test() {
  let mut msg = BoolRules {
    const_test: true,
    required_test: Some(true),
    ignore_if_zero_value_test: Some(true),
    ignore_always_test: false,
  };

  let baseline = msg;

  assert!(msg.validate().is_ok(), "basic validation");

  macro_rules! assert_violation {
    ($violation:expr, $error:literal) => {
      assert_violation_id(&msg, $violation, $error);
      msg = baseline;
    };
  }

  msg.const_test = false;
  assert_violation!("bool.const", "const rule");

  msg.required_test = None;
  assert_violation!("required", "required rule");

  msg.ignore_if_zero_value_test = None;
  assert!(msg.validate().is_ok(), "Should ignore if None");

  msg.ignore_if_zero_value_test = Some(false);
  assert!(msg.validate().is_ok(), "Should ignore if zero value");
}
