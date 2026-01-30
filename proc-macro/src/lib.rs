#![allow(
  clippy::single_match,
  clippy::collapsible_if,
  clippy::collapsible_else_if
)]

use std::{
  borrow::Cow,
  fmt::Display,
  ops::{Deref, Range},
};

use attributes::*;
use bool_enum::bool_enum;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{
  Attribute, Error, Expr, Field, Fields, Ident, ItemEnum, ItemStruct, Lit, LitBool, LitStr, Meta,
  Path, RangeLimits, Token, Type, Variant, Visibility, bracketed,
  meta::ParseNestedMeta,
  parse::{Parse, Parser},
  parse_macro_input, parse_quote, parse_quote_spanned,
  spanned::Spanned,
  token,
};
use syn_utils::*;

use crate::{
  enum_proc_macro::*, extension_derive::*, file_macro::*, impls::*, item_cloners::*,
  message_proc_macro::*, oneof_proc_macro::*, package_macro::*, path_utils::*, proto_field::*,
  proto_map::*, proto_types::*, service_derive::*,
};

mod attributes;
mod builder_macro;
#[cfg(feature = "cel")]
mod cel_try_into;
#[cfg(feature = "reflection")]
mod enum_derive;
mod enum_proc_macro;
mod extension_derive;
mod field_data;
mod file_macro;
mod impls;
mod item_cloners;
mod message_proc_macro;
mod message_schema_impl;
mod oneof_proc_macro;
mod oneof_schema_impl;
mod package_macro;
mod path_utils;
mod proto_field;
mod proto_map;
mod proto_types;
#[cfg(feature = "reflection")]
mod reflection;
mod service_derive;
mod well_known_type_impl;

#[doc(hidden)]
#[proc_macro_derive(__AttrForwarding, attributes(forward))]
pub fn attr_forwarding_derive_test(_: TokenStream) -> TokenStream {
  TokenStream::new()
}

/// Implements the [`CelOneof`](prelude::CelOneof) trait on an enum. Automatically implemented
/// by the [`proto_oneof`](prelude::proto_oneof) macro when the `cel` feature is enabled.
#[cfg(feature = "cel")]
#[proc_macro_derive(CelOneof, attributes(cel))]
pub fn cel_oneof_derive(input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as ItemEnum);

  match cel_try_into::derive_cel_value_oneof(&item) {
    Ok(tokens) => tokens.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

/// Implements the [`CelValue`](prelude::CelValue) trait on a struct. Automatically
/// implemented by the [`proto_message`](prelude::proto_message) macro when the `cel` feature is enabled.
#[cfg(feature = "cel")]
#[proc_macro_derive(CelValue, attributes(cel))]
pub fn cel_struct_derive(input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as ItemStruct);

  match cel_try_into::derive_cel_value_struct(&item) {
    Ok(tokens) => tokens.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

#[doc(hidden)]
#[cfg(feature = "reflection")]
#[proc_macro_derive(ValidatedOneof, attributes(proto))]
pub fn validated_oneof_derive(input: TokenStream) -> TokenStream {
  let mut item = parse_macro_input!(input as ItemEnum);

  reflection::reflection_oneof_derive(&mut item).into()
}

#[doc(hidden)]
#[cfg(feature = "reflection")]
#[proc_macro_derive(ProtoEnum, attributes(proto))]
pub fn enum_derive(input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as ItemEnum);

  enum_derive::named_enum_derive(&item).into()
}

#[doc(hidden)]
#[cfg(feature = "reflection")]
#[proc_macro_derive(ValidatedMessage, attributes(proto))]
pub fn validated_message_derive(input: TokenStream) -> TokenStream {
  let mut item = parse_macro_input!(input as ItemStruct);

  reflection::reflection_message_derive(&mut item).into()
}

#[doc(hidden)]
#[proc_macro]
pub fn impl_known_type(input: TokenStream) -> TokenStream {
  match well_known_type_impl::well_known_type_impl_macro(input.into()) {
    Ok(output) => output.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

#[doc(hidden)]
#[proc_macro]
pub fn builder_state_macro(input: TokenStream) -> TokenStream {
  match builder_macro::builder_macro(input.into()) {
    Ok(output) => output.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

/// Creates a new proto file schema, and brings it into scope for the items that are defined in the same module.
///
#[doc = include_str!("../docs/file_macro.md")]
///
/// # Examples
/// ```
/// use prelude::*;
/// proto_package!(MY_PKG, name = "my_pkg");
/// define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);
///
/// #[proto_message]
/// pub struct Msg {
///   pub id: i32
/// }
///
/// assert_eq!(Msg::proto_schema().file, "my_file.proto");
/// ```
#[proc_macro]
pub fn define_proto_file(input: TokenStream) -> TokenStream {
  match process_file_macro(input.into()) {
    Ok(output) => output.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

/// This macro can be used to define file schemas manually when the inventory feature is not available.
///
/// The `file_schema` macro accepts all the inputs of the `define_proto_file` macro, plus the list of messages, enums and services, which are just bracketed lists of paths for each element.
/// Nested messages and enums are defined by using `ParentMessage = { enums = [ NestedEnum ], messages = [ NestedMsg ] }` instead of just the message's name, as shown below.
///
/// # Example
///
/// ```
/// use prelude::*;
///
/// // This would be the no_std crate where you define your models...
/// mod imagine_this_is_the_models_crate {
///   use super::*;
///   
///   // The package and file handles are still needed here,
///   // but they do not collect the items automatically
///   // when the inventory feature is disabled, so we must
///   // create the schemas manually below...
///   proto_package!(MY_PKG, name = "my_pkg");
///   define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);
///   
///   #[proto_message]
///   pub struct Msg1 {
///     pub id: i32
///   }
///
///   #[proto_message]
///   #[proto(parent_message = Msg1)]
///   pub struct Nested {
///     pub id: i32
///   }
///
///   #[proto_message]
///   pub struct Msg2 {
///     pub id: i32
///   }
///
///   #[proto_enum]
///   pub enum Enum1 {
///     Unspecified, A, B
///   }
///
///   #[proto_enum]
///   #[proto(parent_message = Msg1)]
///   pub enum NestedEnum {
///     Unspecified, A, B
///   }
///
///
///   #[proto_service]
///   pub enum MyService {
///     GetMsg {
///       request: Msg1,
///       response: Msg2
///     }
///   }
/// }
///
/// // From an external utility crate, or the build.rs file of the consuming crate:
/// fn main() {
///   use imagine_this_is_the_models_crate::*;
///
///   let manual_file = file_schema!(
///     name = "test.proto",
///     messages = [
///       Msg2,
///       Msg1 = { messages = [ Nested ], enums = [ NestedEnum ] }
///     ],
///     services = [ MyService ],
///     enums = [ Enum1 ],
///     // Imports, options, etc...
///   );
///
///   let manual_pkg = package_schema!("my_pkg", files = [ manual_file ]);
///   // Now we can use the package handle to create the files,
///   // access the `extern_path`s and so on...
/// }
/// ```
#[proc_macro]
pub fn file_schema(input: TokenStream) -> TokenStream {
  match schema_file_macro(input.into()) {
    Ok(output) => output.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

#[allow(clippy::doc_overindented_list_items)]
/// Creates a new package handle, which is used to collect the proto schemas in a crate.
///
/// The first parameter of the macro is the ident that will be used for the generated constant that will hold the package handle, which will be used to generate the package and its proto files.
///
///The other parameters are not positional and are as follows:
///
/// - `name` (required)
///   - Type: string
///   - Example: `proto_package!(MY_PKG, name = "my_pkg")`
///   - Description:
///       The name of the package.
///
///
///   - `no_cel_test`
///   - Type: Ident
///   - Example: `proto_package!(MY_PKG, name = "my_pkg", no_cel_test)`
///   - Description:
///       By default, the macro will automatically generate a test that will check for collisions of CEL rules with the same ID within the same message. You can use this ident to disable this behaviour. The [`check_unique_cel_rules`](crate::Package::check_unique_cel_rules) method will still be available if you want to call it manually inside a test.
///
/// # Examples
/// ```
/// use prelude::*;
///
/// // If we want to skip the automatically generated
/// // Test for conflicting CEL rules in the same scope
/// proto_package!(WITHOUT_TEST, name = "without_test", no_cel_test);
///
/// // We create the package handle
/// proto_package!(MY_PKG, name = "my_pkg");
/// // And use it to assign newly defined files
/// define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);
///
/// ```
#[proc_macro]
pub fn proto_package(input: TokenStream) -> TokenStream {
  match package_macro_impl(input.into()) {
    Ok(output) => output.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

#[doc = include_str!("../docs/message_macro.md")]
///
/// # Field attributes
#[doc = include_str!("../docs/field_ref.md")]
#[proc_macro_attribute]
pub fn proto_message(args: TokenStream, input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as ItemStruct);

  message_proc_macro(item, args.into()).into()
}

#[doc(hidden)]
#[proc_macro_derive(Message, attributes(proto))]
pub fn message_derive(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[doc = include_str!("../docs/extension_macro.md")]
#[proc_macro_attribute]
pub fn proto_extension(args: TokenStream, input: TokenStream) -> TokenStream {
  let mut item = parse_macro_input!(input as ItemStruct);

  let extra_tokens = match process_extension_derive(args.into(), &mut item) {
    Ok(output) => output,
    Err(e) => e.to_compile_error(),
  };

  quote! {
    #[derive(::prelude::macros::Extension)]
    #item

    #extra_tokens
  }
  .into()
}

#[doc(hidden)]
#[proc_macro_derive(Extension, attributes(proto))]
pub fn extension_derive(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[doc = include_str!("../docs/service_macro.md")]
#[proc_macro_attribute]
pub fn proto_service(_args: TokenStream, input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as ItemEnum);

  let output = match process_service_derive(&item) {
    Ok(output) => output,
    Err(e) => return e.to_compile_error().into(),
  };

  output.into()
}

#[doc(hidden)]
#[proc_macro_derive(Service, attributes(proto))]
pub fn service_derive(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[doc = include_str!("../docs/enum_macro.md")]
#[proc_macro_attribute]
pub fn proto_enum(_args: TokenStream, input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as ItemEnum);

  enum_proc_macro(item).into()
}

#[doc(hidden)]
#[proc_macro_derive(Enum, attributes(proto))]
pub fn enum_empty_derive(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[doc = include_str!("../docs/oneof_macro.md")]
///
/// # Variant attributes
#[doc = include_str!("../docs/field_ref.md")]
#[proc_macro_attribute]
pub fn proto_oneof(args: TokenStream, input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as ItemEnum);

  process_oneof_proc_macro(item, args.into()).into()
}

#[doc(hidden)]
#[proc_macro_derive(Oneof, attributes(proto))]
pub fn oneof_derive(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}
