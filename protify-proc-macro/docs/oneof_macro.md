Implements protobuf schema and validation features for a rust enum.

This macro will implement the following:
- Clone
- PartialEq
- [`prost::Oneof`](protify::prost::Oneof)
- [`ProtoOneof`](protify::ProtoOneof)
- [`ValidatedOneof`](protify::ValidatedOneof)
- [`CelOneof`](protify::CelOneof) (if the `cel` feature is enabled)
- A method called `check_validators` (compiled only with `#[cfg(test)]`) for verifying the correctness of the validators used in it
- (If the `skip_checks(validators)` attribute is not used) A test that calls the `check_validators` method and panics on failure.

If the impl is not proxied, these traits and methods will target the struct directly.

If the impl is proxied:
- A new struct with a `Proto` suffix will be generated (i.e. MyOneof -> MyOneofProto) and these traits and methods will target that. An impl for [`ProxiedOneof`](protify::ProxiedOneof) will also be generated.
- The proxy will implement [`OneofProxy`](protify::OneofProxy).

To learn more about proxied implementations, visit the dedicated [section](crate::guide::proxies).

ℹ️ **NOTE**: In `protify`, oneofs are reusable. This means that tags must be set manually for each of the variants, and that the name of the oneof will be the name of the field in the message where it's used. To learn more about oneofs, visit the [reusing oneofs](protify::guide::reusing_oneofs) section.

# Examples

```rust
use protify::*;

#[proto_oneof]
pub enum NormalOneof {
  // Tags must be set manually!
  #[proto(tag = 1)]
  A(i32),
  #[proto(tag = 2)]
  B(u32)
}

// Generates `ProxiedOneofProto` as the proto-facing version
#[proto_oneof(proxied)]
pub enum ProxiedOneof {
  #[proto(tag = 1)]
  A(i32),
  #[proto(tag = 2)]
  B(u32)
}

fn main() {
  use protify::*;

  // `ProxiedOneof` and `OneofProxy` methods
  let oneof = ProxiedOneofProto::A(1);
  let proxy = oneof.into_proxy();
  let oneof_again = proxy.into_oneof();
}
```

## Macro attributes

- `proxied`
    - Type: Ident
    - Example: `#[proto_oneof(proxied)]`
    - Description:
        Creates a proxied oneof

## Container attributes:

- `derive`
    - Type: list of Paths
    - Example: `#[proto(derive(Copy))]`
    - Description:
        In case of a proxied impl, it specifies derives to apply to the Proto oneof.
        Shorthand for `#[proto(attr(derive(..)))]`

- `attr`
    - Type: MetaList
    - Example: `#[proto(attr(serde(default)))]`
    - Description:
        Forwards the given metas to the proto oneof.
        In the example above, the proto oneof would receive the attribute as `#[serde(default)]`.

- `options`
    - Type: Expr
    - Example: `#[proto(options = vec![ my_option_1() ])]`
    - Description:
        Specifies the options for the given oneof. It must resolve to an implementor of IntoIterator<Item = [`ProtoOption`](crate::ProtoOption)>.

- `from_proto`
    - Type: function Path or closure
    - Example: `#[proto(from_proto = my_convert_fn)]` or `#[proto(from_proto = |v| v.some_method())]`
    - Description:
        Overrides the automatically generated conversion from proto to proxy.

- `into_proto`
    - Type: function Path or closure
    - Example: `#[proto(into_proto = my_convert_fn)]` or `#[proto(into_proto = |v| v.some_method())]`
    - Description:
        Overrides the automatically generated conversion from proxy to proto.

- `validate`
    - Type: Expr, or bracketed list of Exprs
    - Example: `#[proto(validate = [ CustomValidator, *STATIC_VALIDATOR ])]`
    - Description:
        Defines the default validators for the given oneof. These will be executed inside the oneof's own [`validate`](crate::ValidatedMessage::validate) method, and whenever the oneof in another message, along with the validators defined for each variant. The expressions is it must resolve to an implementor of [`Validator`](crate::Validator) for the oneof.

- `skip_checks`
    - Type: list of Idents
    - Example: `#[proto(skip_checks(validators))]`
    - Description:
        Disables the generation of tests. 
        Currently, the allowed values are:
        - `validators`: disables the automatic generation of a test that checks the validity of the validators used by the message. The `check_validators` method will still be generated and be available for manual testing.
