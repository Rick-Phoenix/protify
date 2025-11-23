use syn::{GenericArgument, PathArguments, PathSegment};

use crate::*;

pub enum FieldType<'a> {
  Normal(&'a Path),
  Option(&'a Path),
}

impl<'a> FieldType<'a> {
  pub fn path(&self) -> &Path {
    match self {
      FieldType::Normal(path) => path,
      FieldType::Option(path) => path,
    }
  }

  pub fn is_option(&self) -> bool {
    matches!(self, Self::Option(_))
  }
}

fn extract_inner_type(path_segment: &PathSegment) -> Option<&Path> {
  if let PathArguments::AngleBracketed(args) = &path_segment.arguments
    && let GenericArgument::Type(inner_ty) = args.args.first()? && let Type::Path(type_path) = inner_ty {
      return Some(&type_path.path);
    }

  None
}

pub fn extract_type<'a>(ty: &'a Type) -> Result<FieldType<'a>, Error> {
  let field_type = match ty {
    Type::Path(type_path) => &type_path.path,

    _ => return Err(spanned_error!(ty, "Must be a type path")),
  };

  let last_segment = field_type.segments.last().unwrap();

  let extracted_type = if last_segment.ident == "Option" {
    let inner = extract_inner_type(last_segment).unwrap();

    FieldType::Option(inner)
  } else if last_segment.ident == "Box" {
    let inner = extract_inner_type(last_segment).unwrap();

    FieldType::Normal(inner)
  } else {
    FieldType::Normal(field_type)
  };

  Ok(extracted_type)
}

pub fn extract_type_from_path<'a>(ty: &'a Path) -> FieldType<'a> {
  let last_segment = ty.segments.last().unwrap();

  if last_segment.ident == "Option" {
    let inner = extract_inner_type(last_segment).unwrap();

    FieldType::Option(inner)
  } else if last_segment.ident == "Box" {
    let inner = extract_inner_type(last_segment).unwrap();

    FieldType::Normal(inner)
  } else {
    FieldType::Normal(ty)
  }
}
