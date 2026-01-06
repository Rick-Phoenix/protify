#[cfg(feature = "reflection")]
use crate::proto::oneof_tests::TestOneof;

#[cfg(not(feature = "reflection"))]
use test_schemas::TestOneof;

use super::*;

#[test]
fn oneof_tests() {
  let mut msg = OneofTests {
    test_oneof: Some(TestOneof::String("a".to_string())),
  };
  let baseline = msg.clone();

  assert!(msg.validate().is_ok(), "basic validation");

  macro_rules! assert_violation {
    ($violation:expr, $error:expr) => {
      assert_violation_id(&msg, $violation, $error);
      msg = baseline.clone();
    };
  }

  msg.test_oneof = Some(TestOneof::BoxedMsg(Box::new(OneofTests {
    test_oneof: Some(TestOneof::String("b".to_string())),
  })));
  assert_violation!("string_cel_rule", "recursive oneof cel rule");

  msg.test_oneof = Some(TestOneof::BoxedMsg(Box::new(OneofTests {
    test_oneof: Some(TestOneof::String("c".to_string())),
  })));
  assert_violation!("recursive_cel_rule", "recursive oneof cel rule");

  msg.test_oneof = Some(TestOneof::BoxedMsg(Box::new(msg.clone())));
  assert!(msg.validate().is_ok(), "recursive oneof validation");
}
