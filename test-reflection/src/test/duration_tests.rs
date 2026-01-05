use proto_types::Duration;

use crate::proto::DurationRules;

use super::*;

fn one_sec() -> Duration {
  Duration {
    seconds: 1,
    nanos: 0,
  }
}

fn minus_one_sec() -> Duration {
  Duration {
    seconds: -1,
    nanos: 0,
  }
}

fn zero_sec() -> Duration {
  Duration::default()
}

#[allow(unused)]
#[test]
fn duration_tests() {
  let mut msg = DurationRules {
    const_test: Some(zero_sec()),
    lt_test: Some(minus_one_sec()),
    lte_test: Some(zero_sec()),
    gt_test: Some(one_sec()),
    gte_test: Some(zero_sec()),
    in_test: Some(zero_sec()),
    not_in_test: Some(one_sec()),
    required_test: Some(zero_sec()),
    ignore_always_test: Some(one_sec()),
  };

  let baseline = msg;

  assert!(msg.validate().is_ok(), "basic validation");

  macro_rules! assert_violation {
    ($violation:expr, $error:literal) => {
      assert_violation_id(&msg, $violation, $error);
      msg = baseline;
    };
  }

  msg.const_test = Some(one_sec());
  assert_violation!("duration.const", "const rule");

  msg.lt_test = Some(zero_sec());
  assert_violation!("duration.lt", "lt rule");

  msg.lte_test = Some(one_sec());
  assert_violation!("duration.lte", "lte rule");

  msg.gt_test = Some(zero_sec());
  assert_violation!("duration.gt", "gt rule");

  msg.gte_test = Some(minus_one_sec());
  assert_violation!("duration.gte", "gte rule");

  msg.in_test = Some(one_sec());
  assert_violation!("duration.in", "in rule");

  msg.not_in_test = Some(zero_sec());
  assert_violation!("duration.not_in", "not_in rule");

  msg.required_test = None;
  assert_violation!("required", "required rule");
}
