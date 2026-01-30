A file must always be in scope for any given item definition (oneof, enum, message).

The first argument is the ident that will be used to refer to the file handle. 
The other parameters are not positional and are as follows:

- `name` (required)
    - Type: string
    - Example: `define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG)`
    - Description:
        The name of the file.

- `package` (required)
    - Type: Ident
    - Example: `define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG)`
    - Description:
        The ident of the package handle.


- `extern_path`
    - Type: string
    - Example: `define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG, extern_path = "module::path")`
    - Description:
        The rust path to reach the items described in this proto file, when applied from an external crate. The items in this file will inherit the path of their file + their own ident. For example, if a message `Msg1` is assigned to this file, its `extern_path` will be registered as `::module::path::Msg1`. 
        It defaults to `core::module_path!()` and should only be overridden for re-exported items where their path does not match their module's path.

- `options`
    - Type: Expr
    - Example: `define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG, options = vec![ my_option() ])`
    - Description:
        Specifies the options for the given file. It must resolve to an implementor of IntoIterator<Item = [`ProtoOption`](crate::ProtoOption)>.


- `imports`
    - Type: Expr
    - Example: `define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG, imports = vec![ "import1", "import2" ])`
    - Description:
        Specifies the imports for the given file. In most occasions, the necessary imports will be added automatically so this should only be used as a fallback mechanism. It should resolve to an implementor of `IntoIterator` with the items being either `String`, `Arc<str>`, `Box<str>` or `&'static str`.


- `extensions`
    - Type: bracketed list of Paths
    - Example: `define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG, extensions = [ MyExtension ])`
    - Description:
        Specifies the extensions for the given file. The items inside the list should be structs marked with the `#[proto_extension]` macro or implementors of [`ProtoExtension`](crate::ProtoExtension).


- `edition`
    - Type: [`Edition`](crate::Edition)
    - Example: `define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG, edition = Proto3)`
    - Description:
        A value from the [`Edition`](crate::Edition) enum. Supports editions from Proto3 onwards.

ℹ️ NOTE: All inputs except for `package` and `name` are ignored when the `inventory` feature is disabled. In such a scenario, the [`file_schema`](crate::file_schema) macro must be used to add the items manually.

For more information on how to bring a pre-defined file in scope, refer to the [`use_proto_file`](crate::use_proto_file) macro.
