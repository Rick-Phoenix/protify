Creates a new proto file schema, and brings it into scope for the items that are defined in the same module.

A file must always be in scope for any given item definition (oneof, enum, message). This can be done by using this macro, the [`use_proto_file`](protify::use_proto_file) and [`inherit_proto_file`](protify::inherit_proto_file) macros, or by using the `file` attribute on enums or messages.

For more information about how to manage proto files, visit the [dedicated section](protify::guide::managing_files).

## Positional Arguments

The first argument is the ident that will be used for the file handle, which will be a unit struct that implements [`FileSchema`](protify::FileSchema).

In order to distinguish the file handle from a normal rust type, it is advised to use the SCREAMING_CASE casing.

## Named Arguments

- `package` (required)
    - Type: Ident
    - Example: `define_proto_file!(MY_FILE, package = MY_PKG)`
    - Description:
        The ident of the package handle, which is created with the [`proto_package`](protify::proto_package) macro.

- `name`
    - Type: string
    - Example: `define_proto_file!(MY_FILE, name = "my_file.proto")`
    - Description:
        The name of the file. If missing, it will be assumed to be the snake_case name of the file ident. If the file ident ends with the `_FILE` suffix, this will be stripped.

        ```rust
        use protify::*;

        proto_package!(TESTING_PKG, name = "testing");

        define_proto_file!(TESTING_FILE, package = TESTING_PKG);

        fn main() {
        	define_proto_file!(TESTING, package = TESTING_PKG);

        	assert_eq!(TESTING_FILE::NAME, "testing.proto");
        	assert_eq!(TESTING::NAME, "testing.proto");
        }
        ```

- `extern_path`
    - Type: string
    - Example: `define_proto_file!(MY_FILE, extern_path = "module::path")`
    - Description:
        The rust path to reach the items described in this proto file, when applied from an external crate. The items in this file will inherit the path of their file + their own ident. For example, if a message `Msg1` is assigned to this file, its `extern_path` will be registered as `::module::path::Msg1`.
        It defaults to `core::module_path!()` and should only be overridden for re-exported items where their path does not match their module's path.

- `options`
    - Type: Expr
    - Example: `define_proto_file!(MY_FILE, options = vec![ my_option() ])`
    - Description:
        Specifies the options for the given file. It must resolve to an implementor of IntoIterator<Item = [`ProtoOption`](crate::ProtoOption)>.


- `imports`
    - Type: Expr
    - Example: `define_proto_file!(MY_FILE, imports = vec![ "import1", "import2" ])`
    - Description:
        Specifies the imports for the given file. In most occasions, the necessary imports will be **added automatically** so this should only be used as a fallback mechanism. It should resolve to an implementor of `IntoIterator` with the items being either `String`, `Arc<str>`, `Box<str>` or `&'static str`.


- `extensions`
    - Type: bracketed list of Paths
    - Example: `define_proto_file!(MY_FILE, extensions = [ MyExtension ])`
    - Description:
        Specifies the extensions for the given file. The items inside the list should be structs marked with the `#[proto_extension]` macro or implementors of [`ProtoExtension`](crate::ProtoExtension). Unlike other elements, extensions must always be added manually to a file.


- `edition`
    - Type: [`Edition`](crate::Edition)
    - Example: `define_proto_file!(MY_FILE, edition = Proto3)`
    - Description:
        A value from the [`Edition`](crate::Edition) enum. Supports editions from Proto3 onwards.

# Example

```rust
use protify::*;
proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

#[proto_message]
pub struct Msg {
	pub id: i32,
}

assert_eq!(Msg::proto_schema().file, "my_file.proto");
```

## Manual Schema Composition

As explained in the [`package setup`](protify::guide::package_setup) section, when the `inventory` feature is disabled, the schemas must be collected manually.

In such a scenario, we have three more attributes that we can use (and would otherwise be ignored!), to add each element to the target file:
- `messages`
- `services`
- `enums`

### Example Of Manual Composition

```rust
use protify::*;

// We must also add the file manually
proto_package!(MY_PKG, name = "my_pkg", files = [MY_FILE]);

define_proto_file!(
  MY_FILE,
  name = "my_file.proto",
  package = MY_PKG,
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

For more information on how to bring a pre-defined file in scope, refer to the [`use_proto_file`](crate::use_proto_file) macro.
