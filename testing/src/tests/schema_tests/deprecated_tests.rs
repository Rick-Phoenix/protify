#![allow(deprecated)]

use super::*;

#[proto_enum]
#[deprecated]
pub enum DeprecatedEnum1 {
  A,
  B,
}

#[test]
fn deprecated_enum1() {
  let schema = DeprecatedEnum1::proto_schema();

  assert_eq_pretty!(schema.options[0], proto_deprecated());
}

#[proto_enum]
#[proto(deprecated = true)]
pub enum DeprecatedEnum2 {
  A,
  B,
}

#[test]
fn deprecated_enum2() {
  let schema = DeprecatedEnum2::proto_schema();

  assert_eq_pretty!(schema.options[0], proto_deprecated());
}

#[proto_enum]
#[deprecated]
#[proto(deprecated = false)]
pub enum DeprecatedEnum3 {
  A,
  B,
}

#[test]
fn deprecated_enum3() {
  let schema = DeprecatedEnum3::proto_schema();

  assert!(
    schema.options.is_empty(),
    "should override deprecated attribute"
  );
}

#[proto_enum]
pub enum DeprecatedEnumVariant1 {
  #[deprecated]
  A,
  B,
}

#[test]
fn deprecated_enum_variant1() {
  let schema = DeprecatedEnumVariant1::proto_schema();

  assert_eq_pretty!(schema.variants[0].options[0], proto_deprecated());
}

#[proto_enum]
pub enum DeprecatedEnumVariant2 {
  #[proto(deprecated = true)]
  A,
  B,
}

#[test]
fn deprecated_enum_variant2() {
  let schema = DeprecatedEnumVariant2::proto_schema();

  assert_eq_pretty!(schema.variants[0].options[0], proto_deprecated());
}

#[proto_enum]
pub enum DeprecatedEnumVariant3 {
  #[deprecated]
  #[proto(deprecated = false)]
  A,
  B,
}

#[test]
fn deprecated_enum_variant3() {
  let schema = DeprecatedEnumVariant3::proto_schema();

  assert!(
    schema.variants[0].options.is_empty(),
    "should override deprecated attribute"
  );
}

#[proto_message]
#[deprecated]
struct DeprecatedMsg1 {
  pub id: i32,
}

#[test]
fn deprecated_msg1() {
  let schema = DeprecatedMsg1::proto_schema();

  assert_eq_pretty!(schema.options[0], proto_deprecated());
}

#[proto_message]
#[proto(deprecated = true)]
struct DeprecatedMsg2 {
  pub id: i32,
}

#[test]
fn deprecated_msg2() {
  let schema = DeprecatedMsg2::proto_schema();

  assert_eq_pretty!(schema.options[0], proto_deprecated());
}

#[proto_message]
#[deprecated]
#[proto(deprecated = false)]
struct DeprecatedMsg3 {
  pub id: i32,
}

#[test]
fn deprecated_msg3() {
  let schema = DeprecatedMsg3::proto_schema();

  assert!(
    schema.options.is_empty(),
    "should override deprecated attribute"
  );
}

#[proto_message]
struct DeprecatedMsgField1 {
  #[deprecated]
  pub id: i32,
}

#[test]
fn deprecated_msg_field1() {
  let schema = DeprecatedMsgField1::proto_schema();

  assert_eq_pretty!(
    schema.entries[0].as_field().unwrap().options[0],
    proto_deprecated()
  );
}

#[proto_message]
struct DeprecatedMsgField2 {
  #[proto(deprecated = true)]
  pub id: i32,
}

#[test]
fn deprecated_msg_field2() {
  let schema = DeprecatedMsgField2::proto_schema();

  assert_eq_pretty!(
    schema.entries[0].as_field().unwrap().options[0],
    proto_deprecated()
  );
}

#[proto_message]
struct DeprecatedMsgField3 {
  #[deprecated]
  #[proto(deprecated = false)]
  pub id: i32,
}

#[test]
fn deprecated_msg_field3() {
  let schema = DeprecatedMsgField3::proto_schema();

  assert!(
    schema.entries[0]
      .as_field()
      .unwrap()
      .options
      .is_empty(),
    "should override deprecated attribute"
  );
}

#[allow(unused)]
#[proto_oneof]
enum DeprecatedOneofVariant1 {
  #[deprecated]
  #[proto(tag = 1)]
  A(i32),
  #[proto(tag = 2)]
  B(i32),
}

#[test]
fn deprecated_oneof_variant1() {
  let schema = DeprecatedOneofVariant1::proto_schema();

  assert_eq_pretty!(schema.fields[0].options[0], proto_deprecated());
}

#[allow(unused)]
#[proto_oneof]
enum DeprecatedOneofVariant2 {
  #[proto(deprecated = true)]
  #[proto(tag = 1)]
  A(i32),
  #[proto(tag = 2)]
  B(i32),
}

#[test]
fn deprecated_oneof_variant2() {
  let schema = DeprecatedOneofVariant2::proto_schema();

  assert_eq_pretty!(schema.fields[0].options[0], proto_deprecated());
}

#[allow(unused)]
#[proto_oneof]
enum DeprecatedOneofVariant3 {
  #[deprecated]
  #[proto(deprecated = false)]
  #[proto(tag = 1)]
  A(i32),
  #[proto(tag = 2)]
  B(i32),
}

#[test]
fn deprecated_oneof_variant3() {
  let schema = DeprecatedOneofVariant3::proto_schema();

  assert!(
    schema.fields[0].options.is_empty(),
    "should override deprecated attribute"
  );
}

#[proto_service]
#[deprecated]
enum DeprecatedService1 {
  Service {
    request: DeprecatedMsg1,
    response: DeprecatedMsg1,
  },
}

#[test]
fn deprecated_service1() {
  let schema = DeprecatedService1::as_proto_service();

  assert_eq_pretty!(schema.options[0], proto_deprecated());
}

#[proto_service]
#[proto(deprecated = true)]
enum DeprecatedService2 {
  Service {
    request: DeprecatedMsg1,
    response: DeprecatedMsg1,
  },
}

#[test]
fn deprecated_service2() {
  let schema = DeprecatedService2::as_proto_service();

  assert_eq_pretty!(schema.options[0], proto_deprecated());
}

#[proto_service]
#[deprecated]
#[proto(deprecated = false)]
enum DeprecatedService3 {
  Service {
    request: DeprecatedMsg1,
    response: DeprecatedMsg1,
  },
}

#[test]
fn deprecated_service3() {
  let schema = DeprecatedService3::as_proto_service();

  assert!(
    schema.options.is_empty(),
    "should override deprecated attribute"
  );
}

#[proto_service]
enum DeprecatedHandler1 {
  #[deprecated]
  Service {
    request: DeprecatedMsg1,
    response: DeprecatedMsg1,
  },
}

#[test]
fn deprecated_handler1() {
  let schema = DeprecatedHandler1::as_proto_service();

  assert_eq_pretty!(schema.handlers[0].options[0], proto_deprecated());
}

#[proto_service]
enum DeprecatedHandler2 {
  #[proto(deprecated = true)]
  Service {
    request: DeprecatedMsg1,
    response: DeprecatedMsg1,
  },
}

#[test]
fn deprecated_handler2() {
  let schema = DeprecatedHandler2::as_proto_service();

  assert_eq_pretty!(schema.handlers[0].options[0], proto_deprecated());
}

#[proto_service]
enum DeprecatedHandler3 {
  #[deprecated]
  #[proto(deprecated = false)]
  Service {
    request: DeprecatedMsg1,
    response: DeprecatedMsg1,
  },
}

#[test]
fn deprecated_handler3() {
  let schema = DeprecatedHandler3::as_proto_service();

  assert!(
    schema.handlers[0].options.is_empty(),
    "should override deprecated attribute"
  );
}
