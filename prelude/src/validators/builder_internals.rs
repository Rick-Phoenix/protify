use crate::*;

pub struct EmptyBuilder;

#[allow(clippy::from_over_into)]
impl Into<ProtoOption> for EmptyBuilder {
  fn into(self) -> ProtoOption {
    ProtoOption {
      name: "".into(),
      value: OptionValue::Bool(true),
    }
  }
}

impl<T> ValidatorBuilderFor<T> for EmptyBuilder {}

pub struct Empty;

pub trait IsUnset {}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Set<T>(PhantomData<fn() -> T>);
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Unset<T>(PhantomData<fn() -> T>);

#[doc(hidden)]
impl<T> IsUnset for Unset<T> {}
