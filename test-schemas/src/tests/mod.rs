mod consistency_tests;

use super::*;
use similar_asserts::assert_eq as assert_eq_pretty;

#[test]
fn test_keywords() {
  let schema = RustKeywords::proto_schema();

  let keywords = [
    "as", "break", "const", "continue", "else", "enum", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "static", "struct",
    "trait", "true", "type", "unsafe", "use", "where", "while", "abstract", "become", "box", "do",
    "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "async",
    "await",
  ];

  for (field, exp_name) in schema.fields().zip(keywords) {
    assert_eq_pretty!(field.name, exp_name);
  }
}
