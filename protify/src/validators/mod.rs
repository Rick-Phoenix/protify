use crate::*;

use proto_types::protovalidate::*;

/// Utility sealed trait for [`ValidationResult`] that makes it easier to discern the validation status.
pub trait ValidationResultExt {
  #[doc(hidden)]
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  /// Returns `true` if the result is [`Ok`] and [`IsValid::Yes`], and `false` otherwise.
  fn is_valid(&self) -> bool;
  /// Checks if the result is [`Err`] with [`FailFast`].
  fn is_fail_fast(&self) -> bool;
}

impl ValidationResultExt for ValidationResult {
  #[doc(hidden)]
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  #[inline]
  fn is_valid(&self) -> bool {
    match self {
      Ok(outcome) => matches!(outcome, IsValid::Yes),
      Err(_) => false,
    }
  }

  #[inline]
  fn is_fail_fast(&self) -> bool {
    self.is_err()
  }
}

/// Unit struct to signal the `fail fast` setting and trigger early returns with [`ValidationResult`].
#[derive(Debug, Clone, Copy)]
pub struct FailFast;

bool_enum!(pub IsValid, doc = r"A boolean-like enum that represents validation status. It supports all of the bit operators.

For more information on the boolean-like features of this enum, you can refer to the [`bool-enum`](::bool_enum) documentation.
");

impl IsValid {
  #[must_use]
  #[inline]
  pub fn is_valid(&self) -> bool {
    **self
  }
}

/// Alias for `Result<IsValid, FailFast>.`
pub type ValidationResult = Result<IsValid, FailFast>;

/// The trait implemented by the validators, which can be structs or functions used with [`FnValidator`].
///
/// The generic type and the Target type are separated so that this can be used for wrapper types
/// such as [`Sint32`] and others.
///
/// The `Target` is the actual type that will be validated.
///
/// The only required method is [`execute_validation`](Validator::execute_validation), all the other validation methods are
/// automatically implemented.
///
/// Validators can optionally implement the [`schema`](Validator::schema) method, which allows them to be
/// turned into protobuf options for file generation.
pub trait Validator<T: ?Sized>: Send + Sync {
  /// The target of the validation.
  type Target: ToOwned + ?Sized;

  #[doc(hidden)]
  #[inline(never)]
  #[cold]
  // This is necessary to gather rules from validators like repeated or map which need to gather nested rules
  fn cel_rules(&self) -> Vec<CelRule> {
    vec![]
  }

  /// Returns the optional schema representation for this validator.
  ///
  /// If a schema representation is present, whenever a validator is used by a message or a oneof,
  /// its schema representation will be present in the generated protobuf files.
  #[inline(never)]
  #[cold]
  fn schema(&self) -> Option<ValidatorSchema> {
    None
  }

  /// Checks if the inputs of the validators are valid.
  #[inline(never)]
  #[cold]
  fn check_consistency(&self) -> Result<(), Vec<ConsistencyError>> {
    Ok(())
  }

  /// Tests the CEL programs of this validator with the given value.
  ///
  /// # Panics
  ///
  /// Panics if one of the CEL expressions failed to compile.
  #[cfg(feature = "cel")]
  #[inline(never)]
  #[cold]
  fn check_cel_programs_with(
    &self,
    _val: <Self::Target as ToOwned>::Owned,
  ) -> Result<(), Vec<CelError>> {
    Ok(())
  }

  #[cfg(feature = "cel")]
  #[inline(never)]
  #[cold]
  #[doc(hidden)]
  fn check_cel_programs(&self) -> Result<(), Vec<CelError>> {
    Ok(())
  }

  /// Validates the Target, with the `fail_fast` setting set to true.
  #[inline]
  fn validate(&self, val: &Self::Target) -> Result<(), ValidationErrors> {
    let mut ctx = ValidationCtx::default();

    let _ = self.execute_validation(&mut ctx, Some(val));

    if ctx.violations.is_empty() {
      Ok(())
    } else {
      Err(ctx.violations)
    }
  }

  /// Validates the Target, with the `fail_fast` setting set to true.
  #[inline]
  fn validate_option(&self, val: Option<&Self::Target>) -> Result<(), ValidationErrors> {
    let mut ctx = ValidationCtx::default();

    let _ = self.execute_validation(&mut ctx, val);

    if ctx.violations.is_empty() {
      Ok(())
    } else {
      Err(ctx.violations)
    }
  }

  /// Validates the Target, with customized settings.
  #[inline]
  fn validate_with_ctx(
    &self,
    mut ctx: ValidationCtx,
    val: &Self::Target,
  ) -> Result<(), ValidationErrors> {
    let _ = self.execute_validation(&mut ctx, Some(val));

    if ctx.violations.is_empty() {
      Ok(())
    } else {
      Err(ctx.violations)
    }
  }

  /// Validates the Target, with customized settings.
  #[inline]
  fn validate_option_with_ctx(
    &self,
    mut ctx: ValidationCtx,
    val: Option<&Self::Target>,
  ) -> Result<(), ValidationErrors> {
    let _ = self.execute_validation(&mut ctx, val);

    if ctx.violations.is_empty() {
      Ok(())
    } else {
      Err(ctx.violations)
    }
  }

  /// Validates the Target, with customized settings.
  fn execute_validation(
    &self,
    ctx: &mut ValidationCtx,
    val: Option<&Self::Target>,
  ) -> ValidationResult;
}

/// Utility trait for validator builders. Mainly used for default validators.
pub trait ValidatorBuilderFor<T: ?Sized>: Default {
  type Validator: Validator<T>;

  /// Builds the associated [`Validator`].
  fn build_validator(self) -> Self::Validator;
}

/// Trait implemented by targets of proto validators.
///
/// Implemented automatically with the [`proto_message`], [`proto_oneof`] and [`proto_enum`] macros.
///
/// The actual target of the validation is in a dedicated associated Type
/// because this trait is sometimes implemented by wrappers/proxies like [`Sint32`].
pub trait ProtoValidation {
  /// The type that the implementor is actually going to be validated with.
  ///
  /// Defined as a separate type to allow target types to support more than one implementor
  /// (for example, i32 can be the target for [`Sint32`], [`Sfixed32`] or for i32 itself).
  type Target: ?Sized;
  /// The `Stored` type is needed for compatibility with the [`RepeatedValidator`]
  /// and [`MapValidator`]. It is the same as the `Target` in most cases, but
  /// not for [`String`] specifically, because the Target is `str`, but `Stored` is `String`.
  type Stored: Borrow<Self::Target>;
  /// Represent the **default** validator for this type.
  type Validator: Validator<Self, Target = Self::Target> + Clone + Default;
  /// Represent the builder for the default validator of this type, to enable the utility methods
  /// [`validator_builder`](ProtoValidation::validator_builder) and
  /// [`validator_from_closure`](ProtoValidation::validator_from_closure).
  type ValidatorBuilder: ValidatorBuilderFor<Self, Validator = Self::Validator>;

  #[doc(hidden)]
  const HAS_DEFAULT_VALIDATOR: bool = false;
  #[doc(hidden)]
  const HAS_SHALLOW_VALIDATION: bool = false;

  #[doc(hidden)]
  type UniqueStore<'a>: UniqueStore<'a, Item = Self::Target>
  where
    Self: 'a;

  #[inline]
  #[doc(hidden)]
  fn make_unique_store<'a>(_validator: &Self::Validator, cap: usize) -> Self::UniqueStore<'a>
  where
    Self: 'a,
  {
    Self::UniqueStore::default_with_capacity(cap)
  }

  /// Returns the default validator builder for this type.
  ///
  /// # Example
  ///
  /// ```
  /// use protify::*;
  ///
  /// assert_eq!(StringValidator::builder(), String::validator_builder());
  /// ```
  #[inline]
  #[must_use]
  fn validator_builder() -> Self::ValidatorBuilder {
    Self::ValidatorBuilder::default()
  }

  /// Builds the default validator for this type from a closure which receives the validator
  /// builder as the argument.
  ///
  /// # Example
  ///
  /// ```
  /// use protify::*;
  ///
  /// let validator = String::validator_from_closure(|v| v.min_len(3));
  /// let validator2 = StringValidator::builder().min_len(3).build();
  ///
  /// assert_eq!(validator, validator2);
  /// ```
  #[inline]
  fn validator_from_closure<F, FinalBuilder>(config_fn: F) -> Self::Validator
  where
    F: FnOnce(Self::ValidatorBuilder) -> FinalBuilder,
    FinalBuilder: ValidatorBuilderFor<Self, Validator = Self::Validator>,
  {
    let initial_builder = Self::validator_builder();

    config_fn(initial_builder).build_validator()
  }
}

pub(crate) trait IsDefault: Default + PartialEq {
  fn is_default(&self) -> bool {
    (*self) == Self::default()
  }
}
impl<T: Default + PartialEq> IsDefault for T {}

/// Implements [`Validator`] for the wrapped function, similarly to [`FromFn`](core::iter::FromFn).
///
/// It can be created with the [`from_fn`] function.
pub struct FnValidator<F, T: ?Sized> {
  func: F,
  _phantom: PhantomData<T>,
}

impl<F, T> Validator<T> for FnValidator<F, T>
where
  T: ToOwned + ?Sized + Send + Sync,
  F: Fn(&mut ValidationCtx, Option<&T>) -> ValidationResult + Send + Sync,
{
  type Target = T;

  #[inline]
  fn execute_validation(
    &self,
    ctx: &mut ValidationCtx,
    val: Option<&Self::Target>,
  ) -> ValidationResult {
    (self.func)(ctx, val)
  }
}

/// Creates a validator from a function, using [`FnValidator`].
#[inline]
pub const fn from_fn<T, F>(f: F) -> FnValidator<F, T>
where
  T: ?Sized,
  F: Fn(&mut ValidationCtx, Option<&T>) -> ValidationResult,
{
  FnValidator {
    func: f,
    _phantom: PhantomData,
  }
}

/// Stores custom error messages in default validators.
type ErrorMessages<T> = Box<BTreeMap<T, FixedStr>>;

mod builder_internals;
#[doc(hidden)]
use builder_internals::*;

pub mod any;
pub use any::*;

pub mod bool;
pub use bool::*;

pub mod bytes;
pub use bytes::*;

mod cel;
#[doc(inline)]
pub use cel::*;

pub mod duration;
pub use duration::*;

pub mod enums;
pub use enums::*;

mod field_context;
#[doc(inline)]
pub use field_context::*;

pub mod map;
pub use map::*;

pub mod message;
pub use message::*;

pub mod repeated;
pub use repeated::*;

pub mod string;
pub use string::*;

pub mod timestamp;
pub use timestamp::*;

pub mod floats;
pub use floats::*;

pub mod integers;
pub use integers::*;

pub mod field_mask;
pub use field_mask::*;

mod lookup;
pub use lookup::*;

mod oneof;
#[doc(inline)]
pub use oneof::*;

mod violations;
#[doc(inline)]
pub use violations::*;
