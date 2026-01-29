#[doc(hidden)]
pub mod state;
use crate::validators::*;
pub(crate) use state::*;

use proto_types::Duration;

impl ProtoValidation for Duration {
  #[doc(hidden)]
  type Target = Self;
  #[doc(hidden)]
  type Stored = Self;
  type Validator = DurationValidator;
  #[doc(hidden)]
  type ValidatorBuilder = DurationValidatorBuilder;

  type UniqueStore<'a>
    = CopyHybridStore<Self>
  where
    Self: 'a;
}

impl<S: State> ValidatorBuilderFor<Duration> for DurationValidatorBuilder<S> {
  type Target = Duration;
  type Validator = DurationValidator;
  #[inline]
  fn build_validator(self) -> DurationValidator {
    self.build()
  }
}

/// Builder for [`DurationValidator`].
#[derive(Clone, Debug)]
pub struct DurationValidatorBuilder<S: State = Empty> {
  _state: PhantomData<S>,

  data: DurationValidator,
}

impl<S: State> Default for DurationValidatorBuilder<S> {
  #[inline]
  fn default() -> Self {
    Self {
      _state: PhantomData,
      data: DurationValidator::default(),
    }
  }
}

impl DurationValidator {
  #[must_use]
  #[inline]
  pub fn builder() -> DurationValidatorBuilder {
    DurationValidatorBuilder::default()
  }
}

impl<S: State> From<DurationValidatorBuilder<S>> for ProtoOption {
  #[inline(never)]
  #[cold]
  fn from(value: DurationValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}

#[allow(
  clippy::must_use_candidate,
  clippy::use_self,
  clippy::return_self_not_must_use
)]
impl<S: State> DurationValidatorBuilder<S> {
  custom_error_messages_method!(Duration);

  /// Adds a [`CelProgram`] to this validator.
  #[inline]
  pub fn cel(mut self, program: CelProgram) -> DurationValidatorBuilder<S> {
    self.data.cel.push(program);

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that this validator should always be ignored.
  #[inline]
  pub fn ignore_always(mut self) -> DurationValidatorBuilder<SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    self.data.ignore = Ignore::Always;

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the field must be set in order to be valid.
  #[inline]
  pub fn required(mut self) -> DurationValidatorBuilder<SetRequired<S>>
  where
    S::Required: IsUnset,
  {
    self.data.required = true;

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that only the values in this list will be considered valid for this field.
  #[inline]
  pub fn in_(mut self, val: impl IntoSortedList<Duration>) -> DurationValidatorBuilder<SetIn<S>>
  where
    S::In: IsUnset,
  {
    self.data.in_ = Some(val.into_sorted_list());

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the values in this list will be considered NOT valid for this field.
  #[inline]
  pub fn not_in(
    mut self,
    val: impl IntoSortedList<Duration>,
  ) -> DurationValidatorBuilder<SetNotIn<S>>
  where
    S::NotIn: IsUnset,
  {
    self.data.not_in = Some(val.into_sorted_list());

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that only this specific value will be considered valid for this field.
  #[inline]
  pub fn const_(mut self, val: Duration) -> DurationValidatorBuilder<SetConst<S>>
  where
    S::Const: IsUnset,
  {
    self.data.const_ = Some(val);

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the duration must be shorter than the indicated amount in order to pass validation.
  #[inline]
  pub fn lt(mut self, val: Duration) -> DurationValidatorBuilder<SetLt<S>>
  where
    S::Lt: IsUnset,
  {
    self.data.lt = Some(val);

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the duration must be equal to or shorter than the indicated amount in order to pass validation.
  #[inline]
  pub fn lte(mut self, val: Duration) -> DurationValidatorBuilder<SetLte<S>>
  where
    S::Lte: IsUnset,
  {
    self.data.lte = Some(val);

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the duration must be longer than the indicated amount in order to pass validation.
  #[inline]
  pub fn gt(mut self, val: Duration) -> DurationValidatorBuilder<SetGt<S>>
  where
    S::Gt: IsUnset,
  {
    self.data.gt = Some(val);

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the duration must be equal to or longer than the indicated amount in order to pass validation.
  #[inline]
  pub fn gte(mut self, val: Duration) -> DurationValidatorBuilder<SetGte<S>>
  where
    S::Gte: IsUnset,
  {
    self.data.gte = Some(val);

    DurationValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Builds the validator.
  #[inline]
  pub fn build(self) -> DurationValidator {
    self.data
  }
}
