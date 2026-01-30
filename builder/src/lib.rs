#![doc = ::document_features::document_features!()]

use std::fmt::Write;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::{env, path::PathBuf};

use prost_build::Config;
use prost_reflect::{prost::Message as ProstMessage, prost_types::FileDescriptorSet};

/// Configures a [`DescriptorData`] instance to gather information from a protobuf descriptor,
/// which can be useful to apply attributes programmatically to the target items.
///
/// You can then collect the data while also setting up the validators with the [`set_up_validators`](DescriptorDataConfig::set_up_validators) method.
///
/// Should you wish to not collect any data at all, you can call the omonimous function [`set_up_validators`] exported to the root, which will set up the validators and return an empty [`DescriptorData`] instance.
#[derive(Default)]
pub struct DescriptorDataConfig {
  collect_oneofs_data: bool,
  collect_enums_data: bool,
  collect_messages_data: bool,
}

impl DescriptorDataConfig {
  /// Sets up the validators for the specified packages, while also collecting the data that was requested
  /// in the configuration. Refer to the omonimous [`set_up_validators`] function's documentation to learn more about the implementations provided by this method.
  pub fn set_up_validators(
    &self,
    config: &mut Config,
    files: &[impl AsRef<Path>],
    include_paths: &[impl AsRef<Path>],
    packages: &[&str],
  ) -> Result<DescriptorData, Box<dyn std::error::Error>> {
    set_up_validators_inner(self, config, files, include_paths, packages)
  }

  /// Creates a new instance.
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Collects data for all items: oneofs, enums and messages.
  #[must_use]
  pub const fn collect_all_data() -> Self {
    Self {
      collect_oneofs_data: true,
      collect_enums_data: true,
      collect_messages_data: true,
    }
  }

  /// Toggles collection of data for oneofs.
  #[must_use]
  pub const fn collect_oneofs_data(mut self) -> Self {
    self.collect_oneofs_data = true;
    self
  }

  /// Toggles collection of data for enums.
  #[must_use]
  pub const fn collect_enums_data(mut self) -> Self {
    self.collect_enums_data = true;
    self
  }

  /// Toggles collection of data for messages.
  #[must_use]
  pub const fn collect_messages_data(mut self) -> Self {
    self.collect_messages_data = true;
    self
  }
}

/// A struct holding collected data from a protobuf descriptor.
/// This data can then be used to selectively apply attributes programmatically
/// through the helpers from the prost [`Config`].
#[derive(Default)]
pub struct DescriptorData {
  pub oneofs: Vec<Oneof>,
  pub enums: Vec<Enum>,
  pub messages: Vec<Message>,
}

/// Data about a protobuf oneof.
pub struct Oneof {
  pub name: String,
  pub parent_message: String,
  pub package: String,
}

/// Data about a protobuf message.
pub struct Message {
  pub name: String,
  pub parent_message: Option<String>,
  pub package: String,
}

/// Data about a protobuf enum.
pub struct Enum {
  pub name: String,
  pub parent_message: Option<String>,
  pub package: String,
}

impl Message {
  /// Extracts the fully qualified name of the message, which includes the names of the ancestor messages (if there are any), as well as the parent package.
  #[must_use]
  pub fn full_name(&self) -> String {
    let Self {
      name,
      parent_message,
      package,
    } = self;

    let mut str = format!("{package}.");

    if let Some(parent) = parent_message {
      let _ = write!(str, "{parent}.{name}");
    } else {
      let _ = write!(str, "{name}");
    }

    str
  }
}

impl Enum {
  /// Extracts the fully qualified name of the enum, which includes the names of the ancestor messages (if there are any), as well as the parent package.
  #[must_use]
  pub fn full_name(&self) -> String {
    let Self {
      name,
      parent_message,
      package,
    } = self;

    let mut str = format!("{package}.");

    if let Some(parent) = parent_message {
      let _ = write!(str, "{parent}.{name}");
    } else {
      let _ = write!(str, "{name}");
    }

    str
  }
}

impl Oneof {
  /// Extracts the fully qualified name of the oneof, which includes the name of the parent message, as well as the parent package.
  #[must_use]
  pub fn full_name(&self) -> String {
    let Self {
      name,
      parent_message,
      package,
    } = self;

    format!("{package}.{parent_message}.{name}")
  }
}

fn full_ish_name<'a>(item: &'a str, package: &'a str) -> &'a str {
  item
    .strip_prefix(&format!("{package}."))
    .unwrap_or(item)
}

/// Sets up the validators defined via `protovalidate` annotations in the target packages, and returns an empty [`DescriptorData`] instance.
///
/// If you wish to collect data from the descriptor while setting up the validators, look into the [`DescriptorDataConfig`] struct.
///
/// All of the selected messages will implement [`ValidatedMessage`](crate::ValidatedMessage), the selected enums will impement [`ProtoEnum`](crate::ProtoEnum), and the selected oneofs will implement [`ValidatedOneof`](crate::ValidatedOneof), and all of them will impement [`ProtoValidation`](crate::ProtoValidation).
///
/// If the `cel` feature is enabled, [`CelValue`](crate::CelValue) and [`CelOneof`](crate::CelOneof) will also be implemented for messages and oneofs.
///
/// Just like the non-reflection-based version of this crate, this will also automatically generate a `check_validators` method on each message and oneof, as well as a test that automatically calls this method and panics on failure, in order to ensure that validators represent valid configurations.
///
/// Should you wish to disable these checks, you can disable them with the `skip_checks(validators)` attribute, which you can easily set up programmatically by taking advantage of the data collection performed by the [`DescriptorDataConfig`] struct.
pub fn set_up_validators(
  config: &mut Config,
  files: &[impl AsRef<Path>],
  include_paths: &[impl AsRef<Path>],
  packages: &[&str],
) -> Result<DescriptorData, Box<dyn std::error::Error>> {
  set_up_validators_inner(
    &DescriptorDataConfig::default(),
    config,
    files,
    include_paths,
    packages,
  )
}

fn set_up_validators_inner(
  desc_data_config: &DescriptorDataConfig,
  config: &mut Config,
  files: &[impl AsRef<Path>],
  include_paths: &[impl AsRef<Path>],
  packages: &[&str],
) -> Result<DescriptorData, Box<dyn std::error::Error>> {
  let out_dir = env::var("OUT_DIR")
    .map(PathBuf::from)
    .unwrap_or(env::temp_dir());

  config
    .extern_path(".google.protobuf", "::prelude::proto_types")
    .extern_path(".buf.validate", "::prelude::proto_types::protovalidate")
    .compile_well_known_types();

  let temp_descriptor_path = out_dir.join("__temp_file_descriptor_set.bin");
  {
    let mut temp_config = prost_build::Config::new();
    temp_config.file_descriptor_set_path(&temp_descriptor_path);
    temp_config.out_dir(&out_dir);
    temp_config.compile_protos(files, include_paths)?;
  }

  let mut fds_file = std::fs::File::open(&temp_descriptor_path)?;
  let mut fds_bytes = Vec::new();
  fds_file.read_to_end(&mut fds_bytes)?;
  let fds = FileDescriptorSet::decode(fds_bytes.as_slice())?;
  let pool = prost_reflect::DescriptorPool::from_file_descriptor_set(fds)?;

  let mut desc_data = DescriptorData::default();

  for message_desc in pool.all_messages() {
    let package = message_desc.package_name();

    if packages.contains(&package) {
      let message_name = message_desc.full_name();

      if desc_data_config.collect_messages_data {
        desc_data.messages.push(Message {
          name: message_desc.name().to_string(),
          parent_message: message_desc
            .parent_message()
            .map(|p| full_ish_name(p.full_name(), package).to_string()),
          package: package.to_string(),
        });
      }

      config.message_attribute(message_name, "#[derive(::prelude::ValidatedMessage)]");
      #[cfg(feature = "cel")]
      {
        config.message_attribute(message_name, "#[derive(::prelude::CelValue)]");
      }
      config.message_attribute(
        message_name,
        format!(r#"#[proto(name = "{message_name}")]"#),
      );

      for oneof in message_desc.oneofs() {
        let parent_message = oneof.parent_message().full_name();

        if desc_data_config.collect_oneofs_data {
          desc_data.oneofs.push(Oneof {
            name: oneof.name().to_string(),
            parent_message: full_ish_name(parent_message, package).to_string(),
            package: package.to_string(),
          });
        }

        config.enum_attribute(oneof.full_name(), "#[derive(::prelude::ValidatedOneof)]");
        #[cfg(feature = "cel")]
        {
          config.enum_attribute(oneof.full_name(), "#[derive(::prelude::CelOneof)]");
        }
        config.enum_attribute(
          oneof.full_name(),
          format!(r#"#[proto(parent_message = "{parent_message}")]"#),
        );
      }
    }
  }

  for enum_desc in pool.all_enums() {
    let package = enum_desc.package_name();

    if packages.contains(&package) {
      let enum_full_ish_name = full_ish_name(enum_desc.full_name(), package);

      if desc_data_config.collect_enums_data {
        desc_data.enums.push(Enum {
          name: enum_desc.name().to_string(),
          parent_message: enum_desc
            .parent_message()
            .map(|p| full_ish_name(p.full_name(), package).to_string()),
          package: package.to_string(),
        });
      }

      config.enum_attribute(enum_desc.full_name(), "#[derive(::prelude::ProtoEnum)]");
      config.enum_attribute(
        enum_desc.full_name(),
        format!(r#"#[proto(name = "{enum_full_ish_name}")]"#),
      );
    }
  }

  Ok(desc_data)
}

/// A small utility that recursively collects all .proto files in a given directory and its subdirectories.
///
/// Useful if you want to avoid passing each individual .proto file to the prost config.
pub fn get_proto_files_recursive(base_dir: impl Into<PathBuf>) -> io::Result<Vec<String>> {
  let base_dir: PathBuf = base_dir.into();
  let mut proto_files = Vec::new();

  if !base_dir.is_dir() {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("Path {} is not a directory.", base_dir.display()),
    ));
  }

  collect_proto_files_recursive_helper(base_dir.as_path(), &mut proto_files)?;

  Ok(proto_files)
}

fn collect_proto_files_recursive_helper(
  current_dir: &Path,
  proto_files: &mut Vec<String>,
) -> io::Result<()> {
  for entry in fs::read_dir(current_dir)? {
    let entry = entry?;
    let path = entry.path();

    if path.is_file() {
      if path.extension().is_some_and(|ext| ext == "proto") {
        proto_files.push(
          path
            .to_str()
            .ok_or_else(|| {
              io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Path {} contains invalid Unicode.", path.display()),
              )
            })?
            .to_owned(),
        );
      }
    } else if path.is_dir() {
      collect_proto_files_recursive_helper(&path, proto_files)?;
    }
  }
  Ok(())
}
