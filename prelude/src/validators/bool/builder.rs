#[doc(hidden)]
pub mod state;
use crate::validators::*;
pub(crate) use state::*;

#[derive(Clone, Debug)]
pub struct BoolValidatorBuilder<S: State = Empty> {
  _state: PhantomData<S>,
  data: BoolValidator,
}

impl_validator!(BoolValidator, bool);

impl<S: State> Default for BoolValidatorBuilder<S> {
  #[inline]
  fn default() -> Self {
    Self {
      _state: PhantomData,
      data: BoolValidator::default(),
    }
  }
}

impl BoolValidator {
  #[must_use]
  #[inline]
  pub fn builder() -> BoolValidatorBuilder {
    BoolValidatorBuilder::default()
  }
}

impl<S: State> From<BoolValidatorBuilder<S>> for ProtoOption {
  fn from(value: BoolValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}

#[allow(
  clippy::must_use_candidate,
  clippy::use_self,
  clippy::return_self_not_must_use
)]
impl<S: State> BoolValidatorBuilder<S> {
  #[inline]
  pub const fn ignore_always(mut self) -> BoolValidatorBuilder<SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    self.data.ignore = Ignore::Always;

    BoolValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  #[inline]
  pub const fn ignore_if_zero_value(mut self) -> BoolValidatorBuilder<SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    self.data.ignore = Ignore::IfZeroValue;

    BoolValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  #[inline]
  pub const fn required(mut self) -> BoolValidatorBuilder<SetRequired<S>>
  where
    S::Required: IsUnset,
  {
    self.data.required = true;

    BoolValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  #[inline]
  pub const fn const_(mut self, val: bool) -> BoolValidatorBuilder<SetConst<S>>
  where
    S::Const: IsUnset,
  {
    self.data.const_ = Some(val);

    BoolValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  #[inline]
  pub const fn build(self) -> BoolValidator {
    self.data
  }
}
