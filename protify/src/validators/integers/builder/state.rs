use crate::validators::builder_internals::*;
use protify_proc_macro::builder_state_macro;
builder_state_macro!(
	Const,
	Required,
	Ignore,
	Lt,
	Lte,
	Gt,
	Gte,
	In,
	NotIn,
	ErrorMessages
);
