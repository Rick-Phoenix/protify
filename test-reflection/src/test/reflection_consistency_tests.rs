use protify::{ConsistencyError, TestError};

use crate::proto::bad_cel_oneof_test::BadCelOneof;

use super::*;

#[test]
fn bad_field_rules() {
	let TestError {
		item_name: message_name,
		field_errors,
		top_level_errors,
	} = BadFieldRules::check_validators().unwrap_err();

	assert_eq_pretty!(message_name, "BadFieldRules");
	assert_eq_pretty!(field_errors.len(), 1);
	assert_eq_pretty!(top_level_errors.len(), 0);
}

#[test]
fn bad_msg_rules() {
	let TestError {
		item_name: message_name,
		field_errors,
		top_level_errors,
	} = BadMsgRules::check_validators().unwrap_err();

	assert_eq_pretty!(message_name, "BadMsgRules");
	assert_eq_pretty!(field_errors.len(), 0);
	assert_eq_pretty!(top_level_errors.len(), 1);
}

#[test]
fn bad_oneof_rules() {
	let TestError {
		item_name: oneof_name,
		field_errors: errors,
		..
	} = BadCelOneof::check_validators().unwrap_err();

	assert_eq_pretty!(oneof_name, "BadCelOneof");
	assert_eq_pretty!(errors.len(), 1);
	assert!(matches!(errors[0].errors[0], ConsistencyError::CelError(_)));
}
