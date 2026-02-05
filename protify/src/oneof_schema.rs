use crate::*;

/// Implemented for proxied oneofs by the [`proto_oneof`] macro.
pub trait ProxiedOneof: From<Self::Proxy> + Into<Self::Proxy> {
	type Proxy: OneofProxy<Oneof = Self> + From<Self> + Into<Self>;

	/// Converts to the paired proxy.
	#[inline]
	fn into_proxy(self) -> Self::Proxy {
		self.into()
	}
}

/// Implemented for oneof proxies by the [`proto_oneof`] macro.
pub trait OneofProxy: From<Self::Oneof> + Into<Self::Oneof> {
	type Oneof: ProtoOneof + From<Self> + Into<Self>;

	/// Converts to the paired oneof.
	#[inline]
	fn into_oneof(self) -> Self::Oneof {
		self.into()
	}
}

/// Trait responsible for generating the schema representation of a oneof.
pub trait ProtoOneof {
	#[doc(hidden)]
	const TAGS: &[i32];

	/// Returns the protobuf schema representation.
	fn proto_schema() -> Oneof;

	#[doc(hidden)]
	fn __check_tags(
		message_name: &str,
		oneof_name: &str,
		found_tags: &mut [i32],
	) -> Result<(), String> {
		use similar_asserts::SimpleDiff;

		let expected = Self::TAGS;

		found_tags.sort_unstable();

		if expected != found_tags {
			let exp_str = format!("{expected:#?}");
			let found_str = format!("{found_tags:#?}");

			let diff = SimpleDiff::from_str(&exp_str, &found_str, "expected", "found");

			let error = format!(
				"Found tags mismatch for oneof {oneof_name} in message {message_name}:\n{diff}"
			);

			return Err(error);
		}

		Ok(())
	}
}

/// Trait responsible for executing validators that have been assigned to a oneof.
///
/// Implemented by the [`proto_oneof`] macro.
pub trait ValidatedOneof: ProtoValidation + Clone {
	/// Executes validation on this oneof, triggering the validators that have been assigned to it
	/// via macro attributes, if there are any.
	///
	/// Uses the default settings for [`ValidationCtx`], which include `fail_fast: true`.
	#[inline]
	fn validate(&self) -> Result<(), ValidationErrors> {
		if !Self::HAS_DEFAULT_VALIDATOR {
			return Ok(());
		}

		let mut ctx = ValidationCtx::default();

		let _ = self.validate_with_ctx(&mut ctx);

		if ctx.violations.is_empty() {
			Ok(())
		} else {
			Err(ctx.violations)
		}
	}

	/// Executes validation on this oneof, triggering the validators that have been assigned to it
	/// via macro attributes, if there are any, and returns true if the validation was successful.
	///
	/// Uses the default settings for [`ValidationCtx`], which include `fail_fast: true`.
	#[inline]
	fn is_valid(&self) -> bool {
		if Self::HAS_DEFAULT_VALIDATOR {
			self.validate().is_ok()
		} else {
			true
		}
	}

	/// Executes validation on this oneof, triggering the validators that have been assigned to it
	/// via macro attributes, if there are any, and returns the value if the validation was successful.
	///
	/// Uses the default settings for [`ValidationCtx`], which include `fail_fast: true`.
	#[inline]
	fn validated(self) -> Result<Self, ValidationErrors> {
		if !Self::HAS_DEFAULT_VALIDATOR {
			return Ok(self);
		}

		match self.validate() {
			Ok(()) => Ok(self),
			Err(e) => Err(e),
		}
	}

	/// Executes validation on this oneof, triggering the validators that have been assigned to it
	/// via macro attributes, if there are any.
	fn validate_with_ctx(&self, ctx: &mut ValidationCtx) -> ValidationResult;
}

/// Schema representation for a protobuf oneof.
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Oneof {
	pub name: FixedStr,
	pub fields: Vec<Field>,
	pub options: Vec<ProtoOption>,
	pub validators: Vec<ValidatorSchema>,
}

impl Oneof {
	#[must_use]
	pub(crate) const fn has_options(&self) -> bool {
		!self.options.is_empty() || !self.validators.is_empty()
	}

	/// Iterates all the options of the oneof, including those created by the [`ValidatorSchema`]s.
	pub fn options_with_validators(&self) -> impl Iterator<Item = &options::ProtoOption> {
		self.options
			.iter()
			.chain(self.validators.iter().map(|v| &v.schema))
	}

	/// Mutates the name of the oneof.
	#[must_use]
	pub fn with_name(mut self, name: impl Into<FixedStr>) -> Self {
		self.name = name.into();
		self
	}

	/// Adds [`ValidatorSchema`]s to this oneof.
	#[must_use]
	pub fn with_validators<I: IntoIterator<Item = ValidatorSchema>>(
		mut self,
		validators: I,
	) -> Self {
		self.validators.extend(validators);
		self
	}

	/// Adds [`ProtoOption`]s to this oneof.
	#[must_use]
	pub fn with_options<I: IntoIterator<Item = ProtoOption>>(mut self, options: I) -> Self {
		self.options.extend(options);
		self
	}
}
