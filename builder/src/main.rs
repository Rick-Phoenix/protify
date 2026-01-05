use prelude::{
  CelProgram, StringValidator, cel_program, define_proto_file, proto_message, proto_package,
};

fn random_cel() -> CelProgram {
  cel_program!(id = "abc", msg = "abc", expr = "true == true")
}

fn string_validator() -> StringValidator {
  StringValidator::builder()
    .len(5)
    .min_len(5)
    .max_len(5)
    .len_bytes(5)
    .min_bytes(5)
    .max_bytes(6)
    .pattern("abc")
    .prefix("abc")
    .suffix("abc")
    .contains("abc")
    .not_contains("abc")
    .in_(["abc", "cde"])
    .not_in(["abc", "cde"])
    .const_("abc")
    .ignore_if_zero_value()
    .cel(random_cel())
    .build()
}

#[proto_message(no_auto_test)]
struct StringRules {
  #[proto(validate = string_validator())]
  pub string_test: String,
}

proto_package!(REFLECTION, name = "reflection.v1");
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
      struct [< $name Rules >] {
        #[proto(validate = |v| v.required())]
        pub required_test: Option<$typ>,
        #[proto(validate = |v| v.lt(num!($($finite)?)))]
        pub lt_test: $typ,
        #[proto(validate = |v| v.lte(num!($($finite)?)))]
        pub lte_test: $typ,
        #[proto(validate = |v| v.gt(num!($($finite)?)))]
        pub gt_test: $typ,
        #[proto(validate = |v| v.gte(num!($($finite)?)))]
        pub gte_test: $typ,
        #[proto(validate = |v| v.in_([num!($($finite)?)]))]
        pub in_test: $typ,
        #[proto(validate = |v| v.not_in([num!($($finite)?)]))]
        pub not_in_test: $typ,
        #[proto(validate = |v| v.cel(cel_program!(id = "cel_rule", msg = "abc", expr = "this != 0")))]
        pub cel_test: $typ,
        #[proto(validate = |v| v.const_(num!($($finite)?)))]
        pub const_test: $typ,
        #[proto(validate = |v| v.const_(num!($($finite)?)).ignore_if_zero_value())]
        pub ignore_if_zero_value_test: Option<$typ>,
        #[proto(validate = |v| v.const_(num!($($finite)?)).ignore_always())]
        pub ignore_always_test: $typ,
        $(
          #[proto(validate = |v| v.$finite())]
          pub finite_test: $typ,
        )?
      }
    }
  };
}

impl_numeric!(Int64, i64);
impl_numeric!(Int32, i32);
impl_numeric!(Uint64, u64);
impl_numeric!(Uint32, u32);
impl_numeric!(Double, f64, finite);
impl_numeric!(Float, f32, finite);

fn main() {
  REFLECTION
    .get_package()
    .render_files(concat!(
      env!("CARGO_MANIFEST_DIR"),
      "/../test-reflection/proto"
    ))
    .unwrap()
}
