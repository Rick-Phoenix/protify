use super::*;

mod floats;
mod integers;

pub use floats::*;
pub use integers::*;

impl<N> ProtoValidator<IntValidator<N>> for ValidatorMap
where
  N: IntWrapper,
{
  type Builder = IntValidatorBuilder<N>;

  fn builder() -> IntValidatorBuilder<N> {
    IntValidator::builder()
  }
}

impl<N> ProtoValidator<FloatValidator<N>> for ValidatorMap
where
  N: FloatWrapper,
{
  type Builder = FloatValidatorBuilder<N>;

  fn builder() -> FloatValidatorBuilder<N> {
    FloatValidator::builder()
  }
}
