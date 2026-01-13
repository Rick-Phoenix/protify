#[doc(hidden)]
pub mod state;
use crate::validators::*;
use proto_types::Any;
pub(crate) use state::*;

#[derive(Clone, Debug)]
pub struct AnyValidatorBuilder<S: State = Empty> {
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
}

impl<S: State> Default for AnyValidatorBuilder<S> {
  #[inline]
  fn default() -> Self {
    Self {
      _state: PhantomData,
      cel: Default::default(),
      ignore: Default::default(),
      required: Default::default(),
      in_: Default::default(),
      not_in: Default::default(),
    }
  }
}

impl AnyValidator {
  #[must_use]
  #[inline]
  pub fn builder() -> AnyValidatorBuilder {
    AnyValidatorBuilder::default()
  }
}

impl_validator!(AnyValidator, Any);

#[allow(
  clippy::must_use_candidate,
  clippy::use_self,
  clippy::return_self_not_must_use
)]
impl<S: State> AnyValidatorBuilder<S> {
  #[inline]
  pub fn cel(mut self, program: CelProgram) -> AnyValidatorBuilder<S> {
    self.cel.push(program);

    AnyValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: self.ignore,
      required: self.required,
      in_: self.in_,
      not_in: self.not_in,
    }
  }

  #[inline]
  pub fn ignore_always(self) -> AnyValidatorBuilder<SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    AnyValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: Ignore::Always,
      required: self.required,
      in_: self.in_,
      not_in: self.not_in,
    }
  }

  #[inline]
  pub fn required(self) -> AnyValidatorBuilder<SetRequired<S>>
  where
    S::Required: IsUnset,
  {
    AnyValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: self.ignore,
      required: true,
      in_: self.in_,
      not_in: self.not_in,
    }
  }

  #[inline]
  pub fn in_(
    self,
    list: impl IntoIterator<Item = impl Into<SharedStr>>,
  ) -> AnyValidatorBuilder<SetIn<S>>
  where
    S::In: IsUnset,
  {
    AnyValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: self.ignore,
      required: self.required,
      in_: Some(StaticLookup::new(
        list.into_iter().map(Into::<SharedStr>::into),
      )),
      not_in: self.not_in,
    }
  }

  #[inline]
  pub fn not_in(
    self,
    list: impl IntoIterator<Item = impl Into<SharedStr>>,
  ) -> AnyValidatorBuilder<SetNotIn<S>>
  where
    S::NotIn: IsUnset,
  {
    AnyValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      ignore: self.ignore,
      required: self.required,
      not_in: Some(StaticLookup::new(
        list.into_iter().map(Into::<SharedStr>::into),
      )),
      in_: self.in_,
    }
  }

  #[must_use]
  #[inline]
  pub fn build(self) -> AnyValidator {
    AnyValidator {
      cel: self.cel,
      ignore: self.ignore,
      required: self.required,
      in_: self.in_,
      not_in: self.not_in,
    }
  }
}

impl<S: State> From<AnyValidatorBuilder<S>> for ProtoOption {
  fn from(value: AnyValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}
