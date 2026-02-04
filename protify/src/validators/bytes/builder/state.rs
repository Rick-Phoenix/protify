use crate::validators::builder_internals::*;
use protify_proc_macro::builder_state_macro;
builder_state_macro!(
	Ignore,
	WellKnown,
	Required,
	Len,
	MinLen,
	MaxLen,
	Pattern,
	Prefix,
	Suffix,
	Contains,
	In,
	NotIn,
	Const,
	ErrorMessages
);
