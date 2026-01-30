Enables protobuf schema and validation features for the target enum.

It implements [`ProtoEnumSchema`](prelude::ProtoEnumSchema), [`ProtoValidation`](prelude::ProtoValidation) and [`AsProtoType`](prelude::AsProtoType) for the target enum.

If a tag for a variant is not manually defined, it will be automatically generated based on the tags that have already been used and those that are part of the reserved numbers.

# Example
```rust
use prelude::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

#[proto_enum]
#[proto(reserved_numbers(1..10))]
pub enum MyEnum {
  // Assigns 0 automatically
  Unspecified,
  // Manually assigned tag
  A = 10,
  // Tag is generated automatically,
  // taking into account reserved
  // and used tags
  B
}

// Implemented trait methods
assert_eq!(MyEnum::proto_name(), "MyEnum");
let x = MyEnum::from_int_or_default(20);
assert!(x.is_unspecified());
assert_eq!(x.as_int(), 0);

let schema = MyEnum::proto_schema();

let variant_b = schema.variants.last().unwrap();

// Proto variants will have the prefix with the enum name
assert_eq!(variant_b.name, "MY_ENUM_B");
assert_eq!(variant_b.tag, 11);

let proto_repr = schema.render_schema().unwrap();

assert_eq!(proto_repr, 
r"
enum MyEnum {
  reserved 1 to 9;

  MY_ENUM_UNSPECIFIED = 0;
  MY_ENUM_A = 10;
  MY_ENUM_B = 11;
}".trim_start());
```


## Container attributes:

- `reserved_numbers`
    - Type: list of individual numbers or closed ranges
    - Example: `#[proto(reserved_numbers(1, 2..10, 5000..MAX))]`
    - Description:
        Specifies the reserved numbers for the given enum. These will be skipped when automatically generating tags for each field, and copied as such in the proto files output. In order to reserve up to the maximum tag range, use the `MAX` ident as shown above.

- `reserved_names`
    - Type: list of strings
    - Example: `#[proto(reserved_names("MYENUM_ABC", "MYENUM_DEG"))]`
    - Description:
        Specifies the reserved names for the given enum

- `options`
    - Type: Expr
    - Example: `#[proto(options = vec![ my_option_1() ])]`
    - Description:
        Specifies the options for the given enum. It must resolve to an implementor of IntoIterator<Item = [`ProtoOption`](crate::ProtoOption)>.

- `name`
    - Type: string
    - Example: `#[proto(name = "MyEnum")]`
    - Description:
        Specifies the name of the message. Overrides the default behaviour, which uses the name of the rust enum.

- `parent_message`
    - Type: Ident
    - Example: `#[proto(parent_message = MyMsg)]`
    - Description:
        Specifies the parent message of a nested enum.

- `deprecated`
    - Type: Ident
    - Example: `#[proto(deprecated = false)]` or `#[deprecated]`
    - Description:
    Marks the enum as deprecated. The proto output will reflect this setting.
    If for some reason you want to deprecate the rust enum but not the proto enum, you can use `#[deprecated]` along with `#[proto(deprecated = false)]`. Vice versa, you can deprecate the proto enum only by using `#[proto(deprecated = true)]`

## Variant Attributes

- `options`
    - Type: Expr
    - Example: `#[proto(options = vec![ my_option_1() ])]`
    - Description:
        Specifies the options for the given variant. It must resolve to an implementor of IntoIterator<Item = [`ProtoOption`](crate::ProtoOption)>.

- `tag`
    - Type: number
    - Example: `#[proto(tag = 10000)]`
    - Description:
    Specifies the protobuf tag for the given field. Unline in messages or enums, tags must be set manually for extensions.

- `name`
    - Type: string
    - Example: `#[proto(name = "MY_ENUM_ABC")]`
    - Description:
        Specifies the name for the given variant. Defaults to the SCREAMING_CASE name of the variant, prefixed by the name of the enum as per the proto convention.

- `deprecated`
    - Type: Ident
    - Example: `#[proto(deprecated = false)]` or `#[deprecated]`
    - Description:
        Marks the variant as deprecated. The proto output will reflect this setting.
        If for some reason you want to deprecate the rust variant but not the proto variant, you can use `#[deprecated]` along with `#[proto(deprecated = false)]`. Vice versa, you can deprecate the proto variant only by using `#[proto(deprecated = true)]`
