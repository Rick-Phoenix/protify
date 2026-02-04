use super::*;

#[proto_message]
#[proto(skip_checks(all))]
pub struct ParentMessage {
	#[proto(message)]
	pub nested_message: Option<NestedMessage>,
	#[proto(enum_(NestedEnum))]
	pub nested_enum: i32,
}

#[proto_enum]
#[proto(parent_message = NestedMessage)]
pub enum NestedEnum {
	A,
	B,
	C,
}

#[proto_message]
#[proto(skip_checks(all))]
#[proto(parent_message = ParentMessage)]
pub struct NestedMessage {
	#[proto(enum_(NestedEnum))]
	pub nested_enum: i32,
}

#[test]
fn nested_items_tests() {
	assert_eq!(NestedMessage::proto_name(), "ParentMessage.NestedMessage");

	assert_eq!(
		NestedEnum::proto_name(),
		"ParentMessage.NestedMessage.NestedEnum"
	);
}
