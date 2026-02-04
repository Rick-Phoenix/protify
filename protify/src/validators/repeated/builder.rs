#[doc(hidden)]
pub mod state;
use crate::validators::*;
pub(crate) use state::*;

/// Builder for [`RepeatedValidator`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RepeatedValidatorBuilder<T, S: State = Empty>
where
	T: ProtoValidation,
{
	_state: PhantomData<S>,
	_inner_type: PhantomData<T>,

	cel: Vec<CelProgram>,
	items: Option<T::Validator>,
	min_items: Option<usize>,
	max_items: Option<usize>,
	unique: bool,
	ignore: Ignore,

	error_messages: Option<ErrorMessages<RepeatedViolation>>,
}

impl<T, S: State> Default for RepeatedValidatorBuilder<T, S>
where
	T: ProtoValidation,
{
	#[inline]
	fn default() -> Self {
		Self {
			_state: PhantomData,
			_inner_type: PhantomData,
			cel: Default::default(),
			items: Default::default(),
			min_items: Default::default(),
			max_items: Default::default(),
			unique: Default::default(),
			ignore: Default::default(),
			error_messages: None,
		}
	}
}

impl<T> RepeatedValidator<T>
where
	T: ProtoValidation,
{
	#[must_use]
	#[inline]
	pub fn builder() -> RepeatedValidatorBuilder<T> {
		RepeatedValidatorBuilder::default()
	}
}

impl<T, S: State> RepeatedValidatorBuilder<T, S>
where
	T: ProtoValidation,
{
	/// Builds the validator.
	///
	/// If the `items` validator is unset, but the target has `HAS_DEFAULT_VALIDATOR` from [`ProtoValidation`] set to true,
	/// its default validator will be assigned.
	#[inline]
	pub fn build(self) -> RepeatedValidator<T> {
		let Self {
			_inner_type,
			items,
			min_items,
			max_items,
			unique,
			ignore,
			cel,
			error_messages,
			..
		} = self;

		RepeatedValidator {
			_inner_type,
			cel,
			items: items.or_else(|| T::HAS_DEFAULT_VALIDATOR.then(|| T::Validator::default())),
			min_items,
			max_items,
			unique,
			ignore,
			error_messages,
		}
	}

	/// Adds a map with custom error messages to the underlying validator.
	///
	/// If a violation has no custom error message attached to it, it uses the default error message.
	///
	/// NOTE: The custom error messages for the individual items must be handled by their respective validator.
	#[inline]
	pub fn with_error_messages(
		self,
		error_messages: impl IntoIterator<Item = (RepeatedViolation, impl Into<FixedStr>)>,
	) -> RepeatedValidatorBuilder<T, SetErrorMessages<S>>
	where
		S::ErrorMessages: IsUnset,
	{
		let map: BTreeMap<RepeatedViolation, FixedStr> = error_messages
			.into_iter()
			.map(|(v, m)| (v, m.into()))
			.collect();

		RepeatedValidatorBuilder {
			_state: PhantomData,
			_inner_type: self._inner_type,
			items: self.items,
			cel: self.cel,
			min_items: self.min_items,
			max_items: self.max_items,
			unique: self.unique,
			ignore: self.ignore,
			error_messages: Some(Box::new(map)),
		}
	}

	/// Adds a [`CelProgram`] to this validator.
	///
	/// The program will be applied to the vector as a whole. To apply a program to the individual items,
	/// use the items validator.
	#[inline]
	#[must_use]
	pub fn cel(mut self, program: CelProgram) -> Self {
		self.cel.push(program);

		self
	}

	/// Sets the rules to apply to each item of this map.
	///
	/// It receives a closure that that receives the item type's default validator builder as an argument.
	///
	/// # Example
	///
	/// ```
	/// use protify::*;
	///
	/// let validator = RepeatedValidator::<String>::builder().items(|i| i.min_len(5)).build();
	///
	/// assert_eq!(validator.items.unwrap(), StringValidator::builder().min_len(5).build());
	/// ```
	#[inline]
	pub fn items<F, FinalBuilder>(self, config_fn: F) -> RepeatedValidatorBuilder<T, SetItems<S>>
	where
		S::Items: IsUnset,
		T: ProtoValidation,
		FinalBuilder: ValidatorBuilderFor<T, Validator = T::Validator>,
		F: FnOnce(T::ValidatorBuilder) -> FinalBuilder,
	{
		let items_builder = T::validator_from_closure(config_fn);

		RepeatedValidatorBuilder {
			_state: PhantomData,
			_inner_type: self._inner_type,
			items: Some(items_builder),
			cel: self.cel,
			min_items: self.min_items,
			max_items: self.max_items,
			unique: self.unique,
			ignore: self.ignore,
			error_messages: self.error_messages,
		}
	}

	/// Specifies that this validator's rules will be ignored if the vector is empty.
	#[inline]
	pub fn ignore_if_zero_value(self) -> RepeatedValidatorBuilder<T, SetIgnore<S>>
	where
		S::Ignore: IsUnset,
	{
		RepeatedValidatorBuilder {
			_state: PhantomData,
			_inner_type: self._inner_type,
			cel: self.cel,
			items: self.items,
			min_items: self.min_items,
			max_items: self.max_items,
			unique: self.unique,
			ignore: Ignore::IfZeroValue,
			error_messages: self.error_messages,
		}
	}

	/// Specifies that this validator should always be ignored.
	#[inline]
	pub fn ignore_always(self) -> RepeatedValidatorBuilder<T, SetIgnore<S>>
	where
		S::Ignore: IsUnset,
	{
		RepeatedValidatorBuilder {
			_state: PhantomData,
			_inner_type: self._inner_type,
			cel: self.cel,
			items: self.items,
			min_items: self.min_items,
			max_items: self.max_items,
			unique: self.unique,
			ignore: Ignore::Always,
			error_messages: self.error_messages,
		}
	}

	/// Specifies the minimum amount of items that this field must contain in order to be valid.
	#[inline]
	pub fn min_items(self, num: usize) -> RepeatedValidatorBuilder<T, SetMinItems<S>>
	where
		S::MinItems: IsUnset,
	{
		RepeatedValidatorBuilder {
			_state: PhantomData,
			_inner_type: self._inner_type,
			cel: self.cel,
			items: self.items,
			min_items: Some(num),
			max_items: self.max_items,
			unique: self.unique,
			ignore: self.ignore,
			error_messages: self.error_messages,
		}
	}

	/// Specifies the maximum amount of items that this field must contain in order to be valid.
	#[inline]
	pub fn max_items(self, num: usize) -> RepeatedValidatorBuilder<T, SetMaxItems<S>>
	where
		S::MaxItems: IsUnset,
	{
		RepeatedValidatorBuilder {
			_state: PhantomData,
			_inner_type: self._inner_type,
			cel: self.cel,
			items: self.items,
			min_items: self.min_items,
			max_items: Some(num),
			unique: self.unique,
			ignore: self.ignore,
			error_messages: self.error_messages,
		}
	}

	/// Specifies that this field must contain only unique values.
	///
	/// When the items validator is a [`FloatValidator`], the [`abs_tolerance`](FloatValidator::abs_tolerance) and [`rel_tolerance`](FloatValidator::rel_tolerance) fields are taken in consideration for the uniqueness check.
	#[inline]
	pub fn unique(self) -> RepeatedValidatorBuilder<T, SetUnique<S>>
	where
		S::Unique: IsUnset,
	{
		RepeatedValidatorBuilder {
			_state: PhantomData,
			_inner_type: self._inner_type,
			cel: self.cel,
			items: self.items,
			min_items: self.min_items,
			max_items: self.max_items,
			unique: true,
			ignore: self.ignore,
			error_messages: self.error_messages,
		}
	}
}

impl<T, S: State> From<RepeatedValidatorBuilder<T, S>> for ProtoOption
where
	T: ProtoValidation,
{
	#[inline(never)]
	#[cold]
	fn from(value: RepeatedValidatorBuilder<T, S>) -> Self {
		value.build().into()
	}
}
