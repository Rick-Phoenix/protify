#![allow(private_interfaces)]

use std::marker::PhantomData;

use crate::validators::builder_internals::*;

mod sealed {
  pub(super) struct Sealed;
}

mod members {
  pub struct Items;
  pub struct MinItems;
  pub struct MaxItems;
  pub struct Unique;
  pub struct Ignore;
}

pub trait State<S = Empty> {
  type Items;
  type MinItems;
  type MaxItems;
  type Unique;
  type Ignore;
  const SEALED: sealed::Sealed;
}

pub struct SetItems<S: State = Empty>(PhantomData<fn() -> S>);
pub struct SetMinItems<S: State = Empty>(PhantomData<fn() -> S>);
pub struct SetMaxItems<S: State = Empty>(PhantomData<fn() -> S>);
pub struct SetUnique<S: State = Empty>(PhantomData<fn() -> S>);
pub struct SetIgnore<S: State = Empty>(PhantomData<fn() -> S>);

#[doc(hidden)]
impl State for Empty {
  type Items = Unset<members::Items>;
  type MinItems = Unset<members::MinItems>;
  type MaxItems = Unset<members::MaxItems>;
  type Unique = Unset<members::Unique>;
  type Ignore = Unset<members::Ignore>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: State> State for SetItems<S> {
  type Items = Set<members::Items>;
  type MinItems = S::MinItems;
  type MaxItems = S::MaxItems;
  type Unique = S::Unique;
  type Ignore = S::Ignore;
  const SEALED: sealed::Sealed = sealed::Sealed;
}

#[doc(hidden)]
impl<S: State> State for SetUnique<S> {
  type Items = S::Items;
  type MinItems = S::MinItems;
  type MaxItems = S::MaxItems;
  type Unique = Set<members::Unique>;
  type Ignore = S::Ignore;
  const SEALED: sealed::Sealed = sealed::Sealed;
}
#[doc(hidden)]
impl<S: State> State for SetMinItems<S> {
  type Items = S::Items;
  type Unique = S::Unique;
  type MinItems = Set<members::MinItems>;
  type MaxItems = S::MaxItems;
  type Ignore = S::Ignore;
  const SEALED: sealed::Sealed = sealed::Sealed;
}
#[doc(hidden)]
impl<S: State> State for SetMaxItems<S> {
  type Items = S::Items;
  type Unique = S::Unique;
  type MinItems = S::MinItems;
  type MaxItems = Set<members::MaxItems>;
  type Ignore = S::Ignore;
  const SEALED: sealed::Sealed = sealed::Sealed;
}
#[doc(hidden)]
impl<S: State> State for SetIgnore<S> {
  type Items = S::Items;
  type Unique = S::Unique;
  type MinItems = S::MinItems;
  type MaxItems = S::MaxItems;
  type Ignore = Set<members::Ignore>;
  const SEALED: sealed::Sealed = sealed::Sealed;
}
