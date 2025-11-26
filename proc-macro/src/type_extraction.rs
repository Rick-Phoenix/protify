use std::borrow::Cow;

use itertools::Itertools;
use syn::{GenericArgument, PathArguments, PathSegment};

use crate::*;

pub enum RustType {
  Option(Path),
  Boxed(Path),
  Map((Path, Path)),
  Vec(Path),
  Normal(Path),
}

impl ProtoTypes {
  pub fn from_rust_type(rust_type: &RustType) -> Option<Self> {
    let output = match rust_type {
      RustType::Option(path) => Self::from_path(&path),
      RustType::Boxed(path) => Self::from_path(&path),
      RustType::Map((k, v)) => {
        Self::Map((Box::new(Self::from_path(&k)), Box::new(Self::from_path(&v))))
      }
      RustType::Vec(path) => Self::from_path(&path),
      RustType::Normal(path) => Self::from_path(&path),
    };

    Some(output)
  }
}

impl RustType {
  pub fn as_option(&self) -> Option<&Path> {
    if let Self::Option(path) = self {
      Some(path)
    } else {
      None
    }
  }
}

pub struct ProstAttrs {
  pub field_type: ProtoTypes,
  pub cardinality: ProstCardinality,
  pub tag: i32,
}

impl ProstAttrs {
  pub fn from_type_info(type_info: &TypeInfo, tag: i32) -> Self {
    let cardinality = match &type_info.rust_type {
      RustType::Option(_) => ProstCardinality::Optional,
      RustType::Boxed(_) => ProstCardinality::Boxed,
      RustType::Vec(_) => ProstCardinality::Repeated,

      _ => ProstCardinality::Single,
    };

    let field_type = type_info.proto_type.clone();

    Self {
      field_type,
      cardinality,
      tag,
    }
  }
}

impl ToTokens for ProstAttrs {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let Self {
      field_type,
      cardinality,
      tag,
    } = self;

    let tag_as_str = tag.to_string();

    let output = quote! { #[proto(#field_type, #cardinality tag2 = #tag_as_str)] };

    tokens.extend(output);
  }
}

pub enum ProstCardinality {
  Repeated,
  Optional,
  Single,
  Boxed,
}

impl ToTokens for ProstCardinality {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let output = match self {
      ProstCardinality::Repeated => quote! { repeated, },
      ProstCardinality::Optional => quote! { optional, },
      ProstCardinality::Single => TokenStream2::new(),
      ProstCardinality::Boxed => quote! { optional, boxed },
    };

    tokens.extend(output);
  }
}

impl RustType {
  pub fn from_path(path: &Path) -> Self {
    let path_wrapper = PathWrapper::new(Cow::Borrowed(path));

    let last_segment = path_wrapper.last_segment();

    let type_ident = last_segment.ident().to_string();

    match type_ident.as_str() {
      "Option" => {
        let inner = last_segment.first_argument().unwrap();

        if inner.is_ident("Box") {
          let box_wrapper = PathWrapper::new(Cow::Borrowed(inner));

          let last_segment = box_wrapper.last_segment();

          let box_inner = last_segment.first_argument().unwrap();

          Self::Boxed(box_inner.clone())
        } else {
          Self::Option(inner.clone())
        }
      }
      "Vec" | "ProtoRepeated" => {
        let inner = last_segment.first_argument().unwrap();

        Self::Vec(inner.clone())
      }
      "HashMap" | "ProtoMap" => {
        let (key, val) = last_segment.first_two_arguments().unwrap();

        Self::Map((key.clone(), val.clone()))
      }
      _ => Self::Normal(path.clone()),
    }
  }
}

// Need:
// 1. Outer/inner type
// 2. Protobuf types
// 3. Prost attrs

pub struct TypeInfo<'a> {
  pub full_type: Cow<'a, Path>,
  pub custom_type: Option<Path>,
  pub rust_type: RustType,
  pub proto_type: ProtoTypes,
}

impl<'a> TypeInfo<'a> {
  pub fn to_prost_attr(&self, tag: i32) -> ProstAttrs {
    ProstAttrs::from_type_info(self, tag)
  }

  pub fn validator_tokens(&self, validator: &ValidatorExpr) -> TokenStream2 {
    let validator_type = match &self.rust_type {
      RustType::Option(path) => path,
      RustType::Boxed(path) => path,
      RustType::Map(_) => self.full_type.as_ref(),
      RustType::Vec(_) => self.full_type.as_ref(),
      RustType::Normal(path) => path,
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

  pub fn from_type(ty: &'a Type) -> Result<Self, Error> {
    let path = extract_type_path(ty)?;

    Ok(Self::from_path(path))
  }

  pub fn from_path(path: &'a Path) -> Self {
    let rust_type = RustType::from_path(path);
    let proto_type = ProtoTypes::from_rust_type(&rust_type).unwrap();

    Self {
      full_type: Cow::Borrowed(path),
      custom_type: None,
      rust_type,
      proto_type,
    }
  }
}

#[derive(Debug, Clone)]
pub enum ProtoTypes {
  String,
  Bool,
  Bytes,
  Enum(Path),
  Message,
  Int32,
  Map((Box<ProtoTypes>, Box<ProtoTypes>)),
}

impl ToTokens for ProtoTypes {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let output = match self {
      ProtoTypes::String => quote! { string },
      ProtoTypes::Bool => quote! { bool },
      ProtoTypes::Bytes => quote! { bytes = "bytes" },
      ProtoTypes::Enum(path) => {
        let path_as_str = path.to_token_stream().to_string();

        quote! { enumeration = #path_as_str }
      }
      ProtoTypes::Message => quote! { message },
      ProtoTypes::Int32 => quote! { int32 },
      ProtoTypes::Map((k, v)) => {
        let map_as_str = format!("{}, {}", k.to_token_stream(), v.to_token_stream());

        quote! { map = #map_as_str }
      }
    };

    tokens.extend(output)
  }
}

pub struct PathWrapper<'a> {
  pub inner: Cow<'a, Path>,
}

impl<'a> PathWrapper<'a> {
  pub fn new(path: Cow<'a, Path>) -> Self {
    Self { inner: path }
  }

  pub fn last_segment(&'_ self) -> PathSegmentWrapper<'_> {
    PathSegmentWrapper::new(Cow::Borrowed(self.inner.segments.last().unwrap()))
  }
}

pub struct PathSegmentWrapper<'a> {
  pub inner: Cow<'a, PathSegment>,
}

impl<'a> PathSegmentWrapper<'a> {
  pub fn new(segment: Cow<'a, PathSegment>) -> Self {
    Self { inner: segment }
  }

  pub fn ident(&self) -> &Ident {
    &self.inner.ident
  }

  pub fn get_arguments(&self) -> Option<impl Iterator<Item = &Path>> {
    if let PathArguments::AngleBracketed(args) = &self.inner.arguments {
      Some(args.args.iter().filter_map(|arg| {
        if let GenericArgument::Type(typ) = arg && let Type::Path(path) = typ {
        Some(&path.path)
      } else { None }
      }))
    } else {
      None
    }
  }

  pub fn first_argument(&self) -> Option<&Path> {
    self
      .get_arguments()
      .and_then(|args| args.find_or_first(|_| true))
  }

  pub fn first_two_arguments(&self) -> Option<(&Path, &Path)> {
    self.get_arguments().and_then(|args| {
      let mut first_arg: Option<&Path> = None;
      let mut second_arg: Option<&Path> = None;
      for (i, arg) in args.enumerate() {
        if i == 0 {
          first_arg = Some(arg);
        } else if i == 1 {
          second_arg = Some(arg);
          break;
        }
      }

      if let Some(first) = first_arg && let Some(second) = second_arg {
          Some((first, second))
        } else {
          None
        }
    })
  }
}

impl ProtoTypes {
  pub fn from_path(path: &Path) -> Self {
    let last_segment = PathSegmentWrapper::new(Cow::Borrowed(path.segments.last().unwrap()));
    let type_ident = last_segment.ident().to_string();

    match type_ident.as_str() {
      "String" => Self::String,
      "bool" => Self::Bool,
      "i32" => Self::Int32,
      _ => Self::Message,
    }
  }
}

fn extract_inner_type(path_segment: &PathSegment) -> Option<Path> {
  if let PathArguments::AngleBracketed(args) = &path_segment.arguments
    && let GenericArgument::Type(inner_ty) = args.args.first()? && let Type::Path(type_path) = inner_ty {
      return Some(type_path.path.clone());
    }

  None
}

pub fn extract_type_path(ty: &Type) -> Result<&Path, Error> {
  match ty {
    Type::Path(type_path) => Ok(&type_path.path),

    _ => Err(spanned_error!(ty, "Must be a type path")),
  }
}

pub fn extract_oneof_ident(ty: &Type) -> Result<Ident, Error> {
  let path = extract_type_path(ty)?;

  let path_wrapper = PathWrapper::new(Cow::Borrowed(path));
  let last_segment = path_wrapper.last_segment();

  if last_segment.ident() != "Option" {
    return Err(spanned_error!(ty, "Oneofs must be wrapped in Option"));
  }

  last_segment
    .first_argument()
    .ok_or(spanned_error!(ty, "Could not find argument to Option"))?
    .require_ident()
    .cloned()
}
