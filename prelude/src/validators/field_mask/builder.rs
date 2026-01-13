#[doc(hidden)]
pub mod state;
use crate::validators::*;
use proto_types::FieldMask;
pub(crate) use state::*;

#[derive(Clone, Debug)]
pub struct FieldMaskValidatorBuilder<S: State = Empty> {
  _state: PhantomData<S>,

  /// Adds custom validation using one or more [`CelRule`]s to this field.
  cel: Vec<CelProgram>,

  ignore: Ignore,

  /// Specifies that the field must be set in order to be valid.
  required: bool,

  /// Specifies that only the values in this list will be considered valid for this field.
  in_: Option<StaticLookup<SharedStr>>,

  /// Specifies that the values in this list will be considered NOT valid for this field.
  not_in: Option<StaticLookup<SharedStr>>,

  /// Specifies that only this specific value will be considered valid for this field.
  const_: Option<StaticLookup<SharedStr>>,
}

impl<S: State> ValidatorBuilderFor<FieldMask> for FieldMaskValidatorBuilder<S> {
  type Target = FieldMask;
  type Validator = FieldMaskValidator;

  #[inline]
  #[doc(hidden)]
  fn build_validator(self) -> Self::Validator {
    self.build()
  }
}

impl<S: State> Default for FieldMaskValidatorBuilder<S> {
  #[inline]
  fn default() -> Self {
    Self {
      _state: PhantomData,
      cel: Default::default(),
      ignore: Default::default(),
      required: Default::default(),
      in_: Default::default(),
      not_in: Default::default(),
      const_: Default::default(),
    }
  }
}

impl<S: State> From<FieldMaskValidatorBuilder<S>> for ProtoOption {
  fn from(value: FieldMaskValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}

impl FieldMaskValidator {
  #[must_use]
  #[inline]
  pub fn builder() -> FieldMaskValidatorBuilder {
    FieldMaskValidatorBuilder::default()
  }
}

#[allow(
  clippy::must_use_candidate,
  clippy::use_self,
  clippy::return_self_not_must_use
)]
impl<S: State> FieldMaskValidatorBuilder<S> {
  #[inline]
  pub fn cel(mut self, program: CelProgram) -> FieldMaskValidatorBuilder<S> {
    self.cel.push(program);

    FieldMaskValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: self.ignore,
      required: self.required,
      in_: self.in_,
      not_in: self.not_in,
      const_: self.const_,
    }
  }

  #[inline]
  pub fn ignore_always(self) -> FieldMaskValidatorBuilder<SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    FieldMaskValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: Ignore::Always,
      required: self.required,
      in_: self.in_,
      not_in: self.not_in,
      const_: self.const_,
    }
  }

  #[inline]
  pub fn required(self) -> FieldMaskValidatorBuilder<SetRequired<S>>
  where
    S::Required: IsUnset,
  {
    FieldMaskValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: self.ignore,
      required: true,
      in_: self.in_,
      not_in: self.not_in,
      const_: self.const_,
    }
  }

  #[inline]
  pub fn in_(
    self,
    val: impl IntoIterator<Item = impl Into<SharedStr>>,
  ) -> FieldMaskValidatorBuilder<SetIn<S>>
  where
    S::In: IsUnset,
  {
    FieldMaskValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: self.ignore,
      required: self.required,
      in_: Some(StaticLookup::new(
        val.into_iter().map(Into::<SharedStr>::into),
      )),
      not_in: self.not_in,
      const_: self.const_,
    }
  }

  #[inline]
  pub fn not_in(
    self,
    val: impl IntoIterator<Item = impl Into<SharedStr>>,
  ) -> FieldMaskValidatorBuilder<SetNotIn<S>>
  where
    S::NotIn: IsUnset,
  {
    FieldMaskValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: self.ignore,
      required: self.required,
      in_: self.in_,
      not_in: Some(StaticLookup::new(
        val.into_iter().map(Into::<SharedStr>::into),
      )),
      const_: self.const_,
    }
  }

  #[inline]
  pub fn const_(
    self,
    val: impl IntoIterator<Item = impl Into<SharedStr>>,
  ) -> FieldMaskValidatorBuilder<SetConst<S>>
  where
    S::Const: IsUnset,
  {
    FieldMaskValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: self.ignore,
      required: self.required,
      in_: self.in_,
      not_in: self.not_in,
      const_: Some(StaticLookup::new(
        val.into_iter().map(Into::<SharedStr>::into),
      )),
    }
  }

  #[inline]
  #[must_use]
  pub fn build(self) -> FieldMaskValidator {
    let Self {
      cel,
      ignore,
      required,
      in_,
      not_in,
      const_,
      ..
    } = self;

    FieldMaskValidator {
      cel,
      ignore,
      required,
      in_,
      not_in,
      const_,
    }
  }
}
