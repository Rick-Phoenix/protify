use crate::validators::builder_internals::*;
use protify_proc_macro::builder_state_macro;
builder_state_macro!(
	Const,
	Required,
	Ignore,
	WellKnown,
	Len,
	MinLen,
	MaxLen,
	LenBytes,
	MinBytes,
	MaxBytes,
	Pattern,
	Prefix,
	Suffix,
	NotContains,
	Contains,
	In,
	NotIn,
	ErrorMessages
);
