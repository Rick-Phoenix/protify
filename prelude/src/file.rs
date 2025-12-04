use std::fmt::Display;

use crate::*;

#[derive(Default, Debug, PartialEq, Template)]
#[template(path = "file.proto.j2")]
pub struct ProtoFile {
  pub name: &'static str,
  pub package: &'static str,
  pub imports: Vec<&'static str>,
  pub messages: Vec<Message>,
  pub enums: Vec<Enum>,
  pub options: Vec<ProtoOption>,
  pub edition: Edition,
  pub services: Vec<Service>,
  pub extensions: Vec<Extension>,
}

#[derive(Default, Debug, PartialEq)]
pub enum Edition {
  Proto2,
  #[default]
  Proto3,
  E2023,
}

impl Display for Edition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Edition::Proto2 => write!(f, "syntax = \"proto2\""),
      Edition::Proto3 => write!(f, "syntax = \"proto3\""),
      Edition::E2023 => write!(f, "edition = \"2023\""),
    }
  }
}

pub(crate) struct FileImports {
  pub imports: HashSet<&'static str>,
  pub file: &'static str,
}

impl FileImports {
  pub fn new(file: &'static str) -> Self {
    Self {
      file,
      imports: HashSet::new(),
    }
  }

  pub fn insert(&mut self, path: &ProtoPath) {
    if path.file != self.file {
      self.imports.insert(path.file);
    }
  }

  pub fn into_sorted_vec(self) -> Vec<&'static str> {
    let mut imports: Vec<&'static str> = self.imports.into_iter().collect();

    imports.sort();

    imports
  }
}

impl ProtoFile {
  pub fn new(name: &'static str, package: &'static str) -> Self {
    Self {
      name,
      package,
      ..Default::default()
    }
  }

  pub fn merge_with(&mut self, other: Self) {
    self.imports.extend(other.imports);
    self.messages.extend(other.messages);
    self.enums.extend(other.enums);
  }

  pub fn add_messages<I: IntoIterator<Item = Message>>(&mut self, messages: I) {
    let mut file_imports = FileImports::new(self.name);

    for message in messages.into_iter() {
      message.register_imports(&mut file_imports);

      self.messages.push(message);
    }

    self.imports = file_imports.into_sorted_vec();
  }

  pub fn add_enums<I: IntoIterator<Item = Enum>>(&mut self, enums: I) {
    for enum_ in enums.into_iter() {
      self.enums.push(enum_);
    }
  }
}

impl ProtoFile {
  pub(crate) fn render_options(&self) -> Option<String> {
    if self.options.is_empty() {
      return None;
    }

    let mut options_str = String::new();

    for option in &self.options {
      render_option(option, &mut options_str, OptionKind::NormalOption);
      options_str.push('\n');
    }

    Some(options_str)
  }
}

#[derive(Debug, PartialEq)]
pub struct Extension {
  pub name: &'static str,
  pub fields: Vec<ProtoField>,
}
