use super::*;

use proto_types::field_descriptor_proto::Type as ProtoPrimitive;
use proto_types::protovalidate::field_path_element::Subscript;

/// The context for the field being validated.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldContext {
	pub name: FixedStr,
	pub tag: i32,
	pub subscript: Option<Subscript>,
	pub map_key_type: Option<ProtoPrimitive>,
	pub map_value_type: Option<ProtoPrimitive>,
	pub field_type: ProtoPrimitive,
	pub field_kind: FieldKind,
}

impl FieldContext {
	/// Converts the [`FieldContext`] to the [`FieldPathElement`] representation.
	#[must_use]
	#[inline(never)]
	#[cold]
	pub fn as_path_element(&self) -> FieldPathElement {
		FieldPathElement {
			field_number: Some(self.tag),
			field_name: Some(self.name.to_string()),
			field_type: Some(self.field_type as i32),
			key_type: self.map_key_type.map(|t| t as i32),
			value_type: self.map_value_type.map(|t| t as i32),
			subscript: self.subscript.clone(),
		}
	}
}

/// Specifies whether the target of the validation is a field/element as a whole,
/// a value or key in a map field, or an item in a repeated field.
///
/// For example, if the validation is performed on a `repeated` field but the validation analyzes the collection as a whole (for example, its length), the [`FieldKind`] would be [`Normal`](FieldKind::Normal), whereas if the validation is performed on its individual items, then it would be set to [`RepeatedItem`](FieldKind::RepeatedItem).
#[derive(Clone, Default, Debug, Copy, PartialEq, Eq, Hash)]
pub enum FieldKind {
	MapKey,
	MapValue,
	RepeatedItem,
	/// Signifies that the target of validation is either an item as a whole (for validators defined at the top level of messages/oneofs) or a field.
	#[default]
	Normal,
}

impl FieldKind {
	/// Returns `true` if the field kind is [`MapKey`].
	///
	/// [`MapKey`]: FieldKind::MapKey
	#[must_use]
	#[inline]
	pub const fn is_map_key(&self) -> bool {
		matches!(self, Self::MapKey)
	}

	/// Returns `true` if the field kind is [`MapValue`].
	///
	/// [`MapValue`]: FieldKind::MapValue
	#[must_use]
	#[inline]
	pub const fn is_map_value(&self) -> bool {
		matches!(self, Self::MapValue)
	}

	/// Returns `true` if the field kind is [`RepeatedItem`].
	///
	/// [`RepeatedItem`]: FieldKind::RepeatedItem
	#[must_use]
	#[inline]
	pub const fn is_repeated_item(&self) -> bool {
		matches!(self, Self::RepeatedItem)
	}

	/// Returns `true` if the field kind is [`Normal`].
	///
	/// [`Normal`]: FieldKind::Normal
	#[must_use]
	#[inline]
	pub const fn is_normal(&self) -> bool {
		matches!(self, Self::Normal)
	}
}

/// The context for a given validation execution.
///
/// ℹ️ **NOTE**: By default, `fail_fast` is set to true even if this is slightly unitiomatic for a boolean,
/// because this is by far the most desired behaviour for most applications.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ValidationCtx {
	pub field_context: Option<FieldContext>,
	pub parent_elements: Vec<FieldPathElement>,
	pub violations: ValidationErrors,
	/// Whether validation should be interrupted at the first failure.
	pub fail_fast: bool,
}

impl Default for ValidationCtx {
	/// Default values for [`ValidationCtx`].
	///
	/// NOTE: By default, `fail_fast` is set to true even if this is slightly unitiomatic for a boolean,
	/// because this is by far the most desired behaviour for most applications.
	#[inline]
	fn default() -> Self {
		Self {
			field_context: None,
			parent_elements: vec![],
			violations: ValidationErrors::new(),
			fail_fast: true,
		}
	}
}

impl ValidationCtx {
	/// Sets the [`FieldContext`] to [`None`].
	///
	/// Mainly useful for validators defined at the top level of a oneof or an unnested message, which do not have a proto
	/// field context (tag, name, etc) of their own.
	#[inline]
	pub fn without_field_context(&mut self) -> &mut Self {
		self.field_context = None;
		self
	}

	/// Mutates the [`FieldContext`].
	#[inline]
	pub fn with_field_context(&mut self, field_context: FieldContext) -> &mut Self {
		self.field_context = Some(field_context);
		self
	}

	/// Adds a new known violation to the list of errors.
	#[inline(never)]
	#[cold]
	pub fn add_violation(
		&mut self,
		kind: ViolationKind,
		error_message: impl Into<String>,
	) -> ValidationResult {
		let violation = create_violation_core(
			None,
			self.field_context.as_ref(),
			&self.parent_elements,
			kind.data(),
			error_message.into(),
		);

		self.violations.push(ViolationCtx {
			meta: ViolationMeta {
				kind,
				field_kind: self.field_kind(),
			},
			data: violation,
		});

		if self.fail_fast {
			Err(FailFast)
		} else {
			Ok(IsValid::No)
		}
	}

	/// Extracts the [`FieldKind`]. If the [`FieldContext`] is absent (for a top level validator), it falls back to [`FieldKind::Normal`].
	#[inline]
	#[must_use]
	pub fn field_kind(&self) -> FieldKind {
		self.field_context
			.as_ref()
			.map(|fc| fc.field_kind)
			.unwrap_or_default()
	}

	/// Adds a new known violation to the list of errors, overriding the rule ID.
	#[inline(never)]
	#[cold]
	pub fn add_violation_with_custom_id(
		&mut self,
		rule_id: impl Into<String>,
		kind: ViolationKind,
		error_message: impl Into<String>,
	) -> ValidationResult {
		let violation = create_violation_core(
			Some(rule_id.into()),
			self.field_context.as_ref(),
			&self.parent_elements,
			kind.data(),
			error_message.into(),
		);

		self.violations.push(ViolationCtx {
			data: violation,
			meta: ViolationMeta {
				kind,
				field_kind: self.field_kind(),
			},
		});

		if self.fail_fast {
			Err(FailFast)
		} else {
			Ok(IsValid::No)
		}
	}

	/// Adds a new violation related to a [`CelRule`].
	#[inline]
	#[cold]
	pub fn add_cel_violation(&mut self, rule: &CelRule) -> ValidationResult {
		self.add_violation_with_custom_id(&rule.id, ViolationKind::Cel, &rule.message)
	}

	/// Adds a violation for a required oneof. If no custom error message is specified, the default will be used.
	#[inline]
	#[cold]
	pub fn add_required_oneof_violation(
		&mut self,
		error_message: Option<String>,
	) -> ValidationResult {
		if let Some(msg) = error_message {
			self.add_violation(ViolationKind::RequiredOneof, msg)
		} else {
			self.add_violation(
				ViolationKind::RequiredOneof,
				"at least one value must be set",
			)
		}
	}

	/// Adds a violation for a required field. If no custom error message is specified, the default will be used.
	#[inline]
	#[cold]
	pub fn add_required_violation(&mut self, error_message: Option<String>) -> ValidationResult {
		if let Some(msg) = error_message {
			self.add_violation(ViolationKind::Required, msg)
		} else {
			self.add_violation(ViolationKind::Required, "is required")
		}
	}

	#[cfg(feature = "cel")]
	#[inline(never)]
	#[cold]
	pub(crate) fn add_cel_error_violation(&mut self, error: CelError) -> ValidationResult {
		self.violations.push(ViolationCtx {
			meta: ViolationMeta {
				kind: ViolationKind::Cel,
				field_kind: self.field_kind(),
			},
			data: error.into_violation(self.field_context.as_ref(), &self.parent_elements),
		});

		if self.fail_fast {
			Err(FailFast)
		} else {
			Ok(IsValid::No)
		}
	}
}
