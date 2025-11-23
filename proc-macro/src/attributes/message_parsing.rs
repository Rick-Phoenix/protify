use crate::*;

pub struct MessageData {
  pub tokens: StructRaw,
  pub fields: Vec<FieldData>,
  pub data: MessageAttrs,
  pub used_tags: Vec<i32>,
}

impl From<MessageData> for ItemStruct {
  fn from(value: MessageData) -> Self {
    let fields: Punctuated<Field, Token![,]> = value
      .fields
      .into_iter()
      .map(|field| field.field_raw)
      .collect();

    let fields_unnamed = FieldsUnnamed {
      unnamed: fields,
      paren_token: value.tokens.fields_paren_token,
    };

    Self {
      attrs: value.tokens.attrs,
      vis: value.tokens.vis,
      struct_token: value.tokens.struct_token,
      ident: value.tokens.ident,
      generics: value.tokens.generics,
      fields: Fields::Unnamed(fields_unnamed),
      semi_token: value.tokens.semi_token,
    }
  }
}

pub struct FieldData {
  pub field_raw: Field,
  pub data: FieldAttrs,
}

pub struct StructRaw {
  pub attrs: Vec<Attribute>,
  pub vis: Visibility,
  pub struct_token: Token![struct],
  pub fields_paren_token: Paren,
  pub ident: Ident,
  pub generics: Generics,
  pub semi_token: Option<Token![;]>,
}

pub fn parse_message(msg: ItemStruct) -> Result<MessageData, Error> {
  let ItemStruct {
    attrs,
    vis,
    struct_token,
    ident,
    generics,
    fields,
    semi_token,
  } = msg;

  let FieldsUnnamed {
    paren_token: fields_paren_token,
    unnamed: fields,
  } = if let Fields::Unnamed(fields) = fields {
    fields
  } else {
    return Err(spanned_error!(ident, "Must be a struct with named fields"));
  };

  let message_attrs = process_message_attrs(&ident, &attrs)?;

  let mut used_tags: Vec<i32> = Vec::new();
  let mut fields_data: Vec<FieldData> = Vec::new();

  for field in fields {
    let data = if let Some(field_attrs) = process_field_attrs(&ident, &attrs)? {
      field_attrs
    } else {
      continue;
    };

    if let Some(tag) = data.tag {
      used_tags.push(tag);
    }

    fields_data.push(FieldData {
      field_raw: field,
      data,
    });
  }

  Ok(MessageData {
    tokens: StructRaw {
      attrs,
      vis,
      struct_token,
      fields_paren_token,
      ident,
      generics,
      semi_token,
    },
    used_tags,
    fields: fields_data,
    data: message_attrs,
  })
}
