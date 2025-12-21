use std::sync::OnceLock;

use crate::*;

// static REGISTRY: OnceLock<HashMap<&'static str, >>

pub struct RegistryPath {
  pub file: &'static str,
  pub package: &'static str,
}

pub fn collect() {
  for entry in inventory::iter::<RegistryMessage>() {
    let msg = (entry.message)();

    println!("{msg:#?}");
  }
}

pub struct RegistryMessage {
  pub message: fn() -> Message,
}

pub struct RegistryEnum {
  pub enum_: fn() -> Enum,
}

inventory::collect!(RegistryEnum);

inventory::collect!(RegistryMessage);
