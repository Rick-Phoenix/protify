use crate::*;

/// Struct that represents a protobuf field.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
  pub name: FixedStr,
  pub tag: i32,
  pub type_: FieldType,
  pub options: Vec<ProtoOption>,
  pub validators: Vec<ValidatorSchema>,
}

/// A schema representation for a validator.
///
/// It allows the validator to be turned into a protobuf representation
/// that can be generated inside a message's options in the target protobuf file.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidatorSchema {
  pub schema: ProtoOption,
  /// The CEL rules belonging to this validator. They are stored separately so that the containing [`Package`] can check if there are multiple rules with the same ID within the same message scope.
  pub cel_rules: Vec<CelRule>,
  /// The imports that should be added to the receiving proto file.
  pub imports: Vec<FixedStr>,
}

impl Field {
  /// Collects all the options of the field, including those created by the schema representations of validators.
  pub fn options_with_validators(&self) -> impl Iterator<Item = &options::ProtoOption> {
    self
      .options
      .iter()
      .chain(self.validators.iter().map(|v| &v.schema))
  }

  pub(crate) fn register_import_path(&self, imports: &mut FileImports) {
    for import in self
      .validators
      .iter()
      .flat_map(|v| v.imports.clone())
    {
      imports.insert_internal(import);
    }

    match &self.type_ {
      FieldType::Normal(ty) | FieldType::Repeated(ty) | FieldType::Optional(ty) => {
        ty.register_import(imports)
      }
      FieldType::Map { keys, values } => {
        keys.into_type().register_import(imports);
        values.register_import(imports);
      }
    };
  }
}
