use bytes::Bytes;

use super::*;

#[test]
fn unique_enums() {
  let msg = UniqueEnums {
    unique_enums: vec![DummyEnum::A as i32, DummyEnum::A as i32],
  };

  let err = msg.validate().unwrap_err();

  assert_eq!(err.first().unwrap().rule_id(), "repeated.unique");
}

#[test]
fn unique_floats() {
  let msg = UniqueFloats {
    unique_floats: vec![1.1, 1.1],
  };

  let err = msg.validate().unwrap_err();

  assert_eq!(err.first().unwrap().rule_id(), "repeated.unique");
}

#[test]
fn unique_messages() {
  let msg = UniqueMessages {
    unique_messages: vec![DummyMsg { id: 1 }, DummyMsg { id: 1 }],
  };

  let err = msg.validate().unwrap_err();

  assert_eq!(err.first().unwrap().rule_id(), "repeated.unique");
}

#[test]
fn unique_bytes() {
  let msg = UniqueBytes {
    unique_bytes: vec![Bytes::default(), Bytes::default()],
  };

  let err = msg.validate().unwrap_err();

  assert_eq!(err.first().unwrap().rule_id(), "repeated.unique");
}

#[test]
fn min_items() {
  let msg = MinItems { items: vec![] };

  let err = msg.validate().unwrap_err();

  assert_eq!(err.first().unwrap().rule_id(), "repeated.min_items");
}

#[test]
fn max_items() {
  let msg = MaxItems { items: vec![1, 2] };

  let err = msg.validate().unwrap_err();

  assert_eq!(err.first().unwrap().rule_id(), "repeated.max_items");
}
