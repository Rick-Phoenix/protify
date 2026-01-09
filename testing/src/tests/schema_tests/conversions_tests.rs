use super::*;

#[derive(PartialEq)]
struct IntWrapper(i32);

impl From<i32> for IntWrapper {
  fn from(value: i32) -> Self {
    Self(value)
  }
}

impl From<IntWrapper> for i32 {
  fn from(value: IntWrapper) -> Self {
    value.0
  }
}

#[proto_oneof(proxied, no_auto_test)]
#[derive(PartialEq)]
enum OneofWithDefault {
  #[proto(tag = 1)]
  A(String),
  #[proto(tag = 2, int32, from_proto = |v| IntWrapper(v), into_proto = |_| 1)]
  B(IntWrapper),
}

impl Default for OneofWithDefaultProto {
  fn default() -> Self {
    Self::B(5)
  }
}

#[proto_message(proxied, no_auto_test)]
struct WithDefaultOneof {
  #[proto(oneof(proxied, default, tags(1, 2)))]
  field: OneofWithDefault,
}

#[test]
fn oneof_with_default() {
  let msg = WithDefaultOneofProto::default();

  // The conversion should have used the default impl
  let converted: WithDefaultOneof = msg.into();

  assert_eq_pretty!(converted.field, OneofWithDefault::B(IntWrapper(5)));
}

#[proto_oneof(proxied, no_auto_test)]
#[derive(PartialEq)]
enum OneofCustomFieldConversions {
  #[proto(tag = 1, string, from_proto = |_| "from_proto".into(), into_proto = |_| "into_proto".to_string())]
  A(Arc<str>),
  #[proto(tag = 2, int32, from_proto = |_| IntWrapper(0), into_proto = |_| 1)]
  B(IntWrapper),
}

#[proto_message(proxied, no_auto_test)]
struct WithOneofWithCustomFieldConversions {
  #[proto(oneof(proxied, tags(1, 2)))]
  field: Option<OneofCustomFieldConversions>,
}

#[test]
fn oneof_with_custom_field_conversions() {
  let proxy = WithOneofWithCustomFieldConversions {
    field: Some(OneofCustomFieldConversions::A(Default::default())),
  };

  let msg = proxy.into_message();

  assert_eq_pretty!(
    msg.field,
    Some(OneofCustomFieldConversionsProto::A("into_proto".into()))
  );

  let new_proxy: WithOneofWithCustomFieldConversions = msg.into();

  assert_eq_pretty!(
    new_proxy.field,
    Some(OneofCustomFieldConversions::A("from_proto".into()))
  );
}

#[proto_oneof(proxied, no_auto_test)]
#[derive(PartialEq)]
enum OneofFieldIntoProtoOnly {
  #[proto(tag = 1, string, into_proto = |_| "into_proto".into())]
  A(Arc<str>),
  #[proto(tag = 2, int32)]
  B(i32),
}

#[test]
fn oneof_field_into_proto_only() {
  let proxy = OneofFieldIntoProtoOnly::A("abc".into());
  let oneof = proxy.into_oneof();

  assert_eq_pretty!(oneof, OneofFieldIntoProtoOnlyProto::A("into_proto".into()));
}

#[proto_oneof(proxied, no_auto_test)]
#[derive(PartialEq)]
enum OneofFieldFromProtoOnly {
  #[proto(tag = 1, string)]
  A(String),
  #[proto(tag = 2, int32, from_proto = |_| IntWrapper(1))]
  B(IntWrapper),
}

#[test]
fn oneof_field_from_proto_only() {
  let oneof = OneofFieldFromProtoOnlyProto::B(0);
  let proxy: OneofFieldFromProtoOnly = oneof.into();

  assert_eq_pretty!(proxy, OneofFieldFromProtoOnly::B(IntWrapper(1)));
}

// Correct if it compiles
#[proto_oneof(proxied, no_auto_test)]
#[derive(PartialEq)]
enum OneofDefaultConversion {
  #[proto(tag = 1, sint32)]
  A(IntWrapper),
  #[proto(tag = 2, int32)]
  B(IntWrapper),
}

#[proto_oneof(proxied, no_auto_test)]
#[derive(PartialEq)]
#[proto(from_proto = |_| OneofCustomConversions::A("from_proto".to_string()))]
#[proto(into_proto = |_| OneofCustomConversionsProto::A("into_proto".to_string()))]
enum OneofCustomConversions {
  #[proto(tag = 1, string)]
  A(String),
  #[allow(unused)]
  #[proto(tag = 2, int32)]
  B(IntWrapper),
}

#[test]
fn oneof_custom_conversions() {
  let mut proxy = OneofCustomConversions::A("abc".to_string());

  let oneof = proxy.into_oneof();

  assert_eq_pretty!(
    oneof,
    OneofCustomConversionsProto::A("into_proto".to_string())
  );

  proxy = oneof.into();

  assert_eq_pretty!(proxy, OneofCustomConversions::A("from_proto".to_string()));
}

#[proto_oneof(proxied, no_auto_test)]
#[derive(PartialEq)]
#[proto(into_proto = |_| OneofIntoProtoOnlyProto::A("into_proto".to_string()))]
enum OneofIntoProtoOnly {
  #[proto(tag = 1, string)]
  A(String),
  #[allow(unused)]
  #[proto(tag = 2, int32)]
  B(IntWrapper),
}

#[test]
fn oneof_into_proto_only() {
  let proxy = OneofIntoProtoOnly::A("abc".to_string());

  let oneof = proxy.into_oneof();

  assert_eq_pretty!(oneof, OneofIntoProtoOnlyProto::A("into_proto".to_string()));
}

#[proto_oneof(proxied, no_auto_test)]
#[derive(PartialEq)]
#[proto(from_proto = |_| OneofFromProtoOnly::A("from_proto".to_string()))]
enum OneofFromProtoOnly {
  #[proto(tag = 1, string)]
  A(String),
  #[allow(unused)]
  #[proto(tag = 2, int32)]
  B(IntWrapper),
}

#[test]
fn oneof_from_proto_only() {
  let oneof = OneofFromProtoOnlyProto::A("abc".to_string());

  let proxy: OneofFromProtoOnly = oneof.into();

  assert_eq_pretty!(proxy, OneofFromProtoOnly::A("from_proto".to_string()));
}
