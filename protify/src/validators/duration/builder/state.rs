use crate::validators::builder_internals::*;
use protify_proc_macro::builder_state_macro;
builder_state_macro!(
	Ignore,
	Required,
	In,
	NotIn,
	Const,
	Lt,
	Lte,
	Gt,
	Gte,
	ErrorMessages
);
