use crate::*;

impl OneofCtx<'_> {
  pub fn generate_consistency_checks(&self) -> TokenStream2 {
    generate_validators_consistency_checks(
      self.proto_enum_ident(),
      &self.variants,
      self.oneof_attrs.auto_tests,
      &self.oneof_attrs.validators,
    )
  }
}
