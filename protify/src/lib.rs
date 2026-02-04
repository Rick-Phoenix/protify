#![no_std]
#![deny(clippy::alloc_instead_of_core)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! Protify is a library that aims to vastly simplify working with protobuf in a rust project. It offers a rust-first approach in defining protobuf models, so that every element in a protobuf package (messages, enums, oneofs, services, extensions, files) can be fully defined in rust code, and then the respective proto files can be generated from it as a compilation artifact, rather than it being the other way around.
//!
//! Whereas working with protobuf can often feel like an "alien" experience in rust, as we have to interact with structs and enums that are locked away in an included file outside of our reach and control, this crate aims to provide an experience that feels almost as simple and native as using `serde`.
//!
//! >ℹ️ **NOTE**: This readme is generated from the rust documentation. Read this in docs.rs to enable all links.
//!
#![doc = include_str!("./guide/schema_features.md")]
//!
//! For a full guide on how to set up a package, visit the [package setup](crate::guide::package_setup) section.
//!
#![doc = include_str!("./guide/proxies.md")]
//!
#![doc = include_str!("./guide/database_mapping.md")]
//!
#![doc = include_str!("./guide/validators.md")]
//!
//! # Feature Flags
//!
#![cfg_attr(
		feature = "document-features",
		doc = ::document_features::document_features!()
)]

#[cfg(any(test, feature = "std"))]
extern crate std;

pub use ::bytes::Bytes;
pub use prost;

#[doc(hidden)]
pub use alloc::{
	boxed::Box, collections::BTreeMap, format, string::String, string::ToString, sync::Arc, vec,
	vec::Vec,
};

#[cfg(doc)]
pub mod guide;

#[doc(hidden)]
pub extern crate alloc;

#[cfg(feature = "cel")]
pub use ::cel;

#[macro_use]
mod decl_macros;

use alloc::{borrow::Cow, borrow::ToOwned, collections::BTreeSet};
use core::{
	borrow::Borrow,
	fmt::{Debug, Display, Write},
	hash::Hash,
	marker::{PhantomData, Sized},
	ops::Deref,
	ops::Range,
};

#[cfg(not(feature = "std"))]
use hashbrown::HashMap;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(feature = "std")]
use askama::Template;
use bool_enum::bool_enum;
use float_eq::{FloatEq, float_eq};
use ordered_float::{FloatCore, OrderedFloat};
use owo_colors::OwoColorize;
use paste::paste;
use protify_proc_macro::impl_known_type;
use thiserror::Error;

#[doc(hidden)]
#[cfg(feature = "inventory")]
pub use inventory;

pub use macros::*;
pub mod macros {
	#[cfg(feature = "cel")]
	pub use protify_proc_macro::{CelOneof, CelValue};

	#[cfg(feature = "reflection")]
	pub use protify_proc_macro::{ProtoEnum, ValidatedMessage, ValidatedOneof};

	pub use protify_proc_macro::{
		Enum, Extension, Message, Oneof, Service, define_proto_file, proto_enum, proto_extension,
		proto_message, proto_oneof, proto_package, proto_service,
	};
}

pub use proto_types;
pub use proto_types::protovalidate::{FieldPathElement, Violations};

mod oneof_schema;
#[doc(inline)]
pub use oneof_schema::*;

#[cfg(not(feature = "std"))]
use hashbrown::HashSet;
#[cfg(feature = "std")]
use std::collections::HashSet;

mod rendering_utils;
use rendering_utils::*;

#[doc(inline)]
pub use field::*;
mod field;

mod enum_schema;
#[doc(inline)]
pub use enum_schema::*;

mod file;
#[doc(inline)]
pub use file::*;

mod package;
#[doc(inline)]
pub use package::*;

mod types;
#[doc(inline)]
pub use types::*;

mod service;
#[doc(inline)]
pub use service::*;

mod test_utils;
#[doc(inline)]
pub use test_utils::*;

mod options;
#[doc(inline)]
pub use options::*;

mod well_known_types;

mod message_schema;
#[doc(inline)]
pub use message_schema::*;

pub mod validators;
pub use validators::*;

mod registry;
#[doc(hidden)]
pub use registry::*;

mod extension;
#[doc(inline)]
pub use extension::*;

#[cfg(feature = "serde")]
pub(crate) mod serde_impls;

#[cfg(not(feature = "std"))]
mod lazy;
#[cfg(not(feature = "std"))]
pub use lazy::Lazy;

#[cfg(feature = "std")]
pub use std::sync::LazyLock as Lazy;

#[doc(hidden)]
#[inline]
pub fn apply<I, O, F>(input: I, f: F) -> O
where
	F: FnOnce(I) -> O,
{
	f(input)
}

/// Helper trait to convert a type to [`Bytes`].
///
/// It retains `& 'static` when possible, otherwise takes ownership of the value.
pub trait IntoBytes {
	fn into_bytes(self) -> Bytes;
}

impl<const N: usize> IntoBytes for &'static [u8; N] {
	#[inline]
	fn into_bytes(self) -> Bytes {
		Bytes::from_static(self)
	}
}

impl IntoBytes for &'static [u8] {
	#[inline]
	fn into_bytes(self) -> Bytes {
		Bytes::from_static(self)
	}
}

impl IntoBytes for Bytes {
	#[inline]
	fn into_bytes(self) -> Bytes {
		self
	}
}

impl IntoBytes for &Bytes {
	#[inline]
	fn into_bytes(self) -> Bytes {
		self.clone()
	}
}

#[cfg(feature = "regex")]
pub use regex_impls::*;

#[cfg(feature = "regex")]
mod regex_impls {
	use regex::Regex;
	use regex::bytes::Regex as BytesRegex;

	/// Utility trait to create a [`Regex`].
	pub trait IntoRegex {
		fn into_regex(self) -> Regex;
	}

	impl IntoRegex for &str {
		#[track_caller]
		#[inline]
		fn into_regex(self) -> Regex {
			Regex::new(self).unwrap()
		}
	}

	impl IntoRegex for Regex {
		fn into_regex(self) -> Regex {
			self
		}
	}

	impl IntoRegex for &Regex {
		#[inline]
		fn into_regex(self) -> Regex {
			self.clone()
		}
	}

	/// Utility trait to create a [`Regex`](BytesRegex).
	pub trait IntoBytesRegex {
		fn into_regex(self) -> BytesRegex;
	}

	impl IntoBytesRegex for &str {
		#[track_caller]
		#[inline]
		fn into_regex(self) -> BytesRegex {
			BytesRegex::new(self).unwrap()
		}
	}

	impl IntoBytesRegex for BytesRegex {
		#[inline]
		fn into_regex(self) -> BytesRegex {
			self
		}
	}

	impl IntoBytesRegex for &BytesRegex {
		#[inline]
		fn into_regex(self) -> BytesRegex {
			self.clone()
		}
	}
}

#[doc(hidden)]
#[cfg(feature = "serde")]
pub trait MaybeSerde: serde::Serialize + serde::de::DeserializeOwned {}

#[cfg(feature = "serde")]
impl<T: serde::Serialize + serde::de::DeserializeOwned> MaybeSerde for T {}

#[cfg(not(feature = "serde"))]
pub trait MaybeSerde {}

#[cfg(not(feature = "serde"))]
impl<T> MaybeSerde for T {}

#[inline(never)]
#[cold]
#[doc(hidden)]
pub fn filter_validators(
	validators: impl IntoIterator<Item = Option<ValidatorSchema>>,
) -> impl IntoIterator<Item = ValidatorSchema> {
	validators.into_iter().flatten()
}

#[inline(never)]
#[cold]
#[doc(hidden)]
pub fn collect_validators(
	validators: impl IntoIterator<Item = Option<ValidatorSchema>>,
) -> Vec<ValidatorSchema> {
	validators.into_iter().flatten().collect()
}

#[doc(hidden)]
pub static DEFAULT_MESSAGE_VALIDATOR: Lazy<MessageValidator> =
	Lazy::new(|| MessageValidator::default());

#[doc(hidden)]
pub static DEFAULT_ONEOF_VALIDATOR: Lazy<OneofValidator> = Lazy::new(|| OneofValidator::default());

#[doc(hidden)]
pub static DEFAULT_REQUIRED_ONEOF_VALIDATOR: Lazy<OneofValidator> = Lazy::new(|| OneofValidator {
	required: true,
	error_message: None,
});

#[inline(never)]
#[cold]
#[doc(hidden)]
pub fn collect_options<I>(options: I, deprecated: bool) -> Vec<ProtoOption>
where
	I: IntoIterator<Item = ProtoOption>,
{
	let mut output: Vec<ProtoOption> = options.into_iter().collect();

	if deprecated {
		output.push(proto_deprecated());
	}

	output
}
