#[doc(hidden)]
pub mod state;
use crate::validators::*;
pub(crate) use state::*;

/// Builder for [`MapValidator`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MapValidatorBuilder<K, V, S: State = Empty>
where
  K: ProtoValidation,
  V: ProtoValidation,
{
  keys: Option<K::Validator>,

  _state: PhantomData<S>,
  _key_type: PhantomData<K>,
  _value_type: PhantomData<V>,

  cel: Vec<CelProgram>,
  values: Option<V::Validator>,
  min_pairs: Option<usize>,
  max_pairs: Option<usize>,
  ignore: Ignore,
  error_messages: Option<ErrorMessages<MapViolation>>,
}

impl<K, V, S: State> Default for MapValidatorBuilder<K, V, S>
where
  K: ProtoValidation,
  V: ProtoValidation,
{
  #[inline]
  fn default() -> Self {
    Self {
      keys: Default::default(),
      _state: PhantomData,
      _key_type: PhantomData,
      _value_type: PhantomData,
      cel: Default::default(),
      values: Default::default(),
      min_pairs: Default::default(),
      max_pairs: Default::default(),
      ignore: Default::default(),
      error_messages: None,
    }
  }
}

impl<K, V> MapValidator<K, V>
where
  K: ProtoValidation,
  V: ProtoValidation,
{
  #[must_use]
  #[inline]
  pub fn builder() -> MapValidatorBuilder<K, V> {
    MapValidatorBuilder::default()
  }
}

impl<K, V, S: State> From<MapValidatorBuilder<K, V, S>> for ProtoOption
where
  K: ProtoValidation,
  V: ProtoValidation,
{
  #[inline(never)]
  #[cold]
  fn from(value: MapValidatorBuilder<K, V, S>) -> Self {
    value.build().into()
  }
}

impl<S: State, K, V> MapValidatorBuilder<K, V, S>
where
  K: ProtoValidation,
  V: ProtoValidation,
{
  /// Builds the validator.
  ///
  /// If the values validator is unset, but the target has `HAS_DEFAULT_VALIDATOR` from [`ProtoValidation`] set to true,
  /// its default validator will be assigned.
  #[inline]
  pub fn build(self) -> MapValidator<K, V> {
    let Self {
      keys,
      _key_type,
      _value_type,
      values,
      min_pairs,
      max_pairs,
      ignore,
      cel,
      error_messages,
      ..
    } = self;

    MapValidator {
      keys,
      _key_type,
      _value_type,
      cel,
      values: values.or_else(|| V::HAS_DEFAULT_VALIDATOR.then(|| V::Validator::default())),
      min_pairs,
      max_pairs,
      ignore,
      error_messages,
    }
  }

  /// Adds a map with custom error messages to the underlying validator.
  ///
  /// If a violation has no custom error message attached to it, it uses the default error message.
  ///
  /// NOTE: The custom errors for items and keys must be handled in their respective validators.
  #[inline]
  pub fn with_error_messages(
    self,
    error_messages: impl IntoIterator<Item = (MapViolation, impl Into<FixedStr>)>,
  ) -> MapValidatorBuilder<K, V, SetErrorMessages<S>>
  where
    S::ErrorMessages: IsUnset,
  {
    let map: BTreeMap<MapViolation, FixedStr> = error_messages
      .into_iter()
      .map(|(v, m)| (v, m.into()))
      .collect();

    MapValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      keys: self.keys,
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: self.min_pairs,
      max_pairs: self.max_pairs,
      ignore: self.ignore,
      error_messages: Some(Box::new(map)),
    }
  }

  /// Adds a [`CelProgram`] to this validator.
  ///
  /// The program will be applied to the map as a whole.
  ///
  /// To apply a program to the individual keys/values, use their respective validators.
  #[inline]
  #[must_use]
  pub fn cel(mut self, program: CelProgram) -> Self {
    self.cel.push(program);

    self
  }

  /// Specifies the minimum amount of key-value pairs that this field should have in order to be valid.
  #[inline]
  pub fn min_pairs(self, num: usize) -> MapValidatorBuilder<K, V, SetMinPairs<S>>
  where
    S::MinPairs: IsUnset,
  {
    MapValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      keys: self.keys,
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: Some(num),
      max_pairs: self.max_pairs,
      ignore: self.ignore,
      error_messages: self.error_messages,
    }
  }

  /// Specifies the maximum amount of key-value pairs that this field should have in order to be valid.
  #[inline]
  pub fn max_pairs(self, num: usize) -> MapValidatorBuilder<K, V, SetMaxPairs<S>>
  where
    S::MaxPairs: IsUnset,
  {
    MapValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      keys: self.keys,
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: self.min_pairs,
      max_pairs: Some(num),
      ignore: self.ignore,
      error_messages: self.error_messages,
    }
  }

  /// Specifies that this validator's rules will be ignored if the map is empty.
  #[inline]
  pub fn ignore_if_zero_value(self) -> MapValidatorBuilder<K, V, SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    MapValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      keys: self.keys,
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: self.min_pairs,
      max_pairs: self.max_pairs,
      ignore: Ignore::IfZeroValue,
      error_messages: self.error_messages,
    }
  }

  /// Specifies that this validator should always be ignored.
  #[inline]
  pub fn ignore_always(self) -> MapValidatorBuilder<K, V, SetIgnore<S>>
  where
    S::Ignore: IsUnset,
  {
    MapValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      keys: self.keys,
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: self.min_pairs,
      max_pairs: self.max_pairs,
      ignore: Ignore::Always,
      error_messages: self.error_messages,
    }
  }

  /// Sets the rules to apply to each key of this map.
  ///
  /// It receives a closure that that receives the key type's default validator builder as an argument.
  ///
  /// # Example
  /// ```
  /// use protify::*;
  ///
  /// let validator = MapValidator::<String, String>::builder().keys(|k| k.min_len(5)).build();
  ///
  /// assert_eq!(validator.keys.unwrap(), StringValidator::builder().min_len(5).build());
  /// ```
  #[inline]
  pub fn keys<F, FinalBuilder>(self, config_fn: F) -> MapValidatorBuilder<K, V, SetKeys<S>>
  where
    S::Keys: IsUnset,
    FinalBuilder: ValidatorBuilderFor<K, Validator = K::Validator>,
    F: FnOnce(K::ValidatorBuilder) -> FinalBuilder,
  {
    let keys_opts = K::validator_from_closure(config_fn);

    MapValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      keys: Some(keys_opts),
      _key_type: self._key_type,
      _value_type: self._value_type,
      values: self.values,
      min_pairs: self.min_pairs,
      max_pairs: self.max_pairs,
      ignore: self.ignore,
      error_messages: self.error_messages,
    }
  }

  /// Sets the rules to apply to each value of this map.
  ///
  /// It receives a closure that that receives the value type's default validator builder as an argument.
  /// # Example
  /// ```
  /// use protify::*;
  ///
  /// let validator = MapValidator::<String, String>::builder().values(|v| v.min_len(5)).build();
  ///
  /// assert_eq!(validator.values.unwrap(), StringValidator::builder().min_len(5).build());
  /// ```
  #[inline]
  pub fn values<F, FinalBuilder>(self, config_fn: F) -> MapValidatorBuilder<K, V, SetValues<S>>
  where
    V: ProtoValidation,
    FinalBuilder: ValidatorBuilderFor<V, Validator = V::Validator>,
    F: FnOnce(V::ValidatorBuilder) -> FinalBuilder,
  {
    let values_opts = V::validator_from_closure(config_fn);

    MapValidatorBuilder {
      _state: PhantomData,
      cel: self.cel,
      keys: self.keys,
      values: Some(values_opts),
      _key_type: self._key_type,
      _value_type: self._value_type,
      min_pairs: self.min_pairs,
      max_pairs: self.max_pairs,
      ignore: self.ignore,
      error_messages: self.error_messages,
    }
  }
}
