# Managing Files

A file must always be in scope for any given item definition (oneof, enum, message).

This can be done by using one of these:

- The [`define_proto_file`](crate::define_proto_file) macro, which both defines a new file schema and brings it into scope within the module
- The [`use_proto_file`](crate::use_proto_file) and [`inherit_proto_file`](crate::inherit_proto_file) macros, which bring a pre-existing file schema in scope
- The `file` attribute (on enums or messages only) for manual assignment.

A simple example of the [`define_proto_file`](crate::define_proto_file) macro looks like this:

```rust
use protify::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);
```

For the full macro reference, visit the documentation for [`define_proto_file!`](crate::define_proto_file).

ℹ️ **NOTE**: Only one file can be in scope for a given module. You can use the `file` attribute to assign a file manually for items in a module if you want to override the file that they are assigned to.

## Reusing Files

If you want to define items in different rust files and place them into the same proto file, you can use one of two macros to bring the file into scope, with slightly different behaviour:

- The [`use_proto_file`](crate::use_proto_file) macro brings the file into scope and applies the extern path of its own `module_path!()` output
- The [`inherit_proto_file`](crate::inherit_proto_file) macro does the same but keeps the import path of the parent module (for re-exported items).

```rust
mod root {
	use protify::*;

	proto_package!(MY_PKG, name = "my_pkg");
	define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

	pub mod submod {
		use super::*;

		// The file is now in scope, and
		// will be picked up automatically
		// by all items defined in this module
		use_proto_file!(MY_FILE);

		// This message will have the extern path
		// of the `module_path!()` output in here,
		// so `::cratename::submod`
		#[proto_message]
		pub struct Msg {
			pub id: i32,
		}
	}

	pub use re_exported::Msg;

	mod re_exported {
		use super::*;

		// The file is now in scope, and
		// will be picked up automatically by
		// all items defined in this module
		inherit_proto_file!(MY_FILE);

		// This message will have the extern path
		// of the parent module
		#[proto_message]
		pub struct Msg {
			pub id: i32,
		}
	}
}
```

## ⚠️ Caveats with glob imports

Under the hood, the file macros generate a constant named `__PROTO_FILE` that is picked up by the macro output for each item. This is so that every file does not have to be manually assigned to each element. This constant is private to the module and hidden so that it cannot be brought into scope by other crates or modules accidentally, but it still available to children modules, so using glob imports from children modules like `use super::*` will bring it into scope.

It's not recommended to rely on this method to bring the file into scope and to use the [`use_proto_file`](crate::use_proto_file) or [`inherit_proto_file`](crate::inherit_proto_file) macros for more clarity.

In a case where you need to use a global import from the parent module but you want the items to be in a separate proto file, then you must make sure to define a new file with the [`define_proto_file`](crate::define_proto_file) macro (which will shadow the imported constant) or use the `file` attribute, or otherwise the items will be picked up by the wrong file.

# Manually Assigning Files

It is also possible to manually select a file for a given item.

```rust
use protify::*;

proto_package!(MY_PKG, name = "my_pkg");

define_proto_file!(FILE, name = "file.proto", package = MY_PKG);

pub use submod::{ReExportedEnum, ReExportedMsg};

pub mod submod {
	use super::*;

	pub fn submod_path() -> &'static str {
		module_path!()
	}
	#[proto_message]
	// NOTE: This inherits the `extern_path`
	// of the file, which by default is
	// the output of `module_path!()`
	// where the file is defined.
	#[proto(file = FILE)]
	pub struct ReExportedMsg {
		pub id: i32,
	}
	#[proto_message]
	#[proto(file = FILE)]
	// If we want to NOT inherit the module path
	#[proto(module_path = module_path!())]
	pub struct NonReExportedMsg {
		pub id: i32,
	}
	// Same logic works for enums
	#[proto_enum]
	#[proto(file = FILE)]
	pub enum ReExportedEnum {
		Unspecified,
		A,
		B,
	}
}

fn main() {
	use crate::submod::NonReExportedMsg;

	let re_exported_msg = ReExportedMsg::proto_schema();
	let re_exported_enum = ReExportedEnum::proto_schema();
	let non_re_exported_msg = NonReExportedMsg::proto_schema();

	assert_eq!(re_exported_msg.file, "file.proto");
	assert_eq!(re_exported_enum.file, "file.proto");
	assert_eq!(non_re_exported_msg.file, "file.proto");

	assert_eq!(
		re_exported_msg.rust_path,
		concat!("::", module_path!(), "::ReExportedMsg")
	);
	assert_eq!(
		re_exported_enum.rust_path,
		concat!("::", module_path!(), "::ReExportedEnum")
	);
	assert_eq!(
		non_re_exported_msg.rust_path,
		format!("::{}::NonReExportedMsg", submod::submod_path())
	);
}
```
