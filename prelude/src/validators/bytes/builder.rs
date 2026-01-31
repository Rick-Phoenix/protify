#[doc(hidden)]
pub mod state;
use crate::validators::*;
pub(crate) use state::*;

use ::bytes::Bytes;

/// Builder for [`BytesValidator`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BytesValidatorBuilder<S: State = Empty> {
  _state: PhantomData<S>,
  data: BytesValidator,
}

impl ProtoValidation for Bytes {
  #[doc(hidden)]
  type Target = [u8];
  #[doc(hidden)]
  type Stored = Self;
  type Validator = BytesValidator;
  type ValidatorBuilder = BytesValidatorBuilder;

  #[doc(hidden)]
  type UniqueStore<'a>
    = RefHybridStore<'a, [u8]>
  where
    Self: 'a;

  #[doc(hidden)]
  const HAS_DEFAULT_VALIDATOR: bool = false;
}

impl<S: State> ValidatorBuilderFor<Bytes> for BytesValidatorBuilder<S> {
  type Validator = BytesValidator;
  #[inline]
  fn build_validator(self) -> BytesValidator {
    self.build()
  }
}

impl<S: State> Default for BytesValidatorBuilder<S> {
  #[inline]
  fn default() -> Self {
    Self {
      _state: PhantomData,
      data: BytesValidator::default(),
    }
  }
}

impl<S: State> From<BytesValidatorBuilder<S>> for ProtoOption {
  #[inline(never)]
  #[cold]
  fn from(value: BytesValidatorBuilder<S>) -> Self {
    value.build().into()
  }
}

impl BytesValidator {
  #[must_use]
  #[inline]
  pub fn builder() -> BytesValidatorBuilder {
    BytesValidatorBuilder::default()
  }
}

macro_rules! well_known_impl {
  ($name:ident, $doc:literal) => {
    paste::paste! {
      #[inline]
      #[doc = $doc]
      #[allow(rustdoc::bare_urls)]
      pub fn [< $name:snake >](mut self) -> BytesValidatorBuilder<SetWellKnown<S>>
        where
          S::WellKnown: IsUnset,
        {
          self.data.well_known = Some(WellKnownBytes::$name);

          BytesValidatorBuilder {
            _state: PhantomData,
            data: self.data
          }
        }
    }
  };
}

impl<S: State> BytesValidatorBuilder<S> {
  well_known_impl!(
    Ip,
    "Specifies that the value must be a valid IP address (v4 or v6) in byte format."
  );
  well_known_impl!(
    Ipv4,
    "Specifies that the value must be a valid IPv4 address in byte format."
  );
  well_known_impl!(
    Ipv6,
    "Specifies that the value must be a valid IPv6 address in byte format."
  );
  #[cfg(feature = "regex")]
  well_known_impl!(
    Uuid,
    "Specifies that the value must be a valid UUID in byte format."
  );
}

#[allow(
  clippy::must_use_candidate,
  clippy::use_self,
  clippy::return_self_not_must_use
)]
impl<S: State> BytesValidatorBuilder<S> {
  custom_error_messages_method!(Bytes);

  /// Specifies that this validator should always be ignored.
  #[inline]
  pub fn ignore_always(mut self) -> BytesValidatorBuilder<SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    self.data.ignore = Ignore::Always;

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that this validator should be ignored if the value is either unset or equal to its protobuf zero value.
  #[inline]
  pub fn ignore_if_zero_value(mut self) -> BytesValidatorBuilder<SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    self.data.ignore = Ignore::IfZeroValue;

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Adds a [`CelProgram`] to this validator.
  #[inline]
  pub fn cel(mut self, program: CelProgram) -> BytesValidatorBuilder<S> {
    self.data.cel.push(program);

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the given `bytes` field must be of this exact length.
  #[inline]
  pub fn len(mut self, val: usize) -> BytesValidatorBuilder<SetLen<S>>
  where
    S::Len: IsUnset,
  {
    self.data.len = Some(val);

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the given `bytes` field must have a length that is equal to or higher than the given value.
  #[inline]
  pub fn min_len(mut self, val: usize) -> BytesValidatorBuilder<SetMinLen<S>>
  where
    S::MinLen: IsUnset,
  {
    self.data.min_len = Some(val);

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the given `bytes` field must have a length that is equal to or lower than the given value.
  #[inline]
  pub fn max_len(mut self, val: usize) -> BytesValidatorBuilder<SetMaxLen<S>>
  where
    S::MaxLen: IsUnset,
  {
    self.data.max_len = Some(val);

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies a regex pattern that must be matches by the value to pass validation.
  #[cfg(feature = "regex")]
  #[inline]
  pub fn pattern(mut self, val: impl IntoBytesRegex) -> BytesValidatorBuilder<SetPattern<S>>
  where
    S::Pattern: IsUnset,
  {
    self.data.pattern = Some(val.into_regex());

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies a prefix that the value must start with in order to pass validation.
  #[inline]
  pub fn prefix(mut self, val: impl IntoBytes) -> BytesValidatorBuilder<SetPrefix<S>>
  where
    S::Prefix: IsUnset,
  {
    self.data.prefix = Some(val.into_bytes());

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies a suffix that the value must end with in order to pass validation.
  #[inline]
  pub fn suffix(mut self, val: impl IntoBytes) -> BytesValidatorBuilder<SetSuffix<S>>
  where
    S::Suffix: IsUnset,
  {
    self.data.suffix = Some(val.into_bytes());

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies a subset of bytes that the value must contain in order to pass validation.
  #[inline]
  pub fn contains(mut self, val: impl IntoBytes) -> BytesValidatorBuilder<SetContains<S>>
  where
    S::Contains: IsUnset,
  {
    self.data.contains = Some(val.into_bytes());

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the values in this list will be considered NOT valid for this field.
  #[inline]
  pub fn not_in(mut self, list: impl IntoSortedList<Bytes>) -> BytesValidatorBuilder<SetNotIn<S>>
  where
    S::NotIn: IsUnset,
  {
    self.data.not_in = Some(list.into_sorted_list());

    BytesValidatorBuilder {
      _state: PhantomData,

      data: self.data,
    }
  }

  /// Specifies that only the values in this list will be considered valid for this field.
  #[inline]
  pub fn in_(mut self, list: impl IntoSortedList<Bytes>) -> BytesValidatorBuilder<SetIn<S>>
  where
    S::In: IsUnset,
  {
    self.data.in_ = Some(list.into_sorted_list());

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that only this specific value will be considered valid for this field.
  #[inline]
  pub fn const_(mut self, val: impl IntoBytes) -> BytesValidatorBuilder<SetConst<S>>
  where
    S::Const: IsUnset,
  {
    self.data.const_ = Some(val.into_bytes());

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Specifies that the field must be set (if optional) or not equal to its zero value (if not optional) in order to be valid.
  #[inline]
  pub fn required(mut self) -> BytesValidatorBuilder<SetRequired<S>>
  where
    S::Required: IsUnset,
  {
    self.data.required = true;

    BytesValidatorBuilder {
      _state: PhantomData,
      data: self.data,
    }
  }

  /// Builds the validator.
  #[inline]
  pub fn build(self) -> BytesValidator {
    self.data
  }
}
