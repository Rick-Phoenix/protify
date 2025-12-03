use crate::*;
mod common_strings;
use std::{collections::HashSet, fmt::Debug, hash::Hash, sync::Arc};

use common_strings::*;
use proto_types::protovalidate::Ignore;

pub trait ValidatorBuilderFor<T>: Into<ProtoOption> {}

pub trait ProtoValidator<T> {
  type Builder;

  fn builder() -> Self::Builder;

  #[track_caller]
  fn from_builder<B>(builder: B) -> ProtoOption
  where
    B: ValidatorBuilderFor<T>,
  {
    builder.into()
  }

  #[track_caller]
  fn build_rules<F, FinalBuilder>(config_fn: F) -> ProtoOption
  where
    F: FnOnce(Self::Builder) -> FinalBuilder,
    FinalBuilder: ValidatorBuilderFor<T>,
  {
    let initial_builder = Self::builder();

    let final_builder = config_fn(initial_builder);

    final_builder.into()
  }
}

pub struct ValidatorMap;

type OptionValueList = Vec<(Arc<str>, OptionValue)>;

impl From<Ignore> for OptionValue {
  fn from(value: Ignore) -> Self {
    let name = match value {
      Ignore::Unspecified => IGNORE_UNSPECIFIED.clone(),
      Ignore::IfZeroValue => IGNORE_IF_ZERO_VALUE.clone(),
      Ignore::Always => IGNORE_ALWAYS.clone(),
    };

    OptionValue::Enum(name)
  }
}

fn create_string_list<T: Into<Arc<str>>, I: IntoIterator<Item = T>>(list: I) -> Arc<[Arc<str>]> {
  let new_list: Vec<Arc<str>> = list.into_iter().map(|i| i.into()).collect();

  new_list.into()
}

macro_rules! impl_ignore {
  ($builder:ident) => {
    $crate::paste! {
      impl < S: [< $builder:snake >]::State> $builder< S>
      where
        S::Ignore: [< $builder:snake >]::IsUnset,
      {
        /// Rules defined for this field will be ignored if the field is set to its protobuf zero value.
        pub fn ignore_if_zero_value(self) -> $builder< [< $builder:snake >]::SetIgnore<S>> {
          self.ignore(Ignore::IfZeroValue)
        }

        /// Rules set for this field will always be ignored.
        pub fn ignore_always(self) -> $builder< [< $builder:snake >]::SetIgnore<S>> {
          self.ignore(Ignore::Always)
        }
      }
    }
  };
}

fn validate_comparables<T>(
  lt: Option<T>,
  lte: Option<T>,
  gt: Option<T>,
  gte: Option<T>,
) -> Result<(), &'static str>
where
  T: Copy + PartialEq + PartialOrd,
{
  if lt.is_some() && lte.is_some() {
    return Err("Cannot use lt and lte together");
  }

  if gt.is_some() && gte.is_some() {
    return Err("Cannot use gt and gte together");
  }

  if let Some(lt_val) = lt {
    if let Some(gt_val) = gt && lt_val <= gt_val {
      return Err("Lt cannot be smaller than or equal to gt");
    }

    if let Some(gte_val) = gte && lt_val < gte_val {
      return Err("Lt cannot be smaller than gte");
    }
  }

  if let Some(lte_val) = lte {
    if let Some(gt_val) = gt && lte_val < gt_val {
      return Err("Lte cannot be smaller than to gt");
    }

    if let Some(gte_val) = gte && lte_val < gte_val {
      return Err("Lte cannot be smaller than gte");
    }
  }

  Ok(())
}

fn validate_lists<'a, T>(
  in_list: Option<&'a [T]>,
  not_in_list: Option<&'a [T]>,
) -> Result<(), String>
where
  T: Eq + Hash + Debug + Clone,
{
  let in_list = in_list.unwrap_or(&[]);
  let not_in_list = not_in_list.unwrap_or(&[]);

  if in_list.is_empty() || not_in_list.is_empty() {
    return Ok(());
  }

  let in_list_set: HashSet<&T> = in_list.iter().collect();
  let not_in_list_set: HashSet<&T> = not_in_list.iter().collect();

  let mut invalid_vals: Vec<T> = Vec::new();

  in_list_set
    .intersection(&not_in_list_set)
    .for_each(|&v| invalid_vals.push(v.to_owned()));

  if !invalid_vals.is_empty() {
    Err(format!(
      "The following values are present inside of 'in' and 'not_in': {:?}",
      invalid_vals
    ))
  } else {
    Ok(())
  }
}

#[macro_use]
mod macros {
  macro_rules! insert_cel_rules {
    ($validator:ident, $values:ident) => {
      if let Some(cel_rules) = $validator.cel {
        let rule_values: Vec<OptionValue> =
          cel_rules.iter().cloned().map(OptionValue::from).collect();
        $values.push((CEL.clone(), OptionValue::List(rule_values.into())));
      }
    };
  }

  macro_rules! insert_option {
    (
    $validator:ident,
    $values:ident,
    $field:ident
  ) => {
      $crate::paste! {
        if let Some(value) = $validator.$field {
          $values.push(([< $field:snake:upper >].clone(), value.into()))
        }
      }
    };
  }
}

mod any;
mod bool;
mod bytes;
mod cel;
mod duration;
mod enums;
mod map;
mod message;
mod numeric;
mod oneof;
mod repeated;
mod string;
mod timestamp;

pub use any::*;
pub use bool::*;
pub use bytes::*;
pub use cel::*;
pub use duration::*;
pub use enums::*;
pub use map::*;
pub use message::*;
pub use numeric::*;
pub use oneof::*;
pub use repeated::{repeated_validator_builder, *};
pub use string::*;
pub use timestamp::*;
