use syn::spanned::Spanned;

use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum ProtoMapKeys {
  String,
  Bool,
  Int32,
  Int64,
  Sint32,
  Sint64,
  Sfixed32,
  Sfixed64,
  Fixed32,
  Fixed64,
  Uint32,
  Uint64,
}

impl From<ProtoMapKeys> for ProtoType {
  fn from(value: ProtoMapKeys) -> Self {
    match value {
      ProtoMapKeys::String => Self::String,
      ProtoMapKeys::Int32 => Self::Int32,
      ProtoMapKeys::Sint32 => Self::Sint32,
      ProtoMapKeys::Bool => Self::Bool,
      ProtoMapKeys::Int64 => Self::Int64,
      ProtoMapKeys::Sint64 => Self::Sint64,
      ProtoMapKeys::Sfixed32 => Self::Sfixed32,
      ProtoMapKeys::Sfixed64 => Self::Sfixed64,
      ProtoMapKeys::Fixed32 => Self::Fixed32,
      ProtoMapKeys::Fixed64 => Self::Fixed64,
      ProtoMapKeys::Uint32 => Self::Uint32,
      ProtoMapKeys::Uint64 => Self::Uint64,
    }
  }
}

impl ProtoMapKeys {
  pub fn validator_target_type(&self) -> TokenStream2 {
    let pt: ProtoType = (*self).into();
    pt.validator_target_type()
  }

  pub fn output_proto_type(&self) -> TokenStream2 {
    let pt: ProtoType = (*self).into();
    pt.output_proto_type()
  }

  pub fn field_proto_type_tokens(&self) -> TokenStream2 {
    let pt: ProtoType = (*self).into();
    pt.field_proto_type_tokens()
  }
}

impl ProtoMapKeys {
  pub fn from_path(path: &Path) -> Result<Self, Error> {
    let ident = path.require_ident()?;
    let ident_str = ident.to_string();

    let output = match ident_str.as_str() {
      "String" | "string" => Self::String,
      "int32" | "i32" => Self::Int32,
      "int64" | "i64" => Self::Int64,
      "uint32" | "u32" => Self::Uint32,
      "uint64" | "u64" => Self::Uint64,
      "bool" => Self::Bool,
      "sint64" => Self::Sint64,
      "sint32" => Self::Sint32,
      "sfixed32" => Self::Sfixed32,
      "sfixed64" => Self::Sfixed64,
      "fixed32" => Self::Fixed32,
      "fixed64" => Self::Fixed64,
      _ => bail!(
        ident,
        "Type {ident_str} is not a supported map key primitive"
      ),
    };

    Ok(output)
  }
}

impl Display for ProtoMapKeys {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let pt: ProtoType = (*self).into();

    // Same implementation
    write!(f, "{}", pt.as_prost_map_value())
  }
}

#[derive(Debug, Clone)]
pub struct ProtoMap {
  pub keys: ProtoMapKeys,
  pub values: ProtoType,
}

pub fn parse_map_with_context(
  input: syn::parse::ParseStream,
  rust_type: &RustType,
) -> syn::Result<ProtoMap> {
  let mut metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

  if metas.len() != 2 {
    bail!(
      metas,
      "Expected a list of two items indicating the types of keys and values"
    );
  }

  let values = match metas.pop().unwrap().into_value() {
    Meta::Path(path) => {
      let ident = path.require_ident()?.to_string();
      let span = path.span();

      let fallback = if let RustType::HashMap((_, v)) = rust_type {
        v.as_path()
      } else {
        None
      };

      ProtoType::from_ident(&ident, span, fallback.as_ref())?
        .ok_or(error_with_span!(span, "Unrecognized map keys type"))?
    }
    Meta::List(list) => {
      let list_ident = ident_string!(list.path);
      let span = list.span();

      let fallback = if let RustType::HashMap((_, v)) = rust_type {
        v.as_path()
      } else {
        None
      };

      ProtoType::from_meta_list(&list_ident, list, fallback.as_ref())
        .map_err(|e| input.error(e))?
        .ok_or(error_with_span!(span, "Unrecognized map values type"))?
    }
    Meta::NameValue(nv) => bail!(nv, "Expected the values to be a list or path"),
  };

  let keys_input = metas.pop().unwrap().into_value();
  let keys = ProtoMapKeys::from_path(keys_input.require_path_only()?)?;

  Ok(ProtoMap { keys, values })
}
