use crate::*;

#[track_caller]
pub fn check_cel_programs(message_name: &str, errors: Vec<CelError>) {
  let mut error = String::new();

  writeln!(
    error,
    "❌ Testing CEL programs for message `{}` has failed:",
    message_name.bright_yellow()
  )
  .unwrap();

  for err in errors {
    writeln!(error, "  - {err}").unwrap();
  }

  panic!("{error}")
}
