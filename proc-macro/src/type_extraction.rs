use std::borrow::Cow;

use crate::*;

#[derive(Clone)]
pub struct TypeInfo<'a> {
  pub full_type: Cow<'a, Path>,
  pub custom_type: Option<Path>,
  pub rust_type: RustType,
}

impl<'a> TypeInfo<'a> {
  pub fn validator_tokens(&self, validator: &ValidatorExpr) -> TokenStream2 {
    let validator_type = if let Some(custom_type) = &self.custom_type {
      custom_type
    } else {
      match &self.rust_type {
        RustType::Option(path) => path,
        RustType::Boxed(path) => path,
        RustType::Map(_) => self.full_type.as_ref(),
        RustType::Vec(_) => self.full_type.as_ref(),
        RustType::Normal(path) => path,
      }
    };

    match validator {
      ValidatorExpr::Call(call) => {
        quote! { Some(<ValidatorMap as ProtoValidator<#validator_type>>::from_builder(#call)) }
      }

      ValidatorExpr::Closure(closure) => {
        quote! { Some(<ValidatorMap as ProtoValidator<#validator_type>>::build_rules(#closure)) }
      }
    }
  }

  pub fn is_option(&self) -> bool {
    matches!(self.rust_type, RustType::Option(_))
  }

  pub fn as_inner_option_path(&self) -> Option<&Path> {
    if let RustType::Option(path) = &self.rust_type {
      Some(path)
    } else {
      None
    }
  }

  pub fn from_path(path: &'a Path, custom_type: Option<Path>) -> Result<Self, Error> {
    let rust_type = RustType::from_path(path);

    Ok(Self {
      full_type: Cow::Borrowed(path),
      custom_type,
      rust_type,
    })
  }

  pub fn from_type(ty: &'a Type, custom_type: Option<Path>) -> Result<Self, Error> {
    let path = extract_type_path(ty)?;
    let rust_type = RustType::from_path(path);

    Ok(Self {
      full_type: Cow::Borrowed(path),
      custom_type,
      rust_type,
    })
  }
}
