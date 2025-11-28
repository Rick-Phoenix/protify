use std::{borrow::Cow, fmt::Display};

use syn::spanned::Spanned;

use crate::*;

#[derive(Clone)]
pub struct TypeInfo {
  pub rust_type: RustType,
  pub span: Span,
  pub proto_type: ProtoType,
}

impl TypeInfo {
  pub fn from_proto(&self) -> TokenStream2 {
    let conversion_call = self.proto_type.default_from_proto();

    match &self.rust_type {
      RustType::Option(_) => quote! { map(|v| v.#conversion_call) },
      RustType::Boxed(_) => quote! { map(|v| Box::new((*v).into())) },
      RustType::Map(_) => {
        let value_conversion = if let ProtoType::Map(map) = &self.proto_type && map.has_enum_values() {
          quote! { try_into().unwrap_or_default() }
        } else {
          quote! { into() }
        };

        quote! { into_iter().map(|(k, v)| (k, v.#value_conversion)).collect() }
      }
      RustType::Vec(_) => quote! { into_iter().map(|v| v.#conversion_call).collect() },
      RustType::Normal(_) => conversion_call,
    }
  }

  pub fn validator_tokens(
    &self,
    validator: &ValidatorExpr,
    proto_type: &ProtoType,
  ) -> TokenStream2 {
    let mut target_type = proto_type.validator_target_type();

    if matches!(self.rust_type, RustType::Vec(_)) {
      target_type = quote! { Vec<#target_type> };
    }

    match validator {
      ValidatorExpr::Call(call) => {
        quote! { Some(<ValidatorMap as ProtoValidator<#target_type>>::from_builder(#call)) }
      }

      ValidatorExpr::Closure(closure) => {
        quote! { Some(<ValidatorMap as ProtoValidator<#target_type>>::build_rules(#closure)) }
      }
    }
  }

  pub fn as_proto_type_trait_expr(&self, proto_type: &ProtoType) -> TokenStream2 {
    let base_target_type = proto_type.as_proto_type_trait_target();

    let target_type = match &self.rust_type {
      RustType::Option(_) => quote! { Option<#base_target_type> },
      RustType::Boxed(_) => quote! { Option<#base_target_type> },
      RustType::Map(_) => base_target_type,
      RustType::Vec(_) => quote! { Vec<#base_target_type> },
      RustType::Normal(_) => base_target_type,
    };

    quote! { <#target_type as AsProtoType>::proto_type() }
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

  pub fn from_type(ty: &Type, field_kind: ProtoFieldKind) -> Result<Self, Error> {
    let path = extract_type_path(ty)?;
    let rust_type = RustType::from_path(path);

    let proto_type = extract_proto_type(&rust_type, field_kind, ty)?;

    let span = ty.span();

    Ok(Self {
      rust_type,
      span,
      proto_type,
    })
  }

  pub fn error(&self, error: impl Display) -> Error {
    syn::Error::new(self.span, error)
  }
}
