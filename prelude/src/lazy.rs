use alloc::boxed::Box;
use once_cell::race::OnceBox;

use crate::*;

pub struct Lazy<T, F = fn() -> T> {
  cell: OnceBox<T>,
  init: F,
}

impl<T, F> Lazy<T, F> {
  pub const fn new(f: F) -> Self {
    Self {
      cell: OnceBox::new(),
      init: f,
    }
  }
}

impl<T, F: Fn() -> T> Deref for Lazy<T, F> {
  type Target = T;

  fn deref(&self) -> &T {
    self.cell.get_or_init(|| Box::new((self.init)()))
  }
}
