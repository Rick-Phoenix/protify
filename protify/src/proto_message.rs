use crate::{validators::CelRule, *};

/// Implemented for proxied messages by the [`proto_message`] macro.
pub trait ProxiedMessage: From<Self::Proxy> + Into<Self::Proxy> {
  type Proxy: MessageProxy<Message = Self> + From<Self> + Into<Self>;

  #[inline]
  fn into_proxy(self) -> Self::Proxy {
    self.into()
  }
}

/// Implemented for message proxies by the [`proto_message`] macro.
pub trait MessageProxy: From<Self::Message> + Into<Self::Message> {
  type Message: ProtoMessage + ValidatedMessage + From<Self> + Into<Self>;

  #[inline]
  fn into_message(self) -> Self::Message {
    self.into()
  }

  /// Consumes the proxy and converts into the related message, and then validates the result.
  #[inline]
  fn into_validated_message(self) -> Result<Self::Message, ValidationErrors> {
    let msg = self.into_message();

    match msg.validate() {
      Ok(()) => Ok(msg),
      Err(e) => Err(e),
    }
  }

  /// Validates the message, and converts to the proxy if the validation is successful.
  #[inline]
  fn from_validated_message(msg: Self::Message) -> Result<Self, ValidationErrors> {
    match msg.validate() {
      Ok(()) => Ok(Self::from(msg)),
      Err(e) => Err(e),
    }
  }
}

/// A trait that returns the protobuf path of a given message.
///
/// It must be implemented for the targets of the enums annotated with the [`proto_service`] macro.
pub trait MessagePath {
  fn proto_path() -> ProtoPath;
}

/// A trait that is responsible for generating a protobuf schema representation for a rust struct.
///
/// Implemented by the [`proto_message`] macro.
pub trait ProtoMessage: Default + MessagePath {
  /// The package to which this message belongs.
  const PACKAGE: &str;
  /// The short name of the message. It excludes the names of the parent message and package.
  const SHORT_NAME: &str;

  /// Returns the protobuf schema representation.
  fn proto_schema() -> MessageSchema;

  /// Returns the name of the message, with the name of the parent message if there is one (i.e. "ParentMessage.ChildMessage").
  fn proto_name() -> &'static str;
  /// Returns the full name of the message, which includes the name of the ancestor messages (if there are any) and the package to which it belongs (i.e. "package.Parent.Child").
  fn full_name() -> &'static str;
  /// Returns the type url for this type, which is the full name preceded by a '/'.
  fn type_url() -> &'static str;
}

/// A struct that represents a protobuf message.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Template))]
#[cfg_attr(feature = "std", template(path = "message.proto.j2"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageSchema {
  pub short_name: FixedStr,
  pub name: FixedStr,
  pub package: FixedStr,
  pub file: FixedStr,
  pub entries: Vec<MessageEntry>,
  pub messages: Vec<Self>,
  pub enums: Vec<EnumSchema>,
  pub options: Vec<ProtoOption>,
  pub reserved_names: Vec<FixedStr>,
  pub reserved_numbers: Vec<Range<i32>>,
  pub validators: Vec<ValidatorSchema>,
  pub rust_path: FixedStr,
}

impl MessageSchema {
  #[cfg(feature = "std")]
  /// Renders the message in its protobuf representation.
  pub fn render_schema(&self) -> Result<String, askama::Error> {
    self.render()
  }

  pub(crate) fn options_with_validators(&self) -> Vec<ProtoOption> {
    self
      .options
      .clone()
      .into_iter()
      .chain(self.validators.iter().map(|v| v.schema.clone()))
      .collect()
  }

  /// Returns an iterator that flattens the fields of the message and those contained in its oneofs.
  pub fn fields(&self) -> impl Iterator<Item = &Field> {
    self.entries.iter().flat_map(|entry| {
      let (field_opt, oneof_vec) = match entry {
        MessageEntry::Field(f) => (Some(f), None),
        MessageEntry::Oneof(oneof) => (None, Some(&oneof.fields)),
      };

      field_opt
        .into_iter()
        .chain(oneof_vec.into_iter().flatten())
    })
  }

  /// Adds one or more nested [`MessageSchema`]s to this message.
  #[must_use]
  pub fn with_nested_messages(mut self, messages: impl IntoIterator<Item = Self>) -> Self {
    self.messages.extend(messages);
    self
  }

  /// Adds one or more nested [`EnumSchema`]s to this message.
  #[must_use]
  pub fn with_nested_enums(mut self, enums: impl IntoIterator<Item = EnumSchema>) -> Self {
    self.enums.extend(enums);
    self
  }
}

/// An entry in a message, which can be a field or a oneof containing several fields.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageEntry {
  Field(Field),
  Oneof(Oneof),
}

impl MessageEntry {
  pub(crate) fn cel_rules(self) -> impl Iterator<Item = CelRule> {
    let (field_opt, oneof_vec) = match self {
      Self::Field(f) => (Some(f), None),
      Self::Oneof(oneof) => (None, Some(oneof.fields)),
    };

    field_opt
      .into_iter()
      .chain(oneof_vec.into_iter().flatten())
      .flat_map(|f| f.validators)
      .flat_map(|v| v.cel_rules)
  }

  /// Returns `true` if the message entry is [`Field`].
  ///
  /// [`Field`]: MessageEntry::Field
  #[must_use]
  pub const fn is_field(&self) -> bool {
    matches!(self, Self::Field(..))
  }

  /// Returns `true` if the message entry is [`Oneof`].
  ///
  /// [`Oneof`]: MessageEntry::Oneof
  #[must_use]
  pub const fn is_oneof(&self) -> bool {
    matches!(self, Self::Oneof { .. })
  }

  /// Attempts to match the enum to a [`Field`] instance.
  #[must_use]
  pub const fn as_field(&self) -> Option<&Field> {
    if let Self::Field(v) = self {
      Some(v)
    } else {
      None
    }
  }

  /// Attempts to match the enum to a [`Oneof`] instance.
  #[must_use]
  pub const fn as_oneof(&self) -> Option<&Oneof> {
    if let Self::Oneof(v) = self {
      Some(v)
    } else {
      None
    }
  }
}

impl MessageSchema {
  pub(crate) fn render_reserved_names(&self) -> Option<String> {
    render_reserved_names(&self.reserved_names)
  }

  pub(crate) fn render_reserved_numbers(&self) -> Option<String> {
    render_reserved_numbers(&self.reserved_numbers)
  }

  pub(crate) fn register_imports(&self, imports: &mut FileImports) {
    for import in self
      .validators
      .iter()
      .flat_map(|v| v.imports.clone())
    {
      imports.insert_internal(import);
    }

    for entry in &self.entries {
      match entry {
        MessageEntry::Field(field) => field.register_import_path(imports),
        MessageEntry::Oneof(oneof) => {
          for field in &oneof.fields {
            field.register_import_path(imports)
          }
        }
      }
    }

    for nested_msg in &self.messages {
      nested_msg.register_imports(imports);
    }
  }
}
