use crate::*;

#[derive(Clone)]
pub struct FieldData {
  pub span: Span,
  pub ident: Ident,
  pub type_info: TypeInfo,
  pub ident_str: String,
  pub tag: Option<ParsedNum>,
  pub validators: Validators,
  pub options: TokensOr<TokenStream2>,
  pub proto_name: String,
  pub proto_field: ProtoField,
  pub from_proto: Option<PathOrClosure>,
  pub into_proto: Option<PathOrClosure>,
  pub deprecated: bool,
  pub forwarded_attrs: Vec<Meta>,
}

impl FieldData {
  pub const fn has_custom_conversions(&self) -> bool {
    self.from_proto.is_some() && self.into_proto.is_some()
  }
}

// No point in boxing `Normal` since it's the most common path
#[allow(clippy::large_enum_variant)]
pub enum FieldDataKind {
  Ignored {
    ident: Ident,
    from_proto: Option<PathOrClosure>,
    // This is only for converting oneof variants
    // that are ignored in the proto enum
    into_proto: Option<PathOrClosure>,
  },
  Normal(FieldData),
}

impl FieldDataKind {
  pub const fn ident(&self) -> &Ident {
    match self {
      Self::Ignored { ident, .. } => ident,
      Self::Normal(field_data) => &field_data.ident,
    }
  }

  pub const fn as_normal(&self) -> Option<&FieldData> {
    if let Self::Normal(v) = self {
      Some(v)
    } else {
      None
    }
  }

  /// Returns `true` if the field data kind is [`Ignored`].
  ///
  /// [`Ignored`]: FieldDataKind::Ignored
  #[must_use]
  pub const fn is_ignored(&self) -> bool {
    matches!(self, Self::Ignored { .. })
  }
}

#[allow(clippy::needless_pass_by_value)]
pub fn process_field_data(field: FieldOrVariant) -> Result<FieldDataKind, Error> {
  let field_span = field.ident()?.span();

  let mut validators = Validators::default();
  let mut tag: Option<ParsedNum> = None;
  let mut options = TokensOr::<TokenStream2>::new(|_| quote! { [] });
  let mut name: Option<String> = None;
  let mut proto_field: Option<ProtoField> = None;
  let mut is_ignored = false;
  let mut from_proto: Option<PathOrClosure> = None;
  let mut into_proto: Option<PathOrClosure> = None;
  let mut deprecated = false;
  let mut forwarded_attrs: Vec<Meta> = Vec::new();
  let field_ident = field.ident()?.clone();
  let type_info = TypeInfo::from_type(field.get_type()?)?;

  for attr in field.attributes() {
    let ident = if let Some(ident) = attr.path().get_ident() {
      ident.to_string()
    } else {
      continue;
    };

    match ident.as_str() {
      "deprecated" => {
        deprecated = true;
      }
      "proto" => {
        attr.parse_nested_meta(|meta| {
          let ident = meta.path.require_ident()?.to_string();

          match ident.as_str() {
            "attr" => {
              forwarded_attrs = meta.parse_list::<PunctuatedItems<Meta>>()?.list;
            }
            "deprecated" => {
              let boolean = meta.parse_value::<LitBool>()?;

              deprecated = boolean.value();
            }
            "options" => {
              options.span = meta.input.span();
              options.set(meta.expr_value()?.into_token_stream());
            }
            "tag" => {
              tag = Some(meta.parse_value::<ParsedNum>()?);
            }
            "name" => {
              name = Some(meta.expr_value()?.as_string()?);
            }
            "validate" => {
              validators = meta.parse_value::<Validators>()?;
            }
            "from_proto" => {
              from_proto = Some(meta.expr_value()?.as_path_or_closure()?);
            }
            "into_proto" => {
              into_proto = Some(meta.expr_value()?.as_path_or_closure()?);
            }
            "ignore" => {
              is_ignored = true;
            }

            _ => {
              proto_field = Some(ProtoField::from_meta(&ident, &meta, &type_info)?);
            }
          };

          Ok(())
        })?;
      }
      _ => {}
    }
  }

  if is_ignored {
    return Ok(FieldDataKind::Ignored {
      from_proto,
      ident: field_ident,
      into_proto,
    });
  }

  let proto_field = if let Some(mut field) = proto_field {
    // We try to infer if a field is `Option` or `Vec` but
    // wasn't explicitely marked as optional/repeated
    if let ProtoField::Single(proto_type) = &mut field
      && (type_info.is_option() || type_info.is_vec())
    {
      let inner = std::mem::take(proto_type);

      field = if type_info.is_option() {
        ProtoField::Optional(inner)
      } else {
        ProtoField::Repeated(inner)
      };
    }

    field
  } else {
    // Field received no type information at all, we try to do some basic inference
    match type_info.type_.as_ref() {
      RustType::HashMap((k, v)) | RustType::BTreeMap((k, v)) => {
        let keys = ProtoMapKeys::from_path(&k.require_path()?)?;

        let values = ProtoType::from_primitive(&v.require_path()?)?;

        let proto_map = ProtoMap {
          keys,
          values,
          is_btree_map: type_info.is_btree_map(),
        };

        ProtoField::Map(proto_map)
      }
      RustType::Option(inner) => {
        if inner.is_box() {
          return Err(error!(
            inner,
            "You seem to be using Option<Box<T>>, but a proto type is not specified. If you are using a boxed message, mark the field as a message"
          ));
        } else {
          ProtoField::Optional(ProtoType::from_primitive(&inner.require_path()?)?)
        }
      }
      RustType::Box(inner) => {
        return Err(error!(
          inner,
          "You seem to be using Box<T>. If you meant to use a boxed message, mark it as a message"
        ));
      }
      RustType::Vec(inner) => {
        ProtoField::Repeated(ProtoType::from_primitive(&inner.require_path()?)?)
      }
      RustType::Other(inner) => ProtoField::Single(ProtoType::from_primitive(&inner.path)?),
      _ => {
        let path = type_info.as_path().ok_or_else(|| {
          error_with_span!(
            field_span,
            "Failed to infer the protobuf type. Please set it manually"
          )
        })?;

        ProtoField::Single(ProtoType::from_primitive(&path)?)
      }
    }
  };

  // These handle the default validator logic inside of them
  if !validators.has_closure_validator
    && let Some(default) = proto_field.default_validator_expr(field_span)
  {
    validators.validators.push(default);
  }

  if !validators.validators.is_empty() {
    validators.adjust_closures(&proto_field)?;
  }

  let proto_name = name.unwrap_or_else(|| {
    if field.is_variant() {
      to_snake_case(&field_ident.to_string())
    } else {
      rust_ident_to_proto_name(&field_ident)
    }
  });

  Ok(FieldDataKind::Normal(FieldData {
    validators,
    tag,
    options,
    proto_name,
    proto_field,
    from_proto,
    into_proto,
    span: field_span,
    ident_str: field_ident.to_string(),
    ident: field_ident,
    type_info,
    deprecated,
    forwarded_attrs,
  }))
}

pub fn rust_ident_to_proto_name(rust_ident: &Ident) -> String {
  let str = rust_ident.to_string();

  if let Some(escaped) = str
    .strip_prefix("r#")
    .or_else(|| str.strip_suffix("_"))
  {
    escaped.to_string()
  } else {
    str
  }
}
