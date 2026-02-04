#[cfg(feature = "reflection")]
mod proto {
	include!(concat!(env!("OUT_DIR"), "/test_schemas.v1.rs"));
}

#[allow(clippy::clone_on_copy, unused_assignments, clippy::redundant_clone)]
#[cfg(test)]
mod test {
	use maplit::hashmap;
	use protify::ValidatedMessage;
	use proto_types::protovalidate::Violation;

	#[cfg(feature = "reflection")]
	use crate::proto::*;

	#[cfg(not(feature = "reflection"))]
	use test_schemas::*;

	use protify::proto_types::*;

	#[allow(unused)]
	#[track_caller]
	pub(crate) fn full_rule_id_path<T: ValidatedMessage>(msg: &T) -> String {
		let violations = msg.validate().unwrap_err().into_violations();

		let first = violations.first().unwrap();

		first.rule_path_str().unwrap()
	}

	#[allow(unused)]
	#[track_caller]
	pub(crate) fn first_violation<T: ValidatedMessage>(msg: &T) -> Violation {
		let mut violations = msg.validate().unwrap_err().into_violations();

		violations.violations.remove(0)
	}

	#[allow(unused)]
	#[track_caller]
	pub(crate) fn inspect_violations<T: ValidatedMessage>(msg: &T) {
		let violations = msg.validate().unwrap_err().into_violations();

		eprintln!("{violations:#?}");
	}

	#[allow(unused)]
	#[track_caller]
	pub(crate) fn get_rules_ids<T: ValidatedMessage>(msg: &T) -> Vec<String> {
		let violations = msg.validate().unwrap_err().into_violations();

		violations
			.into_iter()
			.map(|v| v.rule_id().to_string())
			.collect()
	}

	#[track_caller]
	pub(crate) fn assert_violation_id(msg: &impl ValidatedMessage, expected: &str, error: &str) {
		let violations = msg.validate().unwrap_err().into_violations();

		assert_eq!(violations.len(), 1, "Expected a single violation");
		assert_eq!(violations.first().unwrap().rule_id(), expected, "{error}");
	}

	use similar_asserts::assert_eq as assert_eq_pretty;

	mod any_tests;
	mod bool_tests;
	mod bytes_tests;
	mod const_rules_tests;
	mod duration_tests;
	mod enums_tests;
	mod fail_fast_tests;
	mod field_mask_tests;
	mod map_tests;
	mod message_tests;
	mod numeric_tests;
	mod oneof_tests;
	#[cfg(feature = "reflection")]
	mod reflection_consistency_tests;
	mod repeated_tests;
	mod string_tests;
	mod timestamp_tests;
}
