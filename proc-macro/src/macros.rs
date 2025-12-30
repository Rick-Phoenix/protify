macro_rules! tokens_or_default {
  ($tokens:expr, $default:expr) => {
    $tokens
      .as_ref()
      .map_or_else(|| $default, |val| val.to_token_stream())
  };
}

macro_rules! get_ident_or_continue {
  ($path:expr) => {
    if let Some(ident) = $path.get_ident() {
      ident.to_string()
    } else {
      continue;
    }
  };
}
