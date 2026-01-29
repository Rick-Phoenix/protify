mod builder;
pub use builder::OneofValidatorBuilder;

use super::*;

#[non_exhaustive]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct OneofValidator {
  /// Specifies that at least one of the variants of the oneof must be set.
  pub required: bool,
  /// Specifies a custom error message to display for the `required` violation.
  pub error_message: Option<FixedStr>,
}

impl OneofValidator {
  #[inline]
  #[must_use]
  pub fn builder() -> OneofValidatorBuilder {
    OneofValidatorBuilder::default()
  }
}

impl<T: ValidatedOneof> Validator<T> for OneofValidator {
  type Target = T;

  #[cold]
  #[inline(never)]
  fn schema(&self) -> Option<ValidatorSchema> {
    self.required.then(|| ValidatorSchema {
      schema: ProtoOption {
        name: "(buf.validate.oneof).required".into(),
        value: true.into(),
      },
      cel_rules: vec![],
      imports: vec!["buf/validate/validate.proto".into()],
    })
  }

  // Should be inlined because if the assoc. constant is false, it may promote
  // dead code elimination.
  #[inline]
  fn validate_core<V>(&self, ctx: &mut ValidationCtx, val: Option<&V>) -> ValidationResult
  where
    V: Borrow<Self::Target> + ?Sized,
  {
    match val {
      Some(oneof) => {
        if T::HAS_DEFAULT_VALIDATOR {
          oneof.borrow().validate(ctx)
        } else {
          Ok(IsValid::Yes)
        }
      }
      None => {
        if self.required {
          ctx.add_required_oneof_violation(self.error_message.as_ref().map(|e| e.to_string()))
        } else {
          Ok(IsValid::Yes)
        }
      }
    }
  }
}
