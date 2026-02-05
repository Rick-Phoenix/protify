#[doc(hidden)]
pub mod state;
use crate::validators::*;
pub(crate) use state::*;

/// Builder for [`FloatValidator`].
#[derive(Debug, Clone, PartialEq)]
pub struct FloatValidatorBuilder<Num, S = Empty>
where
	S: State,
	Num: FloatWrapper,
{
	_wrapper: PhantomData<Num>,
	_state: PhantomData<S>,

	data: FloatValidator<Num>,
}

impl<Num, S> Default for FloatValidatorBuilder<Num, S>
where
	S: State,
	Num: FloatWrapper,
{
	#[inline]
	fn default() -> Self {
		Self {
			_wrapper: PhantomData,
			_state: PhantomData,
			data: FloatValidator::default(),
		}
	}
}

impl<Num, S> FloatValidatorBuilder<Num, S>
where
	S: State,
	Num: FloatWrapper,
{
	/// Adds a map with custom error messages to the underlying validator.
	///
	/// If a violation has no custom error message attached to it, it uses the default error message.
	#[inline]
	pub fn with_error_messages(
		mut self,
		error_messages: impl IntoIterator<Item = (Num::ViolationEnum, impl Into<FixedStr>)>,
	) -> FloatValidatorBuilder<Num, SetErrorMessages<S>>
	where
		S::ErrorMessages: IsUnset,
	{
		self.data.error_messages = Some(collect_error_messages(error_messages));

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that this validator should always be ignored.
	#[inline]
	pub fn ignore_always(mut self) -> FloatValidatorBuilder<Num, SetIgnore<S>>
	where
		S::Ignore: IsUnset,
	{
		self.data.ignore = Ignore::Always;

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that this validator should be ignored if the value is either unset or equal to its protobuf zero value.
	#[inline]
	pub fn ignore_if_zero_value(mut self) -> FloatValidatorBuilder<Num, SetIgnore<S>>
	where
		S::Ignore: IsUnset,
	{
		self.data.ignore = Ignore::IfZeroValue;

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Adds a [`CelProgram`] to this validator.
	#[inline]
	#[allow(clippy::use_self, clippy::return_self_not_must_use)]
	pub fn cel(mut self, program: CelProgram) -> FloatValidatorBuilder<Num, S> {
		self.data.cel.push(program);

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that the field must be set (if optional) or not equal to its zero value (if not optional) in order to be valid.
	#[inline]
	pub fn required(mut self) -> FloatValidatorBuilder<Num, SetRequired<S>>
	where
		S::Required: IsUnset,
	{
		self.data.required = true;

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Sets a value that represents the `abs` parameter in the [`float_eq!`] macro, and is used in all arithmetic operations for the validated floats. For more information, see the [float_eq guide](https://jtempest.github.io/float_eq-rs/book/how_to/compare_floating_point_numbers.html).
	///
	/// # Example
	///
	/// ```
	/// use protify::*;
	///
	/// let validator = FloatValidator::<f32>::builder().const_(1.0).abs_tolerance(0.00001).build();
	///
	/// assert!(validator.validate(&1.000001).is_ok());
	/// ```
	#[inline]
	pub fn abs_tolerance(mut self, val: Num) -> FloatValidatorBuilder<Num, SetAbsTolerance<S>>
	where
		S::AbsTolerance: IsUnset,
	{
		self.data.abs_tolerance = val;

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Sets a value that represents the `r2nd` parameter in the [`float_eq!`] macro, and is used in all arithmetic operations for the validated floats. For more information, see the [float_eq guide](https://jtempest.github.io/float_eq-rs/book/how_to/compare_floating_point_numbers.html).
	///
	/// # Example
	///
	/// ```
	/// use protify::*;
	///
	/// let validator = FloatValidator::<f32>::builder().const_(1.0).rel_tolerance(0.00001).build();
	///
	/// assert!(validator.validate(&1.000001).is_ok());
	/// ```
	#[inline]
	pub fn rel_tolerance(mut self, val: Num) -> FloatValidatorBuilder<Num, SetRelTolerance<S>>
	where
		S::RelTolerance: IsUnset,
	{
		self.data.rel_tolerance = val;

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that this field must be finite (i.e. it can't represent Infinity or NaN).
	#[inline]
	pub fn finite(mut self) -> FloatValidatorBuilder<Num, SetFinite<S>>
	where
		S::Finite: IsUnset,
	{
		self.data.finite = true;

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that only this specific value will be considered valid for this field.
	#[inline]
	pub fn const_(mut self, val: Num) -> FloatValidatorBuilder<Num, SetConst<S>>
	where
		S::Const: IsUnset,
	{
		self.data.const_ = Some(val);

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that this field's value will be valid only if it is smaller than the specified amount
	#[inline]
	pub fn lt(mut self, val: Num) -> FloatValidatorBuilder<Num, SetLt<S>>
	where
		S::Lt: IsUnset,
	{
		self.data.lt = Some(val);

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that this field's value will be valid only if it is smaller than, or equal to, the specified amount
	#[inline]
	pub fn lte(mut self, val: Num) -> FloatValidatorBuilder<Num, SetLte<S>>
	where
		S::Lte: IsUnset,
	{
		self.data.lte = Some(val);

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that this field's value will be valid only if it is greater than the specified amount
	#[inline]
	pub fn gt(mut self, val: Num) -> FloatValidatorBuilder<Num, SetGt<S>>
	where
		S::Gt: IsUnset,
	{
		self.data.gt = Some(val);

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that this field's value will be valid only if it is smaller than, or equal to, the specified amount
	#[inline]
	pub fn gte(mut self, val: Num) -> FloatValidatorBuilder<Num, SetGte<S>>
	where
		S::Gte: IsUnset,
	{
		self.data.gte = Some(val);

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that the values in this list will be considered NOT valid for this field.
	#[inline]
	pub fn not_in(
		mut self,
		list: impl IntoSortedList<OrderedFloat<Num>>,
	) -> FloatValidatorBuilder<Num, SetNotIn<S>>
	where
		S::NotIn: IsUnset,
	{
		self.data.not_in = Some(list.into_sorted_list());

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Specifies that only the values in this list will be considered valid for this field.
	#[inline]
	pub fn in_(
		mut self,
		list: impl IntoSortedList<OrderedFloat<Num>>,
	) -> FloatValidatorBuilder<Num, SetIn<S>>
	where
		S::In: IsUnset,
	{
		self.data.in_ = Some(list.into_sorted_list());

		FloatValidatorBuilder {
			_state: PhantomData,
			_wrapper: self._wrapper,
			data: self.data,
		}
	}

	/// Builds the validator.
	#[inline]
	pub fn build(self) -> FloatValidator<Num> {
		self.data
	}
}
