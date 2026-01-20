use prelude::{ConsistencyError, MessageTestError, OneofErrors};

use crate::proto::bad_cel_oneof_test::BadCelOneof;

use super::*;

#[test]
fn bad_field_rules() {
  let MessageTestError {
    message_full_name,
    field_errors,
    top_level_errors,
  } = BadFieldRules::check_validators_consistency().unwrap_err();

  assert_eq_pretty!(message_full_name, "test_schemas.v1.BadFieldRules");
  assert_eq_pretty!(field_errors.len(), 1);
  // Top level rules, which don't apply here
  assert_eq_pretty!(top_level_errors.len(), 0);
}

#[test]
fn bad_msg_rules() {
  let MessageTestError {
    message_full_name,
    field_errors,
    top_level_errors,
  } = BadMsgRules::check_validators_consistency().unwrap_err();

  assert_eq_pretty!(message_full_name, "test_schemas.v1.BadMsgRules");
  assert_eq_pretty!(field_errors.len(), 0);
  assert_eq_pretty!(top_level_errors.len(), 1);
}

#[test]
fn bad_oneof_rules() {
  let OneofErrors {
    oneof_name,
    field_errors: errors,
  } = BadCelOneof::check_validators_consistency().unwrap_err();

  assert_eq_pretty!(oneof_name, "BadCelOneof");
  assert_eq_pretty!(errors.len(), 1);
  assert!(matches!(errors[0].errors[0], ConsistencyError::CelError(_)));
}
