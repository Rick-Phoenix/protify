use super::*;

#[derive(Clone, Copy)]
pub enum ValidatorKind {
  Closure,
  ClosureOneof,
  Reflection,
  ReflectionOneof,
  Custom,
  DefaultCollection,
  DefaultMessage,
  DefaultOneof,
}

impl ValidatorKind {
  pub const fn should_check_consistency(self) -> bool {
    !self.is_default() && !matches!(self, Self::ReflectionOneof | Self::ClosureOneof)
  }

  #[must_use]
  pub const fn is_default(self) -> bool {
    matches!(
      self,
      Self::DefaultCollection | Self::DefaultOneof | Self::DefaultMessage
    )
  }

  pub const fn should_be_cached(self) -> bool {
    matches!(
      self,
      Self::Closure | Self::Reflection | Self::DefaultCollection
    )
  }

  #[must_use]
  pub const fn is_custom(self) -> bool {
    matches!(self, Self::Custom)
  }

  #[must_use]
  pub const fn is_closure(self) -> bool {
    matches!(self, Self::Closure | Self::ClosureOneof)
  }
}

#[derive(Clone)]
pub struct ValidatorTokens {
  pub expr: TokenStream2,
  pub kind: ValidatorKind,
  pub span: Span,
}

#[derive(Clone, Default)]
pub struct Validators {
  pub validators: Vec<ValidatorTokens>,
  pub has_closure_validator: bool,
}

impl Validators {
  pub fn add(&mut self, tokens: ValidatorTokens) {
    if tokens.kind.is_closure() {
      self.has_closure_validator = true;
    }

    self.validators.push(tokens);
  }
}

impl<'a> IntoIterator for &'a Validators {
  type Item = &'a ValidatorTokens;
  type IntoIter = std::slice::Iter<'a, ValidatorTokens>;

  fn into_iter(self) -> Self::IntoIter {
    self.validators.iter()
  }
}

impl Deref for Validators {
  type Target = [ValidatorTokens];

  fn deref(&self) -> &Self::Target {
    &self.validators
  }
}

impl Validators {
  pub fn merge(&mut self, other: Self) {
    self.validators.extend(other.validators);

    self.has_closure_validator &= other.has_closure_validator;
  }

  pub fn span(&self) -> Span {
    self
      .validators
      .first()
      .as_ref()
      .map_or_else(|| Span::call_site(), |v| v.span)
  }

  pub fn from_single(validator: ValidatorTokens) -> Self {
    Self {
      has_closure_validator: validator.kind.is_closure(),
      validators: vec![validator],
    }
  }
  pub fn iter(&self) -> std::slice::Iter<'_, ValidatorTokens> {
    self.validators.iter()
  }

  pub fn adjust_closures(&mut self, proto_field: &ProtoField) -> syn::Result<()> {
    for validator in &mut self.validators {
      if validator.kind.is_closure() {
        let validator_target_type = proto_field.validator_target_type(validator.span);

        validator.expr = quote_spanned! {validator.span=> <#validator_target_type as ::protify::ProtoValidation>::validator_from_closure(#validator) };

        if proto_field.is_oneof() {
          validator.kind = ValidatorKind::ClosureOneof;
        }
      }
    }

    Ok(())
  }
}

impl Parse for ValidatorTokens {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let validator: Expr = input.parse()?;

    let kind = match &validator {
      Expr::Closure(_) => ValidatorKind::Closure,
      _ => ValidatorKind::Custom,
    };

    Ok(Self {
      span: validator.span(),
      kind,
      expr: validator.into_token_stream(),
    })
  }
}

impl Parse for Validators {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut validators: Vec<ValidatorTokens> = Vec::new();
    let mut has_closure_validator = false;

    if input.peek(token::Bracket) {
      let content;
      bracketed!(content in input);

      while !content.is_empty() {
        let validator = content.parse::<ValidatorTokens>()?;

        if validator.kind.is_closure() {
          has_closure_validator = true;
        }

        validators.push(validator);

        if content.is_empty() {
          break;
        }

        let _comma: token::Comma = content.parse()?;
      }
    } else {
      let validator = input.parse::<ValidatorTokens>()?;

      if validator.kind.is_closure() {
        has_closure_validator = true;
      }

      validators.push(validator);
    }

    Ok(Self {
      validators,
      has_closure_validator,
    })
  }
}

impl ToTokens for ValidatorTokens {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.expr.to_tokens(tokens);
  }
}
