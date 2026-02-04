#[doc(hidden)]
pub mod state;
use crate::validators::*;
pub(crate) use state::*;

/// Builder for [`OneofValidator`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OneofValidatorBuilder<S: State = Empty> {
	_state: PhantomData<S>,
	data: OneofValidator,
}

impl<S: State> Default for OneofValidatorBuilder<S> {
	fn default() -> Self {
		Self {
			_state: Default::default(),
			data: Default::default(),
		}
	}
}

impl<S, T> ValidatorBuilderFor<T> for OneofValidatorBuilder<S>
where
	S: State,
	T: ValidatedOneof,
{
	type Validator = OneofValidator;

	#[inline]
	fn build_validator(self) -> Self::Validator {
		self.build()
	}
}

impl<S: State> OneofValidatorBuilder<S> {
	/// Specifies a custom error message to display for the `required` violation.
	#[must_use]
	pub fn with_error_message(
		mut self,
		msg: impl Into<FixedStr>,
	) -> OneofValidatorBuilder<SetRequiredErrorMessage<S>>
	where
		S::RequiredErrorMessage: IsUnset,
		S::Required: IsSet,
	{
		self.data.error_message = Some(msg.into());

		OneofValidatorBuilder {
			_state: PhantomData,
			data: self.data,
		}
	}

	/// Specifies that at least one of the variants of the oneof must be set.
	#[must_use]
	pub fn required(mut self) -> OneofValidatorBuilder<SetRequired<S>>
	where
		S::Required: IsUnset,
	{
		self.data.required = true;

		OneofValidatorBuilder {
			_state: PhantomData,
			data: self.data,
		}
	}

	/// Builds the validator.
	#[inline]
	#[must_use]
	pub fn build(self) -> OneofValidator {
		self.data
	}
}
