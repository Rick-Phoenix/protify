# Validators

Whenever models or API contracts are defined and used, the need for validation follows close behind. Because of this, `protify` ships with a validation framework that provides both type-safety and customization while also making validators portable by turning them into protobuf schemas.

The implementors of [`Validator`](crate::Validator) hold two roles at the same time: on the one hand, they handle the validation logic on the rust side, and on the other hand, they also produce a schema representation, so that their settings can be included as options in the proto files, and be picked up by other clients of the same contract, and ported between different applications.

All the provided validators map their options to the [protovalidate](https://github.com/bufbuild/protovalidate) options, but you can also create customized validators that map to customized protobuf options.

Because every validator is type-safe and comes with ergonomic builder methods to be built with, defining validators using `protify` is a vastly superior experience to defining them manually in proto files, where the process is repetitive, rigid, and lacking the essential features of modern programming such as type safety and LSP integration, as well as programmatic composition.

Validators can be assigned to oneofs/messages as a whole, or to individual fields/variants to be incorporated in thier validation logic, or be used to validate values on-demand.

## Example

```rust
use protify::*;
use std::collections::HashMap;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

// We can define logic to programmatically compose validators
fn prefix_validator(prefix: &'static str) -> StringValidator {
    StringValidator::builder()
        .prefix(prefix)
        .build()
}

#[proto_message]
// Top level validation using a CEL program
#[proto(validate = |v| v.cel(cel_program!(id = "my_rule", msg = "oopsie", expr = "this.id == 50")))]
pub struct MyMsg { 
    // Field validator
    // Type-safe and lsp-friendly!
    // The argument of the closure is the IntValidator builder,
    // so we are going to get autocomplete suggestions
    // for its specific methods.
    #[proto(validate = |v| v.gt(0))]
    pub id: i32,

    // Repeated validator
    #[proto(validate = |v| v.items(|i| i.gt(0)))]
    pub repeated_nums: Vec<i32>,

    // Map validator
    #[proto(validate = |m| m.keys(|k| k.gt(0)).values(|v| v.min_len(5)))]
    pub map_field: HashMap<i32, String>,

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
    // Multiple validators, including a programmatically built one!
    #[proto(tag = 2, validate = [ |v| v.min_len(5), prefix_validator("abc") ])]
    B(String),
}
```

ℹ️ **NOTE**: Validators that are provided via closures will be cached in a Lazy struct (normally a [`LazyLock`](std::sync::LazyLock) or a wrapper for [`OnceBox`](once_cell::race::OnceBox) in a no_std environment), so they are only initialized once (except for the [`OneofValidator`](crate::OneofValidator) which is small enough).

# Custom Validators

The [`Validator`](crate::Validator) trait allows for the construction of custom validators.

A validator can be a struct (stateful) or just a function, wrapped with the [`from_fn`](crate::from_fn) helper.

Each validator only needs to implement a single method, [`execute_validation`](crate::Validator::execute_validation), which receives a [`ValidationCtx`](crate::ValidationCtx) and an [`Option`] of the target type. All the other methods are automatically derived.

The [`schema`](crate::Validator::schema) and [`check_consistency`](crate::Validator::check_consistency) methods can be optionally implemented, as described later in this section and in the [`correctness`](crate::guide::correctness) section.

```rust
use protify::*;

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
        )?; // Validators use `Result`s to handle `fail-fast` scenarios
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

    fn execute_validation(&self, ctx: &mut ValidationCtx, val: Option<&Self::Target>) -> ValidationResult
    {
        validate_number(ctx, val.map(|v| &v.id))
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

    fn execute_validation(&self, ctx: &mut ValidationCtx, val: Option<&Self::Target>) -> ValidationResult
    {
        if let Some(oneof) = val 
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

You can have a look at the testing crates in the repo for more complex examples of custom validator usage.

## Customizing Error Messages

In order to facilitate things like i18n, every provided validator allows for customization of the error messages, without requiring a whole custom validator to be designed purely for this purpose.

Each builder features a method called `with_error_messages`, which accepts a [`BTreeMap`](std::collections::BTreeMap) that maps the violations that it may produce to error messages.

### Example

```rust
use protify::*;
use protify::proto_types::{
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

In order to make validation settings portable, each validator can optionally implement the [`schema`](crate::Validator::schema) method, which outputs a [`ProtoOption`](crate::ProtoOption) that will be added to the receiving message/oneof in the proto file. 

All default validators implement this method and output the options in the `protovalidate` format.

### Example

```rust
use protify::*;

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
