macro_rules! get_ident_or_continue {
  ($path:expr) => {
    if let Some(ident) = $path.get_ident() {
      ident.to_string()
    } else {
      continue;
    }
  };
}

macro_rules! ident_string {
  ($item:expr) => {{
    let item_ident = $item.require_ident()?;
    item_ident.to_string()
  }};
}

macro_rules! error {
  ($span:expr, $error:expr) => {
    syn::Error::new($span, $error)
  };
}

macro_rules! spanned_error {
  ($expr:expr, $error:expr) => {
    syn::Error::new_spanned($expr, $error)
  };
}
