#[doc(hidden)]
pub mod state;
use crate::validators::*;
pub(crate) use state::*;

#[derive(Debug, Clone)]
pub struct MessageValidatorBuilder<T: ValidatedMessage, S: State = Empty> {
  _state: PhantomData<S>,

  data: MessageValidator<T>,
}

impl<T: ValidatedMessage, S: State> Default for MessageValidatorBuilder<T, S> {
  #[inline]
  fn default() -> Self {
    Self {
      _state: PhantomData,
      data: MessageValidator::default(),
    }
  }
}

impl<T: ValidatedMessage> MessageValidator<T> {
  #[must_use]
  #[inline]
  pub fn builder() -> MessageValidatorBuilder<T> {
    MessageValidatorBuilder::default()
  }
}

impl<T: ValidatedMessage, S: State> From<MessageValidatorBuilder<T, S>> for ProtoOption {
  fn from(value: MessageValidatorBuilder<T, S>) -> Self {
    value.build().into()
  }
}

#[allow(
  clippy::must_use_candidate,
  clippy::use_self,
  clippy::return_self_not_must_use
)]
impl<T: ValidatedMessage, S: State> MessageValidatorBuilder<T, S> {
  #[inline]
  pub fn cel(mut self, program: CelProgram) -> MessageValidatorBuilder<T, S> {
    self.data.cel.push(program);

    MessageValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  #[inline]
  pub fn ignore_always(mut self) -> MessageValidatorBuilder<T, SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    self.data.ignore = Ignore::Always;

    MessageValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  #[inline]
  pub fn required(mut self) -> MessageValidatorBuilder<T, SetRequired<S>>
  where
    S::Required: IsUnset,
  {
    self.data.required = true;

    MessageValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  #[inline]
  pub fn build(self) -> MessageValidator<T> {
    self.data
  }
}
