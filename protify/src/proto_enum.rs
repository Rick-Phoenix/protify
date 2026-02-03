use crate::*;

/// Trait for enums with a protobuf representation.
///
/// It provides some methods that can be useful when converting to/from integers.
///
/// Implemented by the [`proto_enum`] macro.
pub trait ProtoEnum: TryFrom<i32> + Copy + Default + Into<i32> + Send + Sync {
  /// Returns the name of the enum, preceded by the parent message, if there is one (i.e. "ParentMessage.EnumName").
  fn proto_name() -> &'static str;

  /// Turns the enum into its integer representation.
  #[inline]
  fn as_int(self) -> i32 {
    self.into()
  }

  /// Checks if the enum's integer representation is 0, which signifies the `UNSPECIFIED` variant in the protobuf convention.
  #[inline]
  fn is_unspecified(&self) -> bool {
    self.as_int() == 0
  }

  /// Attempts conversion from a plain integer, falling back to the variant 0 (unspecified) upon failure.
  #[inline]
  #[must_use]
  fn from_int_or_default(int: i32) -> Self {
    int.try_into().unwrap_or_default()
  }
}

/// Trait for enums with a protobuf representation.
///
/// It provides methods that enables the generation of the protobuf schema representation for an enum.
///
/// Implemented by the [`proto_enum`] macro.
pub trait ProtoEnumSchema: TryFrom<i32> + Default + ProtoEnum {
  fn proto_path() -> ProtoPath;
  fn proto_schema() -> EnumSchema;

  /// Converts the enum variant to its corresponding name in the protobuf representation.
  fn as_proto_name(&self) -> &'static str;
  /// Attempts conversion from the protobuf name of a given variant.
  fn from_proto_name(name: &str) -> Option<Self>;

  /// Checks if the received integer is among the known variants for this enum.
  #[inline]
  #[must_use]
  fn is_known_variant(int: i32) -> bool {
    Self::try_from(int).is_ok()
  }
}

/// A struct representing a protobuf enum.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Template))]
#[cfg_attr(feature = "std", template(path = "enum.proto.j2"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumSchema {
  pub short_name: FixedStr,
  pub name: FixedStr,
  pub package: FixedStr,
  pub file: FixedStr,
  pub variants: Vec<EnumVariant>,
  pub reserved_numbers: Vec<Range<i32>>,
  pub reserved_names: Vec<FixedStr>,
  pub options: Vec<ProtoOption>,
  pub rust_path: FixedStr,
}

/// A struct representing a protobuf enum variant.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumVariant {
  pub name: FixedStr,
  pub tag: i32,
  pub options: Vec<ProtoOption>,
}

impl EnumSchema {
  /// Renders the protobuf representation.
  #[cfg(feature = "std")]
  pub fn render_schema(&self) -> Result<String, askama::Error> {
    self.render()
  }

  pub(crate) fn render_reserved_names(&self) -> Option<String> {
    render_reserved_names(&self.reserved_names)
  }

  pub(crate) fn render_reserved_numbers(&self) -> Option<String> {
    render_reserved_numbers(&self.reserved_numbers)
  }
}
