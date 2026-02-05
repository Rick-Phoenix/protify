use crate::*;

#[doc(hidden)]
#[must_use]
#[inline(never)]
#[cold]
pub fn __proto_deprecated() -> ProtoOption {
	ProtoOption {
		name: "deprecated".into(),
		value: OptionValue::Bool(true),
	}
}
