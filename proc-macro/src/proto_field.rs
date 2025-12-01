use crate::*;

#[derive(Clone)]
pub enum ProtoField {
  Map(ProtoMap),
  Oneof {
    path: Path,
    tags: Vec<i32>,
    default: bool,
    is_proxied: bool,
  },
  Repeated(ProtoType),
  Optional(ProtoType),
  Single(ProtoType),
}

impl ProtoField {
  pub fn validator_tokens(&self, validator: ValidatorExpr) -> TokenStream2 {
    match self {
      ProtoField::Map(proto_map) => todo!(),
      ProtoField::Oneof {
        path,
        tags,
        default,
        is_proxied,
      } => todo!(),
      ProtoField::Repeated(proto_type) => todo!(),
      ProtoField::Optional(proto_type) => todo!(),
      ProtoField::Single(proto_type) => todo!(),
    }
  }

  pub fn default_from_proto(&self, base_ident: &TokenStream2) -> TokenStream2 {
    match self {
      Self::Oneof {
        default: true,
        is_proxied,
        ..
      } => {
        if *is_proxied {
          quote! { #base_ident.unwrap_or_default().into() }
        } else {
          quote! { #base_ident.unwrap_or_default() }
        }
      }
      _ => quote! { #base_ident.into() },
    }
  }

  pub fn validator_target_type(&self) -> TokenStream2 {
    match self {
      Self::Map(map) => map.validator_target_type(),
      _ => todo!(),
    }
  }

  pub fn as_proto_type_trait_target(&self) -> TokenStream2 {
    match self {
      Self::Map(map) => map.as_proto_type_trait_target(),
      _ => todo!(),
    }
  }

  pub fn as_proto_type_trait_expr(&self) -> TokenStream2 {
    let target_type = self.output_proto_type();

    quote! { <#target_type as AsProtoType>::proto_type() }
  }

  pub fn output_proto_type(&self) -> TokenStream2 {
    match self {
      Self::Map(map) => map.output_proto_type(),
      Self::Oneof { path, .. } => path.to_token_stream(),
      ProtoField::Repeated(inner) => {
        let inner_type = inner.output_proto_type();

        quote! { Vec<#inner_type> }
      }
      ProtoField::Optional(inner) => {
        let inner_type = inner.output_proto_type();

        quote! { Option<#inner_type> }
      }
      ProtoField::Single(inner) => inner.output_proto_type(),
    }
  }

  pub fn as_prost_attr_type(&self) -> TokenStream2 {
    match self {
      Self::Map(map) => map.as_prost_attr_type(),

      _ => todo!(),
    }
  }

  /// Returns `true` if the proto field is [`Oneof`].
  ///
  /// [`Oneof`]: ProtoField::Oneof
  #[must_use]
  pub fn is_oneof(&self) -> bool {
    matches!(self, Self::Oneof { .. })
  }
}
