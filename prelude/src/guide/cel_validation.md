# Cel Validation

The `cel` feature unlocks validation via [`CelProgram`](crate::CelProgram).


If a CEL expressions fails to compile, **it will cause a panic**, so it is strongly advised to test each expression to avoid surprises at runtime. 

Unless manually disabled, these checks are automatically performed in the macros' output, as explained in the [`correctness`](crate::guide::correctness) section.

## Reusing Programs

Each [`CelProgram`](crate::CelProgram) instance wraps a CEL expression that is compiled and initialized once, and reused after that.
The wrapper itself holds an [`Arc`](std::sync::Arc) and is therefore cheap to clone.

If a program is used in different places, it's a good idea to initialize it once and then clone it.

This crate exports a Lazy struct that can be used for this purpose, which is a type alias for [`LazyLock`](std::sync::LazyLock) when the `std` feature is enabled, and otherwise implements a wrapper for [`OnceBox`](once_cell::race::OnceBox).

### Example

```rust
use prelude::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

// The program is compiled once and then wrapped in an Arc, so it can be cheaply cloned
static REUSABLE_PROGRAM: Lazy<CelProgram> = Lazy::new(|| {
    cel_program!(id = "my_prog", msg = "error_message", expr = "this.location == 'Isengard'")
});

#[proto_message]
pub struct TheyAreTakingTheHobbitsTo {
    pub location: String
}

#[proto_message]
pub struct ABalrogOfMorgoth {
    // Here we are just cloning an Arc pointer to the lazily initialized program,
    // not the program itself
    #[proto(message, validate = |v| v.cel(REUSABLE_PROGRAM.clone()))]
    pub what_did_you_say: Option<TheyAreTakingTheHobbitsTo>
}

#[proto_message]
pub struct TellMeWhereIsGandalf {
    #[proto(message, validate = |v| v.cel(REUSABLE_PROGRAM.clone()))]
    pub for_i_much_desire_to_speak_with_him: Option<TheyAreTakingTheHobbitsTo>
}
```

# Enforcing Unique IDs

The instances of [`Package`] provide methods with which to ensure that there aren't rules with the same ID in the same message scope. Refer to the [`correctness`](crate::guide::correctness) section or the [`proto_package`](crate::proto_package) documentation for more details about that.
