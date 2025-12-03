use crate::*;

pub struct ModuleAttrs {
  pub file: String,
  pub package: String,
  pub schema_feature: Option<String>,
  pub backend: Backend,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum Backend {
  #[default]
  Prost,
  Protobuf,
}

impl Display for Backend {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Backend::Prost => write!(f, "prost"),
      Backend::Protobuf => write!(f, "protobuf"),
    }
  }
}

impl Backend {
  pub fn from_expr(expr: &Expr) -> Result<Self, Error> {
    let expr_str = extract_string_lit(expr)?;

    let output = match expr_str.as_str() {
      "prost" => Self::Prost,
      "protobuf" => Self::Protobuf,
      _ => bail!(expr, "Unknown backend value"),
    };

    Ok(output)
  }
}

impl ToTokens for Backend {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    tokens.extend(self.to_string().to_token_stream());
  }
}

impl ModuleAttrs {
  pub fn as_attribute(&self) -> Attribute {
    let Self {
      schema_feature,
      package,
      file,
      backend,
    } = self;

    let schema_feature_tokens = schema_feature
      .as_ref()
      .map(|feature| quote! { , schema_feature = #feature });

    let backend_tokens = if *backend != Backend::default() {
      Some(quote! { , backend = #backend })
    } else {
      None
    };

    parse_quote! { #[proto(file = #file, package = #package #schema_feature_tokens #backend_tokens)] }
  }
}

impl Parse for ModuleAttrs {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut file: Option<String> = None;
    let mut package: Option<String> = None;
    let mut schema_feature: Option<String> = None;
    let mut backend: Option<Backend> = None;

    let args = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

    for arg in args {
      let ident = get_ident_or_continue!(arg.path);

      match ident.as_str() {
        "backend" => {
          backend = Some(Backend::from_expr(&arg.value)?);
        }
        "file" => {
          file = Some(extract_string_lit(&arg.value)?);
        }
        "package" => {
          package = Some(extract_string_lit(&arg.value)?);
        }
        "schema_feature" => {
          schema_feature = Some(extract_string_lit(&arg.value)?);
        }
        _ => {}
      };
    }

    let file = file.ok_or(error!(Span::call_site(), "File attribute is missing"))?;
    let package = package.ok_or(error!(Span::call_site(), "Package attribute is missing"))?;

    Ok(ModuleAttrs {
      file,
      package,
      schema_feature,
      backend: backend.unwrap_or_default(),
    })
  }
}
