use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct ProtoFile {
  pub name: &'static str,
  pub package: &'static str,
  pub imports: Vec<&'static str>,
  pub messages: Vec<Message>,
  pub enums: Vec<Enum>,
  // pub services: Vec<ServiceData>,
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
