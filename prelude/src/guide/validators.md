# Validators

A key component of this library is that is enables a very simple but powerful way of attaching validators to messages and oneofs. The validators hold two roles at the same time: on the one hand, they handle the validation logic on the rust side, and on the other hand, they also offer a schema for conversion into a protobuf option, so that the settings made in rust can be reflected in the protobuf package and be picked up by other clients using the same contracts.

All the provided validators map their options to the protovalidate options, but you ca also create customized validators that map to customized protobuf options.

Validators can be assigned to oneofs/messages as a whole, or to individual fields/variants.

## Example

```rust
use prelude::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

#[proto_message]
// Top level validation using a CEL program
#[proto(validate = |v| v.cel(cel_program!(id = "my_rule", msg = "oopsie", expr = "this.id == 50")))]
pub struct MyMsg { 
    // Field validator
    // The argument of the closure is the IntValidator builder
    #[proto(validate = |v| v.gt(0))]
    pub id: i32,

    #[proto(oneof(tags(1, 2)))]
    #[proto(validate = |v| v.required())]
    pub oneof: Option<MyOneof>
}

#[proto_oneof]
pub enum MyOneof {
    #[proto(tag = 1)]
    // Same thing for oneof variants
    #[proto(validate = |v| v.gt(0))]
    A(i32),
    #[proto(tag = 2)]
    B(u32),
}
```

ℹ️ **NOTE**: Validators that are provided via closures will be cached in a Lazy struct (normally a [`LazyLock`](std::sync::LazyLock) or a wrapper for [`OnceBox`](once_cell::race::OnceBox) in a no_std environment), so they are only initialized once.

# Custom Validators

The [`Validator`](crate::Validator) trait allows for the construction of custom validators.

A validator can be a struct (stateful) or just a function, wrapped with the [`from_fn`](crate::from_fn) helper.

Each validator only needs to implement a single method, [`validate_core`](crate::Validator::validate_core), which receives a [`ValidationCtx`](crate::ValidationCtx) and an [`Option`] of a generic type that supports [`Borrow`](std::borrow::Borrow) with the target type. All the other methods are automatically derived.

```rust
use prelude::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

// Function validator
fn validate_number(ctx: &mut ValidationCtx, val: Option<&i32>) -> ValidationResult {
    let mut is_valid = IsValid::Yes;

    if val.is_none() {
        // IsValid is a boolean-like enum, so it can be used with
        // bit operators
        is_valid &= ctx.add_required_violation(
            // Optionally, we can provide a custom error message
            Some("number must be set".to_string())
        )?; // We can use the result to handle `fail-fast` scenarios
            // and trigger early exit
    }

    Ok(is_valid)
}

pub struct CustomValidator;

// If a validator contains some heavy or complex logic, 
// it can be initialized once and then reused
static CACHED_VALIDATOR: Lazy<CustomValidator> = Lazy::new(|| CustomValidator);

impl Validator<MyMsg> for CustomValidator {
    type Target = MyMsg;

    fn validate_core<V>(&self, ctx: &mut ValidationCtx, val: Option<&V>) -> ValidationResult
    where
        V: std::borrow::Borrow<Self::Target> + ?Sized,
    {
        validate_number(ctx, val.map(|v| &v.borrow().id))
    }
}

#[proto_message]
// Using the custom validators at the top level
#[proto(validate = [ CustomValidator, *CACHED_VALIDATOR ])]
pub struct MyMsg { 
    // Using a function validator
    #[proto(validate = from_fn(validate_number))]
    pub id: i32,

    #[proto(oneof(tags(1, 2)))]
    // Using a variety of closure and custom validators
    #[proto(validate = [ |v| v.required(), CustomValidator, *CACHED_VALIDATOR ])]
    pub oneof: Option<MyOneof>
}

impl Validator<MyOneof> for CustomValidator {
    type Target = MyOneof;

    fn validate_core<V>(&self, ctx: &mut ValidationCtx, val: Option<&V>) -> ValidationResult
    where
        V: std::borrow::Borrow<Self::Target> + ?Sized,
    {
        if let Some(oneof) = val.map(|v| v.borrow()) 
            && let MyOneof::A(int) = oneof
        {
            validate_number(ctx, Some(&int))
        } else {
            Ok(IsValid::Yes)
        }
    }
}

#[proto_oneof]
// Same thing for oneofs
#[proto(validate = [ CustomValidator, *CACHED_VALIDATOR ])]
pub enum MyOneof {
    #[proto(tag = 1, validate = from_fn(validate_number))]
    A(i32),
    #[proto(tag = 2)]
    B(u32),
}
```

You can refer to the tests for more involved examples of custom validator usage.

## Customizing Error Messages

In order to facilitate things like i18n, every provided validator allows for customization of the error messages, withot requiring a whole custom validator to be designed purely for this purpose.

Each builder features a method called `with_error_messages`, which accepts a BTreeMap that maps known violations to error messages.

### Example

```rust
use prelude::*;
use prelude::proto_types::{
  protovalidate::violations_data::*,
};
use std::collections::BTreeMap;
use maplit::btreemap;

let validator = StringValidator::builder().min_len(3).with_error_messages({
    let locale = std::env::var("LOCALE").unwrap_or("en/GB".to_string());
    let error_message = if locale == "it/IT" {
        "il nickname deve contenere almeno 3 caratteri"
    } else {
        "the nickname must be at least three characters long"
    };

    btreemap!{
        StringViolation::MinLen => error_message
    }
}).build();

let violations = validator.validate("a").unwrap_err().into_violations();

assert_eq!(violations[0].message(), "the nickname must be at least three characters long");
```

## Schema Representation

In order to make validation settings portable, each validator can optionally implement the [`schema`](crate::Validator::schema) method, which outputs a protobuf schema representation, so that its contents can be represented as an option in a protobuf file. 

All default validators implement this method and output the options in the protovalidate format.

### Example

```rust
use prelude::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

#[proto_message]
pub struct MyMsg {
    #[proto(validate = |v| v.min_len(3).max_len(32))]
    pub name: String
}

let schema = MyMsg::proto_schema();

assert_eq!(MyMsg::proto_schema().render_schema().unwrap(), 
r"message MyMsg {
  string name = 1 [
    (buf.validate.field) = {
      string: {
        min_len: 3, 
        max_len: 32
      }
    }
  ];
}")
```
