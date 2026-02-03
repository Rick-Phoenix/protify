use rendering_tests::TestEnum;
use std::{collections::HashMap, sync::Arc};

use super::*;
mod conversions_tests;
mod deprecated_tests;
mod inference_tests;
mod nested_items_tests;
mod rendering_tests;
mod type_resolution_tests;

#[proto_oneof(proxied)]
#[proto(skip_checks(all))]
pub enum IgnoredVariant {
  #[proto(tag = 1)]
  A(i32),
  #[proto(tag = 2)]
  B(u32),
  #[proto(ignore)]
  C(i64),
}

impl Default for IgnoredVariantProto {
  fn default() -> Self {
    Self::A(1)
  }
}

#[test]
fn ignored_variant() {
  let schema = IgnoredVariantProto::proto_schema();

  assert!(!schema.fields.iter().any(|f| f.name == "c"));
}

fn custom_conv() -> i32 {
  5
}

#[proto_message(proxied)]
#[proto(skip_checks(all))]
pub struct IgnoredField {
  #[proto(ignore)]
  pub ignored: i32,
  #[proto(ignore, from_proto = custom_conv)]
  pub custom_conv: i32,
  pub present: i32,
}

#[test]
fn ignored_field() {
  let schema = IgnoredFieldProto::proto_schema();

  assert!(
    !schema
      .fields()
      .any(|f| f.name == "ignored" || f.name == "custom_conv")
  );

  let msg = IgnoredFieldProto { present: 1 };

  let proxy = msg.into_proxy();

  assert_eq_pretty!(proxy.present, 1);
  assert_eq_pretty!(proxy.ignored, 0);
  assert_eq_pretty!(proxy.custom_conv, 5);
}
