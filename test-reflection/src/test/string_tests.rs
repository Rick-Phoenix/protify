use super::*;

use crate::proto::StringRules;

macro_rules! string {
  ($str:literal) => {
    $str.to_string()
  };
}

#[test]
fn string_test() {
  let mut msg = StringRules {
    const_test: string!("a"),
    len_test: string!("a"),
    min_len_test: string!("a"),
    max_len_test: string!("a"),
    len_bytes_test: string!("a"),
    min_bytes_test: string!("a"),
    max_bytes_test: string!("a"),
    pattern_test: string!("a"),
    prefix_test: string!("a"),
    suffix_test: string!("a"),
    contains_test: string!("a"),
    not_contains_test: string!("b"),
    cel_test: string!("a"),
    required_test: Some(string!("a")),
    ignore_if_zero_value_test: Some(string!("a")),
    ignore_always_test: string!("b"),
  };
  let baseline = msg.clone();

  assert!(msg.validate().is_ok(), "basic validation");

  macro_rules! assert_violation {
    ($violation:expr, $error:literal) => {
      assert_violation_id(&msg, $violation, $error);
      msg = baseline.clone();
    };
  }

  msg.const_test = string!("b");
  assert_violation!("string.const", "const rule");

  msg.len_test = string!("ab");
  assert_violation!("string.len", "len rule");

  msg.min_len_test = string!("");
  assert_violation!("string.min_len", "min_len rule");

  msg.max_len_test = string!("ab");
  assert_violation!("string.max_len", "max_len rule");

  msg.len_bytes_test = string!("ab");
  assert_violation!("string.len_bytes", "len_bytes rule");

  msg.min_bytes_test = string!("");
  assert_violation!("string.min_bytes", "min_bytes rule");

  msg.max_bytes_test = string!("ab");
  assert_violation!("string.max_bytes", "max_bytes rule");

  msg.prefix_test = string!("b");
  assert_violation!("string.prefix", "prefix rule");

  msg.suffix_test = string!("b");
  assert_violation!("string.suffix", "suffix rule");

  msg.pattern_test = string!("b");
  assert_violation!("string.pattern", "pattern rule");

  msg.contains_test = string!("b");
  assert_violation!("string.contains", "contains rule");

  msg.not_contains_test = string!("a");
  assert_violation!("string.not_contains", "not_contains rule");

  msg.cel_test = string!("b");
  assert_violation!("cel_rule", "cel rule");

  msg.required_test = None;
  assert_violation!("required", "required rule");

  msg.ignore_if_zero_value_test = None;
  assert!(msg.validate().is_ok(), "Should ignore if None");

  msg.ignore_if_zero_value_test = Some(String::new());
  assert!(msg.validate().is_ok(), "Should ignore if empty");
}
