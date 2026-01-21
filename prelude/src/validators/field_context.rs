use super::*;

use proto_types::field_descriptor_proto::Type as ProtoPrimitive;
use proto_types::protovalidate::field_path_element::Subscript;

/// The context for the field being validated.
#[derive(Clone, Debug)]
pub struct FieldContext {
  pub proto_name: &'static str,
  pub tag: i32,
  pub subscript: Option<Subscript>,
  pub map_key_type: Option<ProtoPrimitive>,
  pub map_value_type: Option<ProtoPrimitive>,
  pub field_type: ProtoPrimitive,
  pub field_kind: FieldKind,
}

impl FieldContext {
  #[must_use]
  pub fn as_path_element(&self) -> FieldPathElement {
    FieldPathElement {
      field_number: Some(self.tag),
      field_name: Some(self.proto_name.to_string()),
      field_type: Some(self.field_type as i32),
      key_type: self.map_key_type.map(|t| t as i32),
      value_type: self.map_value_type.map(|t| t as i32),
      subscript: self.subscript.clone(),
    }
  }
}

#[derive(Clone, Default, Debug, Copy, PartialEq, Eq)]
pub enum FieldKind {
  Map,
  MapKey,
  MapValue,
  Repeated,
  RepeatedItem,
  #[default]
  Single,
}

impl FieldKind {
  #[must_use]
  pub const fn is_map_key(&self) -> bool {
    matches!(self, Self::MapKey)
  }

  #[must_use]
  pub const fn is_map_value(&self) -> bool {
    matches!(self, Self::MapValue)
  }

  #[must_use]
  pub const fn is_repeated_item(&self) -> bool {
    matches!(self, Self::RepeatedItem)
  }
}

pub struct ValidationCtx {
  pub field_context: Option<FieldContext>,
  pub parent_elements: Vec<FieldPathElement>,
  pub violations: ViolationsAcc,
  pub fail_fast: bool,
}

impl Default for ValidationCtx {
  #[inline]
  fn default() -> Self {
    Self {
      field_context: None,
      parent_elements: vec![],
      violations: ViolationsAcc::new(),
      fail_fast: true,
    }
  }
}

macro_rules! violation_helpers {
  ($($name:ident),*) => {
    paste::paste! {
      $(
        pub(crate) fn [< add_ $name _violation >](&mut self, kind: [< $name:camel Violation >], error_message: CowStr) -> ValidatorResult {
          self.add_violation(ViolationKind::[< $name:camel >](kind), error_message)
        }
      )*
    }
  };
}

impl ValidationCtx {
  #[inline]
  pub fn reset_field_context(&mut self) {
    self.field_context = None;
  }

  #[inline]
  pub fn with_field_context(&mut self, field_context: FieldContext) -> &mut Self {
    self.field_context = Some(field_context);
    self
  }

  violation_helpers!(
    any, bytes, duration, string, timestamp, enum, field_mask, map, repeated
  );

  #[inline]
  pub fn add_violation(
    &mut self,
    kind: ViolationKind,
    error_message: impl Into<String>,
  ) -> ValidatorResult {
    let violation = create_violation_core(
      None,
      self.field_context.as_ref(),
      &self.parent_elements,
      kind.data(),
      error_message.into(),
    );

    self.violations.push(ViolationCtx {
      kind,
      data: violation,
    });

    if self.fail_fast {
      Err(FailFast)
    } else {
      Ok(IsValid::No)
    }
  }

  #[inline]
  pub fn add_violation_with_custom_id(
    &mut self,
    rule_id: &str,
    kind: ViolationKind,
    error_message: impl Into<String>,
  ) -> ValidatorResult {
    let violation = new_violation_with_custom_id(
      rule_id,
      self.field_context.as_ref(),
      &self.parent_elements,
      kind.data(),
      error_message.into(),
    );

    self.violations.push(ViolationCtx {
      kind,
      data: violation,
    });

    if self.fail_fast {
      Err(FailFast)
    } else {
      Ok(IsValid::No)
    }
  }

  #[inline]
  pub fn add_cel_violation(&mut self, rule: &CelRule) -> ValidatorResult {
    self
      .violations
      .add_cel_violation(rule, self.field_context.as_ref(), &self.parent_elements);

    if self.fail_fast {
      Err(FailFast)
    } else {
      Ok(IsValid::No)
    }
  }

  #[inline]
  pub fn add_required_oneof_violation(&mut self) -> ValidatorResult {
    self
      .violations
      .add_required_oneof_violation(&self.parent_elements);

    if self.fail_fast {
      Err(FailFast)
    } else {
      Ok(IsValid::No)
    }
  }

  #[inline]
  pub fn add_required_violation(&mut self) -> ValidatorResult {
    self.add_violation(ViolationKind::Required, Cow::Borrowed("is required"))
  }

  #[cfg(feature = "cel")]
  pub fn add_cel_error_violation(&mut self, error: CelError) -> ValidatorResult {
    self.violations.push(ViolationCtx {
      kind: ViolationKind::Cel,
      data: error.into_violation(self.field_context.as_ref(), &self.parent_elements),
    });

    if self.fail_fast {
      Err(FailFast)
    } else {
      Ok(IsValid::No)
    }
  }
}

pub struct ViolationsAcc {
  violations: Vec<Violation>,
  kinds: Vec<ViolationKind>,
}

pub struct ViolationCtx {
  pub kind: ViolationKind,
  pub data: Violation,
}

impl ViolationCtx {
  #[must_use]
  pub fn into_violation(self) -> Violation {
    self.into()
  }
}

impl From<ViolationsAcc> for Violations {
  fn from(value: ViolationsAcc) -> Self {
    Self {
      violations: value.violations,
    }
  }
}

impl From<ViolationsAcc> for Vec<Violation> {
  fn from(value: ViolationsAcc) -> Self {
    value.violations
  }
}

impl From<ViolationCtx> for Violation {
  fn from(value: ViolationCtx) -> Self {
    value.data
  }
}

impl IntoIterator for ViolationsAcc {
  type IntoIter = core::iter::Zip<vec::IntoIter<ViolationKind>, vec::IntoIter<Violation>>;
  type Item = (ViolationKind, Violation);

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.kinds.into_iter().zip(self.violations)
  }
}

impl<'a> IntoIterator for &'a ViolationsAcc {
  type Item = (ViolationKind, &'a Violation);

  type IntoIter = core::iter::Zip<
    core::iter::Copied<core::slice::Iter<'a, ViolationKind>>,
    core::slice::Iter<'a, Violation>,
  >;

  fn into_iter(self) -> Self::IntoIter {
    self
      .kinds
      .iter()
      .copied()
      .zip(self.violations.iter())
  }
}

impl<'a> IntoIterator for &'a mut ViolationsAcc {
  type Item = (&'a mut ViolationKind, &'a mut Violation);

  type IntoIter =
    core::iter::Zip<core::slice::IterMut<'a, ViolationKind>, core::slice::IterMut<'a, Violation>>;

  fn into_iter(self) -> Self::IntoIter {
    self
      .kinds
      .iter_mut()
      .zip(self.violations.iter_mut())
  }
}

impl Extend<ViolationCtx> for ViolationsAcc {
  fn extend<T: IntoIterator<Item = ViolationCtx>>(&mut self, iter: T) {
    let iter = iter.into_iter();

    let (lower_bound, _) = iter.size_hint();
    if lower_bound > 0 {
      self.kinds.reserve(lower_bound);
      self.violations.reserve(lower_bound);
    }

    for ctx in iter {
      self.kinds.push(ctx.kind);
      self.violations.push(ctx.data);
    }
  }
}

impl Extend<(ViolationKind, Violation)> for ViolationsAcc {
  fn extend<T: IntoIterator<Item = (ViolationKind, Violation)>>(&mut self, iter: T) {
    let iter = iter.into_iter();

    let (lower_bound, _) = iter.size_hint();
    if lower_bound > 0 {
      self.kinds.reserve(lower_bound);
      self.violations.reserve(lower_bound);
    }

    for (kind, data) in iter {
      self.kinds.push(kind);
      self.violations.push(data);
    }
  }
}

impl ViolationsAcc {
  pub fn merge(&mut self, other: &mut Self) {
    self.kinds.append(&mut other.kinds);
    self.violations.append(&mut other.violations);
  }

  #[must_use]
  #[inline]
  pub fn first(&self) -> Option<(ViolationKind, &Violation)> {
    self
      .kinds
      .first()
      .copied()
      .and_then(|k| self.violations.first().map(|v| (k, v)))
  }

  #[must_use]
  #[inline]
  pub fn last(&self) -> Option<(ViolationKind, &Violation)> {
    self
      .kinds
      .last()
      .copied()
      .and_then(|k| self.violations.last().map(|v| (k, v)))
  }

  #[inline]
  pub fn iter(
    &self,
  ) -> core::iter::Zip<
    core::iter::Copied<core::slice::Iter<'_, ViolationKind>>,
    core::slice::Iter<'_, Violation>,
  > {
    self.into_iter()
  }

  #[inline]
  pub fn iter_mut(
    &mut self,
  ) -> core::iter::Zip<core::slice::IterMut<'_, ViolationKind>, core::slice::IterMut<'_, Violation>>
  {
    self.into_iter()
  }

  pub fn retain<F>(&mut self, mut f: F)
  where
    F: FnMut(ViolationKind, &Violation) -> bool,
  {
    let len = self.violations.len();
    let mut keep_count = 0;

    for i in 0..len {
      let should_keep = f(self.kinds[i], &self.violations[i]);

      if should_keep {
        if keep_count != i {
          self.kinds.swap(keep_count, i);
          self.violations.swap(keep_count, i);
        }
        keep_count += 1;
      }
    }

    self.kinds.truncate(keep_count);
    self.violations.truncate(keep_count);
  }

  #[inline]
  pub fn add_required_oneof_violation(&mut self, parent_elements: &[FieldPathElement]) {
    let violation = new_violation_with_custom_id(
      ONEOF_REQUIRED_VIOLATION.name,
      None,
      parent_elements,
      ONEOF_REQUIRED_VIOLATION,
      "at least one value must be set".into(),
    );

    self.push(ViolationCtx {
      kind: ViolationKind::RequiredOneof,
      data: violation,
    });
  }

  #[inline]
  pub fn add_cel_violation(
    &mut self,
    rule: &CelRule,
    field_context: Option<&FieldContext>,
    parent_elements: &[FieldPathElement],
  ) {
    let violation = new_violation_with_custom_id(
      &rule.id,
      field_context,
      parent_elements,
      CEL_VIOLATION,
      rule.message.to_string(),
    );

    self.push(ViolationCtx {
      kind: ViolationKind::Cel,
      data: violation,
    });
  }

  #[must_use]
  #[inline]
  pub const fn new() -> Self {
    Self {
      kinds: vec![],
      violations: vec![],
    }
  }

  #[inline]
  #[must_use]
  pub fn into_violations(self) -> Violations {
    Violations {
      violations: self.violations,
    }
  }

  #[inline]
  pub fn push(&mut self, v: ViolationCtx) {
    self.kinds.push(v.kind);
    self.violations.push(v.data);
  }

  #[inline]
  #[must_use]
  pub const fn is_empty(&self) -> bool {
    self.violations.is_empty()
  }

  #[inline]
  #[must_use]
  pub const fn len(&self) -> usize {
    self.violations.len()
  }
}

impl Default for ViolationsAcc {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

pub(crate) fn create_violation_core(
  custom_rule_id: Option<&str>,
  field_context: Option<&FieldContext>,
  parent_elements: &[FieldPathElement],
  violation_data: ViolationData,
  error_message: String,
) -> Violation {
  let mut field_elements: Option<Vec<FieldPathElement>> = None;
  let mut rule_elements: Vec<FieldPathElement> = Vec::new();
  let mut is_for_key = false;

  // In case of a top level message with CEL violations applied to the message
  // as a whole, there would be no field path
  if let Some(field_context) = field_context {
    let elements = field_elements.get_or_insert_default();

    elements.extend(parent_elements.iter().cloned());

    let current_elem = field_context.as_path_element();

    elements.push(current_elem);

    match &field_context.field_kind {
      FieldKind::MapKey => {
        is_for_key = true;
        rule_elements.extend(MAP_KEYS_VIOLATION.elements_iter());
      }
      FieldKind::MapValue => rule_elements.extend(MAP_VALUES_VIOLATION.elements_iter()),
      FieldKind::RepeatedItem => rule_elements.extend(REPEATED_ITEMS_VIOLATION.elements_iter()),
      _ => {}
    };
  }

  rule_elements.extend(violation_data.elements_iter());

  Violation {
    rule_id: Some(
      custom_rule_id.map_or_else(|| violation_data.name.to_string(), |id| id.to_string()),
    ),
    message: Some(error_message),
    for_key: Some(is_for_key),
    field: field_elements.map(|elements| FieldPath { elements }),
    rule: Some(FieldPath {
      elements: rule_elements,
    }),
  }
}

type CowStr<'a> = Cow<'a, str>;

pub(crate) fn new_violation_with_custom_id(
  rule_id: &str,
  field_context: Option<&FieldContext>,
  parent_elements: &[FieldPathElement],
  violation_data: ViolationData,
  error_message: String,
) -> Violation {
  create_violation_core(
    Some(rule_id),
    field_context,
    parent_elements,
    violation_data,
    error_message,
  )
}
