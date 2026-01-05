#![allow(clippy::struct_field_names)]

use prelude::{
  CelProgram, StringValidator, cel_program, define_proto_file, proto_message, proto_package,
};

#[proto_message(no_auto_test)]
struct BoolRules {
  #[proto(validate = |v| v.const_(true))]
  pub const_test: bool,
  #[proto(validate = |v| v.required())]
  pub required_test: Option<bool>,
  #[proto(validate = |v| v.const_(true).ignore_if_zero_value())]
  pub ignore_if_zero_value_test: Option<bool>,
  #[proto(validate = |v| v.const_(true).ignore_always())]
  pub ignore_always_test: bool,
}

#[proto_message(no_auto_test)]
struct StringRules {
  #[proto(validate = |v| v.const_("a"))]
  pub const_test: String,
  #[proto(validate = |v| v.len(1))]
  pub len_test: String,
  #[proto(validate = |v| v.min_len(1))]
  pub min_len_test: String,
  #[proto(validate = |v| v.max_len(1))]
  pub max_len_test: String,
  #[proto(validate = |v| v.len_bytes(1))]
  pub len_bytes_test: String,
  #[proto(validate = |v| v.min_bytes(1))]
  pub min_bytes_test: String,
  #[proto(validate = |v| v.max_bytes(1))]
  pub max_bytes_test: String,
  #[proto(validate = |v| v.pattern("a"))]
  pub pattern_test: String,
  #[proto(validate = |v| v.prefix("a"))]
  pub prefix_test: String,
  #[proto(validate = |v| v.suffix("a"))]
  pub suffix_test: String,
  #[proto(validate = |v| v.contains("a"))]
  pub contains_test: String,
  #[proto(validate = |v| v.not_contains("a"))]
  pub not_contains_test: String,
  #[proto(validate = |v| v.cel(cel_program!(id = "cel_rule", msg = "abc", expr = "this == 'a'")))]
  pub cel_test: String,
  #[proto(validate = |v| v.required())]
  pub required_test: Option<String>,
  #[proto(validate = |v| v.const_("a").ignore_if_zero_value())]
  pub ignore_if_zero_value_test: Option<String>,
  #[proto(validate = |v| v.const_("b").ignore_always())]
  pub ignore_always_test: String,
}

proto_package!(REFLECTION, name = "reflection.v1", no_cel_test);
define_proto_file!(
  REFLECTION_FILE,
  file = "reflection.proto",
  package = REFLECTION
);

macro_rules! impl_numeric {
  ($name:ident, $typ:ty $(, $finite:ident)?) => {
    macro_rules! num {
      (finite) => (1.0);
      () => (1);
    }

    paste::paste! {
      #[allow(unused, clippy::struct_field_names)]
      #[proto_message(no_auto_test)]
      struct [< $name:camel Rules >] {
        #[proto($name, validate = |v| v.required())]
        pub required_test: Option<$typ>,
        #[proto($name, validate = |v| v.lt(num!($($finite)?)))]
        pub lt_test: $typ,
        #[proto($name, validate = |v| v.lte(num!($($finite)?)))]
        pub lte_test: $typ,
        #[proto($name, validate = |v| v.gt(num!($($finite)?)))]
        pub gt_test: $typ,
        #[proto($name, validate = |v| v.gte(num!($($finite)?)))]
        pub gte_test: $typ,
        #[proto($name, validate = |v| v.in_([num!($($finite)?)]))]
        pub in_test: $typ,
        #[proto($name, validate = |v| v.not_in([num!($($finite)?)]))]
        pub not_in_test: $typ,
        #[proto($name, validate = |v| v.cel(cel_program!(id = "cel_rule", msg = "abc", expr = "this != 0")))]
        pub cel_test: $typ,
        #[proto($name, validate = |v| v.const_(num!($($finite)?)))]
        pub const_test: $typ,
        #[proto($name, validate = |v| v.const_(num!($($finite)?)).ignore_if_zero_value())]
        pub ignore_if_zero_value_test: Option<$typ>,
        #[proto($name, validate = |v| v.const_(num!($($finite)?)).ignore_always())]
        pub ignore_always_test: $typ,
        $(
          #[proto($name, validate = |v| v.$finite())]
          pub finite_test: $typ,
        )?
      }
    }
  };
}

impl_numeric!(int64, i64);
impl_numeric!(sint64, i64);
impl_numeric!(sfixed64, i64);
impl_numeric!(int32, i32);
impl_numeric!(sint32, i32);
impl_numeric!(sfixed32, i32);
impl_numeric!(uint64, u64);
impl_numeric!(uint32, u32);
impl_numeric!(fixed64, u64);
impl_numeric!(fixed32, u32);
impl_numeric!(double, f64, finite);
impl_numeric!(float, f32, finite);

fn main() {
  REFLECTION
    .get_package()
    .render_files(concat!(
      env!("CARGO_MANIFEST_DIR"),
      "/../test-reflection/proto"
    ))
    .unwrap()
}
