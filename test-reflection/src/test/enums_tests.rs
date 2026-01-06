use super::*;

#[test]
fn enums_tests() {
  let mut msg = EnumRules {
    const_test: TestEnum::One.into(),
    in_test: TestEnum::One.into(),
    not_in_test: TestEnum::Two.into(),
    defined_only_test: TestEnum::One.into(),
    cel_test: TestEnum::One.into(),
    required_test: Some(TestEnum::One.into()),
    ignore_always_test: TestEnum::One.into(),
  };
  let baseline = msg.clone();

  assert!(msg.validate().is_ok(), "basic validation");

  macro_rules! assert_violation {
    ($violation:expr, $error:literal) => {
      assert_violation_id(&msg, $violation, $error);
      msg = baseline.clone();
    };
  }

  msg.const_test = TestEnum::Two.into();
  assert_violation!("enum.const", "const rule");

  msg.in_test = TestEnum::Two.into();
  assert_violation!("enum.in", "in rule");

  msg.not_in_test = TestEnum::One.into();
  assert_violation!("enum.not_in", "not_in rule");

  msg.defined_only_test = 15;
  assert_violation!("enum.defined_only", "defined_only rule");

  msg.cel_test = TestEnum::Two.into();
  assert_violation!("cel_rule", "cel rule");

  msg.required_test = None;
  assert_violation!("required", "required rule");

  msg.not_in_test = TestEnum::Unspecified.into();
  assert!(msg.validate().is_ok(), "Should ignore zero value");
}
