use crate::*;

#[derive(Debug, Error)]
pub enum ConsistencyError {
  #[error("`const` cannot be used with other rules")]
  ConstWithOtherRules,
  #[error(transparent)]
  OverlappingLists(#[from] OverlappingListsError),
  #[error(transparent)]
  CelError(#[from] CelError),
  #[error("{0}")]
  ContradictoryInput(String),
  #[error("{0}")]
  WrongOneofTags(String),
}

#[derive(Debug)]
pub struct OneofErrors {
  pub oneof_name: &'static str,
  pub errors: Vec<(&'static str, Vec<ConsistencyError>)>,
}

impl core::error::Error for OneofErrors {}

impl Display for OneofErrors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let _ = writeln!(
      f,
      "❌ Validator consistency check for oneof `{}` has failed:",
      self.oneof_name.bright_yellow()
    );

    for (variant_name, errs) in &self.errors {
      let _ = writeln!(
        f,
        "  {}{}{}:",
        self.oneof_name.bright_cyan(),
        "::".bright_cyan(),
        variant_name.bright_cyan()
      );

      for err in errs {
        let _ = writeln!(f, "        - {err}");
      }
    }

    Ok(())
  }
}

pub struct MessageTestError {
  pub message_full_name: &'static str,
  pub field_errors: Vec<(&'static str, Vec<ConsistencyError>)>,
  pub cel_errors: Vec<CelError>,
}

impl Display for MessageTestError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let Self {
      message_full_name,
      field_errors,
      cel_errors,
    } = self;

    let _ = writeln!(
      f,
      "❌ Validator consistency check for message `{}` has failed:",
      message_full_name.bright_yellow()
    );

    if !field_errors.is_empty() {
      let _ = writeln!(f, "  Fields errors:");

      for (field_name, errs) in field_errors {
        let _ = writeln!(f, "    {}:", field_name.bright_yellow());

        for err in errs {
          let _ = writeln!(f, "      - {err}");
        }
      }
    }

    if !cel_errors.is_empty() {
      let _ = writeln!(f, "  CEL rules errors:");
      for err in cel_errors {
        let _ = writeln!(f, "    - {err}");
      }
    }

    Ok(())
  }
}
