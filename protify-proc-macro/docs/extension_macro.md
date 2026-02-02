Implements the [`ProtoExtension`](protify::ProtoExtension) trait for the given struct.

Since the item on which this macro is used is intended to be used only for schema purposes, the macro
will generate the impl and then erase all of the fields of the struct, so as to not trigger any "unused"
lints on the fields needlessly.

The only argument to this macro is the "target", which is an ident that must resolve to a valid protobuf extension target from Proto3 onwards, like MessageOptions, FileOptions and so on.

Each field must have a defined tag, and can also support options like fields in messages, enums or oneofs.

# Examples

```rust
use protify::*;

proto_package!(MY_PKG, name = "my_pkg");

#[proto_extension(target = MessageOptions)]
pub struct MyExt {
  #[proto(tag = 5000)]
  cool_opt: String
}

define_proto_file!(
  MY_FILE,
  name = "my_file.proto",
  package = MY_PKG,
  // We can then use the extension in a file
  extensions = [ MyExt ]
);
```

## Macro Attributes

- `target`
    - Type: Ident representing a valid extension target
    - Example: `#[proto_extension(target = MessageOptions)]`
    - Description:
        The target of the given extension. It must match one of the supported targets from proto3 onwards, such as MessageOptions, FileOptions and so on.

## Field Attributes

- `options`
    - Type: Expr
    - Example: `#[proto(options = vec![ my_option_1() ])]`
    - Description:
        Specifies the options for the given field. It must resolve to an implementor of IntoIterator<Item = [`ProtoOption`](crate::ProtoOption)>.

- `tag`
    - Type: number
    - Example: `#[proto(tag = 10000)]`
    - Description:
        Specifies the protobuf tag for the given field. Unline in messages or enums, tags must be set manually for extensions.

- `name`
    - Type: string
    - Example: `#[proto(name = "abc")]`
    - Description:
        Specifies the name for the given field. Defaults to the name of the rust field.
