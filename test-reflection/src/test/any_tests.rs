use proto_types::Any;

use super::*;

#[allow(unused)]
#[test]
fn any_tests() {
	let mut msg = AnyRules {
		in_test: Some(Any {
			type_url: "/type_url".to_string(),
			value: vec![],
		}),
		not_in_test: Some(Any {
			type_url: "/another_type_url".to_string(),
			value: vec![],
		}),
		cel_test: Some(Any {
			type_url: "/type_url".to_string(),
			value: vec![b'a'],
		}),
		required_test: Some(Any {
			type_url: "/type_url".to_string(),
			value: vec![],
		}),
		ignore_always_test: Some(Any {
			type_url: "/type_url".to_string(),
			value: vec![],
		}),
	};

	let baseline = msg.clone();

	assert!(msg.validate().is_ok(), "basic validation");

	macro_rules! assert_violation {
		($violation:expr, $error:literal) => {
			assert_violation_id(&msg, $violation, $error);
			msg = baseline.clone();
		};
	}

	msg.in_test = Some(Any {
		type_url: "/another_type_url".to_string(),
		value: vec![],
	});
	assert_violation!("any.in", "in rule");

	msg.not_in_test = Some(Any {
		type_url: "/type_url".to_string(),
		value: vec![],
	});
	assert_violation!("any.not_in", "not_in rule");

	msg.cel_test = Some(Any {
		type_url: "/type_url".to_string(),
		value: vec![],
	});
	assert_violation!("cel_rule", "cel rule");

	msg.required_test = None;
	assert_violation!("required", "required rule");
}
