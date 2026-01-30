use crate::*;
use hashbrown::HashSet;

/// Struct that represents a protobuf file and its contents.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", derive(Template))]
#[cfg_attr(feature = "std", template(path = "file.proto.j2"))]
pub struct ProtoFile {
  pub name: FixedStr,
  pub package: FixedStr,
  pub imports: FileImports,
  pub messages: Vec<MessageSchema>,
  pub enums: Vec<EnumSchema>,
  pub options: Vec<ProtoOption>,
  pub edition: Edition,
  pub services: Vec<Service>,
  pub extensions: Vec<Extension>,
}

#[doc(hidden)]
pub struct FileReference {
  pub name: &'static str,
  pub package: &'static str,
  pub extern_path: &'static str,
}

/// The protobuf edition for a file.
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Edition {
  #[default]
  Proto3,
  E2023,
}

impl Display for Edition {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::Proto3 => write!(f, "syntax = \"proto3\""),
      Self::E2023 => write!(f, "edition = \"2023\""),
    }
  }
}

/// HashSet wrapper for a file's imports. Skips insertion if the file is equal to the origin file.
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileImports {
  pub set: HashSet<FixedStr>,
  pub file: FixedStr,
  pub added_validate_proto: bool,
}

impl Extend<FixedStr> for FileImports {
  fn extend<T: IntoIterator<Item = FixedStr>>(&mut self, iter: T) {
    self.set.extend(iter);
  }
}

impl IntoIterator for FileImports {
  type Item = FixedStr;
  type IntoIter = hashbrown::hash_set::IntoIter<FixedStr>;

  fn into_iter(self) -> Self::IntoIter {
    self.set.into_iter()
  }
}

impl FileImports {
  /// Creates a new instance.
  #[must_use]
  pub fn new(file: impl Into<FixedStr>) -> Self {
    Self {
      file: file.into(),
      set: HashSet::default(),
      added_validate_proto: false,
    }
  }

  pub(crate) fn insert_validate_proto(&mut self) {
    if !self.added_validate_proto {
      self
        .set
        .insert("buf/validate/validate.proto".into());
      self.added_validate_proto = true;
    }
  }

  pub(crate) fn insert_internal<S>(&mut self, import: S)
  where
    S: AsRef<str> + Into<FixedStr>,
  {
    let import_str = import.as_ref();

    if *import_str != *self.file {
      if import_str == "buf/validate/validate.proto" {
        self.insert_validate_proto();
      } else {
        self.set.insert(import.into());
      }
    }
  }

  pub fn insert<S>(&mut self, import: S)
  where
    S: AsRef<str> + Into<FixedStr>,
  {
    if self.file != import.as_ref() {
      self.set.insert(import.into());
    }
  }

  pub fn insert_from_path(&mut self, path: &ProtoPath) {
    if path.file != self.file {
      self.set.insert(path.file.clone());
    }
  }

  #[must_use]
  pub fn as_sorted_vec(&self) -> Vec<FixedStr> {
    let mut imports: Vec<FixedStr> = self.set.iter().cloned().collect();

    imports.sort_unstable();

    imports
  }
}

impl ProtoFile {
  #[must_use]
  pub fn new(name: &'static str, package: &'static str) -> Self {
    Self {
      name: name.into(),
      package: package.into(),
      imports: FileImports::new(name),
      messages: Default::default(),
      enums: Default::default(),
      options: Default::default(),
      edition: Default::default(),
      services: Default::default(),
      extensions: Default::default(),
    }
  }

  pub fn with_options(&mut self, options: impl IntoIterator<Item = ProtoOption>) -> &mut Self {
    self.options.extend(options);
    self
  }

  pub fn with_imports(
    &mut self,
    imports: impl IntoIterator<Item = impl Into<FixedStr>>,
  ) -> &mut Self {
    self
      .imports
      .extend(imports.into_iter().map(|s| s.into()));
    self
  }

  pub const fn with_edition(&mut self, edition: Edition) -> &mut Self {
    self.edition = edition;
    self
  }

  #[track_caller]
  pub fn merge_with(&mut self, other: Self) -> &mut Self {
    if self.name != other.name {
      panic!(
        "Cannot merge file `{}` with file `{}` as they have different names",
        self.name, other.name
      );
    }

    if self.package != other.package {
      panic!(
        "Cannot merge file `{}` with file `{}` as they belong to different packages",
        self.name, other.name
      );
    }

    self.imports.extend(other.imports);
    self.messages.extend(other.messages);
    self.enums.extend(other.enums);
    self.services.extend(other.services);
    self.extensions.extend(other.extensions);
    self.options.extend(other.options);

    self
  }

  pub fn with_messages<I: IntoIterator<Item = MessageSchema>>(&mut self, messages: I) -> &mut Self {
    for mut message in messages {
      message.register_imports(&mut self.imports);
      message.file = self.name.clone();

      self.messages.push(message);
    }

    self
  }

  pub fn with_enums<I: IntoIterator<Item = EnumSchema>>(&mut self, enums: I) -> &mut Self {
    for mut enum_ in enums {
      enum_.file = self.name.clone();

      self.enums.push(enum_);
    }

    self
  }

  pub fn with_services<I: IntoIterator<Item = Service>>(&mut self, services: I) -> &mut Self {
    for service in services {
      for (request, response) in service
        .handlers
        .iter()
        .map(|h| (&h.request, &h.response))
      {
        self.imports.insert_from_path(request);
        self.imports.insert_from_path(response);
      }

      if *service.file != *self.name {
        self.imports.set.insert(service.file.clone());
      }

      self.services.push(service);
    }

    self
  }

  pub fn with_extensions<I: IntoIterator<Item = Extension>>(&mut self, extensions: I) -> &mut Self {
    self
      .imports
      .set
      .insert("google/protobuf/descriptor.proto".into());

    for ext in extensions {
      self.extensions.push(ext);
    }

    self
  }
}
