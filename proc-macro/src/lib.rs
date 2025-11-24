#[macro_use]
mod macros;

use std::{collections::HashMap, ops::Range, rc::Rc};

use attributes::*;
pub(crate) use convert_case::ccase;
use proc_macro::TokenStream;
use proc_macro2::Span;
pub(crate) use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
  parse::Parse,
  parse_macro_input, parse_quote,
  punctuated::Punctuated,
  token,
  token::{Brace, Paren, Semi, Struct},
  Attribute, Data, DeriveInput, Error, Expr, ExprClosure, Field, Fields, FieldsUnnamed, Generics,
  Ident, Item, ItemEnum, ItemFn, ItemMod, ItemStruct, Lit, LitStr, Meta, Path, RangeLimits, Token,
  Type, Variant, Visibility,
};
use type_extraction::*;

use crate::{enum_derive::*, message_derive::*, oneof_derive::*};

mod enum_derive;
mod message_derive;
mod oneof_derive;
mod type_extraction;

mod attributes;

#[proc_macro_derive(Message, attributes(proto))]
pub fn message_derive(input: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[proc_macro_derive(Enum, attributes(proto))]
pub fn enum_derive(input: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[proc_macro_derive(Oneof, attributes(proto))]
pub fn oneof_derive(input: TokenStream) -> TokenStream {
  TokenStream::new()
}

#[proc_macro_attribute]
pub fn proto_module(attrs: TokenStream, input: TokenStream) -> TokenStream {
  let module = parse_macro_input!(input as ItemMod);

  let module_attrs = parse_macro_input!(attrs as ModuleAttrs);

  match process_module_items(module_attrs, module) {
    Ok(module) => quote! { #module }.into(),
    Err(e) => e.to_compile_error().into(),
  }
}
