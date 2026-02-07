use core::{
	iter::{Copied, Zip},
	slice,
};

use proto_types::Status;

use super::*;

/// Offers information about the subject of the violation: the violation kind (string, int32, bytes, etc) and the field kind (map key, map value, etc).
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ViolationMeta {
	pub kind: ViolationKind,
	pub field_kind: FieldKind,
}

impl ViolationMeta {
	/// Creates a new instance, setting `field_kind` to [`FieldKind::Normal`].
	#[inline]
	#[must_use]
	pub const fn new(kind: ViolationKind) -> Self {
		Self {
			kind,
			field_kind: FieldKind::Normal,
		}
	}

	/// Changes the [`FieldKind`] of this instance.
	#[inline]
	#[must_use]
	pub const fn with_field_kind(mut self, field_kind: FieldKind) -> Self {
		self.field_kind = field_kind;
		self
	}
}

/// Holds the rich context concerning the validation errors that occur during validation.
///
/// Can be converted into [`Violations`] to be serialized into protobuf.
///
/// When the feature `tonic` is enabled, it can be converted directly into [`tonic::Status`] so that the `?` operator can be used directly inside [`tonic`] handlers.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ValidationErrors {
	metas: Vec<ViolationMeta>,
	violations: Vec<Violation>,
}

#[cfg(feature = "tonic")]
impl From<ValidationErrors> for tonic::Status {
	#[inline(never)]
	#[cold]
	fn from(value: ValidationErrors) -> Self {
		use ::prost::Message;

		let status_inner: Status = value.into();

		Self::with_details(
			tonic::Code::InvalidArgument,
			"Validation failure",
			Bytes::from(status_inner.encode_to_vec()),
		)
	}
}

/// The context of a specific violation.
///
/// It contains the [`Violation`] data, which is used for protobuf serialization, as well as the [`ViolationMeta`], which contains the extra information about the violation's kind and field kind.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ViolationCtx {
	pub meta: ViolationMeta,
	pub data: Violation,
}

impl From<ViolationCtx> for Violation {
	#[inline]
	fn from(value: ViolationCtx) -> Self {
		value.data
	}
}

impl ViolationCtx {
	/// Creates a new instance.
	#[inline]
	#[must_use]
	pub const fn new(meta: ViolationMeta, data: Violation) -> Self {
		Self { meta, data }
	}

	/// Converts into a [`Violation`].
	#[must_use]
	#[inline]
	pub fn into_violation(self) -> Violation {
		self.into()
	}
}

/// Immutable view for [`ViolationCtx`].
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct ViolationCtxRef<'a> {
	pub meta: ViolationMeta,
	pub data: &'a Violation,
}

impl ViolationCtxRef<'_> {
	/// Transforms this view into an owned instance of [`ViolationCtx`].
	#[must_use]
	#[inline(never)]
	#[cold]
	pub fn into_owned(self) -> ViolationCtx {
		ViolationCtx {
			meta: self.meta,
			data: self.data.clone(),
		}
	}
}

/// Mutable view for [`ViolationCtx`].
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ViolationCtxMut<'a> {
	pub meta: &'a mut ViolationMeta,
	pub data: &'a mut Violation,
}

impl ViolationCtxMut<'_> {
	/// Transforms this view into an owned instance of [`ViolationCtx`].
	#[must_use]
	#[inline(never)]
	#[cold]
	pub fn into_owned(&self) -> ViolationCtx {
		ViolationCtx {
			meta: *self.meta,
			data: self.data.clone(),
		}
	}
}

/// Owned iterator for [`ViolationCtx`].
#[derive(Clone, Debug)]
pub struct IntoIter {
	iter: core::iter::Zip<alloc::vec::IntoIter<ViolationMeta>, alloc::vec::IntoIter<Violation>>,
}

impl Iterator for IntoIter {
	type Item = ViolationCtx;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.next()
			.map(|(meta, data)| ViolationCtx { meta, data })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl ExactSizeIterator for IntoIter {
	#[inline]
	fn len(&self) -> usize {
		self.iter.len()
	}
}

/// Ref iterator for [`ViolationCtx`].
#[derive(Clone, Debug)]
pub struct Iter<'a> {
	iter: Zip<Copied<slice::Iter<'a, ViolationMeta>>, slice::Iter<'a, Violation>>,
}

impl<'a> Iterator for Iter<'a> {
	type Item = ViolationCtxRef<'a>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.next()
			.map(|(meta, data)| ViolationCtxRef { meta, data })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl ExactSizeIterator for Iter<'_> {
	#[inline]
	fn len(&self) -> usize {
		self.iter.len()
	}
}

/// Mutable ref iterator for [`ViolationCtx`].
#[derive(Debug)]
pub struct IterMut<'a> {
	iter: Zip<core::slice::IterMut<'a, ViolationMeta>, core::slice::IterMut<'a, Violation>>,
}

impl<'a> Iterator for IterMut<'a> {
	type Item = ViolationCtxMut<'a>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.next()
			.map(|(meta, data)| ViolationCtxMut { meta, data })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl ExactSizeIterator for IterMut<'_> {
	#[inline]
	fn len(&self) -> usize {
		self.iter.len()
	}
}

impl From<ValidationErrors> for Violations {
	#[inline]
	fn from(value: ValidationErrors) -> Self {
		Self {
			violations: value.violations,
		}
	}
}

impl From<ValidationErrors> for Vec<Violation> {
	#[inline]
	fn from(value: ValidationErrors) -> Self {
		value.violations
	}
}

impl From<ValidationErrors> for Status {
	#[inline]
	#[cold]
	fn from(value: ValidationErrors) -> Self {
		value.into_violations().into()
	}
}

impl IntoIterator for ValidationErrors {
	type IntoIter = IntoIter;
	type Item = ViolationCtx;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
			iter: self.metas.into_iter().zip(self.violations),
		}
	}
}

impl<'a> IntoIterator for &'a ValidationErrors {
	type Item = ViolationCtxRef<'a>;

	type IntoIter = Iter<'a>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		Iter {
			iter: self
				.metas
				.iter()
				.copied()
				.zip(self.violations.iter()),
		}
	}
}

impl<'a> IntoIterator for &'a mut ValidationErrors {
	type Item = ViolationCtxMut<'a>;

	type IntoIter = IterMut<'a>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		IterMut {
			iter: self
				.metas
				.iter_mut()
				.zip(self.violations.iter_mut()),
		}
	}
}

impl Extend<ViolationCtx> for ValidationErrors {
	fn extend<T: IntoIterator<Item = ViolationCtx>>(&mut self, iter: T) {
		let iter = iter.into_iter();

		let (lower_bound, _) = iter.size_hint();
		if lower_bound > 0 {
			self.metas.reserve(lower_bound);
			self.violations.reserve(lower_bound);
		}

		for v in iter {
			self.metas.push(v.meta);
			self.violations.push(v.data);
		}
	}
}

impl ValidationErrors {
	/// Appends the contents of another [`ValidationErrors`] to the original instance.
	#[inline]
	pub fn merge(&mut self, other: &mut Self) {
		self.metas.append(&mut other.metas);
		self.violations.append(&mut other.violations);
	}

	/// Returns a ref iterator.
	#[inline]
	#[must_use]
	pub fn iter(&self) -> Iter<'_> {
		self.into_iter()
	}

	/// Returns a mutable ref iterator.
	#[inline]
	pub fn iter_mut(&mut self) -> IterMut<'_> {
		self.into_iter()
	}

	/// Removes the violations that match the given predicate.
	pub fn retain<F>(&mut self, mut f: F)
	where
		F: FnMut(ViolationCtxRef) -> bool,
	{
		let len = self.violations.len();
		let mut keep_count = 0;

		for i in 0..len {
			let should_keep = f(ViolationCtxRef {
				meta: self.metas[i],
				data: &self.violations[i],
			});

			if should_keep {
				if keep_count != i {
					self.metas.swap(keep_count, i);
					self.violations.swap(keep_count, i);
				}
				keep_count += 1;
			}
		}

		self.metas.truncate(keep_count);
		self.violations.truncate(keep_count);
	}

	/// Creates a new instance.
	#[must_use]
	#[inline]
	pub const fn new() -> Self {
		Self {
			metas: vec![],
			violations: vec![],
		}
	}

	/// Turns into [`Status`], the equivalent of the `google.rpc.Status` message.
	#[inline]
	#[must_use]
	pub fn into_status(self) -> Status {
		self.into()
	}

	/// Converts into [`Violations`].
	#[inline]
	#[must_use]
	pub fn into_violations(self) -> Violations {
		Violations {
			violations: self.violations,
		}
	}

	/// Adds a new violation to the list.
	#[inline]
	pub fn push(&mut self, v: ViolationCtx) {
		self.metas.push(v.meta);
		self.violations.push(v.data);
	}

	/// Checks if this instance contains any violations.
	#[inline]
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.violations.is_empty()
	}

	/// Returns the amount of violations contained.
	#[inline]
	#[must_use]
	pub const fn len(&self) -> usize {
		self.violations.len()
	}
}

impl Default for ValidationErrors {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

#[inline(never)]
#[cold]
pub(crate) fn create_violation_core(
	custom_rule_id: Option<String>,
	field_context: Option<&FieldContext>,
	parent_elements: &[FieldPathElement],
	violation_data: ViolationData,
	error_message: String,
) -> Violation {
	let mut field_elements: Option<Vec<FieldPathElement>> = None;
	let mut rule_elements: Vec<FieldPathElement> = Vec::new();
	let mut is_for_key = false;

	// In case of a top level validator there would be no field path
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
			FieldKind::RepeatedItem => {
				rule_elements.extend(REPEATED_ITEMS_VIOLATION.elements_iter())
			}
			_ => {}
		};
	}

	rule_elements.extend(violation_data.elements_iter());

	Violation {
		rule_id: Some(custom_rule_id.unwrap_or_else(|| violation_data.name.to_string())),
		message: Some(error_message),
		for_key: Some(is_for_key),
		field: field_elements.map(|elements| FieldPath { elements }),
		rule: Some(FieldPath {
			elements: rule_elements,
		}),
	}
}
