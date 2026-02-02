use crate::*;

/// A struct representing a protobuf Extension.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Extension {
  pub target: ExtensionTarget,
  pub fields: Vec<Field>,
}

/// Implemented by the [`proto_extension`] macro.
pub trait ProtoExtension {
  fn proto_schema() -> Extension;
}

/// Valid extension targets for a Proto3 file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExtensionTarget {
  FileOptions,
  MessageOptions,
  FieldOptions,
  OneofOptions,
  EnumOptions,
  EnumValueOptions,
  ServiceOptions,
  MethodOptions,
}

impl ExtensionTarget {
  /// Returns the string representation.
  #[must_use]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::FileOptions => "google.protobuf.FileOptions",
      Self::MessageOptions => "google.protobuf.MessageOptions",
      Self::FieldOptions => "google.protobuf.FieldOptions",
      Self::OneofOptions => "google.protobuf.OneofOptions",
      Self::EnumOptions => "google.protobuf.EnumOptions",
      Self::EnumValueOptions => "google.protobuf.EnumValueOptions",
      Self::ServiceOptions => "google.protobuf.ServiceOptions",
      Self::MethodOptions => "google.protobuf.MethodOptions",
    }
  }
}

impl Display for ExtensionTarget {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}
