//! # Guide
//!
//! This is the guide for `protify`.
//!
//!
//! ## Setup
//! - [Setup with full schema features](crate::guide::package_setup)
//! - [Setup with pre-built protos (validation only)](crate::guide::reflection_usage)
//! - [Managing files](crate::guide::managing_files)
//!	## Features
//!	- [Validators](crate::guide::validators)
//!	- [Proxies](crate::guide::proxies)
//!	- [Reusing oneofs](crate::guide::reusing_oneofs)
//!	- [Correctness](crate::guide::correctness)
//!	- [CEL validation](crate::guide::cel_validation)
//!	## Usage Examples
//!	- [Database mapping](crate::guide::database_mapping)
//!	- [Usage with tonic](crate::guide::usage_with_tonic)
//!	## Things To Know
//!	- [Tips](crate::guide::tips)
//!	- [Code elimination](crate::guide::code_elimination)
//!	- [Limitations](crate::guide::limitations)
//!	- [Roadmap](crate::guide::roadmap)

macro_rules! doc_mod {
  ($name:ident) => {
    #[doc = include_str!(concat!(stringify!($name), ".md"))]
    pub mod $name {}
  };
}

/// # Maximizing Code Elimination
///  
/// If you were to use `cargo expand` on the [`proto_message`](crate::proto_message) or [`proto_oneof`](crate::proto_oneof), you'll realize that the [`ProtoValidation`](crate::ProtoValidation), [`ValidatedMessage`](crate::ValidatedMessage) and [`ValidatedOneof`](crate::ValidatedOneof) traits use associated constants to check if oneofs and messages have validators in them or not.
///  
/// This is done with the intent of maximizing code elimination, so that the `.validate()` method will be completely eliminated by the compiler if a oneof/message has no validators, and any call to it will also be eliminated.
///  
/// This is very important because it allows us to implement these traits freely without hurting performance or binary size, and it also means that we can make blanket calls to .validate() for all incoming messages, because if some of those messages do not have validators, that call will be a no-op and it will be eliminated by the compiler entirely, rather than needlessly setting up stack frames.
///  
/// The crate has a dedicated test to guard against regressions in this area.
///
/// ```rust
#[doc = include_str!("../../tests/code_elimination.rs")]
/// ```
pub mod code_elimination {}

doc_mod!(reusing_oneofs);
doc_mod!(usage_with_tonic);
doc_mod!(reflection_usage);
doc_mod!(validators);
doc_mod!(correctness);
doc_mod!(proxies);
doc_mod!(package_setup);
doc_mod!(cel_validation);
doc_mod!(roadmap);
doc_mod!(limitations);
doc_mod!(tips);
doc_mod!(managing_files);
doc_mod!(database_mapping);
