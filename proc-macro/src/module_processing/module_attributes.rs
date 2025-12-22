use crate::*;

pub struct ModuleAttrs {
  pub file: String,
  pub package: String,
  pub backend: Backend,
  pub module_path: Option<String>,
}

impl ModuleAttrs {
  pub fn as_attribute(&self) -> Attribute {
    let Self {
      package,
      file,
      backend,
      ..
    } = self;

    let backend_tokens = if *backend != Backend::default() {
      Some(quote! { , backend = #backend })
    } else {
      None
    };

    parse_quote! { #[proto(file = #file, package = #package #backend_tokens)] }
  }
}

impl Parse for ModuleAttrs {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut file: Option<String> = None;
    let mut package: Option<String> = None;
    let mut backend: Option<Backend> = None;
    let mut module_path: Option<String> = None;

    let args = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

    for arg in args {
      let ident = arg.path.require_ident()?.to_string();

      match ident.as_str() {
        "backend" => {
          backend = Some(Backend::from_expr(&arg.value)?);
        }
        "file" => {
          file = Some(arg.value.as_string()?);
        }
        "package" => {
          package = Some(arg.value.as_string()?);
        }
        "module_path" => {
          module_path = Some(arg.value.as_string()?);
        }

        _ => bail!(arg.path, "Unknown attribute `{ident}`"),
      };
    }

    let file = file.ok_or(error_call_site!("File attribute is missing"))?;
    let package = package.ok_or(error_call_site!("Package attribute is missing"))?;

    Ok(ModuleAttrs {
      file,
      package,
      backend: backend.unwrap_or_default(),
      module_path,
    })
  }
}
