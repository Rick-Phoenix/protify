use super::*;

mod enum_attributes;
mod enum_variant_attributes;
mod field_attributes;
mod message_attributes;
mod message_info;
mod oneof_attributes;
mod oneof_info;
mod reserved_numbers;
mod service_attributes;
mod tag_allocator;

pub use enum_attributes::*;
pub use enum_variant_attributes::*;
pub use field_attributes::*;
pub use message_attributes::*;
pub use message_info::*;
pub use oneof_attributes::*;
pub use oneof_info::*;
pub use reserved_numbers::*;
pub use service_attributes::*;
pub use tag_allocator::*;

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
    let expr_str = expr.as_string()?;

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
