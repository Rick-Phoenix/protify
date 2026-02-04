# Schema Features

Each element of a protobuf file can be defined with a proc or attribute macro. Options for each element can be composed and assigned programmatically.

Unlike in prost, enums representing protobuf oneofs can be reused for multiple messages (with limitations explained in the [reusing oneofs](crate::guide::reusing_oneofs) section).

```rust
use protify::*;

// Creates a new package
proto_package!(MY_PKG, name = "my_pkg");
// Creates a new file
define_proto_file!(
	MY_FILE,
	name = "my_file.proto",
	package = MY_PKG,
	extensions = [MyExt]
);

fn create_option(value: i32) -> ProtoOption {
	proto_option!("(my_custom_opt)" => value)
}

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
#[proto(reserved_numbers(22, 23..30))]
#[proto(reserved_names("name1", "name2"))]
pub struct MyMsg {
	// Programmatically creating options
	#[proto(options = [ create_option(25) ])]
	pub id: i32,
	#[proto(oneof(tags(1, 2)))]
	pub oneof: Option<MyOneof>,
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
	// Reusing the same oneof
	#[proto(oneof(tags(1, 2)))]
	pub oneof: Option<MyOneof>,
}

// Tags are assigned automatically
// and take the reserved numbers in consideration
#[proto_enum]
#[proto(reserved_numbers(20, 25..30))]
pub enum MyEnum {
	Unspecified,
	A,
	B,
}
```

