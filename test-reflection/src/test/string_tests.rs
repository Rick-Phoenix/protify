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
    email_test: string!("hobbits@shire.com"),
    hostname_test: string!("hobbits.shire.com"),
    ip_test: string!("127.0.0.1"),
    ipv4_test: string!("127.0.0.1"),
    ipv6_test: string!("91bd:9c4e:f03a:e782:cdbc:96ba:7241:9f00"),
    uri_test: string!("https://hobbits.com/hobbits?name=bilbo#bilbo-baggins"),
    uri_ref_test: string!("./hobbits/frodo"),
    address_test: string!("::1"),
    ulid_test: string!("01KE7RH05B5RMVC0262ZTERDZZ"),
    uuid_test: string!("019b8f7c-d933-7be4-8b69-5110ed453a75"),
    tuuid_test: string!("019b8f7cd9337be48b695110ed453a75"),
    ip_with_prefixlen_test: string!("192.168.0.1/32"),
    ipv4_with_prefixlen_test: string!("192.168.0.1/32"),
    ipv6_with_prefixlen_test: string!("91bd:9c4e:f03a::/64"),
    ip_prefix_test: string!("192.168.0.0/32"),
    ipv4_prefix_test: string!("192.168.0.0/32"),
    ipv6_prefix_test: string!("91bd:9c4e:f03a::/64"),
    host_and_port_test: string!("hobbits.com:3000"),
    header_name_strict_test: string!("content-type"),
    header_name_loose_test: string!("X-Héader"),
    header_value_strict_test: string!("application/json; charset=uft-8"),
    header_value_loose_test: string!("value\u{007F}with\u{007F}del"),
  };
  let baseline = msg.clone();

  assert!(msg.validate().is_ok(), "basic validation");

  macro_rules! assert_violation {
    ($violation:expr, $error:expr) => {
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

  macro_rules! assert_well_known_violations {
    ($($name:ident),*) => {
      paste::paste! {
        $(
          msg.[< $name _test >] = string!("ol' Tom Bombadil, he's a merry fellow!");
          assert_violation!(concat!("string.", stringify!($name)), concat!(stringify!($name), " rule"));
        )*
      }
    };
  }

  assert_well_known_violations!(
    email,
    hostname,
    ip,
    ipv4,
    ipv6,
    uri,
    uri_ref,
    address,
    uuid,
    ulid,
    tuuid,
    ip_with_prefixlen,
    ipv4_with_prefixlen,
    ipv6_with_prefixlen,
    ip_prefix,
    ipv4_prefix,
    ipv6_prefix,
    host_and_port
  );

  msg.header_name_loose_test = string!("");
  assert_violation!("string.well_known_regex", "header name loose rule");

  msg.header_name_strict_test = string!("");
  assert_violation!("string.well_known_regex", "header name strict rule");

  msg.header_value_loose_test = string!("");
  assert_violation!("string.well_known_regex", "header value loose rule");

  msg.header_value_strict_test = string!("");
  assert_violation!("string.well_known_regex", "header value strict rule");

  msg.cel_test = string!("b");
  assert_violation!("cel_rule", "cel rule");

  msg.required_test = None;
  assert_violation!("required", "required rule");

  msg.ignore_if_zero_value_test = None;
  assert!(msg.validate().is_ok(), "Should ignore if None");

  msg.ignore_if_zero_value_test = Some(String::new());
  assert!(msg.validate().is_ok(), "Should ignore if empty");
}
