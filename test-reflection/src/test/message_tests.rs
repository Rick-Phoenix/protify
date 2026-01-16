use super::*;

#[test]
fn recursive_message() {
  let valid_msg = BoxedMsg { msg: None, id: 1 };

  let mut msg = BoxedMsg {
    msg: Some(valid_msg.clone().into()),
    id: 1,
  };
  let baseline = msg.clone();

  assert!(msg.validate().is_ok(), "basic validation");

  macro_rules! assert_violation {
    ($violation:expr, $error:expr) => {
      assert_violation_id(&msg, $violation, $error);
      msg = baseline.clone();
    };
  }

  msg.id = 2;
  let invalid = msg.clone();

  assert_violation!("int32.const", "outer rule");

  msg.msg = Some(Box::new(invalid));
  assert_violation!("int32.const", "inner rule");
}

#[test]
fn default_validator_msg() {
  let mut msg = DefaultValidatorTestMsg {
    msg_with_default_validator: Some(DefaultValidatorTestCel { id: 1 }),
  };

  assert!(msg.validate().is_ok(), "basic validation");

  msg.msg_with_default_validator = Some(DefaultValidatorTestCel { id: 2 });
  assert_violation_id(&msg, "id_is_1", "cel rule");
}

#[test]
fn default_validator_vec() {
  let mut msg = DefaultValidatorTestVec {
    repeated_test: vec![DefaultValidatorTestCel { id: 1 }],
  };

  assert!(msg.validate().is_ok(), "basic validation");

  *msg.repeated_test.get_mut(0).unwrap() = DefaultValidatorTestCel { id: 2 };

  let violation = first_violation(&msg);

  assert_eq_pretty!(
    violation.rule_path_str().unwrap(),
    "repeated.items.cel",
    "default repeated message validator"
  );
  assert_eq_pretty!(
    violation.rule_id(),
    "id_is_1",
    "default repeated message validator"
  );
}

#[test]
fn default_validator_map() {
  let mut msg = DefaultValidatorTestMap {
    map_test: hashmap! { 0 => DefaultValidatorTestCel { id: 1 } },
  };

  assert!(msg.validate().is_ok(), "basic validation");

  *msg.map_test.get_mut(&0).unwrap() = DefaultValidatorTestCel { id: 2 };

  let violation = first_violation(&msg);

  assert_eq_pretty!(
    violation.rule_path_str().unwrap(),
    "map.values.cel",
    "default repeated message validator"
  );
  assert_eq_pretty!(
    violation.rule_id(),
    "id_is_1",
    "default map message validator"
  );
}
