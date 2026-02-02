Protify is a library that aims to vastly simplify working with protobuf in a rust project. It offers a rust-first approach in defining protobuf models, so that every element in a protobuf package (messages, enums, oneofs, services, extensions, files) can be fully defined in rust code, and then the respective proto files can be generated from it as a compilation artifact, rather than it being the other way around.

 Whereas working with protobuf can often feel like an "alien" experience in rust, as we have to interact with structs and enums that are locked away in an included file outside of our reach and control, to an experience that feels almost as native as just working with `serde`.

# Schema Features

We can use the provided macros to map a rust struct or enum to a protobuf item (messages, services, oneofs, etc).

We can add options programmatically, and even reuse the same oneof for multiple messages (with limitations explained in the [reusing oneofs](crate::guide::reusing_oneofs) section).

```rust
use protify::*;

// Creates a new package
proto_package!(MY_PKG, name = "my_pkg");
// Creates a new file
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG, extensions = [ MyExt ]);

fn create_option(value: i32) -> ProtoOption {
    proto_option!("(my_custom_opt)" => value)
}

#[proto_extension(target = MessageOptions)]
pub struct MyExt {
    #[proto(tag = 5000)]
    cool_opt: String
}

#[proto_service]
enum MyService {
    Service1 {
        request: MyMsg,
        response: MyMsg
    }
}

#[proto_message]
#[proto(reserved_numbers(22, 23..30))]
#[proto(reserved_names("name1", "name2"))]
pub struct MyMsg {
    // Programmatically creating options!
    #[proto(options = [ create_option(25) ])]
    pub id: i32,
    #[proto(oneof(tags(1, 2)))]
    pub oneof: Option<MyOneof>
}

#[proto_oneof]
pub enum MyOneof {
    #[proto(tag = 1)]
    A(i32),
    #[proto(tag = 2)]
    B(u32),
}

#[proto_message]
pub struct MyMsg2 {
    pub id: i32,
    // Reusing the same oneof!
    #[proto(oneof(tags(1, 2)))]
    pub oneof: Option<MyOneof>
}

// Tags are assigned automatically
// and take the reserved numbers in consideration
#[proto_enum]
#[proto(reserved_numbers(20, 25..30))]
pub enum MyEnum {
    Unspecified,
    A,
    B
}
```



 For a full guide on how to set up a package, visit the [package setup](crate::guide::package_setup) section.

# Database Mapping

An important benefit that comes from having a "rust-first" approach when defining our models is that they can easily be used for operations such as db queries, without needing to create separate structs to map the generated protos, or injecting the attributes as plain text with the prost-build helper.

Our models are not hidden behind a generated file injected with `include!`, they are under our full control and we can use macros and attributes on them like on any other normal rust struct.

And with proxies, the interactions with a database becomes even easier, because we can have the proto message take a certain shape, while the proxy can represent the state of a message after its data has been mapped, for example, to an item queried from the database.

```rust
use protify::*;
use protify::proto_types::Timestamp;
use diesel::prelude::*;

proto_package!(DB_TEST, name = "db_test", no_cel_test);
define_proto_file!(DB_TEST_FILE, name = "db_test.proto", package = DB_TEST);

mod schema {
  diesel::table! {
    users {
      id -> Integer,
      name -> Text,
      created_at -> Timestamp
    }
  }
}

// If we want to use the message as it is for the db model
#[proto_message]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
  #[diesel(skip_insertion)]
  pub id: i32,
  pub name: String,
  #[diesel(skip_insertion)]
  // We need this to keep `Option` for this field
  // which is necessary for protobuf
  #[diesel(select_expression = schema::users::columns::created_at.nullable())]
  #[proto(timestamp)]
  pub created_at: Option<Timestamp>,
}

// If we want to use the proxy as the db model, for example
// to avoid having `created_at` as `Option`
#[proto_message(proxied)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProxiedUser {
  #[diesel(skip_insertion)]
  pub id: i32,
  pub name: String,
  #[diesel(skip_insertion)]
  #[proto(timestamp, from_proto = |v| v.unwrap_or_default())]
  pub created_at: Timestamp,
}

fn main() {
    use schema::users::dsl::*;

    let conn = &mut SqliteConnection::establish(":memory:").unwrap();

    let table_query = r"
    CREATE TABLE users (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
      );
    ";

    diesel::sql_query(table_query)
      .execute(conn)
      .expect("Failed to create the table");

    let insert_user = User {
        id: 0,
        name: "Gandalf".to_string(),
        created_at: None
    };

    diesel::insert_into(users)
        .values(&insert_user)
        .execute(conn).expect("Failed to insert user");

    let queried_user = users
        .filter(id.eq(1))
        .select(User::as_select())
        .get_result(conn).expect("Failed to query user");

    assert_eq!(queried_user.id, 1);
    assert_eq!(queried_user.name, "Gandalf");
    // The timestamp will be populated by the database upon insertion
    assert_ne!(queried_user.created_at.unwrap(), Timestamp::default());

    let proxied_user = ProxiedUser {
        id: 0,
        name: "Aragorn".to_string(),
        created_at: Default::default()
    };

    diesel::insert_into(users)
        .values(&proxied_user)
        .execute(conn).expect("Failed to insert user");

    let queried_proxied_user = users
        .filter(id.eq(2))
        .select(ProxiedUser::as_select())
        .get_result(conn).expect("Failed to query user");

    assert_eq!(queried_proxied_user.id, 2);
    assert_eq!(queried_proxied_user.name, "Aragorn");

    // Now we have the message, with the created_at field populated
    let msg = queried_proxied_user.into_message();

    assert_ne!(msg.created_at.unwrap(), Timestamp::default());
}
```


# Proxied Implementations

Messages and oneofs can be proxied. Doing so will generate a new struct/enum with the same name, followed by a `Proto` suffix.

Proxied messages/oneofs unlock the following features:

- A field/variant can be missing from the proto struct, but present in the proxy
- Enums can use their actual rust enum type, rather than being pure integers
- Oneofs don't need to be wrapped in `Option`
- Messages don't need to be wrapped in `Option`
- We can use types that are not supported by prost
- We can map an incoming type from another type via custom conversions

By default, the macro will generate a conversion from proxy to proto and vice versa that just calls `.into()` for each field/variant.

To provide custom conversions, you can use the `from_proto` and `into_proto` attributes on the container (to replace the automatically generated impl as a whole) or on individual fields/variants.

The messages/oneofs will also implement [`ProxiedMessage`](crate::ProxiedMessage) or [`ProxiedOneof`](crate::ProxiedOneof), whereas the proxies will implement [`MessageProxy`](crate::MessageProxy) and [`OneofProxy`](crate::OneofProxy).

## Examples

```rust
use protify::*;
use std::sync::Arc;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);


// Generates a MsgProto struct that is protobuf-compatible
#[proto_message(proxied)]
pub struct Msg {
    // Requires setting the type manually as the type
    // is not prost-compatible
    #[proto(string)]
    // Must provide a custom `into_proto` impl because `Arc<str>` does not support `Into<String>`
    #[proto(into_proto = |v| v.as_ref().to_string())]
    pub name: Arc<str>,
    // Ignored field. Conversion from proto will use `Default::default()` unless a custom
    // conversion is specified
    #[proto(ignore)]
    pub rust_only: i32,
    // In proxied messages, we can use `default` for oneofs
    // so that using `Option` is not required.
    // The default conversion will call `ProxiedOneofProto::default().into()`
    // if the field is `None` in the proto struct.
    #[proto(oneof(proxied, default, tags(1, 2)))]
    pub oneof: ProxiedOneof,
    // We can do the same for messages too
    #[proto(message(default))]
    pub message_with_default: Msg2,
    // We can use the enum directly as the type
    #[proto(enum_)]
    pub enum_: TestEnum
}

#[proto_enum]
pub enum TestEnum {
    Unspecified, A, B
}

#[proto_message]
pub struct Msg2 {
    pub id: i32,
    // In direct impls, enums are just integers
    #[proto(enum_(TestEnum))]
    pub enum_: i32
}

// Generates the `ProxiedOneofProto` enum
#[proto_oneof(proxied)]
pub enum ProxiedOneof {
    #[proto(string, tag = 1, into_proto = |v| v.as_ref().to_string())]
    A(Arc<str>),
    #[proto(tag = 2)]
    B(u32),
}

impl Default for ProxiedOneofProto {
    fn default() -> Self {
        Self::B(1)
    }
}

fn main() {
    let msg = MsgProto::default();
    // Using the `ProxiedMessage` trait
    let proxy = msg.into_proxy();
    // Using the `MessageProxy` trait
    let msg_again = proxy.into_message();
}
```


# Validators

Whenever models are defined and used, the need for validation follows close behind. `protify` provides a series of pre-built validators to handle all primitives and well known types.

The validators hold two roles at the same time: on the one hand, they handle the validation logic on the rust side, and on the other hand, they also produce a schema representation, so that their settings can be included as options in the proto files.

All the provided validators map their options to the [protovalidate](https://github.com/bufbuild/protovalidate) options, but you can also create customized validators that map to customized protobuf options.

Validators can be assigned to oneofs/messages as a whole, or to individual fields/variants.

## Example

```rust
use protify::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

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

Each validator only needs to implement a single method, [`execute_validation`](crate::Validator::execute_validation), which receives a [`ValidationCtx`](crate::ValidationCtx) and an [`Option`] of a generic type that supports [`Borrow`](std::borrow::Borrow) with the target type. All the other methods are automatically derived.

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

    fn execute_validation<V>(&self, ctx: &mut ValidationCtx, val: Option<&V>) -> ValidationResult
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

    fn execute_validation<V>(&self, ctx: &mut ValidationCtx, val: Option<&V>) -> ValidationResult
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

In order to make validation settings portable, each validator can optionally implement the [`schema`](crate::Validator::schema) method, which outputs a protobuf schema representation, so that its contents can be represented as an option in a protobuf file.

All default validators implement this method and output the options in the protovalidate format.

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


 # Feature Flags

* **`tonic`** —  Enables direct conversion from validation errors to tonic::Status
* **`serde`** —  Enables serde for all schema representations.
* **`inventory`** *(enabled by default)* —  Collects elements of a package automatically.
* **`chrono`** *(enabled by default)* —  Enables timestamp `now` features in CEL and timestamp validation.
* **`chrono-wasm`** —  Enables usage of wasmbind for chrono's `now` methods.
* **`std`** *(enabled by default)* —  Enables the std library features.
* **`common-types`** —  Enables schema features for `google.type` types.
* **`rpc-types`** —  Enables schema features for `google.rpc` types.
* **`reflection`** —  Enables usage with reflection.
* **`cel`** *(enabled by default)* —  Enables CEL validation.
* **`regex`** *(enabled by default)* —  Enables regex-based validators.