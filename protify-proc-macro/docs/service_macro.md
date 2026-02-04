Implements [`ProtoService`](protify::ProtoService) for the given enum.

Since the enum is only meant to be used for schema purposes, the macro will erase all the fields in it
and change its type to emit a unit struct, so as to not trigger false positives for "unused" variants.

Each variant represents a protobuf method for the given service, and it must contain two named fields, `request` and `response`, with the target message for each.

The target of a method must implement [`MessagePath`](protify::MessagePath), which is automatically implemented by the
[`proto_message`](protify::proto_message) macro and for all types from the [`proto_types`](protify::proto_types) crate.

# Example
```rust
use indoc::indoc;
use protify::*;

proto_package!(MY_PKG, name = "my_pkg");

// A file must be in scope to be picked up by the service schema
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

#[proto_message]
pub struct User {
	pub id: i32,
	pub name: String,
}

#[proto_message]
pub struct UserId {
	pub id: i32,
}

#[proto_message]
pub struct Status {
	pub success: bool,
}

#[proto_service]
enum UserService {
	GetUser { request: UserId, response: User },
	UpdateUser { request: User, response: Status },
}

fn main() {
	let proto_rep = UserService::proto_schema();

	assert_eq!(
		proto_rep.render_schema().unwrap(),
		indoc! {"
            service UserService {
              rpc GetUser (UserId) returns (User);
              rpc UpdateUser (User) returns (Status);
            }"
		}
	);
}
```
