use ::bytes::Bytes;
use bon::Builder;
use bytes_validator_builder::{IsUnset, SetWellKnown, State};
use regex::Regex;

use super::*;

impl_validator!(BytesValidator, Vec<u8>);
impl_validator!(BytesValidator, Bytes);
impl_into_option!(BytesValidator);
impl_ignore!(BytesValidatorBuilder);

macro_rules! insert_bytes_option {
  ($validator:ident, $values:ident, $field:ident) => {
    $validator.$field.map(|v| {
      $values.push((
        $crate::paste!([< $field:upper >]).clone(),
        OptionValue::String(format_bytes_as_proto_string_literal(&v).into()),
      ))
    })
  };

  ($validator:ident, $values:ident, $field:ident, list) => {
    $validator.$field.map(|v| {
      $values.push((
        $crate::paste!([< $field:upper >]).clone(),
        OptionValue::List(
          v.iter()
            .map(|i| OptionValue::String(format_bytes_as_proto_string_literal(i).into()))
            .collect::<Vec<OptionValue>>()
            .into(),
        ),
      ))
    })
  };
}

#[derive(Clone, Debug, Builder)]
pub struct BytesValidator {
  /// Specifies the exact length for this bytes field to be considered valid.
  pub len: Option<u64>,
  /// The minimum length for this field in order to be considered valid.
  pub min_len: Option<u64>,
  /// The maximum length for this field in order to be considered valid.
  pub max_len: Option<u64>,
  /// The pattern that this field must match in order to be valid.
  pub pattern: Option<Regex>,
  /// A prefix that this field must contain in order to be valid.
  pub prefix: Option<Bytes>,
  /// A suffix that this field must contain in order to be valid.
  pub suffix: Option<Bytes>,
  /// A subset of bytes that this field must contain in order to be valid.
  pub contains: Option<Bytes>,
  /// Only the values in this list will be considered valid for this field.
  #[builder(into)]
  pub in_: Option<Arc<[Bytes]>>,
  /// The values in this list will be considered invalid for this field.
  #[builder(into)]
  pub not_in: Option<Arc<[Bytes]>>,
  #[builder(setters(vis = "", name = well_known))]
  pub well_known: Option<WellKnownBytes>,
  /// Only this specific value will be considered valid for this field.
  pub const_: Option<Bytes>,
  /// Adds custom validation using one or more [`CelRule`]s to this field.
  #[builder(into)]
  pub cel: Option<Arc<[CelRule]>>,
  #[builder(with = || true)]
  /// Marks the field as invalid if unset.
  pub required: Option<bool>,
  #[builder(setters(vis = "", name = ignore))]
  pub ignore: Option<Ignore>,
}

impl From<BytesValidator> for ProtoOption {
  #[track_caller]
  fn from(validator: BytesValidator) -> Self {
    let mut rules: OptionValueList = Vec::new();

    if let Some(const_val) = validator.const_ {
      rules.push((
        CONST_.clone(),
        OptionValue::String(format_bytes_as_proto_string_literal(&const_val).into()),
      ));
    }

    validate_lists(validator.in_.as_deref(), validator.not_in.as_deref()).unwrap();

    if validator.len.is_none() {
      insert_option!(validator, rules, min_len);
      insert_option!(validator, rules, max_len);
    } else {
      insert_option!(validator, rules, len);
    }

    if let Some(pattern) = validator.pattern {
      rules.push((
        PATTERN.clone(),
        OptionValue::String(pattern.as_str().into()),
      ))
    }

    insert_bytes_option!(validator, rules, contains);
    insert_bytes_option!(validator, rules, prefix);
    insert_bytes_option!(validator, rules, suffix);
    insert_bytes_option!(validator, rules, in_, list);
    insert_bytes_option!(validator, rules, not_in, list);

    if let Some(v) = validator.well_known {
      v.to_option(&mut rules);
    }

    let mut outer_rules: OptionValueList = vec![];

    outer_rules.push((BYTES.clone(), OptionValue::Message(rules.into())));

    insert_cel_rules!(validator, outer_rules);
    insert_option!(validator, outer_rules, required);
    insert_option!(validator, outer_rules, ignore);

    ProtoOption {
      name: BUF_VALIDATE_FIELD.clone(),
      value: OptionValue::Message(outer_rules.into()),
    }
  }
}

#[doc(hidden)]
#[derive(Clone, Debug, Copy)]
pub enum WellKnownBytes {
  Ip,
  Ipv4,
  Ipv6,
}

macro_rules! well_known_impl {
  ($name:ident, $doc:literal) => {
    paste::paste! {
      #[doc = $doc]
      pub fn [< $name:snake >](self) -> BytesValidatorBuilder<SetWellKnown<S>>
        where
          S::WellKnown: IsUnset,
        {
          self.well_known(WellKnownBytes::$name)
        }
    }
  };
}

impl<S: State> BytesValidatorBuilder<S> {
  well_known_impl!(Ip, "The value must be an IPv4/Ipv6 address");
  well_known_impl!(Ipv4, "The value must be an IPv4 address");
  well_known_impl!(Ipv6, "The value must be an IPv6 address");
}

impl WellKnownBytes {
  pub(crate) fn to_option(self, option_values: &mut OptionValueList) {
    let name = match self {
      WellKnownBytes::Ip => IP.clone(),
      WellKnownBytes::Ipv4 => IPV4.clone(),
      WellKnownBytes::Ipv6 => IPV6.clone(),
    };

    option_values.push((name, OptionValue::Bool(true)));
  }
}

fn format_bytes_as_proto_string_literal(bytes: &[u8]) -> String {
  let mut result = String::new();

  for &byte in bytes {
    match byte {
      0x20..=0x21 | 0x23..=0x5B | 0x5D..=0x7E => {
        result.push(byte as char);
      }
      b'\\' => result.push_str("\\\\"),
      b'"' => result.push_str("\\\""),
      _ => {
        result.push_str(&format!("\\x{:02x}", byte));
      }
    }
  }

  result
}
