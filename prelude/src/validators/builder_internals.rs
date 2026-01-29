use crate::*;

#[doc(hidden)]
#[derive(Default)]
pub struct Empty;

#[doc(hidden)]
pub trait IsUnset {}
#[doc(hidden)]
pub trait IsSet {}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Set<T>(PhantomData<fn() -> T>);
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Unset<T>(PhantomData<fn() -> T>);

#[doc(hidden)]
impl<T> IsUnset for Unset<T> {}
#[doc(hidden)]
impl<T> IsSet for Set<T> {}
