# Creating A Package

Start by creating a package using the [`proto_package`](crate::proto_package) macro.

```rust
use protify::*;

proto_package!(MY_PKG, name = "my_pkg");
```

The ident that is used as the first argument will be the ident for this package handle, which will be a unit struct that implements [`PackageSchema`](crate::PackageSchema). There should be only one of these for each package.

ℹ️ **NOTE**: By default, if the `cel` feature is enabled, this macro also generates a test that ensures that there aren't CEL rules with the same ID within the same message scope. If you want to disable this, you can provide `no_cel_test` as one of the arguments.

This handle will serve two purposes. One is to serve as a reference for each file definition, and the other one is to be able to generate the fully built [`Package`](crate::Package) struct, which can then be used to generate the `.proto` files, and to access the rust import path of each item to use when setting up tonic, with the [`extern_paths`](crate::Package::extern_paths) method.

# Creating Files

For each item definition (oneof, message, enum), a proto file reference must be brought in scope so that the item can be assigned to it (or it can also be assigned manually with the `file` attribute). A new file can be defined with the [`define_proto_file`](crate::define_proto_file) macro.

```rust
use protify::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);
```

Once a file is brought into scope, we can define the items that will be assigned to it.

For more information on how to reuse the same file for different modules, refer to the [`managing files`](crate::guide::managing_files) section.

```rust
use protify::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(
	MY_FILE,
	name = "my_file.proto",
	package = MY_PKG,
	extensions = [MyExt]
);

#[proto_extension(target = MessageOptions)]
pub struct MyExt {
	#[proto(tag = 5000)]
	cool_opt: String,
}

#[proto_service]
enum MyService {
	Service1 { request: MyMsg, response: MyMsg },
}

#[proto_message]
pub struct MyMsg {
	pub id: i32,
}

#[proto_oneof]
pub enum MyOneof {
	#[proto(tag = 1)]
	A(i32),
	#[proto(tag = 2)]
	B(u32),
}

#[proto_enum]
pub enum MyEnum {
	Unspecified,
	A,
	B,
}
```

To learn more about each element, visit the reference for [messages](crate::proto_message), [extensions](crate::proto_extension), [oneofs](crate::proto_oneof) and [enums](crate::proto_enum).

# Collecting A Package

When the `inventory` feature is enabled, all the elements in a package are collected automatically. To get the fully built package, all you need to do is to call the [`get_package`](crate::PackageSchema::get_package) method.

This will give you access to the [`render_files`](crate::Package::render_files) method, which is what you need to generate the `.proto` files associated with that package, or the [`extern_paths`](crate::Package::extern_paths) method, which is what you can use to map messages to their Rust path when using `tonic`.

## no_std usage

The inventory feature relies on the [inventory](https://crates.io/crates/inventory) crate which is not available in a `no_std` environment, if we are in such a scenario we need to one of these workarounds to collect the full package.

### Build Script Hack
The first method is a bit more hacky but more advantageous for packages with many files, because it enables automatic collection just like std-compatible crates.

You basically need to add `protify` as a `build-dependency` of the consuming crate and enable the `inventory` feature. This means that the `build.rs` file will have access to the fully collected package, whereas the runtime crate will not use the feature and remain `no_std` compatible.

However, there is a catch. Since rust-analyzer compiles only one version per crate in a workspace (as of today), if you apply this workaround, your IDE will show all sorts of errors, because it will call `cargo check` in your `no_std` crate and applying the logic of `protify` with the `inventory` and `std` features enabled, even if your crate is not actually using these features in it.

You can check out how this works by cloning the repo and looking inside the [no-std-models](https://github.com/Rick-Phoenix/protify/tree/main/no-std-models) and [test-no-std](https://github.com/Rick-Phoenix/protify/tree/main/test-no-std) crates, where I applied this process and came across these limitations.

If you can find a workaround for the LSP issues, then this is the easiest way to go.

### Composing Schemas Manually

Alternatively, you can compose the schemas manually, by adding the files to the [`proto_package`](crate::proto_package) macro call, and by adding the services, messages and enums to the [`define_proto_file`](crate::define_proto_file) macro call. This is a bit more verbose but plays more nicely with the LSP.

#### Example of manual definition

```rust
use protify::*;

// We add the file manually
proto_package!(MANUAL_PKG, name = "manual_pkg", files = [MANUAL_FILE]);

// NOTE: Even if we are adding these elements manually,
// a file must still be in the module scope of where
// the items are defined, or manually assigned
// with the `#[proto(file = MY_FILE)]` attribute.
define_proto_file!(
	MANUAL_FILE,
	name = "manual_file.proto",
	package = MANUAL_PKG,
	// Adding messages manually
	messages = [
	  Msg2,
	  // Nested elements
	  Msg1 = { messages = [ Nested ], enums = [ NestedEnum ] }
	],
	services = [ MyService ],
	enums = [ Enum1 ],
);

#[proto_message]
pub struct Msg1 {
	pub id: i32,
}

#[proto_message]
#[proto(parent_message = Msg1)]
pub struct Nested {
	pub id: i32,
}

#[proto_message]
pub struct Msg2 {
	pub id: i32,
}

#[proto_enum]
pub enum Enum1 {
	Unspecified,
	A,
	B,
}

#[proto_enum]
#[proto(parent_message = Msg1)]
pub enum NestedEnum {
	Unspecified,
	A,
	B,
}

#[proto_service]
pub enum MyService {
	GetMsg { request: Msg1, response: Msg2 },
}
```
