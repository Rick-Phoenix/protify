# Reusing Oneofs

An extra feature brought by this library is the ability to reuse the same rust enum as a oneof across different messages.

When a oneof is used in a message, a completely new oneof will be generated inside that message's definition in its proto file. It will contain the same variants and tags of the original instance, as well as all of the options defined on the original instance. The name will match the name of the field in the rust struct (unless manually overridden with the `#[proto(name = "..")]` attribute). 

All validators and options defined on the oneof as a field will be local to that instance.

```rust
use protify::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

#[proto_message]
pub struct Msg1 {
    // Tags have to be defined explicitly
    // at the call site
    #[proto(oneof(tags(1, 2)))]
    pub oneof_1: Option<ReusedOneof>,
}

#[proto_message]
pub struct Msg2 {
    // A new oneof will be created inside
    // this message's representation,
    // and any extra options
    // are local to this instance
    #[proto(oneof(tags(1, 2)), validate = |v| v.required())]
    pub oneof_2: Option<ReusedOneof>,
}

#[proto_oneof]
pub enum ReusedOneof {
    // Tags have to be defined explicitly for oneofs
    #[proto(tag = 1)]
    ThingA(i32),
    #[proto(tag = 2)]
    ThingB(u32)
}

let msg1 = Msg1::proto_schema().render_schema().unwrap();
let msg2 = Msg2::proto_schema().render_schema().unwrap();

assert_eq!(msg1, 
r"
message Msg1 {
  oneof oneof_1 {
    int32 thing_a = 1;
    uint32 thing_b = 2;
  }
}".trim_start());

// Notice how the oneof has a different name
// And an option that is local to it
assert_eq!(msg2, 
r"
message Msg2 {
  oneof oneof_2 {
    option (buf.validate.oneof).required = true;

    int32 thing_a = 1;
    uint32 thing_b = 2;
  }
}".trim_start());
```

# ⚠️ Avoiding Tag Mismatches

Since prost was not originally programmed to support reusable oneofs, it does not check if the tags used with it are wrong and would just produce silent errors at runtime. In order to avoid this, all tags for oneofs must be defined explicitly (unlike tags for other fields), and furthermore, the `#[proto_message]` macro will automatically generate a test that checks if the tags specified on the oneof as a field match the oneof's tags and panics on failure.
