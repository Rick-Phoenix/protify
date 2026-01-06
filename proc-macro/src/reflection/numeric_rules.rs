use super::*;
use ::proto_types::protovalidate::*;

pub fn get_numeric_validator<T: NumericRules>(ctx: &RulesCtx) -> TokenStream2 {
  let type_tokens = T::type_tokens();
  let mut validator = if T::IS_FLOAT {
    quote! { ::prelude::FloatValidator::<#type_tokens>::builder() }
  } else {
    quote! { ::prelude::IntValidator::<#type_tokens>::builder() }
  };

  ctx.tokenize_ignore(&mut validator);
  ctx.tokenize_required(&mut validator);
  ctx.tokenize_cel_rules(&mut validator);

  if let Some(rules) = T::from_field_rules(&ctx.rules) {
    if let Some(val) = rules.const_() {
      validator.extend(quote! { .const_(#val) });
    }

    macro_rules! rule {
      ($name:ident) => {
        if let Some($name) = rules.$name() {
          validator.extend(quote! { .$name(#$name) });
        }
      };
    }

    rule!(lte);
    rule!(lt);
    rule!(gte);
    rule!(gt);

    let in_list = rules.in_();
    if !in_list.is_empty() {
      validator.extend(quote! { .in_([ #(#in_list),* ]) });
    }

    let not_in_list = rules.not_in();
    if !not_in_list.is_empty() {
      validator.extend(quote! { .not_in([ #(#not_in_list),* ]) });
    }

    if rules.finite() {
      validator.extend(quote! { .finite() });
    }
  }

  quote! { #validator.build() }
}

pub trait NumericRules {
  type Unit: ToTokens + Copy;
  const IS_FLOAT: bool = false;

  fn type_tokens() -> TokenStream2;
  fn from_field_rules(field_rules: &FieldRules) -> Option<&Self>;
  fn const_(&self) -> Option<Self::Unit>;
  fn lte(&self) -> Option<Self::Unit>;
  fn lt(&self) -> Option<Self::Unit>;
  fn gte(&self) -> Option<Self::Unit>;
  fn gt(&self) -> Option<Self::Unit>;
  fn in_(&self) -> &[Self::Unit];
  fn not_in(&self) -> &[Self::Unit];
  fn finite(&self) -> bool;
}

macro_rules! impl_float_methods {
  (float) => {
    const IS_FLOAT: bool = true;

    fn finite(&self) -> bool {
      self.finite()
    }
  };

  () => {
    fn finite(&self) -> bool {
      false
    }
  };
}

macro_rules! impl_numeric_rules {
  ($name:ident, $unit:ty, $wrapper:ty $(, $float:ident)?) => {
    paste::paste! {
      impl NumericRules for [< $name Rules >] {
        type Unit = $unit;

        fn from_field_rules(field_rules: &FieldRules) -> Option<&Self> {
          field_rules
          .r#type
          .as_ref()
          .and_then(|rt| match rt {
            RulesType::[< $name:camel >](rules) => Some(rules),
            _ => None,
          })
        }

        fn type_tokens() -> TokenStream2 {
          quote! { $wrapper }
        }

        fn const_(&self) -> Option<Self::Unit> {
          self.r#const
        }

        fn lte(&self) -> Option<Self::Unit> {
          self.less_than.and_then(|lt| match lt {
            [< $name:snake _rules >]::LessThan::Lte(val) => Some(val),
            _ => None,
          })
        }

        fn lt(&self) -> Option<Self::Unit> {
          self.less_than.and_then(|lt| match lt {
            [< $name:snake _rules >]::LessThan::Lt(val) => Some(val),
            _ => None,
          })
        }

        fn gte(&self) -> Option<Self::Unit> {
          self.greater_than.and_then(|lt| match lt {
            [< $name:snake _rules >]::GreaterThan::Gte(val) => Some(val),
            _ => None,
          })
        }

        fn gt(&self) -> Option<Self::Unit> {
          self.greater_than.and_then(|lt| match lt {
            [< $name:snake _rules >]::GreaterThan::Gt(val) => Some(val),
            _ => None,
          })
        }

        fn in_(&self) -> &[Self::Unit] {
          &self.r#in
        }

        fn not_in(&self) -> &[Self::Unit] {
          &self.not_in
        }

        impl_float_methods!($($float)?);
      }
    }
  };
}

impl_numeric_rules!(Int64, i64, i64);
impl_numeric_rules!(SInt64, i64, Sint64);
impl_numeric_rules!(SFixed64, i64, Sfixed64);
impl_numeric_rules!(Int32, i32, i32);
impl_numeric_rules!(SInt32, i32, Sint32);
impl_numeric_rules!(SFixed32, i32, Sfixed32);
impl_numeric_rules!(UInt64, u64, u64);
impl_numeric_rules!(Fixed64, u64, Fixed64);
impl_numeric_rules!(UInt32, u32, u32);
impl_numeric_rules!(Fixed32, u32, Fixed32);
impl_numeric_rules!(Float, f32, f32, float);
impl_numeric_rules!(Double, f64, f64, float);
