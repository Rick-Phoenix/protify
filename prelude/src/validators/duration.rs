use bon::Builder;
use duration_validator_builder::{IsUnset, SetIgnore, State};
use proto_types::Duration;

use super::*;

impl_validator!(DurationValidator, Duration);
impl_into_option!(DurationValidator);

/// Used by the [`duration`](crate::duration) macro to define validation rules.
#[derive(Clone, Debug, Builder)]
pub struct DurationValidator {
  /// Only the values in this list will be considered valid for this field.
  #[builder(into)]
  pub in_: Option<Arc<[Duration]>>,
  /// The values in this list will be considered invalid for this field.
  #[builder(into)]
  pub not_in: Option<Arc<[Duration]>>,
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<Duration>,
  /// This field's value will be valid only if it is smaller than the specified amount
  pub lt: Option<Duration>,
  /// This field's value will be valid only if it is smaller than, or equal to, the specified amount
  pub lte: Option<Duration>,
  /// This field's value will be valid only if it is greater than the specified amount
  pub gt: Option<Duration>,
  /// This field's value will be valid only if it is greater than, or equal to, the specified amount
  pub gte: Option<Duration>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Arc<[CelRule]>>,
  #[builder(with = || true)]
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl<S: State> DurationValidatorBuilder<S>
where
  S::Ignore: IsUnset,
{
  /// Rules set for this field will always be ignored.
  pub fn ignore_always(self) -> DurationValidatorBuilder<SetIgnore<S>> {
    self.ignore(Ignore::Always)
  }
}

impl From<DurationValidator> for ProtoOption {
  #[track_caller]
  fn from(validator: DurationValidator) -> Self {
    let mut rules: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      rules.push((CONST_.clone(), OptionValue::Duration(const_val)));
    }

    validate_comparables(validator.lt, validator.lte, validator.gt, validator.gte).unwrap();
    validate_lists(validator.in_.as_deref(), validator.not_in.as_deref()).unwrap();

    insert_option!(validator, rules, lt);
    insert_option!(validator, rules, lte);
    insert_option!(validator, rules, gt);
    insert_option!(validator, rules, gte);
    insert_option!(validator, rules, in_);
    insert_option!(validator, rules, not_in);

    let mut outer_rules: OptionValueList = vec![];

    outer_rules.push((DURATION.clone(), OptionValue::Message(rules.into())));

    insert_cel_rules!(validator, outer_rules);
    insert_option!(validator, outer_rules, required);
    insert_option!(validator, outer_rules, ignore);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}
