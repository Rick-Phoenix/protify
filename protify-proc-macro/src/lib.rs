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
  enum_proc_macro::*, extension_macro::*, file_macro::*, impls::*, item_cloners::*,
  message_proc_macro::*, oneof_proc_macro::*, package_macro::*, path_utils::*, proto_field::*,
  proto_map::*, proto_types::*, service_macro::*,
};

mod attributes;
mod builder_macro;
#[cfg(feature = "cel")]
mod cel_try_into;
mod enum_proc_macro;
mod extension_macro;
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
mod service_macro;
mod well_known_type_impl;

#[doc(hidden)]
#[proc_macro_derive(__AttrForwarding, attributes(forward))]
pub fn attr_forwarding_derive_test(_: TokenStream) -> TokenStream {
  TokenStream::new()
}

/// Implements the [`CelOneof`](protify::CelOneof) trait on an enum. Automatically implemented
/// by the [`proto_oneof`](protify::proto_oneof) macro when the `cel` feature is enabled.
#[cfg(feature = "cel")]
#[proc_macro_derive(CelOneof, attributes(cel))]
pub fn cel_oneof_derive(input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as ItemEnum);

  match cel_try_into::derive_cel_value_oneof(&item) {
    Ok(tokens) => tokens.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

/// Implements the [`CelValue`](protify::CelValue) trait on a struct. Automatically
/// implemented by the [`proto_message`](protify::proto_message) macro when the `cel` feature is enabled.
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

  reflection::enum_reflection_derive(&item).into()
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

#[doc = include_str!("../docs/file_macro.md")]
#[proc_macro]
pub fn define_proto_file(input: TokenStream) -> TokenStream {
  match process_file_macro(input.into()) {
    Ok(output) => output.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

#[allow(clippy::doc_overindented_list_items)]
/// Creates a new package handle, which is used to collect the proto schemas in a crate.
///
/// For a comprehensive guide of how to set up a package, visit the [`package setup`](protify::guide::package_setup) section.
///
/// The **first parameter** of the macro is the ident that will be used for the generated constant that will hold the package handle, which will be used to generate the package and its proto files.
///
/// To distinguish the package handle from a normal rust type, it is advised to use SCREAMING_CASE casing.
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
///       By default, if the `cel` feature is enabled, the macro will automatically generate a test that will check for collisions of CEL rules with the same ID within the same message. You can use this ident to disable this behaviour. The [`check_unique_cel_rules`](crate::Package::check_unique_cel_rules) method will still be available if you want to call it manually inside a test.
///
/// As explained in the [`package setup`](protify::guide::package_setup) section, when the `inventory` feature is disabled, the files of a given package must be added manually, inside a bracketed list.
///
/// # Examples
/// ```
/// use protify::*;
///
/// // If we want to skip the automatically generated
/// // Test for conflicting CEL rules in the same scope
/// proto_package!(WITHOUT_TEST, name = "without_test", no_cel_test);
///
/// // If the `inventory` feature is disabled, we add the files manually
/// proto_package!(MY_PKG, name = "my_pkg", files = [ MY_FILE ]);
/// define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);
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
    #[derive(::protify::macros::Extension)]
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
