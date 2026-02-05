use crate::*;

/// Struct that represents a protobuf field.
#[derive(Debug, Clone, PartialEq, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct Field {
	pub name: FixedStr,
	pub tag: i32,
	pub type_: FieldType,
	pub options: Vec<ProtoOption>,
	pub validators: Vec<ValidatorSchema>,
}

impl Field {
	/// Collects all the options of the field, including those created by the schema representations of validators.
	pub fn options_with_validators(&self) -> impl Iterator<Item = &options::ProtoOption> {
		self.options
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

/// A schema representation for a validator.
///
/// It allows the validator to be turned into a protobuf representation
/// that can be generated inside a message's options in the target protobuf file.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidatorSchema {
	pub(crate) schema: ProtoOption,
	/// The CEL rules belonging to this validator. They are stored separately so that the containing [`Package`] can check if there are multiple rules with the same ID within the same message scope.
	pub(crate) cel_rules: Vec<CelRule>,
	/// The imports that should be added to the receiving proto file.
	pub(crate) imports: Vec<FixedStr>,
}

impl ValidatorSchema {
	pub const fn new(schema: ProtoOption) -> Self {
		Self {
			schema,
			cel_rules: vec![],
			imports: vec![],
		}
	}

	#[must_use]
	pub fn with_cel_rules(mut self, cel_rules: Vec<CelRule>) -> Self {
		self.cel_rules = cel_rules;
		self
	}

	#[must_use]
	pub fn with_imports(mut self, imports: Vec<FixedStr>) -> Self {
		self.imports = imports;
		self
	}

	pub fn cel_rules(&self) -> &[CelRule] {
		&self.cel_rules
	}
}
