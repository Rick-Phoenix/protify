use any_validator_builder::{IsUnset, SetIgnore, SetIn, SetNotIn, State};
use bon::Builder;
use proto_types::Any;

use super::*;

impl_validator!(AnyValidator, Any);
impl_into_option!(AnyValidator);

#[derive(Clone, Debug, Builder)]
pub struct AnyValidator {
  /// Only the type_urls defined in this list will be considered valid for this field.
  #[builder(setters(vis = "", name = in_internal))]
  pub in_: Option<Arc<[Arc<str>]>>,
  /// The type_urls defined in this list will be considered invalid for this field.
  #[builder(setters(vis = "", name = not_in_internal))]
  pub not_in: Option<Arc<[Arc<str>]>>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Arc<[CelRule]>>,
  /// Marks the field as invalid if unset.
  #[builder(with = || true)]
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl<S: State> AnyValidatorBuilder<S>
where
  S::In: IsUnset,
{
  /// Only the type URLs contained in this list will be considered valid
  pub fn in_<T: Into<Arc<str>>, I: IntoIterator<Item = T>>(
    self,
    list: I,
  ) -> AnyValidatorBuilder<SetIn<S>> {
    let list = create_string_list(list);
    self.in_internal(list)
  }
}

impl<S: State> AnyValidatorBuilder<S>
where
  S::NotIn: IsUnset,
{
  /// Only the type URLs not contained in this list will be considered valid
  pub fn not_in<T: Into<Arc<str>>, I: IntoIterator<Item = T>>(
    self,
    list: I,
  ) -> AnyValidatorBuilder<SetNotIn<S>> {
    let list = create_string_list(list);
    self.not_in_internal(list)
  }
}

impl<S: State> AnyValidatorBuilder<S>
where
  S::Ignore: IsUnset,
{
  /// Rules set for this field will always be ignored.
  pub fn ignore_always(self) -> AnyValidatorBuilder<SetIgnore<S>> {
    self.ignore(Ignore::Always)
  }
}

impl From<AnyValidator> for ProtoOption {
  #[track_caller]
  fn from(validator: AnyValidator) -> Self {
    let mut rules: OptionValueList = Vec::new();

    validate_lists(validator.in_.as_deref(), validator.not_in.as_deref()).unwrap();

    insert_option!(validator, rules, in_);
    insert_option!(validator, rules, not_in);

    let mut outer_rules: OptionValueList = vec![];

    outer_rules.push((ANY.clone(), OptionValue::Message(rules.into())));

    insert_cel_rules!(validator, outer_rules);
    insert_option!(validator, outer_rules, required);
    insert_option!(validator, outer_rules, ignore);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}
