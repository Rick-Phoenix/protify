//! # Summary

macro_rules! doc_mod {
  ($name:ident) => {
    #[doc = include_str!(concat!(stringify!($name), ".md"))]
    pub mod $name {}
  };
}

doc_mod!(field_ref);
doc_mod!(message_ref);
doc_mod!(enum_ref);
doc_mod!(oneof_ref);
doc_mod!(service_ref);
doc_mod!(reusing_oneofs);
doc_mod!(extension_ref);
doc_mod!(server_usage);
doc_mod!(reflection_usage);
doc_mod!(validators);
doc_mod!(correctness);
doc_mod!(proxied);
doc_mod!(collecting_items);
doc_mod!(generating_files);
doc_mod!(cel_validation);
