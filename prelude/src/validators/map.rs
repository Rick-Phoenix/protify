use std::{collections::HashMap, marker::PhantomData};

use map_validator_builder::{
  SetCel, SetIgnore, SetKeys, SetMaxPairs, SetMinPairs, SetValues, State,
};

use super::*;

pub struct ProtoMap<K, V>(PhantomData<K>, PhantomData<V>);

macro_rules! impl_map {
  ($name:ident) => {
    impl_map_validator!($name);

    impl<K: AsProtoField, V: AsProtoField> AsProtoField for $name<K, V> {
      fn as_proto_field() -> ProtoFieldInfo {
        let keys = match K::as_proto_field() {
          ProtoFieldInfo::Single(data) => data,
          _ => invalid_type_output("Map keys cannot be repeated, optional or nested maps"),
        };

        let values = match V::as_proto_field() {
          ProtoFieldInfo::Single(data) => data,
          _ => invalid_type_output("Map values cannot be repeated, optional or nested maps"),
        };

        ProtoFieldInfo::Map { keys, values }
      }
    }
  };
}

macro_rules! impl_map_validator {
  ($name:ident) => {
    impl<K, V> ProtoValidator<$name<K, V>> for ValidatorMap
    where
      ValidatorMap: ProtoValidator<K>,
      ValidatorMap: ProtoValidator<V>,
    {
      type Builder = MapValidatorBuilder<K, V>;

      fn builder() -> Self::Builder {
        MapValidator::builder()
      }
    }

    impl<K, V, KB, VB, S: State> ValidatorBuilderFor<$name<K, V>>
      for MapValidatorBuilder<K, V, KB, VB, S>
    where
      KB: ValidatorBuilderFor<K>,
      VB: ValidatorBuilderFor<V>,
    {
    }
  };
}

impl_map!(ProtoMap);
impl_map!(HashMap);

#[derive(Clone, Debug)]
pub struct MapValidator<K, V, KB = EmptyBuilder, VB = EmptyBuilder>
where
  KB: ValidatorBuilderFor<K>,
  VB: ValidatorBuilderFor<V>,
{
  /// The validation rules to apply to the keys of this map field.
  pub keys: Option<KB>,

  _key_type: PhantomData<K>,
  _value_type: PhantomData<V>,

  /// The validation rules to apply to the keys of this map field.
  pub values: Option<VB>,
  /// The minimum amount of key-value pairs that this field should have in order to be valid.
  pub min_pairs: Option<usize>,
  /// The maximum amount of key-value pairs that this field should have in order to be valid.
  pub max_pairs: Option<usize>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  /// These will apply to the map field as a whole.
  /// To apply cel rules to the individual keys or values, use the validators for those instead.
  pub cel: Option<Arc<[CelRule]>>,
  pub ignore: Option<Ignore>,
}

impl<K, V, KB, VB> MapValidator<K, V, KB, VB>
where
  KB: ValidatorBuilderFor<K>,
  VB: ValidatorBuilderFor<V>,
{
  pub fn builder() -> MapValidatorBuilder<K, V, KB, VB> {
    MapValidatorBuilder {
      _state: PhantomData,
      _key_type: PhantomData,
      _value_type: PhantomData,
      values: None,
      keys: None,
      min_pairs: None,
      max_pairs: None,
      cel: None,
      ignore: None,
    }
  }
}

#[derive(Clone, Debug, Default)]
pub struct MapValidatorBuilder<K, V, KB = EmptyBuilder, VB = EmptyBuilder, S: State = Empty>
where
  KB: ValidatorBuilderFor<K>,
  VB: ValidatorBuilderFor<V>,
{
  pub keys: Option<KB>,

  _state: PhantomData<S>,
  _key_type: PhantomData<K>,
  _value_type: PhantomData<V>,

  pub values: Option<VB>,
  pub min_pairs: Option<usize>,
  pub max_pairs: Option<usize>,
  pub cel: Option<Arc<[CelRule]>>,
  pub ignore: Option<Ignore>,
}

impl<S: State, K, V, KB, VB> MapValidatorBuilder<K, V, KB, VB, S>
where
  KB: ValidatorBuilderFor<K>,
  VB: ValidatorBuilderFor<V>,
{
  pub fn build(self) -> MapValidator<K, V, KB, VB> {
    let Self {
      keys,
      _key_type,
      _value_type,
      values,
      min_pairs,
      max_pairs,
      cel,
      ignore,
      ..
    } = self;

    MapValidator {
      keys,
      _key_type,
      _value_type,
      values,
      min_pairs,
      max_pairs,
      cel,
      ignore,
    }
  }

  pub fn cel(self, rules: impl Into<Arc<[CelRule]>>) -> MapValidatorBuilder<K, V, KB, VB, SetCel<S>>
  where
    S::Cel: IsUnset,
  {
    MapValidatorBuilder {
      _state: PhantomData,
      keys: self.keys,
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: self.min_pairs,
      max_pairs: self.max_pairs,
      cel: Some(rules.into()),
      ignore: self.ignore,
    }
  }

  pub fn min_pairs(self, num: usize) -> MapValidatorBuilder<K, V, KB, VB, SetMinPairs<S>>
  where
    S::MinPairs: IsUnset,
  {
    MapValidatorBuilder {
      _state: PhantomData,
      keys: self.keys,
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: Some(num),
      max_pairs: self.max_pairs,
      cel: self.cel,
      ignore: self.ignore,
    }
  }

  pub fn max_pairs(self, num: usize) -> MapValidatorBuilder<K, V, KB, VB, SetMaxPairs<S>>
  where
    S::MaxPairs: IsUnset,
  {
    MapValidatorBuilder {
      _state: PhantomData,
      keys: self.keys,
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: self.min_pairs,
      max_pairs: Some(num),
      cel: self.cel,
      ignore: self.ignore,
    }
  }

  /// Rules set for this field will always be ignored.
  pub fn ignore_always(self) -> MapValidatorBuilder<K, V, KB, VB, SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    MapValidatorBuilder {
      _state: PhantomData,
      keys: self.keys,
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: self.min_pairs,
      max_pairs: self.max_pairs,
      cel: self.cel,
      ignore: Some(Ignore::Always),
    }
  }

  /// Sets the rules for the keys of this map field.
  pub fn keys<F, NewKB>(self, config_fn: F) -> MapValidatorBuilder<K, V, NewKB, VB, SetKeys<S>>
  where
    S::Keys: IsUnset,
    ValidatorMap: ProtoValidator<K>,
    NewKB: ValidatorBuilderFor<K>,
    F: FnOnce(<ValidatorMap as ProtoValidator<K>>::Builder) -> NewKB,
  {
    let keys_opts = ValidatorMap::builder_from_closure(config_fn);

    MapValidatorBuilder {
      _state: PhantomData,
      keys: Some(keys_opts),
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: self.min_pairs,
      max_pairs: self.max_pairs,
      cel: self.cel,
      ignore: self.ignore,
    }
  }

  /// Sets the rules for the values of this map field.
  pub fn values<F, NewVB>(self, config_fn: F) -> MapValidatorBuilder<K, V, KB, NewVB, SetValues<S>>
  where
    ValidatorMap: ProtoValidator<V>,
    NewVB: ValidatorBuilderFor<V>,
    F: FnOnce(<ValidatorMap as ProtoValidator<V>>::Builder) -> NewVB,
  {
    let values_opts = ValidatorMap::builder_from_closure(config_fn);

    MapValidatorBuilder {
      _state: PhantomData,
      keys: self.keys,
      values: Some(values_opts),
      _key_type: self._key_type,
      _value_type: self._value_type,
      min_pairs: self.min_pairs,
      max_pairs: self.max_pairs,
      cel: self.cel,
      ignore: self.ignore,
    }
  }
}

impl<K, V, KB, VB, S: State> From<MapValidatorBuilder<K, V, KB, VB, S>> for ProtoOption
where
  KB: ValidatorBuilderFor<K>,
  VB: ValidatorBuilderFor<V>,
{
  fn from(value: MapValidatorBuilder<K, V, KB, VB, S>) -> Self {
    value.build().into()
  }
}

impl<KeyItems, ValueItems, KB, VB> From<MapValidator<KeyItems, ValueItems, KB, VB>> for ProtoOption
where
  KB: ValidatorBuilderFor<KeyItems>,
  VB: ValidatorBuilderFor<ValueItems>,
{
  fn from(validator: MapValidator<KeyItems, ValueItems, KB, VB>) -> Self {
    let mut rules: OptionValueList = Vec::new();

    insert_option!(validator, rules, min_pairs);
    insert_option!(validator, rules, max_pairs);

    if let Some(keys_option) = validator.keys {
      let keys_schema: ProtoOption = keys_option.into();

      rules.push((KEYS.clone(), keys_schema.value));
    }

    if let Some(values_option) = validator.values {
      let values_schema: ProtoOption = values_option.into();

      rules.push((VALUES.clone(), values_schema.value));
    }

    let mut outer_rules: OptionValueList = vec![];

    outer_rules.push((MAP.clone(), OptionValue::Message(rules.into())));

    insert_cel_rules!(validator, outer_rules);
    insert_option!(validator, outer_rules, ignore);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}

#[allow(private_interfaces)]
mod map_validator_builder {
  use std::marker::PhantomData;

  use crate::validators::builder_internals::*;

  mod members {
    pub struct Keys;
    pub struct Values;
    pub struct MinPairs;
    pub struct MaxPairs;
    pub struct Cel;
    pub struct Ignore;
  }

  mod sealed {
    pub(super) struct Sealed;
  }

  pub trait State<S = Empty> {
    type Keys;
    type Values;
    type MinPairs;
    type MaxPairs;
    type Cel;
    type Ignore;
    const SEALED: sealed::Sealed;
  }

  pub struct SetKeys<S: State = Empty>(PhantomData<fn() -> S>);
  pub struct SetValues<S: State = Empty>(PhantomData<fn() -> S>);
  pub struct SetMinPairs<S: State = Empty>(PhantomData<fn() -> S>);
  pub struct SetMaxPairs<S: State = Empty>(PhantomData<fn() -> S>);
  pub struct SetCel<S: State = Empty>(PhantomData<fn() -> S>);

  pub struct SetIgnore<S: State = Empty>(PhantomData<fn() -> S>);

  #[doc(hidden)]
  impl State for Empty {
    type Keys = Unset<members::Keys>;
    type Values = Unset<members::Values>;
    type MinPairs = Unset<members::MinPairs>;
    type MaxPairs = Unset<members::MaxPairs>;
    type Cel = Unset<members::Cel>;
    type Ignore = Unset<members::Ignore>;
    const SEALED: sealed::Sealed = sealed::Sealed;
  }

  #[doc(hidden)]
  impl<S: State> State for SetKeys<S> {
    type Keys = Set<members::Keys>;
    type Values = S::Values;
    type MinPairs = S::MinPairs;
    type MaxPairs = S::MaxPairs;
    type Cel = S::Cel;
    type Ignore = S::Ignore;
    const SEALED: sealed::Sealed = sealed::Sealed;
  }

  #[doc(hidden)]
  impl<S: State> State for SetValues<S> {
    type Keys = S::Keys;
    type Values = Set<members::Values>;
    type MinPairs = S::MinPairs;
    type MaxPairs = S::MaxPairs;
    type Cel = S::Cel;
    type Ignore = S::Ignore;
    const SEALED: sealed::Sealed = sealed::Sealed;
  }
  #[doc(hidden)]
  impl<S: State> State for SetMinPairs<S> {
    type Keys = S::Keys;
    type Values = S::Values;
    type MinPairs = Set<members::MinPairs>;
    type MaxPairs = S::MaxPairs;
    type Cel = S::Cel;
    type Ignore = S::Ignore;
    const SEALED: sealed::Sealed = sealed::Sealed;
  }
  #[doc(hidden)]
  impl<S: State> State for SetMaxPairs<S> {
    type Keys = S::Keys;
    type Values = S::Values;
    type MinPairs = S::MinPairs;
    type MaxPairs = Set<members::MaxPairs>;
    type Cel = S::Cel;
    type Ignore = S::Ignore;
    const SEALED: sealed::Sealed = sealed::Sealed;
  }
  #[doc(hidden)]
  impl<S: State> State for SetCel<S> {
    type Keys = S::Keys;
    type Values = S::Values;
    type MinPairs = S::MinPairs;
    type MaxPairs = S::MaxPairs;
    type Cel = Set<members::Cel>;
    type Ignore = S::Ignore;
    const SEALED: sealed::Sealed = sealed::Sealed;
  }
  #[doc(hidden)]
  impl<S: State> State for SetIgnore<S> {
    type Keys = S::Keys;
    type Values = S::Values;
    type MinPairs = S::MinPairs;
    type MaxPairs = S::MaxPairs;
    type Cel = S::Cel;
    type Ignore = Set<members::Ignore>;
    const SEALED: sealed::Sealed = sealed::Sealed;
  }
}
